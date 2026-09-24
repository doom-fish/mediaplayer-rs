//! Wrapper for `MPNowPlayingInfoCenter` and related now-playing metadata types.

use core::ffi::{c_char, c_int, c_void};
use std::collections::BTreeMap;
use std::ffi::CString;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::artwork::{AnimatedArtwork, Artwork};
use crate::{constants, ffi, unsupported, MediaPlayerError};

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

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum NowPlayingValue {
    String(String),
    Double(f64),
    Integer(i64),
    UnsignedInteger(u64),
    Bool(bool),
    Date(SystemTime),
    Url(String),
    Other(String),
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
    pub values: BTreeMap<String, NowPlayingValue>,
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
    pub fn value(mut self, key: impl Into<String>, value: NowPlayingValue) -> Self {
        self.values.insert(key.into(), value);
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

    pub fn set_now_playing_info(&self, info: &NowPlayingInfo) -> Result<(), MediaPlayerError> {
        self.set_now_playing_info_with_artwork(info, None)
    }

    pub fn set_now_playing_info_with_artwork(
        &self,
        info: &NowPlayingInfo,
        artwork: Option<&Artwork>,
    ) -> Result<(), MediaPlayerError> {
        let info_box = unsafe { ffi::mp_now_playing_info_box_new() };
        if info_box.is_null() {
            return Err(MediaPlayerError::Framework(
                "failed to allocate a now-playing info dictionary".to_string(),
            ));
        }
        let filled = unsafe { Self::fill_info_box(info_box, info, artwork) };
        unsafe {
            if filled.is_ok() {
                ffi::mp_now_playing_apply_info_box(info_box);
            }
            ffi::mp_now_playing_info_box_release(info_box);
        }
        filled
    }

    #[allow(clippy::too_many_lines)]
    unsafe fn fill_info_box(
        info_box: *mut c_void,
        info: &NowPlayingInfo,
        artwork: Option<&Artwork>,
    ) -> Result<(), MediaPlayerError> {
        let mk = |value: &str| {
            CString::new(value)
                .map_err(|error| MediaPlayerError::InvalidArgument(error.to_string()))
        };
        let set_string = |key: NowPlayingKey, value: &str| {
            let value = mk(value)?;
            now_playing_status(
                unsafe {
                    ffi::mp_now_playing_info_box_set_string(info_box, key as i32, value.as_ptr())
                },
                key.symbol(),
            )
        };
        let set_url = |key: NowPlayingKey, value: &str| {
            let value = mk(value)?;
            now_playing_status(
                unsafe {
                    ffi::mp_now_playing_info_box_set_url(info_box, key as i32, value.as_ptr())
                },
                key.symbol(),
            )
        };
        let set_double = |key: NowPlayingKey, value: f64| {
            now_playing_status(
                unsafe { ffi::mp_now_playing_info_box_set_double(info_box, key as i32, value) },
                key.symbol(),
            )
        };
        let set_u64 = |key: NowPlayingKey, value: u64| {
            now_playing_status(
                unsafe { ffi::mp_now_playing_info_box_set_u64(info_box, key as i32, value) },
                key.symbol(),
            )
        };
        let set_bool = |key: NowPlayingKey, value: bool| {
            now_playing_status(
                unsafe {
                    ffi::mp_now_playing_info_box_set_bool(info_box, key as i32, i32::from(value))
                },
                key.symbol(),
            )
        };
        let set_date = |key: NowPlayingKey, value: SystemTime| {
            now_playing_status(
                unsafe {
                    ffi::mp_now_playing_info_box_set_date_seconds(
                        info_box,
                        key as i32,
                        system_time_to_unix_seconds(value),
                    )
                },
                key.symbol(),
            )
        };
        let set_animated_artwork = |key: NowPlayingKey, artwork: &AnimatedArtwork| {
            now_playing_status(
                unsafe {
                    ffi::mp_now_playing_info_box_set_animated_artwork(
                        info_box,
                        key as i32,
                        artwork.ptr,
                    )
                },
                key.symbol(),
            )
        };

        if let Some(value) = info.title.as_deref() {
            set_string(NowPlayingKey::Title, value)?;
        }
        if let Some(value) = info.artist.as_deref() {
            set_string(NowPlayingKey::Artist, value)?;
        }
        if let Some(value) = info.album_title.as_deref() {
            set_string(NowPlayingKey::AlbumTitle, value)?;
        }
        if let Some(value) = info.playback_duration {
            set_double(NowPlayingKey::PlaybackDuration, value)?;
        }
        if let Some(value) = info.elapsed_playback_time {
            set_double(NowPlayingKey::ElapsedPlaybackTime, value)?;
        }
        if let Some(value) = info.playback_rate {
            set_double(NowPlayingKey::PlaybackRate, value)?;
        }
        if let Some(value) = info.default_playback_rate {
            set_double(NowPlayingKey::DefaultPlaybackRate, value)?;
        }
        if let Some(value) = info.playback_queue_index {
            set_u64(NowPlayingKey::PlaybackQueueIndex, value)?;
        }
        if let Some(value) = info.playback_queue_count {
            set_u64(NowPlayingKey::PlaybackQueueCount, value)?;
        }
        if let Some(value) = info.chapter_number {
            set_u64(NowPlayingKey::ChapterNumber, value)?;
        }
        if let Some(value) = info.chapter_count {
            set_u64(NowPlayingKey::ChapterCount, value)?;
        }
        if let Some(value) = info.is_live_stream {
            set_bool(NowPlayingKey::IsLiveStream, value)?;
        }
        if let Some(value) = info.collection_identifier.as_deref() {
            set_string(NowPlayingKey::CollectionIdentifier, value)?;
        }
        if let Some(value) = info.external_content_identifier.as_deref() {
            set_string(NowPlayingKey::ExternalContentIdentifier, value)?;
        }
        if let Some(value) = info.external_user_profile_identifier.as_deref() {
            set_string(NowPlayingKey::ExternalUserProfileIdentifier, value)?;
        }
        if let Some(value) = info.service_identifier.as_deref() {
            set_string(NowPlayingKey::ServiceIdentifier, value)?;
        }
        if let Some(value) = info.playback_progress {
            set_double(NowPlayingKey::PlaybackProgress, value)?;
        }
        if let Some(value) = info.media_type {
            set_u64(NowPlayingKey::MediaType, value as u64)?;
        }
        if let Some(value) = info.asset_url.as_deref() {
            set_url(NowPlayingKey::AssetURL, value)?;
        }
        if let Some(value) = info.current_playback_date {
            set_date(NowPlayingKey::CurrentPlaybackDate, value)?;
        }
        if let Some(value) = info.credits_start_time {
            set_double(NowPlayingKey::CreditsStartTime, value)?;
        }
        if let Some(value) = info.international_standard_recording_code.as_deref() {
            set_string(NowPlayingKey::InternationalStandardRecordingCode, value)?;
        }
        if let Some(value) = info.exclude_from_suggestions {
            set_bool(NowPlayingKey::ExcludeFromSuggestions, value)?;
        }
        unsafe {
            if let Some(artwork) = artwork {
                ffi::mp_now_playing_info_box_set_artwork(info_box, artwork.ptr);
            }
        }
        if let Some(animated_artwork) = info.animated_artwork_1x1.as_ref() {
            set_animated_artwork(NowPlayingKey::AnimatedArtwork1x1, animated_artwork)?;
        }
        if let Some(animated_artwork) = info.animated_artwork_3x4.as_ref() {
            set_animated_artwork(NowPlayingKey::AnimatedArtwork3x4, animated_artwork)?;
        }
        unsafe {
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
            for (key, value) in &info.values {
                set_named_value(info_box, key, value)?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn now_playing_info(&self) -> Option<BTreeMap<String, NowPlayingValue>> {
        let snapshot = unsafe { ffi::mp_now_playing_info_snapshot() };
        if snapshot.is_null() {
            return None;
        }
        let count = unsafe { ffi::mp_now_playing_snapshot_count(snapshot) };
        let values = (0..count)
            .filter_map(|index| unsafe {
                let key = unsupported::take_string(ffi::mp_now_playing_snapshot_copy_key(
                    snapshot, index,
                ))?;
                let copy_string = || {
                    unsupported::take_string(ffi::mp_now_playing_snapshot_copy_string(
                        snapshot, index,
                    ))
                    .unwrap_or_default()
                };
                let value = match ffi::mp_now_playing_snapshot_kind(snapshot, index) {
                    0 => NowPlayingValue::String(copy_string()),
                    1 => NowPlayingValue::Double(ffi::mp_now_playing_snapshot_double(
                        snapshot, index,
                    )),
                    2 => NowPlayingValue::Integer(ffi::mp_now_playing_snapshot_int64(
                        snapshot, index,
                    )),
                    3 => NowPlayingValue::UnsignedInteger(ffi::mp_now_playing_snapshot_uint64(
                        snapshot, index,
                    )),
                    4 => NowPlayingValue::Bool(
                        ffi::mp_now_playing_snapshot_int64(snapshot, index) != 0,
                    ),
                    5 => NowPlayingValue::Date(unix_seconds_to_system_time(
                        ffi::mp_now_playing_snapshot_double(snapshot, index),
                    )?),
                    6 => NowPlayingValue::Url(copy_string()),
                    kind if kind >= 0 => NowPlayingValue::Other(copy_string()),
                    _ => return None,
                };
                Some((key, value))
            })
            .collect();
        unsafe { ffi::mp_now_playing_snapshot_release(snapshot) };
        Some(values)
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

#[derive(Clone, Copy)]
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

impl NowPlayingKey {
    const fn symbol(self) -> &'static str {
        match self {
            Self::Title => constants::TITLE,
            Self::Artist => constants::ARTIST,
            Self::AlbumTitle => constants::ALBUM_TITLE,
            Self::PlaybackDuration => constants::PLAYBACK_DURATION,
            Self::ElapsedPlaybackTime => constants::ELAPSED_PLAYBACK_TIME,
            Self::PlaybackRate => constants::PLAYBACK_RATE,
            Self::DefaultPlaybackRate => constants::DEFAULT_PLAYBACK_RATE,
            Self::PlaybackQueueIndex => constants::PLAYBACK_QUEUE_INDEX,
            Self::PlaybackQueueCount => constants::PLAYBACK_QUEUE_COUNT,
            Self::ChapterNumber => constants::CHAPTER_NUMBER,
            Self::ChapterCount => constants::CHAPTER_COUNT,
            Self::IsLiveStream => constants::IS_LIVE_STREAM,
            Self::CollectionIdentifier => constants::COLLECTION_IDENTIFIER,
            Self::ExternalContentIdentifier => constants::EXTERNAL_CONTENT_IDENTIFIER,
            Self::ExternalUserProfileIdentifier => constants::EXTERNAL_USER_PROFILE_IDENTIFIER,
            Self::ServiceIdentifier => constants::SERVICE_IDENTIFIER,
            Self::PlaybackProgress => constants::PLAYBACK_PROGRESS,
            Self::MediaType => constants::MEDIA_TYPE,
            Self::AssetURL => constants::ASSET_URL,
            Self::CurrentPlaybackDate => constants::CURRENT_PLAYBACK_DATE,
            Self::CreditsStartTime => constants::CREDITS_START_TIME,
            Self::InternationalStandardRecordingCode => {
                constants::INTERNATIONAL_STANDARD_RECORDING_CODE
            }
            Self::ExcludeFromSuggestions => constants::EXCLUDE_FROM_SUGGESTIONS,
            Self::AnimatedArtwork1x1 => constants::ANIMATED_ARTWORK_1X1,
            Self::AnimatedArtwork3x4 => constants::ANIMATED_ARTWORK_3X4,
        }
    }
}

fn now_playing_status(status: c_int, key: &str) -> Result<(), MediaPlayerError> {
    match status {
        0 => Ok(()),
        1 => Err(MediaPlayerError::InvalidArgument(format!(
            "{key} is not a now-playing key available on this system"
        ))),
        2 => Err(MediaPlayerError::InvalidArgument(format!(
            "{key} holds an object; set it through the typed NowPlayingInfo fields or artwork"
        ))),
        4 => Err(MediaPlayerError::NotAvailable(format!(
            "{key} needs a newer macOS than this system"
        ))),
        _ => Err(MediaPlayerError::InvalidArgument(format!(
            "{key}: the value is not valid for this key"
        ))),
    }
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

fn signed_unix_seconds(value: SystemTime) -> f64 {
    match value.duration_since(UNIX_EPOCH) {
        Ok(after) => after.as_secs_f64(),
        Err(before) => -before.duration().as_secs_f64(),
    }
}

fn unix_seconds_to_system_time(seconds: f64) -> Option<SystemTime> {
    let offset = Duration::try_from_secs_f64(seconds.abs()).ok()?;
    if seconds < 0.0 {
        UNIX_EPOCH.checked_sub(offset)
    } else {
        UNIX_EPOCH.checked_add(offset)
    }
}

unsafe fn set_named_value(
    info_box: *mut c_void,
    key: &str,
    value: &NowPlayingValue,
) -> Result<(), MediaPlayerError> {
    let invalid = |error: std::ffi::NulError| MediaPlayerError::InvalidArgument(error.to_string());
    let key_name = CString::new(key).map_err(invalid)?;
    let mut text = None;
    let (kind, double, int64, uint64) = match value {
        NowPlayingValue::String(value) => {
            text = Some(CString::new(value.as_str()).map_err(invalid)?);
            (0, 0.0, 0, 0)
        }
        NowPlayingValue::Double(value) => (1, *value, 0, 0),
        NowPlayingValue::Integer(value) => (2, 0.0, *value, 0),
        NowPlayingValue::UnsignedInteger(value) => (3, 0.0, 0, *value),
        NowPlayingValue::Bool(value) => (4, 0.0, i64::from(*value), 0),
        NowPlayingValue::Date(value) => (5, signed_unix_seconds(*value), 0, 0),
        NowPlayingValue::Url(value) => {
            text = Some(CString::new(value.as_str()).map_err(invalid)?);
            (6, 0.0, 0, 0)
        }
        NowPlayingValue::Other(_) => {
            return Err(MediaPlayerError::InvalidArgument(format!(
                "{key}: NowPlayingValue::Other is read-only"
            )))
        }
    };
    let text_ptr: *const c_char = text
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    let status = unsafe {
        ffi::mp_now_playing_info_box_set_named(
            info_box,
            key_name.as_ptr(),
            kind,
            text_ptr,
            double,
            int64,
            uint64,
        )
    };
    now_playing_status(status, key)
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_void};
    use std::ffi::CString;

    use super::{
        now_playing_status, LanguageOption, LanguageOptionGroup, LanguageOptionType, NowPlayingKey,
    };
    use crate::{ffi, MediaPlayerError};

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

    #[test]
    fn typed_setters_report_their_status() {
        let info_box = unsafe { ffi::mp_now_playing_info_box_new() };
        assert!(!info_box.is_null());
        let url = CString::new("https://example.com/track").expect("URL should be valid");
        let empty = CString::new("").expect("empty string should be valid");
        unsafe {
            assert_eq!(
                ffi::mp_now_playing_info_box_set_double(
                    info_box,
                    NowPlayingKey::PlaybackRate as i32,
                    1.0
                ),
                0
            );
            assert_eq!(
                ffi::mp_now_playing_info_box_set_double(info_box, 99, 1.0),
                1
            );
            assert_eq!(ffi::mp_now_playing_info_box_set_u64(info_box, -1, 1), 1);
            assert_eq!(
                ffi::mp_now_playing_info_box_set_url(
                    info_box,
                    NowPlayingKey::AssetURL as i32,
                    url.as_ptr()
                ),
                0
            );
            assert_eq!(
                ffi::mp_now_playing_info_box_set_url(
                    info_box,
                    NowPlayingKey::AssetURL as i32,
                    empty.as_ptr()
                ),
                3
            );
            assert_eq!(
                ffi::mp_now_playing_info_box_set_string(
                    info_box,
                    NowPlayingKey::Title as i32,
                    core::ptr::null()
                ),
                3
            );
            assert_eq!(
                ffi::mp_now_playing_info_box_set_animated_artwork(
                    info_box,
                    NowPlayingKey::AnimatedArtwork1x1 as i32,
                    core::ptr::null_mut()
                ),
                3
            );
            ffi::mp_now_playing_info_box_release(info_box);
        }
    }

    #[test]
    fn status_codes_map_to_errors() {
        assert_eq!(now_playing_status(0, "key"), Ok(()));
        let unavailable = now_playing_status(4, NowPlayingKey::CreditsStartTime.symbol());
        assert!(
            matches!(
                &unavailable,
                Err(MediaPlayerError::NotAvailable(message))
                    if message.contains("MPNowPlayingInfoPropertyCreditsStartTime")
            ),
            "{unavailable:?}"
        );
        for status in [1, 2, 3, -1] {
            assert!(matches!(
                now_playing_status(status, "key"),
                Err(MediaPlayerError::InvalidArgument(_))
            ));
        }
    }
}
