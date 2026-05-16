//! Symbolic now-playing metadata key names exposed by `MediaPlayer.framework`.
//!
//! The Swift bridge resolves the real framework constants; these Rust constants
//! are documentation-friendly labels for the supported fields on
//! [`crate::NowPlayingInfo`].

/// `MPMediaItemPropertyTitle`
pub const TITLE: &str = "MPMediaItemPropertyTitle";
/// `MPMediaItemPropertyArtist`
pub const ARTIST: &str = "MPMediaItemPropertyArtist";
/// `MPMediaItemPropertyAlbumTitle`
pub const ALBUM_TITLE: &str = "MPMediaItemPropertyAlbumTitle";
/// `MPMediaItemPropertyPlaybackDuration`
pub const PLAYBACK_DURATION: &str = "MPMediaItemPropertyPlaybackDuration";
/// `MPMediaItemPropertyArtwork`
pub const ARTWORK: &str = "MPMediaItemPropertyArtwork";
/// `MPNowPlayingInfoPropertyElapsedPlaybackTime`
pub const ELAPSED_PLAYBACK_TIME: &str = "MPNowPlayingInfoPropertyElapsedPlaybackTime";
/// `MPNowPlayingInfoPropertyPlaybackRate`
pub const PLAYBACK_RATE: &str = "MPNowPlayingInfoPropertyPlaybackRate";
/// `MPNowPlayingInfoPropertyDefaultPlaybackRate`
pub const DEFAULT_PLAYBACK_RATE: &str = "MPNowPlayingInfoPropertyDefaultPlaybackRate";
/// `MPNowPlayingInfoPropertyPlaybackQueueIndex`
pub const PLAYBACK_QUEUE_INDEX: &str = "MPNowPlayingInfoPropertyPlaybackQueueIndex";
/// `MPNowPlayingInfoPropertyPlaybackQueueCount`
pub const PLAYBACK_QUEUE_COUNT: &str = "MPNowPlayingInfoPropertyPlaybackQueueCount";
/// `MPNowPlayingInfoPropertyChapterNumber`
pub const CHAPTER_NUMBER: &str = "MPNowPlayingInfoPropertyChapterNumber";
/// `MPNowPlayingInfoPropertyChapterCount`
pub const CHAPTER_COUNT: &str = "MPNowPlayingInfoPropertyChapterCount";
/// `MPNowPlayingInfoPropertyIsLiveStream`
pub const IS_LIVE_STREAM: &str = "MPNowPlayingInfoPropertyIsLiveStream";
/// `MPNowPlayingInfoPropertyAvailableLanguageOptions`
pub const AVAILABLE_LANGUAGE_OPTIONS: &str = "MPNowPlayingInfoPropertyAvailableLanguageOptions";
/// `MPNowPlayingInfoPropertyCurrentLanguageOptions`
pub const CURRENT_LANGUAGE_OPTIONS: &str = "MPNowPlayingInfoPropertyCurrentLanguageOptions";
/// `MPNowPlayingInfoCollectionIdentifier`
pub const COLLECTION_IDENTIFIER: &str = "MPNowPlayingInfoCollectionIdentifier";
/// `MPNowPlayingInfoPropertyExternalContentIdentifier`
pub const EXTERNAL_CONTENT_IDENTIFIER: &str = "MPNowPlayingInfoPropertyExternalContentIdentifier";
/// `MPNowPlayingInfoPropertyExternalUserProfileIdentifier`
pub const EXTERNAL_USER_PROFILE_IDENTIFIER: &str =
    "MPNowPlayingInfoPropertyExternalUserProfileIdentifier";
/// `MPNowPlayingInfoPropertyServiceIdentifier`
pub const SERVICE_IDENTIFIER: &str = "MPNowPlayingInfoPropertyServiceIdentifier";
/// `MPNowPlayingInfoPropertyPlaybackProgress`
pub const PLAYBACK_PROGRESS: &str = "MPNowPlayingInfoPropertyPlaybackProgress";
/// `MPNowPlayingInfoPropertyMediaType`
pub const MEDIA_TYPE: &str = "MPNowPlayingInfoPropertyMediaType";
/// `MPNowPlayingInfoPropertyAssetURL`
pub const ASSET_URL: &str = "MPNowPlayingInfoPropertyAssetURL";
/// `MPNowPlayingInfoPropertyCurrentPlaybackDate`
pub const CURRENT_PLAYBACK_DATE: &str = "MPNowPlayingInfoPropertyCurrentPlaybackDate";
/// `MPNowPlayingInfoPropertyCreditsStartTime`
pub const CREDITS_START_TIME: &str = "MPNowPlayingInfoPropertyCreditsStartTime";
/// `MPNowPlayingInfoPropertyInternationalStandardRecordingCode`
pub const INTERNATIONAL_STANDARD_RECORDING_CODE: &str =
    "MPNowPlayingInfoPropertyInternationalStandardRecordingCode";
/// `MPNowPlayingInfoPropertyExcludeFromSuggestions`
pub const EXCLUDE_FROM_SUGGESTIONS: &str = "MPNowPlayingInfoPropertyExcludeFromSuggestions";
