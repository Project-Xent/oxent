use crate::hot::HostState;
use crate::{App, El, ElData, Handler, Theme, Ui, Value, Window, ffi};
use core::cell::{Cell, RefCell};
use core::ffi::c_void;
use core::ptr::null_mut;
use oxent_sys as sys;
use std::collections::{HashMap, VecDeque};

const ZERO_STAMP: sys::XtkChangeStamp = sys::XtkChangeStamp { source: 0, revision: 0 };

struct Shared<A: App> {
    queue: RefCell<VecDeque<A::Msg>>,
    app: Cell<*mut sys::FluxApp>,
    dirty: Cell<bool>,
}

impl<A: App> Shared<A> {
    fn wake(&self) {
        let app = self.app.get();
        if !app.is_null() {
            unsafe { sys::flux_app_redraw(app) };
        }
    }
}

#[repr(C)]
struct NodeState<A: App> {
    shared: *const Shared<A>,
    handlers: RefCell<Vec<(u32, Handler<A::Msg>)>>,
    window: Cell<Option<(i32, i32, i32)>>,
    echo: Cell<sys::XtkChangeStamp>,
    user_events: Cell<u32>,
}

enum Sub {
    Pending(sys::XtkSubRef),
    Live(sys::XtkSubId),
}

#[repr(C)]
struct Node<A: App> {
    ctrl: sys::XtkCtrl,
    local: Option<sys::XtkLocalId>,
    ty: u32,
    key: Option<u64>,
    props: Vec<(u32, Value)>,
    state: Box<NodeState<A>>,
    subs: Vec<(u32, Sub)>,
    kids: Vec<Node<A>>,
}

impl<A: App> Node<A> {
    fn target(&self) -> sys::XtkNodeRef {
        match self.local {
            Some(id) => ffi::local(id),
            None => ffi::existing(self.ctrl),
        }
    }
}

unsafe extern "C" fn trampoline<A: App>(arg: *mut c_void, event: *const sys::XtkEvent) -> u32 {
    let state = unsafe { &*(arg as *const NodeState<A>) };
    let event = unsafe { &*event };
    let shared = unsafe { &*state.shared };
    if event.id == sys::XTK_EVENT_REALIZE {
        let w = unsafe { event.payload.u.window };
        state.window.set(Some((w.first, w.last, w.lanes)));
        shared.dirty.set(true);
        shared.wake();
        return sys::XTK_EVENT_HANDLED;
    }
    if event.change.revision > state.echo.get().revision {
        state.echo.set(event.change);
    }
    if event.cause == sys::XTK_EVENT_CAUSE_USER && event.id < 32 {
        state.user_events.set(state.user_events.get() | 1 << event.id);
    }
    let Ok(handlers) = state.handlers.try_borrow() else { return sys::XTK_EVENT_CONTINUE };
    let Some((_, handler)) = handlers.iter().find(|(id, _)| *id == event.id) else { return sys::XTK_EVENT_CONTINUE };
    let msg = unsafe { ffi::msg(handler, &event.payload) };
    drop(handlers);
    shared.queue.borrow_mut().push_back(msg);
    shared.wake();
    sys::XTK_EVENT_HANDLED
}

struct Recon<'r, 'ui, A: App> {
    ui: &'ui Ui<'ui, A>,
    txn: &'r mut sys::XtkTxn,
    shared: *const Shared<A>,
    graveyard: Vec<Node<A>>,
    resub: bool,
}

fn prop_f32(props: &[(u32, Value)], id: u32, fallback: f32) -> f32 {
    match props.iter().find(|(p, _)| *p == id) {
        Some((_, Value::F32(v))) if *v > 0.0 => *v,
        _ => fallback,
    }
}

fn clamp_window(first: i32, last: i32, lanes: i32, count: i32) -> (i32, i32) {
    if last < count {
        return (first, last);
    }
    let span = last - first;
    let last = count - 1;
    let first = (last - span).max(0);
    (first - first % lanes.max(1), last)
}

