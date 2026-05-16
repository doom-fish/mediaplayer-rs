//! Explicit macOS-unavailable wrapper for `MPMediaItem`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPMediaItem`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MediaItem;

impl MediaItem {
    /// Returns `false` on macOS because Apple marks `MPMediaItem` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_media_item_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMediaItem`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_media_item_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMediaItem is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMediaItem`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMediaItem", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn example_instance() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
