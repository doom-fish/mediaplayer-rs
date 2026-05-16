//! Wrapper for `MPNowPlayingInfoCenter` and related now-playing metadata types.

use core::ffi::c_void;
use std::ffi::CString;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::artwork::{AnimatedArtwork, Artwork};
use crate::{ffi, unsupported, MediaPlayerError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
#[non_exhaustive]
/// Maps to `MPNowPlayingInfoMediaType`.
pub enum NowPlayingMediaType {
    #[default]
    None = 0,
    Audio = 1,
    Video = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
#[non_exhaustive]
/// Maps to `MPNowPlayingPlaybackState`.
pub enum PlaybackState {
    #[default]
    Unknown = 0,
    Playing = 1,
    Paused = 2,
    Stopped = 3,
    Interrupted = 4,
}

impl PlaybackState {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Playing,
            2 => Self::Paused,
            3 => Self::Stopped,
            4 => Self::Interrupted,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
/// Maps to `MPNowPlayingInfoLanguageOptionType`.
pub enum LanguageOptionType {
    #[default]
    Audible = 0,
    Legible = 1,
}

impl LanguageOptionType {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Legible,
            _ => Self::Audible,
        }
    }
}

/// Owned wrapper around `MPNowPlayingInfoLanguageOption`.
pub struct LanguageOption {
    pub(crate) ptr: *mut c_void,
}

// SAFETY: `MPNowPlayingInfoLanguageOption` is immutable and reference-counted.
unsafe impl Send for LanguageOption {}
unsafe impl Sync for LanguageOption {}

impl std::fmt::Debug for LanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageOption")
            .field("type", &self.language_option_type())
            .field("language_tag", &self.language_tag())
            .field("display_name", &self.display_name())
            .field("identifier", &self.identifier())
            .finish()
    }
}

impl Clone for LanguageOption {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::mp_object_retain(self.ptr) };
        Self { ptr }
    }
}

impl LanguageOption {
    /// Create a language option.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] if a string contains an
    /// interior NUL byte, or [`MediaPlayerError::Framework`] if Swift refuses to
    /// construct the option.
    pub fn new(
        language_option_type: LanguageOptionType,
        language_tag: Option<&str>,
        characteristics: &[&str],
        display_name: &str,
        identifier: &str,
    ) -> Result<Self, MediaPlayerError> {
        let display_name = CString::new(display_name)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let identifier = CString::new(identifier)
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let language_tag = language_tag
            .map(CString::new)
            .transpose()
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let characteristics = characteristics
            .iter()
            .map(|value| CString::new(*value))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))?;
        let characteristic_ptrs = characteristics
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();

        let ptr = unsafe {
            ffi::mp_language_option_new(
                language_option_type as i32,
                language_tag
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                if characteristic_ptrs.is_empty() {
                    std::ptr::null()
                } else {
                    characteristic_ptrs.as_ptr()
                },
                characteristic_ptrs.len(),
                display_name.as_ptr(),
                identifier.as_ptr(),
            )
        };

        if ptr.is_null() {
            Err(MediaPlayerError::Framework(
                "failed to create MPNowPlayingInfoLanguageOption".to_string(),
            ))
        } else {
            Ok(Self { ptr })
        }
    }

    #[must_use]
    pub fn language_option_type(&self) -> LanguageOptionType {
        LanguageOptionType::from_raw(unsafe { ffi::mp_language_option_get_type(self.ptr) })
    }

    #[must_use]
    pub fn language_tag(&self) -> Option<String> {
        unsafe { unsupported::take_string(ffi::mp_language_option_copy_language_tag(self.ptr)) }
    }

    #[must_use]
    pub fn characteristics(&self) -> Vec<String> {
        copy_lines(unsafe { ffi::mp_language_option_copy_characteristics(self.ptr) })
    }

    #[must_use]
    pub fn display_name(&self) -> Option<String> {
        unsafe { unsupported::take_string(ffi::mp_language_option_copy_display_name(self.ptr)) }
    }

    #[must_use]
    pub fn identifier(&self) -> Option<String> {
        unsafe { unsupported::take_string(ffi::mp_language_option_copy_identifier(self.ptr)) }
    }

    #[must_use]
    pub fn is_automatic_legible_language_option(&self) -> bool {
        unsafe { ffi::mp_language_option_is_automatic_legible(self.ptr) != 0 }
    }

    #[must_use]
    pub fn is_automatic_audible_language_option(&self) -> bool {
        unsafe { ffi::mp_language_option_is_automatic_audible(self.ptr) != 0 }
    }

    /// Create a language option from an `AVMediaSelectionOption` raw pointer.
    ///
    /// # Safety
    /// `option` must point to a live `AVMediaSelectionOption` object.
    #[must_use]
    pub unsafe fn from_av_media_selection_option_raw(option: *mut c_void) -> Option<Self> {
        let ptr = ffi::mp_language_option_new_from_media_selection_option(option);
        (!ptr.is_null()).then_some(Self { ptr })
    }

    pub(crate) unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for LanguageOption {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_language_option_release(self.ptr) }
        }
    }
}

