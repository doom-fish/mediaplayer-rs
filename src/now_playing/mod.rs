//! Wrapper for `MPNowPlayingInfoCenter` and associated types.

use std::ffi::CString;
use std::ptr;

use crate::artwork::Artwork;
use crate::ffi;

// ── Media type ────────────────────────────────────────────────────────────────

/// Maps to `MPNowPlayingInfoMediaType` (audio/video classification for the
/// system Now Playing widget).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
#[non_exhaustive]
pub enum NowPlayingMediaType {
    #[default]
    None = 0,
    Audio = 1,
    Video = 2,
}

// ── Playback state ────────────────────────────────────────────────────────────

/// Maps to `MPNowPlayingPlaybackState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
#[non_exhaustive]
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

// ── NowPlayingInfo ────────────────────────────────────────────────────────────

/// Metadata pushed to `MPNowPlayingInfoCenter.nowPlayingInfo`.
///
/// Build with the fluent setter methods, then pass to
/// [`NowPlayingInfoCenter::set_now_playing_info`].
///
/// # Example
/// ```no_run
/// use mediaplayer::NowPlayingInfo;
///
/// let info = NowPlayingInfo::new()
///     .title("My Song")
///     .artist("doom-fish")
///     .playback_duration(240.0)
///     .elapsed_playback_time(0.0)
///     .playback_rate(1.0);
/// ```
#[derive(Debug, Clone, Default)]
pub struct NowPlayingInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album_title: Option<String>,
    pub playback_duration: Option<f64>,
    pub elapsed_playback_time: Option<f64>,
    pub playback_rate: Option<f64>,
    pub media_type: Option<NowPlayingMediaType>,
    pub external_content_identifier: Option<String>,
    pub asset_url: Option<String>,
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
    pub fn album_title(mut self, album: impl Into<String>) -> Self {
        self.album_title = Some(album.into());
        self
    }

    #[must_use]
    pub fn playback_duration(mut self, secs: f64) -> Self {
        self.playback_duration = Some(secs);
        self
    }

    #[must_use]
    pub fn elapsed_playback_time(mut self, secs: f64) -> Self {
        self.elapsed_playback_time = Some(secs);
        self
    }

    #[must_use]
    pub fn playback_rate(mut self, rate: f64) -> Self {
        self.playback_rate = Some(rate);
        self
    }

    #[must_use]
    pub fn media_type(mut self, mt: NowPlayingMediaType) -> Self {
        self.media_type = Some(mt);
        self
    }

    #[must_use]
    pub fn external_content_identifier(mut self, id: impl Into<String>) -> Self {
        self.external_content_identifier = Some(id.into());
        self
    }

    #[must_use]
    pub fn asset_url(mut self, url: impl Into<String>) -> Self {
        self.asset_url = Some(url.into());
        self
    }
}

// ── NowPlayingInfoCenter ──────────────────────────────────────────────────────

/// Safe wrapper around `MPNowPlayingInfoCenter.default()`.
///
/// Clears `nowPlayingInfo` on [`Drop`] to avoid leaving stale system state.
#[derive(Debug)]
pub struct NowPlayingInfoCenter {
    _private: (),
}

impl NowPlayingInfoCenter {
    /// Obtain a handle to the default now-playing center.
    #[must_use]
    pub fn default_center() -> Self {
        Self { _private: () }
    }

    /// Push `info` to the system without artwork.
    pub fn set_now_playing_info(&self, info: &NowPlayingInfo) {
        self.set_now_playing_info_with_artwork(info, None);
    }

    /// Push `info` to the system, optionally attaching album artwork.
    pub fn set_now_playing_info_with_artwork(&self, info: &NowPlayingInfo, artwork: Option<&Artwork>) {
        let mk = |s: &str| CString::new(s).unwrap_or_default();

        let title = info.title.as_deref().map(mk);
        let artist = info.artist.as_deref().map(mk);
        let album = info.album_title.as_deref().map(mk);
        let content_id = info.external_content_identifier.as_deref().map(mk);
        let asset_url = info.asset_url.as_deref().map(mk);

        unsafe {
            ffi::mp_now_playing_set_info(
                title.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                artist.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                album.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                info.playback_duration.unwrap_or(-1.0),
                info.elapsed_playback_time.unwrap_or(-1.0),
                info.playback_rate.unwrap_or(-1.0),
                info.media_type.map_or(-1, |m| m as i32),
                content_id.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                asset_url.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                artwork.map_or(ptr::null_mut(), |a| a.ptr),
            );
        }
    }

    /// Set `nowPlayingInfo = nil` immediately, clearing all system UI.
    pub fn clear(&self) {
        unsafe { ffi::mp_now_playing_clear() }
    }

    /// Update the playback state reported to the system.
    pub fn set_playback_state(&self, state: PlaybackState) {
        unsafe { ffi::mp_now_playing_set_playback_state(state as i32) }
    }

    /// Read back the current playback state.
    #[must_use]
    pub fn playback_state(&self) -> PlaybackState {
        let raw = unsafe { ffi::mp_now_playing_get_playback_state() };
        PlaybackState::from_raw(raw)
    }
}

impl Drop for NowPlayingInfoCenter {
    fn drop(&mut self) {
        unsafe { ffi::mp_now_playing_clear() }
    }
}
