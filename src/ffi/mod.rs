#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_double, c_int, c_void};

/// Rust callback invoked by Swift when a remote command event fires.
///
/// Parameters:
/// - `refcon`     — opaque user data pointer registered with the handler
/// - `command_id` — mirrors [`crate::remote_commands::Command`] repr
/// - `timestamp`  — event timestamp (seconds)
/// - `extra`      — skip interval (skip forward/backward) or position time
///   (change playback position); zero for other commands
/// - `seek_type`  — [`MPSeekCommandEventType`] raw value (0=begin, 1=end);
///   zero for non-seek commands
///
/// Return value: [`crate::remote_commands::HandlerStatus`] raw value.
pub type MpCommandCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    command_id: c_int,
    timestamp: c_double,
    extra: c_double,
    seek_type: c_int,
) -> c_int;

extern "C" {
    /// Free a `*mut c_char` allocated by the bridge.
    pub fn mp_string_free(s: *mut c_char);

    // ── Now Playing ──────────────────────────────────────────────────────────

    /// Push a now-playing info dictionary to the system.
    ///
    /// Pass `NULL` for absent string fields and a negative value for absent
    /// numeric fields.  Artwork pointer is an opaque retained object from
    /// [`mp_artwork_new_from_path`]; pass `NULL` to omit.
    pub fn mp_now_playing_set_info(
        title: *const c_char,
        artist: *const c_char,
        album_title: *const c_char,
        playback_duration: c_double,
        elapsed_playback_time: c_double,
        playback_rate: c_double,
        media_type: c_int,
        content_id: *const c_char,
        asset_url: *const c_char,
        artwork: *mut c_void,
    );

    /// Set `nowPlayingInfo = nil` on the default center.
    pub fn mp_now_playing_clear();

    /// Set the playback state on the default center (macOS 10.12.2+).
    pub fn mp_now_playing_set_playback_state(state: c_int);

    /// Get the current playback state raw value (0 = unknown).
    pub fn mp_now_playing_get_playback_state() -> c_int;

    // ── Remote Commands ───────────────────────────────────────────────────────

    /// Register a handler for a remote command.
    ///
    /// Returns a retained opaque token; pass to [`mp_remote_command_remove_handler`]
    /// then [`mp_command_token_release`] when done.
    /// Returns `NULL` if `command_id` is unrecognised or `callback` is `None`.
    pub fn mp_remote_command_add_handler(
        command_id: c_int,
        callback: Option<MpCommandCallback>,
        refcon: *mut c_void,
    ) -> *mut c_void;

    /// Deregister the handler associated with `token` from its command.
    ///
    /// After this call the callback will no longer be invoked.
    /// The caller must still call [`mp_command_token_release`].
    pub fn mp_remote_command_remove_handler(token: *mut c_void);

    /// Release the Swift-side token object.
    pub fn mp_command_token_release(token: *mut c_void);

    // ── Artwork ───────────────────────────────────────────────────────────────

    /// Create an `MPMediaItemArtwork` wrapping the image at `path`.
    ///
    /// Returns a retained opaque pointer, or `NULL` on failure.
    /// Release with [`mp_artwork_release`].
    pub fn mp_artwork_new_from_path(path: *const c_char) -> *mut c_void;

    /// Create an `MPMediaItemArtwork` with an explicit bounds size (width × height).
    pub fn mp_artwork_new_from_path_with_size(
        path: *const c_char,
        width: c_double,
        height: c_double,
    ) -> *mut c_void;

    /// Release an artwork object created by [`mp_artwork_new_from_path`].
    pub fn mp_artwork_release(artwork: *mut c_void);
}
