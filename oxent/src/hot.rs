use crate::App;
use core::ffi::c_void;

use oxent_sys as sys;

pub const ABI: u32 = 1;
pub const SYMBOL: &str = "oxent_hot_module";
pub const TOOLCHAIN: u64 = fnv(0, env!("OXENT_RUSTC").as_bytes());
pub const RUNTIME: u64 = fnv(0, env!("OXENT_RUNTIME").as_bytes());

pub const KIND_BLOB: u32 = 0;
pub const KIND_STRUCT: u32 = 1;
pub const KIND_ENUM: u32 = 2;

pub const fn fnv(mut hash: u64, bytes: &[u8]) -> u64 {
    if hash == 0 {
        hash = 0xcbf2_9ce4_8422_2325;
    }
    let mut i = 0;
    while i < bytes.len() {
        hash = (hash ^ bytes[i] as u64).wrapping_mul(0x0100_0000_01b3);
        i += 1;
    }
    hash
}

pub const fn combine(values: &[u64]) -> u64 {
    let mut hash = fnv(0, b"");
    let mut i = 0;
    while i < values.len() {
        hash = fnv(hash, &values[i].to_le_bytes());
        i += 1;
    }
    hash
}

#[repr(C)]
pub struct FieldDesc {
    pub name: &'static str,
    pub offset: usize,
    pub payload: u64,
    pub ty: &'static TypeDesc,
}

#[repr(C)]
pub struct TypeDesc {
    pub kind: u32,
    pub size: usize,
    pub align: usize,
    pub print: u64,
    pub fields: &'static [FieldDesc],
    pub read: Option<unsafe fn(*const u8) -> u32>,
    pub write: Option<unsafe fn(*mut u8, u32)>,
    pub drop: unsafe fn(*mut u8),
}

unsafe fn drop_value<T>(p: *mut u8) {
    unsafe { core::ptr::drop_in_place(p as *mut T) }
}

const fn shape_print(name: &str, size: usize, align: usize, fields: &[FieldDesc]) -> u64 {
    let mut hash = fnv(fnv(0, name.as_bytes()), &[size as u8, (size >> 8) as u8, (size >> 16) as u8, align as u8]);
    let mut i = 0;
    while i < fields.len() {
        let f = &fields[i];
        hash = fnv(hash, f.name.as_bytes());
        hash = fnv(hash, &(f.offset as u64).to_le_bytes());
        hash = fnv(hash, &f.payload.to_le_bytes());
        hash = fnv(hash, &f.ty.print.to_le_bytes());
        i += 1;
    }
    hash
}

impl TypeDesc {
    pub const fn blob<T>(name: &str, inner: u64) -> TypeDesc {
        let size = size_of::<T>();
        let align = align_of::<T>();
        TypeDesc {
            kind: KIND_BLOB,
            size,
            align,
            print: fnv(shape_print(name, size, align, &[]), &inner.to_le_bytes()),
            fields: &[],
            read: None,
            write: None,
            drop: drop_value::<T>,
        }
    }

    pub const fn structure<T>(name: &str, fields: &'static [FieldDesc]) -> TypeDesc {
        let size = size_of::<T>();
        let align = align_of::<T>();
        TypeDesc {
            kind: KIND_STRUCT,
            size,
            align,
            print: shape_print(name, size, align, fields),
            fields,
            read: None,
            write: None,
            drop: drop_value::<T>,
        }
    }

    pub const fn opaque_enum<T>(name: &str, variants: &'static [FieldDesc]) -> TypeDesc {
        let size = size_of::<T>();
        let align = align_of::<T>();
        TypeDesc {
            kind: KIND_BLOB,
            size,
            align,
            print: shape_print(name, size, align, variants),
            fields: variants,
            read: None,
            write: None,
            drop: drop_value::<T>,
        }
    }

    pub const fn unit_enum<T>(
        name: &str,
        variants: &'static [FieldDesc],
        read: unsafe fn(*const u8) -> u32,
        write: unsafe fn(*mut u8, u32),
    ) -> TypeDesc {
        let size = size_of::<T>();
        let align = align_of::<T>();
        TypeDesc {
            kind: KIND_ENUM,
            size,
            align,
            print: shape_print(name, size, align, variants),
            fields: variants,
            read: Some(read),
            write: Some(write),
            drop: drop_value::<T>,
        }
    }
}

pub static UNIT: &TypeDesc = &TypeDesc::blob::<()>("()", 0);

/// # Safety
/// `DESC` must describe `Self` exactly; derive it with `#[derive(oxent::Model)]`.
pub unsafe trait Layout {
    const DESC: &'static TypeDesc;
}

macro_rules! leaf {
    ($($t:ty),*) => {$(
        unsafe impl Layout for $t {
            const DESC: &'static TypeDesc = &TypeDesc::blob::<$t>(stringify!($t), 0);
        }
    )*};
}

