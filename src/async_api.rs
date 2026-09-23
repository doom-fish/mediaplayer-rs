//! Async stream API for `MediaPlayer.framework`.
//!
//! Enabled by the **`async`** Cargo feature.  Each stream wraps an
//! `MPRemoteCommandCenter` target as an executor-agnostic
//! [`BoundedAsyncStream<T>`][doom_fish_utils::stream::BoundedAsyncStream].
//!
//! ## Streams
//!
//! | Type | Apple surface |
//! |------|---------------|
//! | [`RemoteCommandStream`] | `MPRemoteCommandCenter` command targets |
//! | [`NowPlayingItemChangeStream`] | `MPMusicPlayerControllerNowPlayingItemDidChangeNotification` (unavailable on macOS) |
//! | [`PlaybackStateChangeStream`] | `MPMusicPlayerControllerPlaybackStateDidChangeNotification` (unavailable on macOS) |
//! | [`VolumeChangeStream`] | `MPMusicPlayerControllerVolumeDidChangeNotification` (unavailable on macOS) |
//! | [`MediaLibraryChangeStream`] | `MPMediaLibraryDidChangeNotification` (unavailable on macOS) |
//! | [`NowPlayingSessionStream`] | `MPNowPlayingSessionDelegate` events (unavailable on macOS) |
//!
//! Apple marks the music-player, media-library and now-playing-session
//! surfaces unavailable on macOS, so the `subscribe` constructors of those
//! streams return [`MediaPlayerError::NotAvailable`] instead of an idle stream.
//!
//! ## Capacity and back-pressure
//!
//! All constructors accept a `capacity` of at least 1.  When the internal ring
//! buffer is full and a new event arrives, the **oldest** buffered event is
//! silently dropped (lossy by design — keeps latency bounded for real-time
//! event sources).
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # async fn run() -> Result<(), mediaplayer::MediaPlayerError> {
//! use mediaplayer::async_api::RemoteCommandStream;
//! use mediaplayer::remote_commands::Command;
//!
//! let stream = RemoteCommandStream::subscribe(Command::Play, 16)?;
//! while let Some(event) = stream.next().await {
//!     println!("play command at t={:.3}", event.timestamp);
//! }
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]
#![allow(clippy::module_name_repetitions)]

use doom_fish_utils::stream::{BoundedAsyncStream, NextItem};

use crate::remote_commands::{
    Command, CommandEvent, CommandToken, HandlerStatus, RemoteCommandCenter,
};
use crate::{unsupported, MediaPlayerError};

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

