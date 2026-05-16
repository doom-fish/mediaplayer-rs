//! Wrapper for `MPContentItem`.

use std::ffi::CString;

use crate::{ffi, unsupported, Artwork, MediaPlayerError};

/// Owned wrapper around `MPContentItem`.
pub struct ContentItem {
    pub(crate) ptr: *mut core::ffi::c_void,
}

// SAFETY: The underlying object is a reference-counted ObjC object.
unsafe impl Send for ContentItem {}
unsafe impl Sync for ContentItem {}

impl Clone for ContentItem {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::mp_object_retain(self.ptr) };
        Self { ptr }
    }
}

impl std::fmt::Debug for ContentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContentItem")
            .field("identifier", &self.identifier())
            .field("title", &self.title())
            .field("subtitle", &self.subtitle())
            .field("playback_progress", &self.playback_progress())
            .field("streaming_content", &self.is_streaming_content())
            .field("explicit_content", &self.is_explicit_content())
            .field("container", &self.is_container())
            .field("playable", &self.is_playable())
            .finish()
    }
}

impl ContentItem {
    /// Create a content item with a stable identifier.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] if `identifier` contains an
    /// interior NUL byte, or [`MediaPlayerError::Framework`] if the framework
    /// refuses to create the item.
    pub fn new(identifier: &str) -> Result<Self, MediaPlayerError> {
        let identifier = CString::new(identifier)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let ptr = unsafe { ffi::mp_content_item_new(identifier.as_ptr()) };
        if ptr.is_null() {
            Err(MediaPlayerError::Framework(
                "failed to create MPContentItem".to_string(),
            ))
        } else {
            Ok(Self { ptr })
        }
    }

    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe { unsupported::take_string(ffi::mp_content_item_copy_identifier(self.ptr)) }
            .unwrap_or_default()
    }

    #[must_use]
    pub fn title(&self) -> Option<String> {
        unsafe { unsupported::take_string(ffi::mp_content_item_copy_title(self.ptr)) }
    }

    /// Update the item's title.
    ///
    /// Pass `None` to clear the title.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] if the string contains an
    /// interior NUL byte.
    pub fn set_title(&self, title: Option<&str>) -> Result<(), MediaPlayerError> {
        let title = optional_cstring(title)?;
        unsafe {
            ffi::mp_content_item_set_title(
                self.ptr,
                title
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            );
        }
        Ok(())
    }

    #[must_use]
    pub fn subtitle(&self) -> Option<String> {
        unsafe { unsupported::take_string(ffi::mp_content_item_copy_subtitle(self.ptr)) }
    }

    /// Update the item's subtitle.
    ///
    /// Pass `None` to clear the subtitle.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] if the string contains an
    /// interior NUL byte.
    pub fn set_subtitle(&self, subtitle: Option<&str>) -> Result<(), MediaPlayerError> {
        let subtitle = optional_cstring(subtitle)?;
        unsafe {
            ffi::mp_content_item_set_subtitle(
                self.ptr,
                subtitle
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            );
        }
        Ok(())
    }

    #[must_use]
    pub fn artwork(&self) -> Option<Artwork> {
        let ptr = unsafe { ffi::mp_content_item_copy_artwork(self.ptr) };
        (!ptr.is_null()).then_some(Artwork { ptr })
    }

    pub fn set_artwork(&self, artwork: Option<&Artwork>) {
        unsafe {
            ffi::mp_content_item_set_artwork(
                self.ptr,
                artwork.map_or(std::ptr::null_mut(), |value| value.ptr),
            );
        }
    }

    #[must_use]
    pub fn playback_progress(&self) -> f32 {
        unsafe { ffi::mp_content_item_get_playback_progress(self.ptr) }
    }

    pub fn set_playback_progress(&self, playback_progress: f32) {
        unsafe { ffi::mp_content_item_set_playback_progress(self.ptr, playback_progress) }
    }

    #[must_use]
    pub fn is_streaming_content(&self) -> bool {
        unsafe { ffi::mp_content_item_is_streaming_content(self.ptr) != 0 }
    }

    pub fn set_streaming_content(&self, streaming_content: bool) {
        unsafe {
            ffi::mp_content_item_set_streaming_content(self.ptr, i32::from(streaming_content));
        }
    }

    #[must_use]
    pub fn is_explicit_content(&self) -> bool {
        unsafe { ffi::mp_content_item_is_explicit_content(self.ptr) != 0 }
    }

    pub fn set_explicit_content(&self, explicit_content: bool) {
        unsafe {
            ffi::mp_content_item_set_explicit_content(self.ptr, i32::from(explicit_content));
        }
    }

    #[must_use]
    pub fn is_container(&self) -> bool {
        unsafe { ffi::mp_content_item_is_container(self.ptr) != 0 }
    }

    pub fn set_container(&self, container: bool) {
        unsafe { ffi::mp_content_item_set_container(self.ptr, i32::from(container)) }
    }

    #[must_use]
    pub fn is_playable(&self) -> bool {
        unsafe { ffi::mp_content_item_is_playable(self.ptr) != 0 }
    }

    pub fn set_playable(&self, playable: bool) {
        unsafe { ffi::mp_content_item_set_playable(self.ptr, i32::from(playable)) }
    }
}

impl Drop for ContentItem {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_content_item_release(self.ptr) }
        }
    }
}

fn optional_cstring(value: Option<&str>) -> Result<Option<CString>, MediaPlayerError> {
    value
        .map(CString::new)
        .transpose()
        .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))
}
