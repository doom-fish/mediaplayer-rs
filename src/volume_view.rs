//! Explicit macOS-unavailable wrapper for `MPVolumeView`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPVolumeView`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VolumeView;

impl VolumeView {
    /// Returns `false` on macOS because Apple marks `MPVolumeView` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_volume_view_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPVolumeView`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_volume_view_copy_unavailable_reason())
                .unwrap_or_else(|| "MPVolumeView is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPVolumeView`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPVolumeView", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn new() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
