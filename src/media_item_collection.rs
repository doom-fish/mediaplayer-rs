//! Explicit macOS-unavailable wrapper for `MPMediaItemCollection`.
//!
//! Apple marks this MediaPlayer API as unavailable on macOS. The wrapper exists so
//! the crate documents the area explicitly and fails predictably at runtime.

use crate::{ffi, unsupported, MediaPlayerError};

/// Marker type representing `MPMediaItemCollection`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MediaItemCollection;

impl MediaItemCollection {
    /// Returns `false` on macOS because Apple marks `MPMediaItemCollection` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_media_item_collection_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMediaItemCollection`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_media_item_collection_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMediaItemCollection is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMediaItemCollection`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMediaItemCollection", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn collection_with_items() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
