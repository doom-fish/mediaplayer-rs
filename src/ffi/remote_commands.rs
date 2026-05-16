#![allow(missing_docs)]

use core::ffi::{c_char, c_double, c_int, c_void};

pub type MpCommandCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    command_id: c_int,
    timestamp: c_double,
    extra: c_double,
    seek_type: c_int,
    rating: c_double,
    playback_rate: c_double,
    negative: c_int,
    shuffle_type: c_int,
    repeat_type: c_int,
    preserves_shuffle_mode: c_int,
    preserves_repeat_mode: c_int,
    language_option: *mut c_void,
    language_option_setting: c_int,
) -> c_int;

extern "C" {
    pub fn mp_remote_command_add_handler(
        command_id: c_int,
        callback: Option<MpCommandCallback>,
        refcon: *mut c_void,
    ) -> *mut c_void;
    pub fn mp_remote_command_remove_handler(token: *mut c_void);
    pub fn mp_command_token_release(token: *mut c_void);

    pub fn mp_remote_command_is_enabled(command_id: c_int) -> c_int;
    pub fn mp_remote_command_set_enabled(command_id: c_int, enabled: c_int);

    pub fn mp_skip_command_copy_preferred_intervals(command_id: c_int) -> *mut c_char;
    pub fn mp_skip_command_set_preferred_intervals(
        command_id: c_int,
        intervals: *const c_double,
        count: usize,
    );

    pub fn mp_feedback_command_is_active(command_id: c_int) -> c_int;
    pub fn mp_feedback_command_set_active(command_id: c_int, active: c_int);
    pub fn mp_feedback_command_copy_localized_title(command_id: c_int) -> *mut c_char;
    pub fn mp_feedback_command_set_localized_title(command_id: c_int, title: *const c_char);
    pub fn mp_feedback_command_copy_localized_short_title(command_id: c_int) -> *mut c_char;
    pub fn mp_feedback_command_set_localized_short_title(
        command_id: c_int,
        title: *const c_char,
    );

    pub fn mp_rating_command_get_minimum_rating(command_id: c_int) -> c_double;
    pub fn mp_rating_command_set_minimum_rating(command_id: c_int, rating: c_double);
    pub fn mp_rating_command_get_maximum_rating(command_id: c_int) -> c_double;
    pub fn mp_rating_command_set_maximum_rating(command_id: c_int, rating: c_double);

    pub fn mp_change_playback_rate_copy_supported_rates(command_id: c_int) -> *mut c_char;
    pub fn mp_change_playback_rate_set_supported_rates(
        command_id: c_int,
        rates: *const c_double,
        count: usize,
    );

    pub fn mp_change_shuffle_mode_get_current_shuffle_type(command_id: c_int) -> c_int;
    pub fn mp_change_shuffle_mode_set_current_shuffle_type(command_id: c_int, shuffle_type: c_int);

    pub fn mp_change_repeat_mode_get_current_repeat_type(command_id: c_int) -> c_int;
    pub fn mp_change_repeat_mode_set_current_repeat_type(command_id: c_int, repeat_type: c_int);
}
