//! MediaPlayer framework metadata key constants.
//!
//! These are the string values used as keys in the `nowPlayingInfo`
//! dictionary. They are reproduced here for documentation and completeness;
//! the actual dictionary construction is handled inside the Swift bridge, so
//! Rust callers normally do not need to use these directly.

// ── MPMediaItemProperty* ──────────────────────────────────────────────────────

/// `MPMediaItemPropertyTitle`
pub const TITLE: &str = "title";

/// `MPMediaItemPropertyArtist`
pub const ARTIST: &str = "artist";

/// `MPMediaItemPropertyAlbumTitle`
pub const ALBUM_TITLE: &str = "albumTitle";

/// `MPMediaItemPropertyPlaybackDuration`
pub const PLAYBACK_DURATION: &str = "playbackDuration";

/// `MPMediaItemPropertyArtwork`
pub const ARTWORK: &str = "artwork";

// ── MPNowPlayingInfoProperty* ─────────────────────────────────────────────────

/// `MPNowPlayingInfoPropertyElapsedPlaybackTime`
pub const ELAPSED_PLAYBACK_TIME: &str = "MPNowPlayingInfoPropertyElapsedPlaybackTime";

/// `MPNowPlayingInfoPropertyPlaybackRate`
pub const PLAYBACK_RATE: &str = "MPNowPlayingInfoPropertyPlaybackRate";

/// `MPNowPlayingInfoPropertyMediaType`
pub const MEDIA_TYPE: &str = "MPNowPlayingInfoPropertyMediaType";

/// `MPNowPlayingInfoPropertyExternalContentIdentifier`
pub const EXTERNAL_CONTENT_IDENTIFIER: &str =
    "MPNowPlayingInfoPropertyExternalContentIdentifier";

/// `MPNowPlayingInfoPropertyAssetURL`
pub const ASSET_URL: &str = "MPNowPlayingInfoPropertyAssetURL";