/// Owned wrapper around `MPNowPlayingInfoLanguageOptionGroup`.
pub struct LanguageOptionGroup {
    pub(crate) ptr: *mut c_void,
}

// SAFETY: `MPNowPlayingInfoLanguageOptionGroup` is immutable and reference-counted.
unsafe impl Send for LanguageOptionGroup {}
unsafe impl Sync for LanguageOptionGroup {}

impl std::fmt::Debug for LanguageOptionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageOptionGroup")
            .field("count", &self.count())
            .field(
                "default_language_option_index",
                &self.default_language_option_index(),
            )
            .field("allow_empty_selection", &self.allow_empty_selection())
            .finish()
    }
}

impl Clone for LanguageOptionGroup {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::mp_object_retain(self.ptr) };
        Self { ptr }
    }
}

impl LanguageOptionGroup {
    /// Create a mutually-exclusive language option group.
    ///
    /// # Errors
    /// Returns [`MediaPlayerError::InvalidArgument`] when `language_options` is
    /// empty or the requested default index is out of bounds.
    pub fn new(
        language_options: &[LanguageOption],
        default_language_option_index: Option<usize>,
        allow_empty_selection: bool,
    ) -> Result<Self, MediaPlayerError> {
        if language_options.is_empty() {
            return Err(MediaPlayerError::InvalidArgument(
                "language option groups must contain at least one option".to_string(),
            ));
        }

        if let Some(index) = default_language_option_index {
            if index >= language_options.len() {
                return Err(MediaPlayerError::InvalidArgument(format!(
                    "default language option index {index} is out of bounds"
                )));
            }
        }

        let default_index = default_language_option_index
            .map(|index| {
                i32::try_from(index).map_err(|_| {
                    MediaPlayerError::InvalidArgument(
                        "default language option index does not fit in i32".to_string(),
                    )
                })
            })
            .transpose()?
            .unwrap_or(-1);

        let option_ptrs = language_options
            .iter()
            .map(|option| option.ptr)
            .collect::<Vec<_>>();
        let ptr = unsafe {
            ffi::mp_language_option_group_new(
                option_ptrs.as_ptr(),
                option_ptrs.len(),
                default_index,
                i32::from(allow_empty_selection),
            )
        };

        if ptr.is_null() {
            Err(MediaPlayerError::Framework(
                "failed to create MPNowPlayingInfoLanguageOptionGroup".to_string(),
            ))
        } else {
            Ok(Self { ptr })
        }
    }

    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::mp_language_option_group_get_count(self.ptr) }
    }

    #[must_use]
    pub fn default_language_option_index(&self) -> Option<usize> {
        let raw = unsafe { ffi::mp_language_option_group_get_default_index(self.ptr) };
        usize::try_from(raw).ok()
    }

    #[must_use]
    pub fn allow_empty_selection(&self) -> bool {
        unsafe { ffi::mp_language_option_group_allows_empty_selection(self.ptr) != 0 }
    }

    /// Create a language option group from an `AVMediaSelectionGroup` raw pointer.
    ///
    /// # Safety
    /// `group` must point to a live `AVMediaSelectionGroup` object.
    #[must_use]
    pub unsafe fn from_av_media_selection_group_raw(group: *mut c_void) -> Option<Self> {
        let ptr = ffi::mp_language_option_group_new_from_media_selection_group(group);
        (!ptr.is_null()).then_some(Self { ptr })
    }
}

