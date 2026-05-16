#![allow(missing_docs)]

use core::ffi::{c_char, c_double, c_int, c_void};

extern "C" {
    pub fn mp_artwork_new_from_path(path: *const c_char) -> *mut c_void;
    pub fn mp_artwork_new_from_path_with_size(
        path: *const c_char,
        width: c_double,
        height: c_double,
    ) -> *mut c_void;
    pub fn mp_artwork_copy_bounds(
        artwork: *mut c_void,
        origin_x: *mut c_double,
        origin_y: *mut c_double,
        width: *mut c_double,
        height: *mut c_double,
    ) -> c_int;
    pub fn mp_artwork_release(artwork: *mut c_void);
}
