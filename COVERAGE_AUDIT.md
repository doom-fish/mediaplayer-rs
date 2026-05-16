# mediaplayer coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 121
VERIFIED: 59
GAPS: 61
EXEMPT: 1
COVERAGE_PCT: 49.17%

- Counted top-level public declarations only (interfaces/categories/protocols, typedefs/enums, exported constants/functions), per the audit instructions.
- Exported constants that remain macOS-available were kept in scope even when their owning class/protocol is macOS-unavailable; this notably affects `MPMediaItemProperty*`, `MPMediaPlaylistProperty*`, and `MPMediaPlaybackIsPreparedToPlayDidChangeNotification`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MPErrorCode` | enum | `MPError.h` | ErrorCode |
| `MPErrorDomain` | constant | `MPError.h` | ERROR_DOMAIN |
| `MPMediaItemArtwork` | class | `MPMediaItem.h` | Artwork::{from_path, from_path_with_size, bounds}; NowPlayingInfoCenter::set_now_playing_info_with_artwork (imageWithSize/imageCropRect not surfaced) |
| `MPMediaItemPropertyTitle` | constant | `MPMediaItem.h` | NowPlayingInfo::title |
| `MPMediaItemPropertyAlbumTitle` | constant | `MPMediaItem.h` | NowPlayingInfo::album_title |
| `MPMediaItemPropertyArtist` | constant | `MPMediaItem.h` | NowPlayingInfo::artist |
| `MPMediaItemPropertyPlaybackDuration` | constant | `MPMediaItem.h` | NowPlayingInfo::playback_duration |
| `MPMediaItemPropertyArtwork` | constant | `MPMediaItem.h` | NowPlayingInfoCenter::set_now_playing_info_with_artwork |
| `MPNowPlayingInfoCenter` | class | `MPNowPlayingInfoCenter.h` | NowPlayingInfoCenter::{default_center, set_now_playing_info, set_now_playing_info_with_artwork, clear, set_playback_state, playback_state, supported_animated_artwork_keys} |
| `MPNowPlayingInfoMediaType` | enum | `MPNowPlayingInfoCenter.h` | NowPlayingMediaType |
| `MPNowPlayingPlaybackState` | enum | `MPNowPlayingInfoCenter.h` | PlaybackState |
| `MPNowPlayingInfoPropertyElapsedPlaybackTime` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::elapsed_playback_time |
| `MPNowPlayingInfoPropertyPlaybackRate` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::playback_rate |
| `MPNowPlayingInfoPropertyDefaultPlaybackRate` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::default_playback_rate |
| `MPNowPlayingInfoPropertyPlaybackQueueIndex` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::playback_queue_index |
| `MPNowPlayingInfoPropertyPlaybackQueueCount` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::playback_queue_count |
| `MPNowPlayingInfoPropertyChapterNumber` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::chapter_number |
| `MPNowPlayingInfoPropertyChapterCount` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::chapter_count |
| `MPNowPlayingInfoPropertyIsLiveStream` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::live_stream |
| `MPNowPlayingInfoPropertyAvailableLanguageOptions` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::available_language_option_groups |
| `MPNowPlayingInfoPropertyCurrentLanguageOptions` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::current_language_options |
| `MPNowPlayingInfoCollectionIdentifier` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::collection_identifier |
| `MPNowPlayingInfoPropertyExternalContentIdentifier` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::external_content_identifier |
| `MPNowPlayingInfoPropertyExternalUserProfileIdentifier` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::external_user_profile_identifier |
| `MPNowPlayingInfoPropertyServiceIdentifier` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::service_identifier |
| `MPNowPlayingInfoPropertyPlaybackProgress` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::playback_progress |
| `MPNowPlayingInfoPropertyMediaType` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::media_type |
| `MPNowPlayingInfoPropertyAssetURL` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::asset_url |
| `MPNowPlayingInfoPropertyCurrentPlaybackDate` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::current_playback_date |
| `MPNowPlayingInfoPropertyCreditsStartTime` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::credits_start_time |
| `MPNowPlayingInfoPropertyInternationalStandardRecordingCode` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::international_standard_recording_code |
| `MPNowPlayingInfoPropertyExcludeFromSuggestions` | constant | `MPNowPlayingInfoCenter.h` | NowPlayingInfo::exclude_from_suggestions |
| `MPNowPlayingInfoLanguageOption` | class | `MPNowPlayingInfoLanguageOption.h` | LanguageOption |
| `MPNowPlayingInfoLanguageOptionGroup` | class | `MPNowPlayingInfoLanguageOption.h` | LanguageOptionGroup |
| `MPNowPlayingInfoLanguageOptionType` | enum | `MPNowPlayingInfoLanguageOption.h` | LanguageOptionType |
| `MPRemoteCommandHandlerStatus` | enum | `MPRemoteCommand.h` | HandlerStatus |
| `MPRemoteCommand` | class | `MPRemoteCommand.h` | RemoteCommand::{command, is_enabled, set_enabled, add_handler}; CommandToken (selector target/action overloads not surfaced) |
| `MPSkipIntervalCommand` | class | `MPRemoteCommand.h` | SkipIntervalCommand::{preferred_intervals, set_preferred_intervals} |
| `MPFeedbackCommand` | class | `MPRemoteCommand.h` | FeedbackCommand::{is_active, set_active, localized_title, set_localized_title, localized_short_title, set_localized_short_title} |
| `MPRatingCommand` | class | `MPRemoteCommand.h` | RatingCommand::{minimum_rating, set_minimum_rating, maximum_rating, set_maximum_rating} |
| `MPChangePlaybackRateCommand` | class | `MPRemoteCommand.h` | ChangePlaybackRateCommand::{supported_playback_rates, set_supported_playback_rates} |
| `MPChangePlaybackPositionCommand` | class | `MPRemoteCommand.h` | ChangePlaybackPositionCommand via RemoteCommandCenter and CommandEvent::position |
| `MPChangeShuffleModeCommand` | class | `MPRemoteCommand.h` | ChangeShuffleModeCommand::{current_shuffle_type, set_current_shuffle_type} |
| `MPChangeRepeatModeCommand` | class | `MPRemoteCommand.h` | ChangeRepeatModeCommand::{current_repeat_type, set_current_repeat_type} |
| `MPRemoteCommandCenter` | class | `MPRemoteCommandCenter.h` | RemoteCommandCenter::{shared, command, play_command, pause_command, stop_command, toggle_play_pause_command, next_track_command, previous_track_command, skip_forward_command, skip_backward_command, seek_forward_command, seek_backward_command, change_playback_position_command, enable_language_option_command, disable_language_option_command, change_playback_rate_command, change_repeat_mode_command, change_shuffle_mode_command, rating_command, like_command, dislike_command, bookmark_command, on_* helpers} |
| `MPRemoteCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::timestamp plus shared event metadata |
| `MPSkipIntervalCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::skip_interval |
| `MPSeekCommandEventType` | enum | `MPRemoteCommandEvent.h` | SeekType |
| `MPSeekCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::seek_type |
| `MPRatingCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::rating |
| `MPChangePlaybackRateCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::playback_rate |
| `MPFeedbackCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::feedback_negative |
| `MPChangeLanguageOptionCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::{language_option, language_option_setting} |
| `MPChangePlaybackPositionCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::position |
| `MPChangeShuffleModeCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::{shuffle_type, preserves_shuffle_mode} |
| `MPChangeRepeatModeCommandEvent` | class | `MPRemoteCommandEvent.h` | CommandEvent::{repeat_type, preserves_repeat_mode} |
| `MPShuffleType` | enum | `MPRemoteControlTypes.h` | ShuffleType |
| `MPRepeatType` | enum | `MPRemoteControlTypes.h` | RepeatType |
| `MPChangeLanguageOptionSetting` | enum | `MPRemoteControlTypes.h` | LanguageOptionSetting |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `AVMediaSelectionOption (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | No wrapper for makeNowPlayingInfoLanguageOption(). |
| `AVMediaSelectionGroup (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | No wrapper for makeNowPlayingInfoLanguageOptionGroup(). |
| `MPContentItem` | class | `MPContentItem.h` | No Rust wrapper for the macOS-available content-item type. |
| `MPMediaEntityPersistentID` | typedef | `MPMediaEntity.h` | No Rust alias for MediaPlayer persistent IDs. |
| `MPMediaEntityPropertyPersistentID` | constant | `MPMediaEntity.h` | No public wrapper for the exported entity-property key. |
| `MPMediaItemAnimatedArtwork` | class | `MPMediaItem.h` | Animated artwork construction is not wrapped. |
| `MPMediaType` | enum | `MPMediaItem.h` | No Rust enum for MPMediaType. |
| `MPMediaItemPropertyPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyMediaType` | constant | `MPMediaItem.h` | The crate exposes NowPlayingMediaType, but not the distinct MPMediaType/MPMediaItemPropertyMediaType surface. |
| `MPMediaItemPropertyAlbumPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyArtistPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyAlbumArtist` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyAlbumArtistPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyGenre` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyGenrePersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyComposer` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyComposerPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyAlbumTrackNumber` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyAlbumTrackCount` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyDiscNumber` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyDiscCount` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyIsExplicit` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyLyrics` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyIsCompilation` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyReleaseDate` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyBeatsPerMinute` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyComments` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyAssetURL` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyIsCloudItem` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyHasProtectedAsset` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyPodcastTitle` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyPodcastPersistentID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyPlayCount` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertySkipCount` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyRating` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyLastPlayedDate` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyUserGrouping` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyBookmarkTime` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyDateAdded` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyPlaybackStoreID` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaItemPropertyIsPreorder` | constant | `MPMediaItem.h` | No public wrapper for the exported MPMediaItemProperty constant. |
| `MPMediaPlaybackIsPreparedToPlayDidChangeNotification` | constant | `MPMediaPlayback.h` | Notification constant is not surfaced. |
| `MPMediaPlaylistPropertyPersistentID` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertyCloudGlobalID` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertyName` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertyPlaylistAttributes` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertySeedItems` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertyDescriptionText` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPMediaPlaylistPropertyAuthorDisplayName` | constant | `MPMediaPlaylist.h` | MediaPlaylist is only exposed as an unsupported stub; exported playlist property constants are not surfaced. |
| `MPNowPlayingInfoProperty1x1AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | No animated-artwork setter or MPMediaItemAnimatedArtwork wrapper is exposed. |
| `MPNowPlayingInfoProperty3x4AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | No animated-artwork setter or MPMediaItemAnimatedArtwork wrapper is exposed. |
| `MPLanguageOptionCharacteristicIsMainProgramContent` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicIsAuxiliaryContent` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicTranscribesSpokenDialog` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicDescribesMusicAndSound` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicEasyToRead` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicDescribesVideo` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicLanguageTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicDubbedTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |
| `MPLanguageOptionCharacteristicVoiceOverTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | No Rust constant exports this framework string; callers must hand-roll it. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MPNowPlayingInfoPropertyAdTimeRanges` | constant | `MPNowPlayingInfoCenter.h` | The key is public on macOS, but its required value type MPAdTimeRange is explicitly unavailable on macOS, so safe Rust cannot construct the payload. | MPNowPlayingInfoPropertyAdTimeRanges MP_API(..., macos(13.0)); MPAdTimeRange MP_UNAVAILABLE(watchos, macos) in MPNowPlayingSession.h |
