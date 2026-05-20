use core::ffi::c_char;
use crate::{ffi, MediaPlayerError};

pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| ffi::mp_string_free(p))
}

pub fn not_available(area: &str, reason: Option<String>) -> MediaPlayerError {
    MediaPlayerError::NotAvailable(
        reason.unwrap_or_else(|| format!("{area} is unavailable on macOS")),
    )
}
