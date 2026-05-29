#![allow(missing_docs)]

use core::ffi::{c_int, c_void};

/// Callback type used by every async-stream bridge.
///
/// * `kind`    — event discriminant (stream-type-specific)
/// * `payload` — pointer to POD event data, or null
/// * `ctx`     — Rust context pointer (never null)
pub type StreamEventCallback =
    unsafe extern "C" fn(kind: c_int, payload: *const c_void, ctx: *mut c_void);

/// Reference-counting trampoline handed to the Swift bridge.
///
/// Each bridge object can take/drop a reference on the Rust-owned context. This
/// keeps the context (and the `AsyncStreamSender<T>` it owns) alive while any
/// callback can still be dispatched against it — closing the use-after-free
/// window left by a bare synchronous `removeObserver`/`removeTarget`.
pub type ContextRefCallback = extern "C" fn(ctx: *mut c_void);

extern "C" {
    /// Subscribes to one of the four notification-centre streams.
    /// `kind`: 0=NowPlayingItemChanged, 1=PlaybackStateChanged,
    ///         2=VolumeChanged, 3=MediaLibraryChanged
    pub fn mp_notification_subscribe(
        kind: c_int,
        callback: Option<StreamEventCallback>,
        ctx: *mut c_void,
        context_retain: ContextRefCallback,
        context_release: ContextRefCallback,
    ) -> *mut c_void;

    /// Cancels a notification subscription and releases the bridge object.
    pub fn mp_notification_unsubscribe(handle: *mut c_void);

    /// Subscribes to a single `MPRemoteCommandCenter` command as a stream.
    /// `command_id` uses the same encoding as `mp_remote_command_add_handler`.
    pub fn mp_stream_remote_command_subscribe(
        command_id: c_int,
        callback: Option<StreamEventCallback>,
        ctx: *mut c_void,
        context_retain: ContextRefCallback,
        context_release: ContextRefCallback,
    ) -> *mut c_void;

    /// Cancels a remote-command stream subscription.
    pub fn mp_stream_remote_command_unsubscribe(handle: *mut c_void);

    /// Subscribes to `MPNowPlayingSessionDelegate` events.
    ///
    /// The bundled Swift bridge returns `NULL` on macOS because
    /// `MPNowPlayingSession` is unavailable there.
    pub fn mp_now_playing_session_stream_subscribe(
        callback: Option<StreamEventCallback>,
        ctx: *mut c_void,
        context_retain: ContextRefCallback,
        context_release: ContextRefCallback,
    ) -> *mut c_void;

    /// Tears down an `MPNowPlayingSession` stream subscription.
    pub fn mp_now_playing_session_stream_unsubscribe(handle: *mut c_void);

    /// Cross-language ABI check implemented in the Swift bridge.
    ///
    /// Returns `true` only if the Swift `MemoryLayout` (size, stride and
    /// alignment) and field offsets of `MPStreamCommandPayload` match the
    /// values pinned on the Rust side for `RawCommandPayload`. Verified by
    /// `tests/ffi_layout_tests.rs`.
    pub fn mp_verify_ffi_layout() -> bool;
}
