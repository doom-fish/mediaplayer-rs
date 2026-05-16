//! Explicit macOS-unavailable wrapper for `MPMediaLibrary`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPMediaLibrary`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MediaLibrary;

impl MediaLibrary {
    /// Returns `false` on macOS because Apple marks `MPMediaLibrary` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_media_library_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMediaLibrary`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_media_library_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMediaLibrary is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMediaLibrary`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMediaLibrary", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn default_media_library() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
