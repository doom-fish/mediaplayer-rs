//! `MPMediaType` plus the explicit macOS-unavailable `MPMediaItem` marker.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::{ffi, unsupported, MediaPlayerError};

/// Type alias for `MPMediaEntityPersistentID`.
pub type MediaEntityPersistentId = u64;

/// Bitflag wrapper around `MPMediaType`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaType(u64);

impl MediaType {
    pub const NONE: Self = Self(0);
    pub const MUSIC: Self = Self(1 << 0);
    pub const PODCAST: Self = Self(1 << 1);
    pub const AUDIOBOOK: Self = Self(1 << 2);
    pub const AUDIO_ITUNES_U: Self = Self(1 << 3);
    pub const ANY_AUDIO: Self = Self(0x00ff);
    pub const MOVIE: Self = Self(1 << 8);
    pub const TV_SHOW: Self = Self(1 << 9);
    pub const VIDEO_PODCAST: Self = Self(1 << 10);
    pub const MUSIC_VIDEO: Self = Self(1 << 11);
    pub const VIDEO_ITUNES_U: Self = Self(1 << 12);
    pub const HOME_VIDEO: Self = Self(1 << 13);
    pub const ANY_VIDEO: Self = Self(0xff00);
    pub const ANY: Self = Self(u64::MAX);

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

impl BitOr for MediaType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MediaType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for MediaType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for MediaType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Marker type representing `MPMediaItem`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MediaItem;

impl MediaItem {
    /// Returns `false` on macOS because Apple marks `MPMediaItem` unavailable.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::mp_media_item_is_supported() != 0 }
    }

    /// Returns the framework availability note for `MPMediaItem`.
    #[must_use]
    pub fn unavailable_reason() -> String {
        unsafe {
            unsupported::take_string(ffi::mp_media_item_copy_unavailable_reason())
                .unwrap_or_else(|| "MPMediaItem is unavailable on macOS".to_string())
        }
    }

    /// Returns a typed unavailability error for `MPMediaItem`.
    #[must_use]
    pub fn unsupported_error() -> MediaPlayerError {
        unsupported::not_available("MPMediaItem", Some(Self::unavailable_reason()))
    }

    /// Returns an explicit macOS-unavailable result for the primary constructor-like API.
    pub fn example_instance() -> Result<Self, MediaPlayerError> {
        Err(Self::unsupported_error())
    }
}