impl<'r, 'ui, A: App> Recon<'r, 'ui, A> {
    fn take(&self, id: usize) -> ElData<A> {
        let mut els = self.ui.els.borrow_mut();
        let d = &mut els[id];
        ElData {
            ty: d.ty,
            key: d.key,
            props: core::mem::take(&mut d.props),
            handlers: core::mem::take(&mut d.handlers),
            kids: core::mem::take(&mut d.kids),
            items: d.items.take(),
        }
    }

    fn realize(&self, data: &mut ElData<A>, window: Option<(i32, i32, i32)>) {
        let Some((count, item)) = data.items else { return };
        let count = count.min(i32::MAX as usize) as i32;
        let (first, last, lanes) = window.unwrap_or_else(|| {
            let rows = (prop_f32(&data.props, sys::XTK_PROP_HEIGHT, 600.0)
                / prop_f32(&data.props, sys::XTK_PROP_ITEM_HEIGHT, 40.0))
            .ceil() as i32;
            (0, rows + 3, 1)
        });
        let (first, last) = clamp_window(first, last, lanes, count);
        upsert(&mut data.props, sys::XTK_PROP_REALIZED_FIRST, Value::I32(first.max(0)));
        let span = (last - first + 1).max(1) as usize;
        for i in first.max(0)..=last {
            let row = self.ui.el(sys::XTK_CTRL_LISTITEM, &raw const sys::flux_ctrl_listitem);
            row.key((i as usize % span) as u64);
            row.child(item(self.ui, i as usize));
            data.kids.push(row.id);
        }
    }

    fn subscribe(&mut self, target: sys::XtkNodeRef, event: u32, state: &NodeState<A>) -> Sub {
        let arg = state as *const NodeState<A> as *mut c_void;
        let token = unsafe {
            sys::xtk_txn_on(self.txn, target, event, sys::XTK_EVENT_PHASE_MASK_TARGET, Some(trampoline::<A>), arg)
        };
        assert_ne!(token, u32::MAX, "subscribe event {event} on control");
        Sub::Pending(token)
    }

    fn wanted(data: &ElData<A>) -> Vec<u32> {
        let mut events: Vec<u32> = data.handlers.iter().map(|(e, _)| *e).collect();
        if data.items.is_some() {
            events.push(sys::XTK_EVENT_REALIZE);
        }
        events
    }

    fn create(&mut self, id: usize, parent: sys::XtkNodeRef, before: sys::XtkNodeRef) -> Node<A> {
        let mut data = self.take(id);
        self.realize(&mut data, None);
        let patches: Vec<sys::XtkPropPatch> = data
            .props
            .iter()
            .map(|(id, v)| sys::XtkPropPatch { id: *id, value: ffi::value(v), change: ZERO_STAMP })
            .collect();
        let local =
            unsafe { sys::xtk_txn_create(self.txn, data.ty, parent, before, patches.as_ptr(), patches.len() as u32) };
        assert_ne!(local, u32::MAX, "create control type {}", data.ty);
        let wanted = Self::wanted(&data);
        let state = Box::new(NodeState {
            shared: self.shared,
            handlers: RefCell::new(core::mem::take(&mut data.handlers)),
            window: Cell::new(None),
            echo: Cell::new(ZERO_STAMP),
            user_events: Cell::new(0),
        });
        let subs = wanted.into_iter().map(|e| (e, self.subscribe(ffi::local(local), e, &state))).collect();
        let kids = data.kids.iter().map(|&k| self.create(k, ffi::local(local), ffi::none())).collect();
        Node { ctrl: 0, local: Some(local), ty: data.ty, key: data.key, props: data.props, state, subs, kids }
    }

    fn forced(node: &Node<A>) -> Vec<u32> {
        let mask = node.state.user_events.replace(0);
        let mut ids = Vec::new();
        for event in 1..32u32 {
            if mask & (1 << event) == 0 {
                continue;
            }
            let mut out = [0u32; sys::XTK_MIRROR_MAX as usize];
            let n = unsafe { sys::xtk_catalog_mirrors(node.ty, event, out.as_mut_ptr()) };
            ids.extend_from_slice(&out[..n as usize]);
        }
        ids
    }

