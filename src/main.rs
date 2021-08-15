#![no_main]

use libc::printf;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn register_event(
        target: *mut FlowStatics,
        cb: extern "C" fn(*mut FlowStatics, *mut c_char),
    ) -> c_int;
}

#[repr(C)]
#[derive(Default)]
struct FlowStatics {
    destroy_count: u32,
}

extern "C" fn callback(target: *mut FlowStatics, buf: *mut c_char) {
    unsafe {
        (*target).destroy_count += 1;
        printf(b"%s\n\0".as_ptr() as *const i8, buf);
    }
}

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let mut flow_statics = Box::new(FlowStatics { destroy_count: 0 });
    let ret = unsafe { register_event(&mut *flow_statics, callback) };
    return ret;
}