impl Drop for LanguageOptionGroup {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mp_language_option_group_release(self.ptr) }
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Metadata pushed to `MPNowPlayingInfoCenter.nowPlayingInfo`.
pub struct NowPlayingInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album_title: Option<String>,
    pub playback_duration: Option<f64>,
    pub elapsed_playback_time: Option<f64>,
    pub playback_rate: Option<f64>,
    pub default_playback_rate: Option<f64>,
    pub playback_queue_index: Option<u64>,
    pub playback_queue_count: Option<u64>,
    pub chapter_number: Option<u64>,
    pub chapter_count: Option<u64>,
    pub is_live_stream: Option<bool>,
    pub available_language_option_groups: Vec<LanguageOptionGroup>,
    pub current_language_options: Vec<LanguageOption>,
    pub collection_identifier: Option<String>,
    pub external_content_identifier: Option<String>,
    pub external_user_profile_identifier: Option<String>,
    pub service_identifier: Option<String>,
    pub playback_progress: Option<f64>,
    pub media_type: Option<NowPlayingMediaType>,
    pub asset_url: Option<String>,
    pub current_playback_date: Option<SystemTime>,
    pub credits_start_time: Option<f64>,
    pub international_standard_recording_code: Option<String>,
    pub exclude_from_suggestions: Option<bool>,
    pub animated_artwork_1x1: Option<AnimatedArtwork>,
    pub animated_artwork_3x4: Option<AnimatedArtwork>,
}

