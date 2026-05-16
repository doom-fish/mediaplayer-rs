#![allow(missing_docs)]

use core::ffi::{c_char, c_double, c_int, c_void};

extern "C" {
    pub fn mp_now_playing_info_box_new() -> *mut c_void;
    pub fn mp_now_playing_info_box_release(info: *mut c_void);
    pub fn mp_now_playing_info_box_set_string(
        info: *mut c_void,
        key_id: c_int,
        value: *const c_char,
    );
    pub fn mp_now_playing_info_box_set_double(
        info: *mut c_void,
        key_id: c_int,
        value: c_double,
    );
    pub fn mp_now_playing_info_box_set_u64(info: *mut c_void, key_id: c_int, value: u64);
    pub fn mp_now_playing_info_box_set_bool(info: *mut c_void, key_id: c_int, value: c_int);
    pub fn mp_now_playing_info_box_set_url(
        info: *mut c_void,
        key_id: c_int,
        value: *const c_char,
    );
    pub fn mp_now_playing_info_box_set_date_seconds(
        info: *mut c_void,
        key_id: c_int,
        value: c_double,
    );
    pub fn mp_now_playing_info_box_set_artwork(info: *mut c_void, artwork: *mut c_void);
    pub fn mp_now_playing_info_box_set_available_language_option_groups(
        info: *mut c_void,
        groups: *const *mut c_void,
        count: usize,
    );
    pub fn mp_now_playing_info_box_set_current_language_options(
        info: *mut c_void,
        options: *const *mut c_void,
        count: usize,
    );
    pub fn mp_now_playing_apply_info_box(info: *mut c_void);

    pub fn mp_now_playing_clear();
    pub fn mp_now_playing_set_playback_state(state: c_int);
    pub fn mp_now_playing_get_playback_state() -> c_int;
    pub fn mp_now_playing_copy_supported_animated_artwork_keys() -> *mut c_char;

    pub fn mp_language_option_new(
        option_type: c_int,
        language_tag: *const c_char,
        characteristics: *const *const c_char,
        characteristics_count: usize,
        display_name: *const c_char,
        identifier: *const c_char,
    ) -> *mut c_void;
    pub fn mp_language_option_release(option: *mut c_void);
    pub fn mp_language_option_get_type(option: *mut c_void) -> c_int;
    pub fn mp_language_option_copy_language_tag(option: *mut c_void) -> *mut c_char;
    pub fn mp_language_option_copy_characteristics(option: *mut c_void) -> *mut c_char;
    pub fn mp_language_option_copy_display_name(option: *mut c_void) -> *mut c_char;
    pub fn mp_language_option_copy_identifier(option: *mut c_void) -> *mut c_char;
    pub fn mp_language_option_is_automatic_legible(option: *mut c_void) -> c_int;
    pub fn mp_language_option_is_automatic_audible(option: *mut c_void) -> c_int;

    pub fn mp_language_option_group_new(
        options: *const *mut c_void,
        count: usize,
        default_index: c_int,
        allow_empty_selection: c_int,
    ) -> *mut c_void;
    pub fn mp_language_option_group_release(group: *mut c_void);
    pub fn mp_language_option_group_get_count(group: *mut c_void) -> usize;
    pub fn mp_language_option_group_get_default_index(group: *mut c_void) -> c_int;
    pub fn mp_language_option_group_allows_empty_selection(group: *mut c_void) -> c_int;
}
