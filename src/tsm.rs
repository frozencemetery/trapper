/* here be FFI gunk */
#![allow(non_camel_case_types)]

#![allow(unused_variables)]
#![allow(dead_code)]

extern crate libc;
use self::libc::*;

#[repr(C)]
pub struct tsm_screen {
    pub _hacks: u8,
}

pub type tsm_log_t = Option<unsafe extern "C"
                            fn(data: *mut c_void,
                               file: *mut c_char,
                               line: c_int,
                               func: *const c_char,
                               subs: *const c_char,
                               sev: c_uint,
                               format: *const c_char,
                               ...)>;

#[link(name = "tsm")]
extern {
    pub fn tsm_screen_new(out: *mut *mut tsm_screen,
                          log: tsm_log_t,
                          log_data: *mut c_void) -> c_int;
}