impl NowPlayingInfo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn artist(mut self, artist: impl Into<String>) -> Self {
        self.artist = Some(artist.into());
        self
    }

    #[must_use]
    pub fn album_title(mut self, album_title: impl Into<String>) -> Self {
        self.album_title = Some(album_title.into());
        self
    }

    #[must_use]
    pub fn playback_duration(mut self, seconds: f64) -> Self {
        self.playback_duration = Some(seconds);
        self
    }

    #[must_use]
    pub fn elapsed_playback_time(mut self, seconds: f64) -> Self {
        self.elapsed_playback_time = Some(seconds);
        self
    }

    #[must_use]
    pub fn playback_rate(mut self, rate: f64) -> Self {
        self.playback_rate = Some(rate);
        self
    }

    #[must_use]
    pub fn default_playback_rate(mut self, rate: f64) -> Self {
        self.default_playback_rate = Some(rate);
        self
    }

    #[must_use]
    pub fn playback_queue_index(mut self, index: u64) -> Self {
        self.playback_queue_index = Some(index);
        self
    }

    #[must_use]
    pub fn playback_queue_count(mut self, count: u64) -> Self {
        self.playback_queue_count = Some(count);
        self
    }

    #[must_use]
    pub fn chapter_number(mut self, number: u64) -> Self {
        self.chapter_number = Some(number);
        self
    }

    #[must_use]
    pub fn chapter_count(mut self, count: u64) -> Self {
        self.chapter_count = Some(count);
        self
    }

    #[must_use]
    pub fn live_stream(mut self, is_live_stream: bool) -> Self {
        self.is_live_stream = Some(is_live_stream);
        self
    }

    #[must_use]
    pub fn available_language_option_groups<I>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = LanguageOptionGroup>,
    {
        self.available_language_option_groups = groups.into_iter().collect();
        self
    }

    #[must_use]
    pub fn current_language_options<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = LanguageOption>,
    {
        self.current_language_options = options.into_iter().collect();
        self
    }

    #[must_use]
    pub fn collection_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.collection_identifier = Some(identifier.into());
        self
    }

    #[must_use]
    pub fn external_content_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.external_content_identifier = Some(identifier.into());
        self
    }

    #[must_use]
    pub fn external_user_profile_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.external_user_profile_identifier = Some(identifier.into());
        self
    }

    #[must_use]
    pub fn service_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.service_identifier = Some(identifier.into());
        self
    }

    #[must_use]
    pub fn playback_progress(mut self, progress: f64) -> Self {
        self.playback_progress = Some(progress);
        self
    }

    #[must_use]
    pub fn media_type(mut self, media_type: NowPlayingMediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    #[must_use]
    pub fn asset_url(mut self, url: impl Into<String>) -> Self {
        self.asset_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn current_playback_date(mut self, date: SystemTime) -> Self {
        self.current_playback_date = Some(date);
        self
    }

    #[must_use]
    pub fn credits_start_time(mut self, seconds: f64) -> Self {
        self.credits_start_time = Some(seconds);
        self
    }

    #[must_use]
    pub fn international_standard_recording_code(mut self, code: impl Into<String>) -> Self {
        self.international_standard_recording_code = Some(code.into());
        self
    }

    #[must_use]
    pub fn exclude_from_suggestions(mut self, exclude: bool) -> Self {
        self.exclude_from_suggestions = Some(exclude);
        self
    }

    #[must_use]
    pub fn animated_artwork_1x1(mut self, artwork: AnimatedArtwork) -> Self {
        self.animated_artwork_1x1 = Some(artwork);
        self
    }

    #[must_use]
    pub fn animated_artwork_3x4(mut self, artwork: AnimatedArtwork) -> Self {
        self.animated_artwork_3x4 = Some(artwork);
        self
    }
}

#[derive(Debug)]
/// Safe wrapper around `MPNowPlayingInfoCenter.default()`.
pub struct NowPlayingInfoCenter {
    _private: (),
}

impl NowPlayingInfoCenter {
    #[must_use]
    pub fn default_center() -> Self {
        Self { _private: () }
    }

    pub fn set_now_playing_info(&self, info: &NowPlayingInfo) {
        self.set_now_playing_info_with_artwork(info, None);
    }

    #[allow(clippy::too_many_lines)]
    pub fn set_now_playing_info_with_artwork(
        &self,
        info: &NowPlayingInfo,
        artwork: Option<&Artwork>,
    ) {
        let info_box = unsafe { ffi::mp_now_playing_info_box_new() };
        if info_box.is_null() {
            return;
        }

        let mk = |value: &str| CString::new(value).unwrap_or_default();

        unsafe {
            if let Some(value) = info.title.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::Title as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.artist.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::Artist as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.album_title.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::AlbumTitle as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.playback_duration {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::PlaybackDuration as i32,
                    value,
                );
            }
            if let Some(value) = info.elapsed_playback_time {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::ElapsedPlaybackTime as i32,
                    value,
                );
            }
            if let Some(value) = info.playback_rate {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::PlaybackRate as i32,
                    value,
                );
            }
            if let Some(value) = info.default_playback_rate {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::DefaultPlaybackRate as i32,
                    value,
                );
            }
            if let Some(value) = info.playback_queue_index {
                ffi::mp_now_playing_info_box_set_u64(
                    info_box,
                    NowPlayingKey::PlaybackQueueIndex as i32,
                    value,
                );
            }
            if let Some(value) = info.playback_queue_count {
                ffi::mp_now_playing_info_box_set_u64(
                    info_box,
                    NowPlayingKey::PlaybackQueueCount as i32,
                    value,
                );
            }
            if let Some(value) = info.chapter_number {
                ffi::mp_now_playing_info_box_set_u64(
                    info_box,
                    NowPlayingKey::ChapterNumber as i32,
                    value,
                );
            }
            if let Some(value) = info.chapter_count {
                ffi::mp_now_playing_info_box_set_u64(
                    info_box,
                    NowPlayingKey::ChapterCount as i32,
                    value,
                );
            }
            if let Some(value) = info.is_live_stream {
                ffi::mp_now_playing_info_box_set_bool(
                    info_box,
                    NowPlayingKey::IsLiveStream as i32,
                    i32::from(value),
                );
            }
            if let Some(value) = info.collection_identifier.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::CollectionIdentifier as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.external_content_identifier.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::ExternalContentIdentifier as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.external_user_profile_identifier.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::ExternalUserProfileIdentifier as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.service_identifier.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::ServiceIdentifier as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.playback_progress {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::PlaybackProgress as i32,
                    value,
                );
            }
            if let Some(value) = info.media_type {
                ffi::mp_now_playing_info_box_set_u64(
                    info_box,
                    NowPlayingKey::MediaType as i32,
                    value as u64,
                );
            }
            if let Some(value) = info.asset_url.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_url(
                    info_box,
                    NowPlayingKey::AssetURL as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.current_playback_date {
                ffi::mp_now_playing_info_box_set_date_seconds(
                    info_box,
                    NowPlayingKey::CurrentPlaybackDate as i32,
                    system_time_to_unix_seconds(value),
                );
            }
            if let Some(value) = info.credits_start_time {
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::CreditsStartTime as i32,
                    value,
                );
            }
            if let Some(value) = info.international_standard_recording_code.as_deref() {
                let value = mk(value);
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::InternationalStandardRecordingCode as i32,
                    value.as_ptr(),
                );
            }
            if let Some(value) = info.exclude_from_suggestions {
                ffi::mp_now_playing_info_box_set_bool(
                    info_box,
                    NowPlayingKey::ExcludeFromSuggestions as i32,
                    i32::from(value),
                );
            }
            if let Some(artwork) = artwork {
                ffi::mp_now_playing_info_box_set_artwork(info_box, artwork.ptr);
            }
            if let Some(animated_artwork) = info.animated_artwork_1x1.as_ref() {
                ffi::mp_now_playing_info_box_set_animated_artwork(
                    info_box,
                    NowPlayingKey::AnimatedArtwork1x1 as i32,
                    animated_artwork.ptr,
                );
            }
            if let Some(animated_artwork) = info.animated_artwork_3x4.as_ref() {
                ffi::mp_now_playing_info_box_set_animated_artwork(
                    info_box,
                    NowPlayingKey::AnimatedArtwork3x4 as i32,
                    animated_artwork.ptr,
                );
            }
            if !info.available_language_option_groups.is_empty() {
                let group_ptrs = info
                    .available_language_option_groups
                    .iter()
                    .map(|group| group.ptr)
                    .collect::<Vec<_>>();
                ffi::mp_now_playing_info_box_set_available_language_option_groups(
                    info_box,
                    group_ptrs.as_ptr(),
                    group_ptrs.len(),
                );
            }
            if !info.current_language_options.is_empty() {
                let option_ptrs = info
                    .current_language_options
                    .iter()
                    .map(|option| option.ptr)
                    .collect::<Vec<_>>();
                ffi::mp_now_playing_info_box_set_current_language_options(
                    info_box,
                    option_ptrs.as_ptr(),
                    option_ptrs.len(),
                );
            }
            ffi::mp_now_playing_apply_info_box(info_box);
            ffi::mp_now_playing_info_box_release(info_box);
        }
    }

    pub fn clear(&self) {
        unsafe { ffi::mp_now_playing_clear() }
    }

    pub fn set_playback_state(&self, state: PlaybackState) {
        unsafe { ffi::mp_now_playing_set_playback_state(state as i32) }
    }

    #[must_use]
    pub fn playback_state(&self) -> PlaybackState {
        PlaybackState::from_raw(unsafe { ffi::mp_now_playing_get_playback_state() })
    }

    #[must_use]
    pub fn supported_animated_artwork_keys(&self) -> Vec<String> {
        copy_lines(unsafe { ffi::mp_now_playing_copy_supported_animated_artwork_keys() })
    }
}

