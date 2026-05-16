#![allow(missing_docs)]

use core::ffi::{c_char, c_float, c_int, c_void};

extern "C" {
    pub fn mp_content_item_new(identifier: *const c_char) -> *mut c_void;
    pub fn mp_content_item_release(item: *mut c_void);
    pub fn mp_content_item_copy_identifier(item: *mut c_void) -> *mut c_char;
    pub fn mp_content_item_copy_title(item: *mut c_void) -> *mut c_char;
    pub fn mp_content_item_set_title(item: *mut c_void, title: *const c_char);
    pub fn mp_content_item_copy_subtitle(item: *mut c_void) -> *mut c_char;
    pub fn mp_content_item_set_subtitle(item: *mut c_void, subtitle: *const c_char);
    pub fn mp_content_item_copy_artwork(item: *mut c_void) -> *mut c_void;
    pub fn mp_content_item_set_artwork(item: *mut c_void, artwork: *mut c_void);
    pub fn mp_content_item_get_playback_progress(item: *mut c_void) -> c_float;
    pub fn mp_content_item_set_playback_progress(item: *mut c_void, playback_progress: c_float);
    pub fn mp_content_item_is_streaming_content(item: *mut c_void) -> c_int;
    pub fn mp_content_item_set_streaming_content(item: *mut c_void, streaming_content: c_int);
    pub fn mp_content_item_is_explicit_content(item: *mut c_void) -> c_int;
    pub fn mp_content_item_set_explicit_content(item: *mut c_void, explicit_content: c_int);
    pub fn mp_content_item_is_container(item: *mut c_void) -> c_int;
    pub fn mp_content_item_set_container(item: *mut c_void, container: c_int);
    pub fn mp_content_item_is_playable(item: *mut c_void) -> c_int;
    pub fn mp_content_item_set_playable(item: *mut c_void, playable: c_int);
}
