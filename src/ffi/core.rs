#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mp_string_free(s: *mut c_char);
    pub fn mp_object_retain(ptr: *mut c_void) -> *mut c_void;
}
