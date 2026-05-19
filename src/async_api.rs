//! Async stream API for `MediaPlayer.framework`.
//!
//! Enabled by the **`async`** Cargo feature.  Each stream wraps a
//! notification-centre observer, `MPRemoteCommandCenter` target, or
//! `MPNowPlayingSession` delegate as an executor-agnostic
//! [`BoundedAsyncStream<T>`][doom_fish_utils::stream::BoundedAsyncStream].
//!
//! ## Streams
//!
//! | Type | Apple surface |
//! |------|---------------|
//! | [`NowPlayingItemChangeStream`] | `MPMusicPlayerControllerNowPlayingItemDidChangeNotification` |
//! | [`PlaybackStateChangeStream`] | `MPMusicPlayerControllerPlaybackStateDidChangeNotification` |
//! | [`VolumeChangeStream`] | `MPMusicPlayerControllerVolumeDidChangeNotification` |
//! | [`MediaLibraryChangeStream`] | `MPMediaLibraryDidChangeNotification` |
//! | [`RemoteCommandStream`] | `MPRemoteCommandCenter` command targets |
//! | [`NowPlayingSessionStream`] | `MPNowPlayingSessionDelegate` events (stub — unavailable on macOS) |
//!
//! ## Capacity and back-pressure
//!
//! All constructors accept a `capacity` argument.  When the internal ring
//! buffer is full and a new event arrives, the **oldest** buffered event is
//! silently dropped (lossy by design — keeps latency bounded for real-time
//! event sources).
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # async fn run() {
//! use mediaplayer::async_api::RemoteCommandStream;
//! use mediaplayer::remote_commands::Command;
//!
//! let stream = RemoteCommandStream::subscribe(Command::Play, 16);
//! while let Some(event) = stream.next().await {
//!     println!("play command at t={:.3}", event.timestamp);
//! }
//! # }
//! ```

#![cfg(feature = "async")]
#![allow(clippy::module_name_repetitions)]

use core::ffi::c_void;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream};

use crate::ffi::{self, StreamEventCallback};
use crate::remote_commands::{
    Command, CommandEvent, LanguageOptionSetting, RepeatType, SeekType, ShuffleType,
};

// ─────────────────────────────────────────────────────────────────────────────
// MARK: RAII subscription handle
// ─────────────────────────────────────────────────────────────────────────────

/// Opaque RAII guard for a subscription handle returned by the Swift bridge.
///
/// On drop:
/// 1. Calls the appropriate `_unsubscribe` Swift function, which removes the
///    observer / handler target and releases the Swift bridge object.
/// 2. Reconstitutes and drops the `Box<AsyncStreamSender<T>>` that was leaked
///    into the Swift bridge as the `ctx` pointer, closing the stream.
///
/// Unsubscribe is always called **before** freeing the sender so that no new
/// callback can fire against the deallocated pointer.
struct SubscriptionHandle {
    ptr: *mut c_void,
    /// Leaked `Box<AsyncStreamSender<T>>` that was passed to Swift as `ctx`.
    /// Null only when the subscribe call returned a null handle (stream already
    /// closed) — in that case the sender was freed at subscribe time.
    sender: *mut c_void,
    unsubscribe: unsafe fn(*mut c_void),
    /// Type-erased destructor for the sender box (varies per event type).
    free_sender: unsafe fn(*mut c_void),
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        // Step 1: remove the Apple observer/target so no new callback can touch
        // the sender pointer we are about to free.
        if !self.ptr.is_null() {
            // SAFETY: `self.ptr` is the opaque bridge handle created by the Swift
            // subscribe function.  We have sole ownership; it has not been freed.
            unsafe { (self.unsubscribe)(self.ptr) }
        }
        // Step 2: drop the sender box — this marks the BoundedAsyncStream closed
        // and wakes any pending consumer.
        if !self.sender.is_null() {
            // SAFETY: `self.sender` was created by `Box::into_raw(Box::new(sender))`
            // in the subscribe function and ownership was transferred here.
            // No callback can reach this pointer after step 1 returns because
            // the observer/target was removed synchronously above.
            unsafe { (self.free_sender)(self.sender) }
        }
    }
}

