use core::ffi::c_char;
use std::ffi::CStr;

use crate::{ffi, MediaPlayerError};

pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::mp_string_free(ptr);
    Some(value)
}

pub fn not_available(area: &str, reason: Option<String>) -> MediaPlayerError {
    MediaPlayerError::NotAvailable(
        reason.unwrap_or_else(|| format!("{area} is unavailable on macOS")),
    )
}