macro_rules! unavailable_notification_stream {
    ($(#[$meta:meta])* $name:ident, $surface:literal) => {
        $(#[$meta])*
        pub struct $name {
            inner: BoundedAsyncStream<NotificationEvent>,
        }

        impl $name {
            /// Returns [`MediaPlayerError::NotAvailable`]: Apple marks this
            /// notification unavailable on macOS.
            pub fn subscribe(_capacity: usize) -> Result<Self, MediaPlayerError> {
                Err(unsupported::not_available($surface, None))
            }

            /// Await the next event.
            #[must_use]
            pub fn next(&self) -> NextItem<'_, NotificationEvent> {
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
    };
}

unavailable_notification_stream!(
    /// Stream of [`NotificationEvent`]s fired when the now-playing item changes.
    ///
    /// Wraps `MPMusicPlayerControllerNowPlayingItemDidChangeNotification`.
    ///
    /// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only, so
    /// > [`subscribe`](Self::subscribe) returns [`MediaPlayerError::NotAvailable`].
    NowPlayingItemChangeStream,
    "MPMusicPlayerControllerNowPlayingItemDidChangeNotification"
);

unavailable_notification_stream!(
    /// Stream of [`NotificationEvent`]s fired when the playback state changes.
    ///
    /// Wraps `MPMusicPlayerControllerPlaybackStateDidChangeNotification`.
    ///
    /// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only, so
    /// > [`subscribe`](Self::subscribe) returns [`MediaPlayerError::NotAvailable`].
    PlaybackStateChangeStream,
    "MPMusicPlayerControllerPlaybackStateDidChangeNotification"
);

unavailable_notification_stream!(
    /// Stream of [`NotificationEvent`]s fired when the system volume changes.
    ///
    /// Wraps `MPMusicPlayerControllerVolumeDidChangeNotification`.
    ///
    /// > **macOS note:** `MPMusicPlayerController` is iOS/tvOS-only, so
    /// > [`subscribe`](Self::subscribe) returns [`MediaPlayerError::NotAvailable`].
    VolumeChangeStream,
    "MPMusicPlayerControllerVolumeDidChangeNotification"
);

unavailable_notification_stream!(
    /// Stream of [`NotificationEvent`]s fired when the media library changes.
    ///
    /// Wraps `MPMediaLibraryDidChangeNotification`.
    ///
    /// > **macOS note:** `MPMediaLibrary` is iOS/tvOS-only, so
    /// > [`subscribe`](Self::subscribe) returns [`MediaPlayerError::NotAvailable`].
    MediaLibraryChangeStream,
    "MPMediaLibraryDidChangeNotification"
);

/// Async stream of [`CommandEvent`]s for a single `MPRemoteCommandCenter` command.
///
/// Each event is automatically acknowledged with `.success` to the system.
/// If you need to return a custom [`HandlerStatus`] use the synchronous
/// [`RemoteCommandCenter`] API instead.
///
/// ## Example
///
/// ```no_run
/// # #[cfg(feature = "async")]
/// # async fn run() -> Result<(), mediaplayer::MediaPlayerError> {
/// use mediaplayer::async_api::RemoteCommandStream;
/// use mediaplayer::remote_commands::Command;
///
/// let play_stream = RemoteCommandStream::subscribe(Command::Play, 16)?;
/// while let Some(event) = play_stream.next().await {
///     println!("play pressed at t={:.3}", event.timestamp);
/// }
/// # Ok(())
/// # }
/// ```
pub struct RemoteCommandStream {
    inner: BoundedAsyncStream<CommandEvent>,
    _token: CommandToken,
}

impl RemoteCommandStream {
    /// Subscribe to events for `command` with an internal ring buffer of
    /// `capacity` slots.
    ///
    /// Drop the returned stream to unsubscribe.
    pub fn subscribe(command: Command, capacity: usize) -> Result<Self, MediaPlayerError> {
        if capacity == 0 {
            return Err(MediaPlayerError::InvalidArgument(
                "stream capacity must be at least 1".into(),
            ));
        }
        let (inner, sender) = BoundedAsyncStream::new(capacity);
        let token = RemoteCommandCenter::shared().add_handler(command, move |event| {
            sender.push(event);
            HandlerStatus::Success
        })?;
        Ok(Self {
            inner,
            _token: token,
        })
    }

    /// Await the next remote-command event.
    #[must_use]
    pub fn next(&self) -> NextItem<'_, CommandEvent> {
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

/// Async stream of [`NowPlayingSessionEvent`]s from `MPNowPlayingSessionDelegate`.
///
/// The event enum mirrors `nowPlayingSessionDidChangeActive(_:)` and
/// `nowPlayingSessionDidChangeCanBecomeActive(_:)`.
///
/// `MPNowPlayingSession` is iOS 16.0+ / tvOS 14.0+ only and is explicitly
/// unavailable on macOS, so [`subscribe`](Self::subscribe) returns
/// [`MediaPlayerError::NotAvailable`]. This type is provided for API
/// completeness so code that targets multiple platforms compiles without
/// `#[cfg]` guards.
pub struct NowPlayingSessionStream {
    inner: BoundedAsyncStream<NowPlayingSessionEvent>,
}

impl NowPlayingSessionStream {
    /// Returns [`MediaPlayerError::NotAvailable`]: Apple marks
    /// `MPNowPlayingSession` unavailable on macOS.
    pub fn subscribe(_capacity: usize) -> Result<Self, MediaPlayerError> {
        Err(unsupported::not_available("MPNowPlayingSession", None))
    }

    /// Await the next session event.
    #[must_use]
    pub fn next(&self) -> NextItem<'_, NowPlayingSessionEvent> {
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