// SAFETY: The Swift bridge objects are thread-safe (NSNotificationCenter
// dispatches on main queue; Objective-C ref-counting is thread-safe).
// `AsyncStreamSender<T>` is Send + Sync (Arc<Mutex<...>> internals).
unsafe impl Send for SubscriptionHandle {}
unsafe impl Sync for SubscriptionHandle {}

// ── Type-erased sender drop helpers ─────────────────────────────────────────

unsafe fn free_notification_sender(ptr: *mut c_void) {
    // SAFETY: ptr was created by Box::into_raw(Box::new(AsyncStreamSender<NotificationEvent>)).
    drop(unsafe { Box::from_raw(ptr.cast::<AsyncStreamSender<NotificationEvent>>()) });
}

unsafe fn free_command_sender(ptr: *mut c_void) {
    // SAFETY: ptr was created by Box::into_raw(Box::new(AsyncStreamSender<CommandEvent>)).
    drop(unsafe { Box::from_raw(ptr.cast::<AsyncStreamSender<CommandEvent>>()) });
}

unsafe fn free_session_sender(ptr: *mut c_void) {
    // SAFETY: ptr was created by Box::into_raw(Box::new(AsyncStreamSender<NowPlayingSessionEvent>)).
    drop(unsafe { Box::from_raw(ptr.cast::<AsyncStreamSender<NowPlayingSessionEvent>>()) });
}

// ─────────────────────────────────────────────────────────────────────────────
// MARK: Notification streams
// ─────────────────────────────────────────────────────────────────────────────

/// Which notification fired — carried by every [`NotificationEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationKind {
    /// `MPMusicPlayerControllerNowPlayingItemDidChangeNotification`
    NowPlayingItemDidChange = 0,
    /// `MPMusicPlayerControllerPlaybackStateDidChangeNotification`
    PlaybackStateDidChange = 1,
    /// `MPMusicPlayerControllerVolumeDidChangeNotification`
    VolumeDidChange = 2,
    /// `MPMediaLibraryDidChangeNotification`
    MediaLibraryDidChange = 3,
}

/// Event fired by a notification-centre stream.
///
/// Carries no payload beyond the fact that the notification fired — the
/// consumer should query the relevant framework object for updated state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotificationEvent {
    /// Which notification fired.
    pub kind: NotificationKind,
}

/// `extern "C"` trampoline for all notification callbacks.
unsafe extern "C" fn notification_cb(kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    // SAFETY: ctx is the sender_ptr created by Box::into_raw in subscribe_notification.
    // The pointer is valid for the lifetime of the subscription; the callback is only
    // invoked while the Swift bridge object is alive, which is before SubscriptionHandle
    // calls free_notification_sender.
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<NotificationEvent>>() };
    let notification_kind = match kind {
        0 => NotificationKind::NowPlayingItemDidChange,
        1 => NotificationKind::PlaybackStateDidChange,
        2 => NotificationKind::VolumeDidChange,
        3 => NotificationKind::MediaLibraryDidChange,
        _ => return,
    };
    sender.push(NotificationEvent { kind: notification_kind });
}

/// Subscribes to one notification kind and returns a stream + RAII handle.
fn subscribe_notification(
    kind: NotificationKind,
    capacity: usize,
) -> (BoundedAsyncStream<NotificationEvent>, SubscriptionHandle) {
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    let sender_ptr = Box::into_raw(Box::new(sender));
    // SAFETY: callback is a valid extern "C" fn pointer; sender_ptr is a valid heap
    // allocation just created above; Swift retains the returned handle until unsubscribe.
    let handle_ptr = unsafe {
        ffi::mp_notification_subscribe(
            kind as i32,
            Some(notification_cb as StreamEventCallback),
            sender_ptr.cast(),
        )
    };
    // If Swift returned null (unknown kind), drop the sender to close the stream immediately.
    if handle_ptr.is_null() {
        // SAFETY: sender_ptr was created by Box::into_raw above; handle_ptr is null so
        // SubscriptionHandle will not touch sender (set to null below).
        unsafe { drop(Box::from_raw(sender_ptr)) };
    }
    let handle = SubscriptionHandle {
        ptr: handle_ptr,
        // When handle_ptr is null the sender was already freed above; pass null so
        // SubscriptionHandle::drop does not attempt a double-free.
        sender: if handle_ptr.is_null() { core::ptr::null_mut() } else { sender_ptr.cast() },
        unsubscribe: |ptr| unsafe { ffi::mp_notification_unsubscribe(ptr) },
        free_sender: free_notification_sender,
    };
    (stream, handle)
}

