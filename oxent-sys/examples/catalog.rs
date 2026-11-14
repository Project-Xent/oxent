use core::ffi::CStr;
use oxent_sys::*;

fn main() {
    let desc = unsafe { &*xtk_catalog_control(XTK_CTRL_BUTTON) };
    let name = unsafe { CStr::from_ptr(desc.name) };
    println!("{} props={} events={}", name.to_str().unwrap(), desc.property_count, desc.event_count);
}