    fn diff_props(&mut self, node: &Node<A>, next: &[(u32, Value)]) {
        let echo = node.state.echo.get();
        let forced = Self::forced(node);
        for (id, v) in next {
            let unchanged = node.props.iter().any(|(p, old)| p == id && old.same(v));
            if unchanged && !forced.contains(id) {
                continue;
            }
            self.set(node.ctrl, *id, v, echo);
        }
        for (id, old) in &node.props {
            if next.iter().all(|(p, _)| p != id) {
                self.set(node.ctrl, *id, &old.reset(*id), ZERO_STAMP);
            }
        }
    }

    fn set(&mut self, ctrl: sys::XtkCtrl, id: u32, v: &Value, stamp: sys::XtkChangeStamp) {
        let ok = unsafe { sys::xtk_txn_set(self.txn, ffi::existing(ctrl), id, ffi::value(v), stamp) };
        assert!(ok, "set property {id}");
    }

    fn diff_subs(&mut self, node: &mut Node<A>, wanted: Vec<u32>) {
        let txn = &mut *self.txn;
        let resub = self.resub;
        node.subs.retain(|(event, sub)| {
            let keep = !resub && wanted.contains(event);
            if let (false, Sub::Live(id)) = (keep, sub) {
                unsafe { sys::xtk_txn_off(txn, *id) };
            }
            keep
        });
        for event in wanted {
            if node.subs.iter().all(|(e, _)| *e != event) {
                let sub = self.subscribe(ffi::existing(node.ctrl), event, &node.state);
                node.subs.push((event, sub));
            }
        }
    }

    fn update(&mut self, mut node: Node<A>, id: usize) -> Node<A> {
        let mut data = self.take(id);
        self.realize(&mut data, node.state.window.get());
        self.diff_props(&node, &data.props);
        let wanted = Self::wanted(&data);
        node.state.handlers.replace(core::mem::take(&mut data.handlers));
        self.diff_subs(&mut node, wanted);
        let old = core::mem::take(&mut node.kids);
        node.kids = self.kids(old, &data.kids, ffi::existing(node.ctrl));
        node.props = data.props;
        node.key = data.key;
        node
    }

    fn matches(&self, old: &[Node<A>], new: &[usize]) -> Vec<Option<usize>> {
        let els = self.ui.els.borrow();
        let keyed: HashMap<(u64, u32), usize> =
            old.iter().enumerate().filter_map(|(i, n)| n.key.map(|k| ((k, n.ty), i))).collect();
        let mut unkeyed = old.iter().enumerate().filter(|(_, n)| n.key.is_none()).map(|(i, _)| i);
        new.iter()
            .map(|&id| {
                let d = &els[id];
                match d.key {
                    Some(k) => keyed.get(&(k, d.ty)).copied(),
                    None => unkeyed.next().filter(|&i| old[i].ty == d.ty),
                }
            })
            .collect()
    }

    fn kids(&mut self, old: Vec<Node<A>>, new: &[usize], parent: sys::XtkNodeRef) -> Vec<Node<A>> {
        let assign = self.matches(&old, new);
        let mut slots: Vec<Option<Node<A>>> = old.into_iter().map(Some).collect();
        let mut out = Vec::with_capacity(new.len());
        let mut next = ffi::none();
        let mut min_after = usize::MAX;
        for (i, &id) in new.iter().enumerate().rev() {
            let node = match assign[i].and_then(|oi| slots[oi].take().map(|n| (oi, n))) {
                Some((oi, node)) => {
                    let node = self.update(node, id);
                    if oi < min_after {
                        min_after = oi;
                    } else {
                        unsafe { sys::xtk_txn_move(self.txn, parent, node.target(), next) };
                    }
                    node
                }
                None => self.create(id, parent, next),
            };
            next = node.target();
            out.push(node);
        }
        for node in slots.into_iter().flatten() {
            unsafe { sys::xtk_txn_drop(self.txn, ffi::existing(node.ctrl)) };
            self.graveyard.push(node);
        }
        out.reverse();
        out
    }
}