// ── NowPlayingItemChangeStream ───────────────────────────────────────────────

/// Stream of [`NotificationEvent`]s fired when the now-playing item changes.
///
/// Wraps `MPMusicPlayerControllerNowPlayingItemDidChangeNotification`.
///
/// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only so this
/// > notification never fires on macOS; the stream will be open but idle.
pub struct NowPlayingItemChangeStream {
    inner: BoundedAsyncStream<NotificationEvent>,
    _handle: SubscriptionHandle,
}

impl NowPlayingItemChangeStream {
    /// Subscribe with an internal ring buffer of `capacity` slots.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) =
            subscribe_notification(NotificationKind::NowPlayingItemDidChange, capacity);
        Self { inner, _handle: handle }
    }

    /// Await the next event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, NotificationEvent> {
        self.inner.next()
    }

    /// Non-blocking poll — returns `Some` if an event is already buffered.
    #[must_use]
    pub fn try_next(&self) -> Option<NotificationEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once all senders have been dropped (subscription torn down).
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── PlaybackStateChangeStream ────────────────────────────────────────────────

/// Stream of [`NotificationEvent`]s fired when the playback state changes.
///
/// Wraps `MPMusicPlayerControllerPlaybackStateDidChangeNotification`.
///
/// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only so this
/// > notification never fires on macOS; the stream will be open but idle.
pub struct PlaybackStateChangeStream {
    inner: BoundedAsyncStream<NotificationEvent>,
    _handle: SubscriptionHandle,
}

impl PlaybackStateChangeStream {
    /// Subscribe with an internal ring buffer of `capacity` slots.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) =
            subscribe_notification(NotificationKind::PlaybackStateDidChange, capacity);
        Self { inner, _handle: handle }
    }

    /// Await the next event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, NotificationEvent> {
        self.inner.next()
    }

    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<NotificationEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once all senders have been dropped.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── VolumeChangeStream ────────────────────────────────────────────────────────

/// Stream of [`NotificationEvent`]s fired when the system volume changes.
///
/// Wraps `MPMusicPlayerControllerVolumeDidChangeNotification`.
///
/// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only so this
/// > notification never fires on macOS; the stream will be open but idle.
pub struct VolumeChangeStream {
    inner: BoundedAsyncStream<NotificationEvent>,
    _handle: SubscriptionHandle,
}

impl VolumeChangeStream {
    /// Subscribe with an internal ring buffer of `capacity` slots.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) =
            subscribe_notification(NotificationKind::VolumeDidChange, capacity);
        Self { inner, _handle: handle }
    }

    /// Await the next event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, NotificationEvent> {
        self.inner.next()
    }

    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<NotificationEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once all senders have been dropped.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── MediaLibraryChangeStream ──────────────────────────────────────────────────

/// Stream of [`NotificationEvent`]s fired when the media library changes.
///
/// Wraps `MPMediaLibraryDidChangeNotification`.
///
/// > **macOS note:** `MPMediaLibrary` is iOS/tvOS-only so this notification
/// > never fires on macOS; the stream will be open but idle.
pub struct MediaLibraryChangeStream {
    inner: BoundedAsyncStream<NotificationEvent>,
    _handle: SubscriptionHandle,
}

impl MediaLibraryChangeStream {
    /// Subscribe with an internal ring buffer of `capacity` slots.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) =
            subscribe_notification(NotificationKind::MediaLibraryDidChange, capacity);
        Self { inner, _handle: handle }
    }

    /// Await the next event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, NotificationEvent> {
        self.inner.next()
    }

    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<NotificationEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once all senders have been dropped.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MARK: RemoteCommandStream
// ─────────────────────────────────────────────────────────────────────────────

/// Plain-old-data layout of the command payload marshalled by the Swift bridge.
///
/// Must match `MPStreamCommandPayload` in `AsyncStream.swift`.
#[repr(C)]
#[derive(Clone, Copy)]
struct RawCommandPayload {
    command_id: i32,
    timestamp: f64,
    extra: f64,
    seek_type: i32,
    rating: f64,
    playback_rate: f64,
    negative: i32,
    shuffle_type: i32,
    repeat_type: i32,
    preserves_shuffle_mode: i32,
    preserves_repeat_mode: i32,
    language_option_setting: i32,
}

