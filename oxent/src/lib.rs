mod controls;
mod ffi;
mod reconcile;

pub use controls::*;
pub use oxent_sys as sys;

use core::cell::RefCell;
use core::marker::PhantomData;

pub trait App: Sized + 'static {
    type Msg: Clone + 'static;
    fn update(&mut self, msg: Self::Msg);
    fn view<'ui>(&'ui self, ui: &'ui Ui<'ui, Self>) -> El<'ui, Self>;
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    I32(i32),
    F32(f32),
    F64(f64),
    Enum(u32),
    Color(u32),
    Str(String),
    Resource(u64),
}

impl Value {
    fn reset(&self, id: u32) -> Value {
        match self {
            Value::F32(_) if id == sys::XTK_PROP_WIDTH || id == sys::XTK_PROP_HEIGHT => Value::F32(f32::NAN),
            Value::Bool(_) => Value::Bool(id == sys::XTK_PROP_ENABLED),
            Value::I32(_) => Value::I32(0),
            Value::F32(_) => Value::F32(0.0),
            Value::F64(_) => Value::F64(0.0),
            Value::Enum(_) => Value::Enum(0),
            Value::Color(_) => Value::Color(0),
            Value::Str(_) => Value::Str(String::new()),
            Value::Resource(_) => Value::Resource(0),
        }
    }

    fn same(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::F32(a), Value::F32(b)) => a.to_bits() == b.to_bits(),
            (Value::F64(a), Value::F64(b)) => a.to_bits() == b.to_bits(),
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::I32(a), Value::I32(b)) => a == b,
            (Value::Enum(a), Value::Enum(b)) | (Value::Color(a), Value::Color(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Resource(a), Value::Resource(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollPos {
    pub x: f32,
    pub y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl ScrollPos {
    pub fn at_end(&self) -> bool {
        self.y >= self.max_y - 1.0
    }
}

pub enum Handler<M> {
    Value(M),
    Bool(fn(bool) -> M),
    I32(fn(i32) -> M),
    F32(fn(f32) -> M),
    F64(fn(f64) -> M),
    Str(fn(String) -> M),
    Query(fn(String, i32) -> M),
    Rating(fn(f64, i32) -> M),
    IndexBool(fn(i32, bool) -> M),
    Range(fn(f32, f32) -> M),
    Scroll(fn(ScrollPos) -> M),
}

pub type ItemFn<A> = for<'a> fn(&'a Ui<'a, A>, usize) -> El<'a, A>;

struct ElData<A: App> {
    ty: u32,
    key: Option<u64>,
    props: Vec<(u32, Value)>,
    handlers: Vec<(u32, Handler<A::Msg>)>,
    kids: Vec<usize>,
    items: Option<(usize, ItemFn<A>)>,
}

pub struct Ui<'ui, A: App> {
    model: &'ui A,
    els: RefCell<Vec<ElData<A>>>,
}

pub struct El<'ui, A: App> {
    ui: &'ui Ui<'ui, A>,
    id: usize,
    _own: PhantomData<&'ui A>,
}

impl<A: App> Clone for El<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: App> Copy for El<'_, A> {}

impl<'ui, A: App> Ui<'ui, A> {
    fn new(model: &'ui A) -> Self {
        Ui { model, els: RefCell::new(Vec::new()) }
    }

    pub fn model(&self) -> &'ui A {
        self.model
    }

    pub(crate) fn el(&'ui self, ty: u32, module: *const sys::FluxControlModule) -> El<'ui, A> {
        unsafe { sys::flux_controls_use(module) };
        let mut els = self.els.borrow_mut();
        els.push(ElData { ty, key: None, props: Vec::new(), handlers: Vec::new(), kids: Vec::new(), items: None });
        El { ui: self, id: els.len() - 1, _own: PhantomData }
    }

    fn with<R>(&self, id: usize, f: impl FnOnce(&mut ElData<A>) -> R) -> R {
        f(&mut self.els.borrow_mut()[id])
    }
}

impl<'ui, A: App> El<'ui, A> {
    pub(crate) fn prop(self, id: u32, value: Value) {
        self.ui.with(self.id, |d| match d.props.iter_mut().find(|(p, _)| *p == id) {
            Some(slot) => slot.1 = value,
            None => d.props.push((id, value)),
        });
    }

    pub(crate) fn on(self, event: u32, handler: Handler<A::Msg>) {
        self.ui.with(self.id, |d| {
            d.handlers.retain(|(e, _)| *e != event);
            d.handlers.push((event, handler));
        });
    }

    pub(crate) fn key(self, key: u64) {
        self.ui.with(self.id, |d| d.key = Some(key));
    }

    pub(crate) fn child(self, child: El<'ui, A>) {
        self.ui.with(self.id, |d| d.kids.push(child.id));
    }

    fn items(self, count: usize, item: ItemFn<A>) {
        self.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(count.min(i32::MAX as usize) as i32));
        self.ui.with(self.id, |d| d.items = Some((count, item)));
    }
}

macro_rules! virtualized {
    ($($name:ident),*) => {$(
        impl<'ui, A: App> $name<'ui, A> {
            pub fn items(self, count: usize, item: ItemFn<A>) -> Self {
                self.0.items(count, item);
                self
            }
        }
    )*};
}

virtualized!(List, Listbox, Gridview, Repeater, Itemsview);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

pub struct Window<'a> {
    pub title: &'a str,
    pub width: i32,
    pub height: i32,
    pub theme: Theme,
}

impl Default for Window<'_> {
    fn default() -> Self {
        Window { title: "oxent", width: 960, height: 640, theme: Theme::System }
    }
}

pub fn run<A: App>(window: Window<'_>, app: A) -> i32 {
    reconcile::run(window, app)
}