fn upsert(props: &mut Vec<(u32, Value)>, id: u32, value: Value) {
    match props.iter_mut().find(|(p, _)| *p == id) {
        Some(slot) => slot.1 = value,
        None => props.push((id, value)),
    }
}

fn resolve<A: App>(node: &mut Node<A>, result: &sys::XtkBatchResult) {
    if let Some(local) = node.local.take() {
        node.ctrl = unsafe { (*result.controls.add(local as usize)).control };
    }
    for (_, sub) in node.subs.iter_mut() {
        if let Sub::Pending(token) = *sub {
            *sub = Sub::Live(unsafe { (*result.subscriptions.add(token as usize)).id });
        }
    }
    for kid in node.kids.iter_mut() {
        resolve(kid, result);
    }
}

struct Runtime<A: App> {
    app: *mut A,
    shared: Box<Shared<A>>,
    scene: sys::XtkScene,
    root: sys::XtkCtrl,
    tree: Option<Node<A>>,
    resub: bool,
}

impl<A: App> Runtime<A> {
    fn drain(&mut self) -> bool {
        let mut any = false;
        loop {
            let msg = self.shared.queue.borrow_mut().pop_front();
            let Some(msg) = msg else { return any };
            unsafe { (*self.app).update(msg) };
            any = true;
        }
    }

    fn frame(&mut self) {
        let drained = self.drain();
        if !drained && !self.shared.dirty.replace(false) && self.tree.is_some() {
            return;
        }
        let Runtime { app, shared, scene, root, tree, resub } = self;
        let app = unsafe { &**app };
        let ui = Ui::new(app);
        let top: El<'_, A> = app.view(&ui);
        let mut txn = sys::XtkTxn { scene: null_mut(), batch: null_mut(), borrowed: false };
        assert!(unsafe { sys::xtk_txn_begin(scene, &mut txn) }, "begin transaction");
        let old = tree.take().map(|n| vec![n]).unwrap_or_default();
        let (mut next, graveyard) = {
            let mut recon = Recon { ui: &ui, txn: &mut txn, shared: &**shared, graveyard: Vec::new(), resub: *resub };
            let next = recon.kids(old, &[top.id], ffi::existing(*root));
            (next, recon.graveyard)
        };
        let mut diag = sys::XtkBatchDiag { error: 0, failed_op: 0, stage: 0 };
        let result = unsafe { sys::xtk_txn_commit(&mut txn, &mut diag) };
        assert!(!result.is_null(), "commit failed: stage {} error {} op {}", diag.stage, diag.error, diag.failed_op);
        let mut node = next.pop().expect("view produced a root");
        resolve(&mut node, unsafe { &*result });
        unsafe { sys::xtk_batchresult_destroy(result) };
        *tree = Some(node);
        *resub = false;
        drop(graveyard);
    }

    fn new(app: *mut A, flux: *mut sys::FluxApp, host: *mut sys::XtkHost) -> Box<Self> {
        let shared =
            Box::new(Shared { queue: RefCell::new(VecDeque::new()), app: Cell::new(flux), dirty: Cell::new(true) });
        let scene = unsafe { sys::xtk_scene_fromhost(host) };
        let mut rt = Box::new(Runtime { app, shared, scene, root: 0, tree: None, resub: false });
        rt.root = unsafe { sys::xtk_scene_root(&rt.scene) };
        rt
    }

    fn drop_tree(&mut self, tree: &Node<A>) {
        let mut txn = sys::XtkTxn { scene: null_mut(), batch: null_mut(), borrowed: false };
        if !unsafe { sys::xtk_txn_begin(&mut self.scene, &mut txn) } {
            return;
        }
        let result = unsafe {
            if !sys::xtk_txn_drop(&mut txn, ffi::existing(tree.ctrl)) {
                sys::xtk_txn_abort(&mut txn);
                return;
            }
            sys::xtk_txn_commit(&mut txn, null_mut())
        };
        if result.is_null() {
            unsafe { sys::xtk_txn_abort(&mut txn) };
        } else {
            unsafe { sys::xtk_batchresult_destroy(result) };
        }
    }
}