leaf!(bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, (), String, &'static str);

macro_rules! wrapper {
    ($($name:ident),*) => {$(
        unsafe impl<T: Layout> Layout for $name<T> {
            const DESC: &'static TypeDesc = &TypeDesc::blob::<$name<T>>(stringify!($name), T::DESC.print);
        }
    )*};
}

use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
wrapper!(Vec, Option, Box, VecDeque, Rc, Arc);

unsafe impl<T: Layout, const N: usize> Layout for [T; N] {
    const DESC: &'static TypeDesc = &TypeDesc::blob::<[T; N]>("array", combine(&[T::DESC.print, N as u64]));
}

unsafe impl<K: Layout, V: Layout> Layout for std::collections::HashMap<K, V> {
    const DESC: &'static TypeDesc =
        &TypeDesc::blob::<std::collections::HashMap<K, V>>("HashMap", combine(&[K::DESC.print, V::DESC.print]));
}

unsafe impl<K: Layout, V: Layout> Layout for std::collections::BTreeMap<K, V> {
    const DESC: &'static TypeDesc =
        &TypeDesc::blob::<std::collections::BTreeMap<K, V>>("BTreeMap", combine(&[K::DESC.print, V::DESC.print]));
}

unsafe impl<A: Layout, B: Layout> Layout for (A, B) {
    const DESC: &'static TypeDesc = &TypeDesc::blob::<(A, B)>("tuple2", combine(&[A::DESC.print, B::DESC.print]));
}

unsafe impl<A: Layout, B: Layout, C: Layout> Layout for (A, B, C) {
    const DESC: &'static TypeDesc =
        &TypeDesc::blob::<(A, B, C)>("tuple3", combine(&[A::DESC.print, B::DESC.print, C::DESC.print]));
}

#[repr(C)]
pub struct HostState {
    pub flux: *mut sys::FluxApp,
    pub host: *mut sys::XtkHost,
    pub model: *mut u8,
    pub rt: *mut c_void,
    pub tree: *mut c_void,
    pub running: bool,
}

#[repr(C)]
pub struct Desc {
    pub abi: u32,
    pub toolchain: u64,
    pub runtime: u64,
    pub model: &'static TypeDesc,
    pub init: unsafe extern "C" fn(*mut u8),
    pub drop_model: unsafe extern "C" fn(*mut u8),
    pub migrate: unsafe extern "C" fn(*const u8, &'static TypeDesc, *mut u8),
    pub preflight: unsafe extern "C" fn(*const u8) -> bool,
    pub drain: unsafe extern "C" fn(*mut HostState),
    pub attach: unsafe extern "C" fn(*mut HostState),
    pub detach: unsafe extern "C" fn(*mut HostState, bool),
    pub frame: unsafe extern "C" fn(*mut HostState),
}

pub trait Init<A> {
    fn init() -> A;
}

unsafe extern "C" fn init_model<A, I: Init<A>>(p: *mut u8) {
    unsafe { core::ptr::write(p as *mut A, I::init()) }
}

unsafe extern "C" fn drop_model<A>(p: *mut u8) {
    unsafe { core::ptr::drop_in_place(p as *mut A) }
}

unsafe extern "C" fn migrate_model<A: Layout>(old: *const u8, from: &'static TypeDesc, new: *mut u8) {
    let mut path = String::new();
    let mut reset = Vec::new();
    unsafe { migrate_value(old, from, new, A::DESC, &mut path, &mut reset) };
    let text = if reset.is_empty() {
        "[hot] Model layout changed; all existing fields kept\n".to_string()
    } else {
        format!("[hot] Model layout changed; reset to initial values: {}\n", reset.join(", "))
    };
    say(&text);
}

unsafe fn migrate_value(
    old: *const u8,
    od: &TypeDesc,
    new: *mut u8,
    nd: &TypeDesc,
    path: &mut String,
    reset: &mut Vec<String>,
) {
    if od.print == nd.print && od.size == nd.size {
        unsafe {
            (nd.drop)(new);
            core::ptr::copy_nonoverlapping(old, new, nd.size);
        }
        return;
    }
    if od.kind == KIND_STRUCT && nd.kind == KIND_STRUCT {
        return unsafe { migrate_struct(old, od, new, nd, path, reset) };
    }
    if od.kind == KIND_ENUM && nd.kind == KIND_ENUM && unsafe { migrate_enum(old, od, new, nd) } {
        return;
    }
    reset.push(if path.is_empty() { "Model".to_string() } else { path.clone() });
}

unsafe fn migrate_struct(
    old: *const u8,
    od: &TypeDesc,
    new: *mut u8,
    nd: &TypeDesc,
    path: &mut String,
    reset: &mut Vec<String>,
) {
    for nf in nd.fields {
        let Some(of) = od.fields.iter().find(|f| f.name == nf.name) else { continue };
        let len = path.len();
        if len > 0 {
            path.push('.');
        }
        path.push_str(nf.name);
        unsafe { migrate_value(old.add(of.offset), of.ty, new.add(nf.offset), nf.ty, path, reset) };
        path.truncate(len);
    }
}

unsafe fn migrate_enum(old: *const u8, od: &TypeDesc, new: *mut u8, nd: &TypeDesc) -> bool {
    let (Some(read), Some(write)) = (od.read, nd.write) else { return false };
    let Some(variant) = od.fields.get(unsafe { read(old) } as usize) else { return false };
    let Some(index) = nd.fields.iter().position(|v| v.name == variant.name) else { return false };
    unsafe { write(new, index as u32) };
    true
}

unsafe extern "C" fn preflight<A: App>(model: *const u8) -> bool {
    crate::reconcile::preflight::<A>(unsafe { &*(model as *const A) })
}

unsafe extern "C" fn drain<A: App>(state: *mut HostState) {
    unsafe { crate::reconcile::hot_drain::<A>(&mut *state) }
}

unsafe extern "C" fn attach<A: App>(state: *mut HostState) {
    unsafe { crate::reconcile::hot_attach::<A>(&mut *state) }
}

unsafe extern "C" fn detach<A: App>(state: *mut HostState, keep: bool) {
    unsafe { crate::reconcile::hot_detach::<A>(&mut *state, keep) }
}

unsafe extern "C" fn frame<A: App>(state: *mut HostState) {
    unsafe { crate::reconcile::hot_frame::<A>(&mut *state) }
}

pub const fn desc<A: App + Layout, I: Init<A>>() -> Desc {
    Desc {
        abi: ABI,
        toolchain: TOOLCHAIN,
        runtime: RUNTIME,
        model: A::DESC,
        init: init_model::<A, I>,
        drop_model: drop_model::<A>,
        migrate: migrate_model::<A>,
        preflight: preflight::<A>,
        drain: drain::<A>,
        attach: attach::<A>,
        detach: detach::<A>,
        frame: frame::<A>,
    }
}

#[cfg(feature = "hot")]
fn say(text: &str) {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    unsafe { host::flux_hot_print(bytes.as_ptr() as *const _) };
}

#[cfg(not(feature = "hot"))]
fn say(_text: &str) {}

#[cfg(feature = "hot")]
pub use host::run as host;

#[cfg(feature = "hot")]
mod host {
    use super::*;
    use crate::Window;
    use core::ffi::c_char;
    use core::ptr::null_mut;
    use std::alloc::{Layout as Alloc, alloc, dealloc};

    const RELOADED: i32 = 1;
    const LOAD_FAILED: i32 = 4;
    const FRAME_FAILED: i32 = 5;
    const ABI_CHANGED: i32 = 6;

    #[repr(C)]
    struct HotCfg {
        module: *const u16,
        build: *const u16,
        watch: *const u16,
        build_dir: *const u16,
        debounce_ms: u32,
        max_loaded: u32,
        manual: bool,
        on_status: Option<unsafe extern "C" fn(*mut c_void, *const c_void)>,
        user: *mut c_void,
    }

    type SwapFn = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32;

    #[repr(C)]
    struct Hooks {
        swap: Option<SwapFn>,
        retire: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> bool>,
        unload: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
        ctx: *mut c_void,
    }

    unsafe extern "C" {
        fn flux_hot_loader_open(app: *mut sys::FluxApp, cfg: *const HotCfg, hooks: *const Hooks) -> *mut c_void;
        fn flux_hot_loader_stop(loader: *mut c_void);
        fn flux_hot_loader_close(loader: *mut c_void);
        fn flux_hot_symbol(lib: *mut c_void, name: *const c_char) -> *mut c_void;
        pub(super) fn flux_hot_print(text: *const c_char);
    }

    struct Host {
        state: HostState,
        desc: Option<&'static Desc>,
    }

    fn model_layout(d: &Desc) -> Alloc {
        Alloc::from_size_align(d.model.size.max(1), d.model.align).expect("model layout")
    }

    unsafe fn model_new(d: &Desc) -> *mut u8 {
        let p = unsafe { alloc(model_layout(d)) };
        assert!(!p.is_null(), "out of memory");
        unsafe { (d.init)(p) };
        p
    }

    unsafe fn entry(lib: *mut c_void) -> Option<&'static Desc> {
        let name = format!("{SYMBOL}\0");
        let sym = unsafe { flux_hot_symbol(lib, name.as_ptr() as *const c_char) };
        if sym.is_null() {
            return None;
        }
        let f: unsafe extern "C" fn() -> *const Desc = unsafe { core::mem::transmute(sym) };
        unsafe { f().as_ref() }
    }

    fn same_model(a: &Desc, b: &Desc) -> bool {
        a.model.print == b.model.print && a.model.size == b.model.size && a.model.align == b.model.align
    }

    unsafe fn first(h: &mut Host, nd: &'static Desc) -> i32 {
        h.state.model = unsafe { model_new(nd) };
        unsafe { (nd.attach)(&mut h.state) };
        h.desc = Some(nd);
        RELOADED
    }

    unsafe fn next_model(h: &mut Host, od: &Desc, nd: &Desc) -> *mut u8 {
        if same_model(od, nd) {
            return h.state.model;
        }
        let m = unsafe { model_new(nd) };
        unsafe { (nd.migrate)(h.state.model, od.model, m) };
        m
    }

    unsafe extern "C" fn swap(ctx: *mut c_void, prev: *mut c_void, next: *mut c_void) -> i32 {
        let h = unsafe { &mut *(ctx as *mut Host) };
        let Some(nd) = (unsafe { entry(next) }) else { return LOAD_FAILED };
        if nd.abi != ABI || nd.toolchain != TOOLCHAIN {
            return ABI_CHANGED;
        }
        let od = match (prev.is_null(), h.desc) {
            (false, Some(od)) => od,
            _ => return unsafe { first(h, nd) },
        };
        unsafe { (od.drain)(&mut h.state) };
        let model = unsafe { next_model(h, od, nd) };
        let fresh = model != h.state.model;
        if !unsafe { (nd.preflight)(model) } {
            if fresh {
                unsafe { dealloc(model, model_layout(nd)) };
            }
            return FRAME_FAILED;
        }
        let keep = od.runtime == nd.runtime;
        unsafe { (od.detach)(&mut h.state, keep) };
        if fresh {
            unsafe { dealloc(h.state.model, model_layout(od)) };
            h.state.model = model;
        }
        unsafe { (nd.attach)(&mut h.state) };
        h.desc = Some(nd);
        if !keep {
            say("[hot] the oxent runtime changed; rebuilt the UI, control state was reset\n");
        }
        RELOADED
    }

    unsafe extern "C" fn frame(ctx: *mut c_void) {
        let h = unsafe { &mut *(ctx as *mut Host) };
        if let Some(d) = h.desc {
            unsafe { (d.frame)(&mut h.state) };
        }
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }

    pub fn run(window: Window<'_>, name: &str, dir: &str) -> i32 {
        let title = wide(window.title);
        let cfg = crate::reconcile::app_cfg(&window, &title);
        let flux = unsafe { sys::flux_app_create(&cfg) };
        if flux.is_null() {
            return 1;
        }
        let xhost = unsafe { sys::flux_app_attachhost(flux) };
        let code = if xhost.is_null() { 1 } else { session(flux, xhost, name, dir) };
        unsafe {
            if !xhost.is_null() {
                sys::flux_host_destroy(xhost);
            }
            sys::flux_app_destroy(flux);
        }
        code
    }

    fn session(flux: *mut sys::FluxApp, xhost: *mut sys::XtkHost, name: &str, dir: &str) -> i32 {
        let module = wide(&format!("{name}_view.dll"));
        let build =
            wide(&format!("cargo build --profile {} --features hot --example {name}_view", env!("OXENT_PROFILE")));
        let dir = wide(dir);
        let cfg = HotCfg {
            module: module.as_ptr(),
            build: build.as_ptr(),
            watch: dir.as_ptr(),
            build_dir: dir.as_ptr(),
            debounce_ms: 0,
            max_loaded: 0,
            manual: false,
            on_status: None,
            user: null_mut(),
        };
        let state =
            HostState { flux, host: xhost, model: null_mut(), rt: null_mut(), tree: null_mut(), running: false };
        let mut h = Box::new(Host { state, desc: None });
        let ctx = &mut *h as *mut Host as *mut c_void;
        let hooks = Hooks { swap: Some(swap), retire: None, unload: None, ctx };
        let loader = unsafe { flux_hot_loader_open(flux, &cfg, &hooks) };
        if loader.is_null() {
            return 1;
        }
        h.state.running = true;
        let code = unsafe {
            sys::flux_app_onframe(flux, Some(frame), ctx);
            sys::flux_app_redraw(flux);
            let code = sys::flux_app_run(flux);
            sys::flux_app_onframe(flux, None, null_mut());
            flux_hot_loader_stop(loader);
            code
        };
        if let Some(d) = h.desc {
            unsafe {
                (d.detach)(&mut h.state, false);
                (d.drop_model)(h.state.model);
                dealloc(h.state.model, model_layout(d));
            }
        }
        unsafe { flux_hot_loader_close(loader) };
        code
    }
}
