//! Explicit macOS-unavailable wrapper for `MPMediaPlaylist`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPMediaPlaylist`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MediaPlaylist;

impl MediaPlaylist {
    /// Returns `false` on macOS because Apple marks `MPMediaPlaylist` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_media_playlist_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMediaPlaylist`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_media_playlist_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMediaPlaylist is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMediaPlaylist`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMediaPlaylist", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn playlist_named() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