unsafe extern "C" fn remote_command_cb(
    _kind: i32,
    payload: *const c_void,
    ctx: *mut c_void,
) {
    if payload.is_null() {
        return;
    }
    // SAFETY: payload points to a stack-allocated MPStreamCommandPayload created by
    // the Swift bridge immediately before this callback returns; the pointer is valid
    // for the duration of this call and the layout matches RawCommandPayload (#[repr(C)]).
    let raw = unsafe { &*payload.cast::<RawCommandPayload>() };
    // SAFETY: ctx is the sender_ptr created by Box::into_raw in RemoteCommandStream::subscribe.
    // Valid for the lifetime of the subscription; freed in SubscriptionHandle::drop after
    // mp_stream_remote_command_unsubscribe removes the command target.
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<CommandEvent>>() };
    let command = Command::from_id(raw.command_id).unwrap_or(Command::Play);
    let event = CommandEvent {
        command,
        timestamp: raw.timestamp,
        skip_interval: matches!(command, Command::SkipForward | Command::SkipBackward)
            .then_some(raw.extra)
            .filter(|v| !v.is_nan()),
        seek_type: matches!(command, Command::SeekForward | Command::SeekBackward)
            .then(|| SeekType::from_raw(raw.seek_type))
            .filter(|_| raw.seek_type >= 0),
        position: matches!(command, Command::ChangePlaybackPosition)
            .then_some(raw.extra)
            .filter(|v| !v.is_nan()),
        rating: (!raw.rating.is_nan()).then_some(raw.rating),
        playback_rate: (!raw.playback_rate.is_nan()).then_some(raw.playback_rate),
        feedback_negative: (raw.negative >= 0).then_some(raw.negative != 0),
        shuffle_type: (raw.shuffle_type >= 0).then(|| ShuffleType::from_raw(raw.shuffle_type)),
        repeat_type: (raw.repeat_type >= 0).then(|| RepeatType::from_raw(raw.repeat_type)),
        preserves_shuffle_mode: (raw.preserves_shuffle_mode >= 0)
            .then_some(raw.preserves_shuffle_mode != 0),
        preserves_repeat_mode: (raw.preserves_repeat_mode >= 0)
            .then_some(raw.preserves_repeat_mode != 0),
        language_option: None,
        language_option_setting: (raw.language_option_setting >= 0)
            .then(|| LanguageOptionSetting::from_raw(raw.language_option_setting)),
    };
    sender.push(event);
}

/// Async stream of [`CommandEvent`]s for a single `MPRemoteCommandCenter` command.
///
/// Each event is automatically acknowledged with `.success` to the system.
/// If you need to return a custom [`HandlerStatus`][crate::remote_commands::HandlerStatus]
/// use the synchronous [`RemoteCommandCenter`][crate::remote_commands::RemoteCommandCenter]
/// API instead.
///
/// ## Example
///
/// ```no_run
/// # #[cfg(feature = "async")]
/// # async fn run() {
/// use mediaplayer::async_api::RemoteCommandStream;
/// use mediaplayer::remote_commands::Command;
///
/// let play_stream = RemoteCommandStream::subscribe(Command::Play, 16);
/// while let Some(event) = play_stream.next().await {
///     println!("play pressed at t={:.3}", event.timestamp);
/// }
/// # }
/// ```
pub struct RemoteCommandStream {
    inner: BoundedAsyncStream<CommandEvent>,
    _handle: SubscriptionHandle,
}

impl RemoteCommandStream {
    /// Subscribe to events for `command` with an internal ring buffer of
    /// `capacity` slots.
    ///
    /// Drop the returned stream to unsubscribe.
    #[must_use]
    pub fn subscribe(command: Command, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        // SAFETY: callback is a valid extern "C" fn pointer; sender_ptr is a valid heap
        // allocation just created above; Swift retains the returned handle until unsubscribe.
        let handle_ptr = unsafe {
            ffi::mp_stream_remote_command_subscribe(
                command as i32,
                Some(remote_command_cb as StreamEventCallback),
                sender_ptr.cast(),
            )
        };
        if handle_ptr.is_null() {
            // SAFETY: sender_ptr was created by Box::into_raw above; handle_ptr is null
            // so SubscriptionHandle will not touch sender (set to null below).
            unsafe { drop(Box::from_raw(sender_ptr)) };
        }
        let handle = SubscriptionHandle {
            ptr: handle_ptr,
            sender: if handle_ptr.is_null() { core::ptr::null_mut() } else { sender_ptr.cast() },
            unsubscribe: |ptr| unsafe { ffi::mp_stream_remote_command_unsubscribe(ptr) },
            free_sender: free_command_sender,
        };
        Self { inner: stream, _handle: handle }
    }

