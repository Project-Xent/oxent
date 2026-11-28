use crate::{Value, ffi};
use core::mem::MaybeUninit;
use oxent_sys as sys;

const ZERO_STAMP: sys::XtkChangeStamp = sys::XtkChangeStamp { source: 0, revision: 0 };

pub struct Channel {
    raw: *mut sys::XtkChannel,
}

impl Channel {
    pub fn from_scene(scene: sys::XtkScene) -> Option<Self> {
        let raw = unsafe { sys::xtk_channel_create(scene) };
        if raw.is_null() { None } else { Some(Self { raw }) }
    }

    pub unsafe fn from_host(host: *mut sys::XtkHost) -> Option<Self> {
        let scene = unsafe { sys::xtk_scene_fromhost(host) };
        Self::from_scene(scene)
    }

    pub fn pending(&self) -> u32 {
        unsafe { sys::xtk_channel_pending(self.raw) }
    }

    pub fn clear(&mut self) {
        unsafe { sys::xtk_channel_clear(self.raw) };
    }

    pub fn set(&mut self, target: sys::XtkCtrl, prop: u32, value: &Value) -> bool {
        let v = ffi::value(value);
        unsafe { sys::xtk_channel_set(self.raw, target, prop, v, ZERO_STAMP) }
    }

    pub fn set_bool(&mut self, target: sys::XtkCtrl, prop: u32, value: bool) -> bool {
        unsafe { sys::xtk_channel_set_bool(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_i32(&mut self, target: sys::XtkCtrl, prop: u32, value: i32) -> bool {
        unsafe { sys::xtk_channel_set_i32(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_f32(&mut self, target: sys::XtkCtrl, prop: u32, value: f32) -> bool {
        unsafe { sys::xtk_channel_set_f32(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_f64(&mut self, target: sys::XtkCtrl, prop: u32, value: f64) -> bool {
        unsafe { sys::xtk_channel_set_f64(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_enum(&mut self, target: sys::XtkCtrl, prop: u32, value: u32) -> bool {
        unsafe { sys::xtk_channel_set_enum(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_color(&mut self, target: sys::XtkCtrl, prop: u32, rgba: u32) -> bool {
        unsafe { sys::xtk_channel_set_color(self.raw, target, prop, sys::XtkColor { rgba }, ZERO_STAMP) }
    }

    pub fn set_resource(&mut self, target: sys::XtkCtrl, prop: u32, value: sys::XtkResource) -> bool {
        unsafe { sys::xtk_channel_set_resource(self.raw, target, prop, value, ZERO_STAMP) }
    }

    pub fn set_str(&mut self, target: sys::XtkCtrl, prop: u32, s: &str) -> bool {
        unsafe {
            sys::xtk_channel_set_string(self.raw, target, prop, s.as_ptr() as *const _, s.len() as u32, ZERO_STAMP)
        }
    }

    pub fn commit(&mut self) -> Result<(), sys::XtkBatchDiag> {
        let mut diag = MaybeUninit::<sys::XtkBatchDiag>::zeroed();
        let ok = unsafe { sys::xtk_channel_commit(self.raw, diag.as_mut_ptr()) };
        if ok { Ok(()) } else { Err(unsafe { diag.assume_init() }) }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { sys::xtk_channel_destroy(self.raw) };
        }
    }
}
