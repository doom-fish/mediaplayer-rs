//! Explicit macOS-unavailable wrapper for `MPPlayableContentDataSource`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPPlayableContentDataSource`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayableContentDataSource;

impl PlayableContentDataSource {
    /// Returns `false` on macOS because Apple marks `MPPlayableContentDataSource` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_playable_content_data_source_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPPlayableContentDataSource`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_playable_content_data_source_copy_unavailable_reason())
                .unwrap_or_else(|| "MPPlayableContentDataSource is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPPlayableContentDataSource`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPPlayableContentDataSource", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn new() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