impl Drop for NowPlayingInfoCenter {
    fn drop(&mut self) {
        unsafe { ffi::mp_now_playing_clear() }
    }
}

#[repr(i32)]
enum NowPlayingKey {
    Title = 0,
    Artist = 1,
    AlbumTitle = 2,
    PlaybackDuration = 3,
    ElapsedPlaybackTime = 4,
    PlaybackRate = 5,
    DefaultPlaybackRate = 6,
    PlaybackQueueIndex = 7,
    PlaybackQueueCount = 8,
    ChapterNumber = 9,
    ChapterCount = 10,
    IsLiveStream = 11,
    CollectionIdentifier = 12,
    ExternalContentIdentifier = 13,
    ExternalUserProfileIdentifier = 14,
    ServiceIdentifier = 15,
    PlaybackProgress = 16,
    MediaType = 17,
    AssetURL = 18,
    CurrentPlaybackDate = 19,
    CreditsStartTime = 20,
    InternationalStandardRecordingCode = 21,
    ExcludeFromSuggestions = 22,
    AnimatedArtwork1x1 = 23,
    AnimatedArtwork3x4 = 24,
}

fn copy_lines(ptr: *mut core::ffi::c_char) -> Vec<String> {
    unsafe { unsupported::take_string(ptr) }
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn system_time_to_unix_seconds(value: SystemTime) -> f64 {
    value
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs_f64()
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_void};
    use std::ffi::CString;

    use super::{LanguageOption, LanguageOptionGroup, LanguageOptionType};

    unsafe extern "C" {
        fn mp_test_media_selection_option_new(
            kind: i32,
            language_tag: *const c_char,
            display_name: *const c_char,
            identifier: *const c_char,
        ) -> *mut c_void;
        fn mp_test_media_selection_option_release(option: *mut c_void);
        fn mp_test_media_selection_group_new(
            options: *const *mut c_void,
            count: usize,
            default_index: i32,
            allow_empty_selection: i32,
        ) -> *mut c_void;
        fn mp_test_media_selection_group_release(group: *mut c_void);
    }

    #[test]
    fn av_media_selection_option_bridge_creates_language_option() {
        let language_tag = CString::new("en").expect("language tag should be valid");
        let display_name = CString::new("English Audio").expect("display name should be valid");
        let identifier = CString::new("audio-en").expect("identifier should be valid");
        let option = unsafe {
            mp_test_media_selection_option_new(
                0,
                language_tag.as_ptr(),
                display_name.as_ptr(),
                identifier.as_ptr(),
            )
        };
        assert!(
            !option.is_null(),
            "fixture AVMediaSelectionOption should be created"
        );

        let language_option = unsafe { LanguageOption::from_av_media_selection_option_raw(option) }
            .expect("audible AVMediaSelectionOption should convert to a language option");
        assert_eq!(
            language_option.language_option_type(),
            LanguageOptionType::Audible
        );
        assert_eq!(language_option.language_tag().as_deref(), Some("en"));
        assert_eq!(
            language_option.display_name().as_deref(),
            Some("English Audio")
        );
        assert!(language_option.identifier().is_some());

        unsafe { mp_test_media_selection_option_release(option) };
    }

    #[test]
    fn av_media_selection_option_bridge_ignores_non_language_media() {
        let display_name = CString::new("Video").expect("display name should be valid");
        let identifier = CString::new("video-main").expect("identifier should be valid");
        let option = unsafe {
            mp_test_media_selection_option_new(
                2,
                std::ptr::null(),
                display_name.as_ptr(),
                identifier.as_ptr(),
            )
        };
        assert!(
            !option.is_null(),
            "fixture AVMediaSelectionOption should be created"
        );
        assert!(unsafe { LanguageOption::from_av_media_selection_option_raw(option) }.is_none());
        unsafe { mp_test_media_selection_option_release(option) };
    }

    #[test]
    fn av_media_selection_group_bridge_creates_language_option_group() {
        let english = unsafe {
            let display_name =
                CString::new("English Subtitles").expect("display name should be valid");
            let identifier = CString::new("subtitles-en").expect("identifier should be valid");
            let language_tag = CString::new("en").expect("language tag should be valid");
            mp_test_media_selection_option_new(
                1,
                language_tag.as_ptr(),
                display_name.as_ptr(),
                identifier.as_ptr(),
            )
        };
        let swedish = unsafe {
            let display_name =
                CString::new("Swedish Subtitles").expect("display name should be valid");
            let identifier = CString::new("subtitles-sv").expect("identifier should be valid");
            let language_tag = CString::new("sv").expect("language tag should be valid");
            mp_test_media_selection_option_new(
                1,
                language_tag.as_ptr(),
                display_name.as_ptr(),
                identifier.as_ptr(),
            )
        };
        let options = [english, swedish];
        let group =
            unsafe { mp_test_media_selection_group_new(options.as_ptr(), options.len(), 1, 1) };
        assert!(
            !group.is_null(),
            "fixture AVMediaSelectionGroup should be created"
        );

        let language_group =
            unsafe { LanguageOptionGroup::from_av_media_selection_group_raw(group) }
                .expect("AVMediaSelectionGroup should convert to a language option group");
        assert_eq!(language_group.count(), 2);
        assert_eq!(language_group.default_language_option_index(), Some(1));
        assert!(language_group.allow_empty_selection());

        unsafe {
            mp_test_media_selection_group_release(group);
            mp_test_media_selection_option_release(english);
            mp_test_media_selection_option_release(swedish);
        }
    }
}