    /// Await the next remote-command event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, CommandEvent> {
        self.inner.next()
    }

    /// Non-blocking poll — returns `Some` if an event is already buffered.
    #[must_use]
    pub fn try_next(&self) -> Option<CommandEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once the subscription has been torn down.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MARK: NowPlayingSessionStream
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminant for [`NowPlayingSessionEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NowPlayingSessionEventKind {
    /// The session changed whether it is the active now-playing session.
    DidChangeActive,
    /// The session changed whether it can become the active now-playing session.
    DidChangeCanBecomeActive,
}

/// Event delivered by [`NowPlayingSessionStream`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NowPlayingSessionEvent {
    /// Which delegate callback fired.
    pub kind: NowPlayingSessionEventKind,
}

unsafe extern "C" fn now_playing_session_cb(
    kind: i32,
    _payload: *const c_void,
    ctx: *mut c_void,
) {
    // SAFETY: ctx is the sender_ptr created by Box::into_raw in NowPlayingSessionStream::subscribe.
    // Valid for the lifetime of the subscription; freed in SubscriptionHandle::drop after
    // mp_now_playing_session_stream_unsubscribe tears down the session.
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<NowPlayingSessionEvent>>() };
    let event_kind = match kind {
        0 => NowPlayingSessionEventKind::DidChangeActive,
        1 => NowPlayingSessionEventKind::DidChangeCanBecomeActive,
        _ => return,
    };
    sender.push(NowPlayingSessionEvent { kind: event_kind });
}

/// Async stream of [`NowPlayingSessionEvent`]s from `MPNowPlayingSessionDelegate`.
///
/// The event enum mirrors `nowPlayingSessionDidChangeActive(_:)` and
/// `nowPlayingSessionDidChangeCanBecomeActive(_:)`.
///
/// `MPNowPlayingSession` is iOS 16.0+ / tvOS 14.0+ only and is explicitly
/// unavailable on macOS. The Swift bridge therefore returns `nil` for the
/// subscription handle on macOS, so the stream is open but idle. This type is
/// provided for API completeness so code that targets multiple platforms
/// compiles without `#[cfg]` guards.
pub struct NowPlayingSessionStream {
    inner: BoundedAsyncStream<NowPlayingSessionEvent>,
    _handle: SubscriptionHandle,
}

impl NowPlayingSessionStream {
    /// Subscribe with a ring buffer of `capacity` slots.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        // SAFETY: callback is a valid extern "C" fn pointer; sender_ptr is a valid heap
        // allocation just created above; Swift retains the returned handle until unsubscribe.
        // On macOS the bridge always returns null, so the sender is freed in the is_null branch.
        let handle_ptr = unsafe {
            ffi::mp_now_playing_session_stream_subscribe(
                Some(now_playing_session_cb as StreamEventCallback),
                sender_ptr.cast(),
            )
        };
        if handle_ptr.is_null() {
            // SAFETY: sender_ptr was created by Box::into_raw above; handle_ptr is null
            // so SubscriptionHandle will not touch sender (set to null below).
            unsafe { drop(Box::from_raw(sender_ptr)) };
        }
        let handle = SubscriptionHandle {
            ptr: handle_ptr,
            sender: if handle_ptr.is_null() { core::ptr::null_mut() } else { sender_ptr.cast() },
            unsubscribe: |ptr| unsafe { ffi::mp_now_playing_session_stream_unsubscribe(ptr) },
            free_sender: free_session_sender,
        };
        Self { inner: stream, _handle: handle }
    }

    /// Await the next session event.
    #[must_use]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, NowPlayingSessionEvent> {
        self.inner.next()
    }

    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<NowPlayingSessionEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns `true` once the session has been torn down.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
