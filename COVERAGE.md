# MediaPlayer.framework coverage audit (macOS SDK 26.2)

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped — unavailable, deprecated, or intentionally deferred with reason

| Header / API | Status | Notes |
| --- | --- | --- |
| `MPNowPlayingInfoCenter`, `MPNowPlayingInfoMediaType`, `MPNowPlayingPlaybackState` | ✅ | `NowPlayingInfoCenter`, `NowPlayingInfo`, `NowPlayingMediaType`, and `PlaybackState` cover default center access, playback state, the supported metadata surface, and reading `nowPlayingInfo` back through `NowPlayingInfoCenter::now_playing_info`. |
| `MPNowPlayingInfoCenter.supportedAnimatedArtworkKeys` | ✅ | Exposed as `NowPlayingInfoCenter::supported_animated_artwork_keys()`. |
| Additional now-playing metadata keys through `MPNowPlayingInfoPropertyExcludeFromSuggestions` | ✅ | Title/artist/album, queue and chapter metadata, playback progress, live-stream state, service/profile identifiers, language options, playback date, credits start time, ISRC, and exclude-from-suggestions have typed `NowPlayingInfo` builders. Every `MPMediaItemProperty*` / `MPNowPlayingInfoProperty*` key in `constants` (composer, genre, album artist, track and disc numbers, persistent IDs, ...) is settable through `NowPlayingInfo::value`. |
| `MPNowPlayingInfoPropertyAdTimeRanges` / `MPAdTimeRange` | ⏭️ skipped | `MPAdTimeRange` is unavailable on macOS in the SDK, so the ad-time-range key cannot be constructed safely from Rust. |
| `MPMediaItemArtwork` constructors and `bounds` | ✅ | `Artwork::from_path`, `Artwork::from_path_with_size`, `Artwork::from_image_data` (in-memory image data), `Artwork::bounds`, `Clone`, and `Drop` are implemented. The request handler aspect-fits the image to the requested size. |
| `MPMediaItemArtwork.imageWithSize` | ✅ | `Artwork::image_png_data` returns the image the artwork produces for a size as PNG bytes. MediaPlayer clamps requests larger than the artwork bounds to the bounds. |
| `MPMediaItemAnimatedArtwork`, `MPNowPlayingInfoProperty1x1AnimatedArtwork`, `MPNowPlayingInfoProperty3x4AnimatedArtwork` | ✅ | `AnimatedArtwork::from_files` plus `NowPlayingInfo::{animated_artwork_1x1, animated_artwork_3x4}`. Requires macOS 26; construction returns an error on older systems. |
| `MPNowPlayingInfoLanguageOption`, `MPNowPlayingInfoLanguageOptionGroup` | ✅ | `LanguageOption` and `LanguageOptionGroup` cover creation, cloning, type inspection, identifiers, language tags, characteristics, and default-selection metadata. The `constants::LANGUAGE_OPTION_CHARACTERISTIC_*` names are translated to the SDK's characteristic values. |
| `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions` | 🟡 partial | `LanguageOption::from_av_media_selection_option_raw` and `LanguageOptionGroup::from_av_media_selection_group_raw` call the category methods, but take raw AVFoundation pointers (`unsafe`); there is no safe AVFoundation type integration. |
| `MPRemoteControlTypes` (`MPShuffleType`, `MPRepeatType`, `MPChangeLanguageOptionSetting`) | ✅ | Exposed as `ShuffleType`, `RepeatType`, and `LanguageOptionSetting`. |
| `MPRemoteCommandHandlerStatus` | ✅ | Exposed as `HandlerStatus`. |
| `MPRemoteCommandCenter` command properties | ✅ | All shared-command-center command slots are mapped to Rust handles. |
| `MPRemoteCommand` closure registration and `enabled` | ✅ | Closure-based handler registration (returning `Result<CommandToken, _>`), deregistration via `CommandToken`, and `enabled` state are implemented for every command handle. Handlers run on the main queue. `async_api::RemoteCommandStream` wraps the same registration. |
| `MPRemoteCommand` Objective-C target/action overloads | 🟡 partial | Safe Rust uses closure registration instead of exposing selector-based target/action APIs. |
| `MPSkipIntervalCommand` | ✅ | `preferred_intervals` getter/setter are implemented. |
| `MPFeedbackCommand` | ✅ | `active`, `localizedTitle`, and `localizedShortTitle` are implemented. |
| `MPRatingCommand` | ✅ | `minimumRating` and `maximumRating` are implemented. |
| `MPChangePlaybackRateCommand` | ✅ | `supportedPlaybackRates` getter/setter are implemented. |
| `MPChangePlaybackPositionCommand` | ✅ | Exposed as a typed command handle and event payload. |
| `MPChangeShuffleModeCommand` | ✅ | `currentShuffleType` getter/setter and event payload are implemented. |
| `MPChangeRepeatModeCommand` | ✅ | `currentRepeatType` getter/setter and event payload are implemented. |
| `MPRemoteCommandEvent` and subclasses | ✅ | Timestamp, skip interval, seek type, playback position, rating, playback rate, feedback negativity, shuffle/repeat selection, and language-option payloads are surfaced in `CommandEvent`. |
| `MPErrorDomain`, `MPErrorCode` | ✅ | Exposed as `ERROR_DOMAIN` and `ErrorCode`. |
| `MPMediaLibrary` and `MPMediaLibraryAuthorizationStatus` | ⏭️ skipped | Apple marks `MPMediaLibrary` unavailable on macOS; `MediaLibrary` is an explicit macOS stub that reports the availability reason, and `async_api::MediaLibraryChangeStream::subscribe` returns `NotAvailable`. |
| `MPMediaQuery`, `MPMediaPredicate`, `MPMediaPropertyPredicate`, `MPMediaGrouping`, and query additions on `MPMediaItem` | ⏭️ skipped | Apple marks the query surface unavailable on macOS; `MediaQuery` is an explicit macOS stub that reports the availability reason. |
| `MPMusicPlayerController`, `MPSystemMusicPlayerController`, playback notifications | ⏭️ skipped | Apple marks the music-player surface unavailable on macOS; `MusicPlayer` and `SystemMusicPlayer` are explicit macOS stubs, and the `async_api` now-playing-item, playback-state and volume streams return `NotAvailable` from `subscribe`. |
| `MPMusicPlayerQueueDescriptor` family | ⏭️ skipped | Queue-descriptor types are iOS/tvOS-only and are not exposed from the macOS crate. |
| `MPMediaEntity` | ⏭️ skipped | The abstract entity base class is unavailable on macOS. |
| `MPMediaItem` | ⏭️ skipped | Apple marks `MPMediaItem` unavailable on macOS; `MediaItem` is an explicit macOS stub. |
| `MPMediaItemCollection` | ⏭️ skipped | Apple marks `MPMediaItemCollection` unavailable on macOS; `MediaItemCollection` is an explicit macOS stub. |
| `MPMediaPlaylist`, `MPMediaPlaylistCreationMetadata` | ⏭️ skipped | Apple marks playlists unavailable on macOS; `MediaPlaylist` is an explicit macOS stub. |
| `MPPlayableContentDataSource`, `MPPlayableContentDelegate`, `MPPlayableContentManager`, `MPPlayableContentManagerContext`, `MPContentItem` | ⏭️ skipped | The playable-content surface is unavailable on macOS; `PlayableContentDataSource` is an explicit macOS stub. |
| `MPVolumeView`, `MPVolumeSettings` | ⏭️ skipped | Both are unavailable on macOS; `VolumeView` is an explicit macOS stub. |
| `MPNowPlayingSession`, `MPNowPlayingSessionDelegate` | ⏭️ skipped | Apple explicitly marks now-playing sessions unavailable on macOS; `async_api::NowPlayingSessionStream::subscribe` returns `NotAvailable`. |
| `AVPlayerItem+MediaPlayerAdditions` | ⏭️ skipped | Apple explicitly marks the category unavailable on macOS. |
| `MPMediaPickerController` | ⏭️ skipped | UIKit-only, unavailable on macOS. |
| `MPMoviePlayerController`, `MPMoviePlayerViewController` | ⏭️ skipped | Deprecated iOS-era playback APIs, unavailable on macOS. |
| `NSUserActivity+MediaPlayerAdditions` | ⏭️ skipped | iOS-only user-activity additions, unavailable on macOS. |
