//! Explicit macOS-unavailable wrapper for `MPMusicPlayer`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPMusicPlayer`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MusicPlayer;

impl MusicPlayer {
    /// Returns `false` on macOS because Apple marks `MPMusicPlayer` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_music_player_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMusicPlayer`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_music_player_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMusicPlayer is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMusicPlayer`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMusicPlayer", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn application_music_player() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
