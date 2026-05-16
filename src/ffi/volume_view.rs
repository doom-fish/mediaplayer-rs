#![allow(missing_docs)]

use core::ffi::{c_char, c_int};

extern "C" {
    pub fn mp_volume_view_is_supported() -> c_int;
    pub fn mp_volume_view_copy_unavailable_reason() -> *mut c_char;
}
