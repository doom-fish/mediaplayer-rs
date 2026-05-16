//! `MPMediaItemArtwork` and `MPMediaItemAnimatedArtwork` wrappers.

use std::ffi::CString;

use apple_cf::cg::{CGRect, CGSize};

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

impl Clone for Artwork {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::mp_object_retain(self.ptr) };
        Self { ptr }
    }
}

impl std::fmt::Debug for Artwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Artwork").field("ptr", &self.ptr).finish()
    }
}

impl Artwork {
    /// Load an image from `path` and wrap it in an `MPMediaItemArtwork`.
    ///
    /// The bounds size is taken from the image's natural dimensions.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::Framework`] if the file cannot be read or
    /// `MPMediaItemArtwork` is unavailable.
    pub fn from_path(path: &str) -> Result<Self, MediaPlayerError> {
        let c_path = CString::new(path)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
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
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
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

    /// Returns the full artwork bounds reported by the framework.
    #[must_use]
    pub fn bounds(&self) -> Option<CGRect> {
        let mut origin_x = 0.0;
        let mut origin_y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;

        let ok = unsafe {
            ffi::mp_artwork_copy_bounds(
                self.ptr,
                &mut origin_x,
                &mut origin_y,
                &mut width,
                &mut height,
            ) != 0
        };

        ok.then_some(CGRect {
            x: origin_x,
            y: origin_y,
            width,
            height,
        })
    }
}

impl Drop for Artwork {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_artwork_release(self.ptr) }
        }
    }
}

/// An `MPMediaItemAnimatedArtwork` backed by a preview image and local video asset.
pub struct AnimatedArtwork {
    pub(crate) ptr: *mut core::ffi::c_void,
}

// SAFETY: The underlying object is a reference-counted ObjC object.
unsafe impl Send for AnimatedArtwork {}
unsafe impl Sync for AnimatedArtwork {}

impl Clone for AnimatedArtwork {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::mp_object_retain(self.ptr) };
        Self { ptr }
    }
}

impl std::fmt::Debug for AnimatedArtwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimatedArtwork")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl AnimatedArtwork {
    /// Create animated artwork backed by a preview image and local video file.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] if any string contains an
    /// interior NUL byte, or [`MediaPlayerError::Framework`] if the framework
    /// refuses to create the animated artwork.
    pub fn from_files(
        artwork_id: &str,
        preview_image_path: &str,
        video_asset_path: &str,
    ) -> Result<Self, MediaPlayerError> {
        let artwork_id = CString::new(artwork_id)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let preview_image_path = CString::new(preview_image_path)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let video_asset_path = CString::new(video_asset_path)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;

        let ptr = unsafe {
            ffi::mp_animated_artwork_new_from_files(
                artwork_id.as_ptr(),
                preview_image_path.as_ptr(),
                video_asset_path.as_ptr(),
            )
        };

        if ptr.is_null() {
            Err(MediaPlayerError::Framework(
                "failed to create MPMediaItemAnimatedArtwork from local files".to_string(),
            ))
        } else {
            Ok(Self { ptr })
        }
    }
}

impl Drop for AnimatedArtwork {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_animated_artwork_release(self.ptr) }
        }
    }
}