fn adopt<A: App>(node: &mut Node<A>, shared: *const Shared<A>) {
    node.state.shared = shared;
    core::mem::forget(node.state.handlers.take());
    for kid in node.kids.iter_mut() {
        adopt(kid, shared);
    }
}

pub(crate) fn preflight<A: App>(app: &A) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ui = Ui::new(app);
        let _ = app.view(&ui);
    }))
    .is_ok()
}

pub(crate) unsafe fn hot_drain<A: App>(state: &mut HostState) {
    let rt = unsafe { &mut *(state.rt as *mut Runtime<A>) };
    rt.drain();
}

pub(crate) unsafe fn hot_attach<A: App>(state: &mut HostState) {
    let mut rt = Runtime::<A>::new(state.model as *mut A, state.flux, state.host);
    if !state.tree.is_null() {
        let mut tree = *unsafe { Box::from_raw(state.tree as *mut Node<A>) };
        adopt(&mut tree, &*rt.shared);
        rt.tree = Some(tree);
        rt.resub = true;
        state.tree = null_mut();
    }
    if state.running {
        rt.frame();
    } else {
        rt.shared.wake();
    }
    state.rt = Box::into_raw(rt) as *mut c_void;
}

pub(crate) unsafe fn hot_detach<A: App>(state: &mut HostState, keep: bool) {
    let mut rt = unsafe { Box::from_raw(state.rt as *mut Runtime<A>) };
    state.rt = null_mut();
    rt.drain();
    let Some(tree) = rt.tree.take() else { return };
    if keep {
        state.tree = Box::into_raw(Box::new(tree)) as *mut c_void;
    } else {
        rt.drop_tree(&tree);
    }
}

pub(crate) unsafe fn hot_frame<A: App>(state: &mut HostState) {
    let rt = unsafe { &mut *(state.rt as *mut Runtime<A>) };
    rt.frame();
}

unsafe extern "C" fn frame<A: App>(ctx: *mut c_void) {
    let rt = unsafe { &mut *(ctx as *mut Runtime<A>) };
    rt.frame();
}

pub(crate) fn app_cfg(window: &Window<'_>, title: &[u16]) -> sys::FluxAppCfg {
    sys::FluxAppCfg {
        title: title.as_ptr(),
        width: window.width,
        height: window.height,
        theme: match window.theme {
            Theme::System => sys::FLUX_THEME_SYSTEM,
            Theme::Light => sys::FLUX_THEME_LIGHT,
            Theme::Dark => sys::FLUX_THEME_DARK,
        },
        backdrop: sys::FLUX_BACKDROP_MICA,
        on_start: None,
        on_stop: None,
        user: null_mut(),
    }
}

pub(crate) fn run<A: App>(window: Window<'_>, app: A) -> i32 {
    let title: Vec<u16> = window.title.encode_utf16().chain(Some(0)).collect();
    let cfg = app_cfg(&window, &title);
    let flux = unsafe { sys::flux_app_create(&cfg) };
    if flux.is_null() {
        return 1;
    }
    let host = unsafe { sys::flux_app_attachhost(flux) };
    if host.is_null() {
        unsafe { sys::flux_app_destroy(flux) };
        return 1;
    }
    let model = Box::into_raw(Box::new(app));
    let mut rt = Runtime::new(model, flux, host);
    let ctx = &mut *rt as *mut Runtime<A> as *mut c_void;
    let code = unsafe {
        sys::flux_app_onframe(flux, Some(frame::<A>), ctx);
        sys::flux_app_redraw(flux);
        let code = sys::flux_app_run(flux);
        sys::flux_app_onframe(flux, None, null_mut());
        code
    };
    rt.shared.app.set(null_mut());
    unsafe {
        sys::flux_host_destroy(host);
        sys::flux_app_destroy(flux);
    }
    drop(rt);
    drop(unsafe { Box::from_raw(model) });
    code
}
