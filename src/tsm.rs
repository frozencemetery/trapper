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

pub type tsm_log_t = Option<
        extern "C" fn(data: *mut c_void,
                      file: *mut c_char,
                      line: c_int,
                      func: *const c_char,
                      subs: *const c_char,
                      sev: c_uint,
                      format: *const c_char,
                      ...)>;

#[repr(C)]
/* #[derive(Debug, Clone, Copy)] */
pub struct tsm_screen_attr {
    pub fccode: i8,
    pub bccode: i8,
    pub fr: u8,
    pub fg: u8,
    pub fb: u8,
    pub br: u8,
    pub bg: u8,
    pub bb: u8,
    pub _bitfield: u8,
}

/* libtsm is 32-bit wide internally */
pub type tsm_symbol_t = u32;

/* There's no explanation for this one, and it isn't quite right anyway */
pub type tsm_age_t = u32;

pub type tsm_screen_draw_cb = Option<
        extern "C" fn(con: *mut tsm_screen,
                      id: u32,
                      ch: *const u32,
                      len: usize,
                      width: c_uint,
                      posx: c_uint,
                      posy: c_uint,
                      attr: *const tsm_screen_attr,
                      age: tsm_age_t,
                      data: *mut c_void) -> c_int>;

#[link(name = "tsm")]
extern {
    pub fn tsm_screen_new(out: *mut *mut tsm_screen,
                          log: tsm_log_t,
                          log_data: *mut c_void) -> c_int;
    pub fn tsm_screen_resize(con: *mut tsm_screen,
                             x: c_uint,
                             y: c_uint) -> c_int;
    pub fn tsm_screen_get_width(con: *mut tsm_screen) -> c_uint;
    pub fn tsm_screen_get_height(con: *mut tsm_screen) -> c_uint;
    pub fn tsm_screen_write(con: *mut tsm_screen,
                            ch: tsm_symbol_t,
                            attr: *const tsm_screen_attr);
    pub fn tsm_screen_draw(con: *mut tsm_screen,
                           draw_cb: tsm_screen_draw_cb,
                           data: *mut c_void) -> tsm_age_t;
}

#[repr(C)]
pub struct tsm_vte {
    pub _hacks: u8,
}

pub type tsm_vte_write_cb = Option<
        extern "C" fn(vte: *mut tsm_vte,
                      u8: *const c_char,
                      len: isize,
                      data: *mut c_void)>;

#[link(name = "tsm")]
extern {
    pub fn tsm_vte_new(out: *mut *mut tsm_vte,
                       con: *mut tsm_screen,
                       write_cb: tsm_vte_write_cb,
                       data: *mut c_void,
                       log: tsm_log_t,
                       log_data: *mut c_void) -> c_int;
    pub fn tsm_vte_input(vte: *mut tsm_vte,
                         u8: *const c_char,
                         len: isize);
}
