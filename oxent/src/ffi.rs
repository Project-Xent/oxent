use crate::{Handler, ScrollPos, Value};
use oxent_sys as sys;

pub(crate) fn existing(ctrl: sys::XtkCtrl) -> sys::XtkNodeRef {
    sys::XtkNodeRef { kind: sys::XTK_REF_EXISTING, u: sys::XtkNodeRef__bindgen_ty_1 { existing: ctrl } }
}

pub(crate) fn local(id: sys::XtkLocalId) -> sys::XtkNodeRef {
    sys::XtkNodeRef { kind: sys::XTK_REF_LOCAL, u: sys::XtkNodeRef__bindgen_ty_1 { local: id } }
}

pub(crate) fn none() -> sys::XtkNodeRef {
    sys::XtkNodeRef { kind: sys::XTK_REF_NONE, u: sys::XtkNodeRef__bindgen_ty_1 { existing: 0 } }
}

pub(crate) fn value(v: &Value) -> sys::XtkValue {
    use sys::XtkValue__bindgen_ty_1 as U;
    let (kind, u) = match v {
        Value::Bool(b) => (sys::XTK_VALUE_BOOL, U { b: *b }),
        Value::I32(i) => (sys::XTK_VALUE_I32, U { i32_: *i }),
        Value::F32(f) => (sys::XTK_VALUE_F32, U { f32_: *f }),
        Value::F64(f) => (sys::XTK_VALUE_F64, U { f64_: *f }),
        Value::Enum(e) => (sys::XTK_VALUE_ENUM, U { u32_: *e }),
        Value::Color(c) => (sys::XTK_VALUE_COLOR, U { color: sys::XtkColor { rgba: *c } }),
        Value::Resource(r) => (sys::XTK_VALUE_RESOURCE, U { resource: *r }),
        Value::Str(s) => (
            sys::XTK_VALUE_STRING,
            U { string: sys::XtkValue__bindgen_ty_1__bindgen_ty_1 { data: s.as_ptr() as *mut _, len: s.len() as u32 } },
        ),
    };
    sys::XtkValue { kind, u }
}

unsafe fn text(data: *const core::ffi::c_char, len: u32) -> String {
    if data.is_null() || len == 0 {
        return String::new();
    }
    let bytes = unsafe { core::slice::from_raw_parts(data as *const u8, len as usize) };
    String::from_utf8_lossy(bytes).into_owned()
}

pub(crate) unsafe fn msg<M: Clone>(handler: &Handler<M>, p: &sys::XtkEventPayload) -> M {
    let u = &p.u;
    unsafe {
        match handler {
            Handler::Value(m) => m.clone(),
            Handler::Bool(f) => f(u.b),
            Handler::I32(f) => f(u.i32_),
            Handler::F32(f) => f(u.f32_),
            Handler::F64(f) => f(u.f64_),
            Handler::Str(f) => f(text(u.string.data, u.string.len)),
            Handler::Query(f) => f(text(u.query.text, u.query.len), u.query.index),
            Handler::Rating(f) => f(u.rating.value, u.rating.count),
            Handler::IndexBool(f) => f(u.index_bool.index, u.index_bool.value),
            Handler::Range(f) => f(u.range.start, u.range.end),
            Handler::Scroll(f) => {
                let s = u.scroll;
                f(ScrollPos { x: s.x, y: s.y, max_x: s.max_x, max_y: s.max_y })
            }
        }
    }
}
