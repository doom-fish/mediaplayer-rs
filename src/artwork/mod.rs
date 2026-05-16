//! `MPMediaItemArtwork` wrapper.

use std::ffi::CString;

use apple_cf::cg::CGSize;

use crate::{ffi, MediaPlayerError};

/// An `MPMediaItemArtwork` instance backed by an image loaded from disk.
///
/// Create via [`Artwork::from_path`] or [`Artwork::from_path_with_size`] and
/// pass to [`crate::NowPlayingInfoCenter::set_now_playing_info_with_artwork`].
///
/// Releasing this value releases the underlying Swift/ObjC object.
pub struct Artwork {
    pub(crate) ptr: *mut core::ffi::c_void,
}

// SAFETY: The underlying object is a reference-counted ObjC object.
// Retain/release are thread-safe on Apple platforms.
unsafe impl Send for Artwork {}
unsafe impl Sync for Artwork {}

impl std::fmt::Debug for Artwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Artwork")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Artwork {
    /// Load an image from `path` and wrap it in an `MPMediaItemArtwork`.
    ///
    /// The bounds size is taken from the image's natural dimensions.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::Framework`] if the file cannot be read or
    /// `MPMediaItemArtwork` is unavailable (< macOS 10.12.2).
    pub fn from_path(path: &str) -> Result<Self, MediaPlayerError> {
        let c_path = CString::new(path)
            .map_err(|e| MediaPlayerError::InvalidArgument(e.to_string()))?;
        let ptr = unsafe { ffi::mp_artwork_new_from_path(c_path.as_ptr()) };
        if ptr.is_null() {
            Err(MediaPlayerError::Framework(format!(
                "failed to load artwork from path: {path}"
            )))
        } else {
            Ok(Self { ptr })
        }
    }

    /// Load an image from `path` with an explicit `bounds_size` hint.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::Framework`] on failure.
    pub fn from_path_with_size(path: &str, bounds_size: CGSize) -> Result<Self, MediaPlayerError> {
        let c_path = CString::new(path)
            .map_err(|e| MediaPlayerError::InvalidArgument(e.to_string()))?;
        let ptr = unsafe {
            ffi::mp_artwork_new_from_path_with_size(
                c_path.as_ptr(),
                bounds_size.width,
                bounds_size.height,
            )
        };
        if ptr.is_null() {
            Err(MediaPlayerError::Framework(format!(
                "failed to load artwork from path: {path}"
            )))
        } else {
            Ok(Self { ptr })
        }
    }
}

impl Drop for Artwork {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_artwork_release(self.ptr) }
        }
    }
}
