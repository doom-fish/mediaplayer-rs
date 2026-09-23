# mediaplayer coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 121
VERIFIED: 112
GAPS: 0
EXEMPT: 9
COVERAGE_PCT: 100.00%

Audit methodology: Re-verified all 121 top-level public declarations (interfaces/categories/protocols, typedefs/enums, exported constants/functions) from v1 against the macOS 26.2 SDK headers. No symbols were removed between versions, and no new top-level public API surface was added in macOS 26.2. The original EXEMPT entry (MPNowPlayingInfoPropertyAdTimeRanges) was re-validated: the key remains macOS-available, but its required value type MPAdTimeRange is explicitly unavailable on macOS via `MP_UNAVAILABLE_BEGIN(watchos, macos)` in MPNowPlayingSession.h. VERIFIED means the symbol has a usable Rust entry point, so the eight constants the crate exposes by name only (`MPMediaPlaylistProperty*` and `MPMediaPlaybackIsPreparedToPlayDidChangeNotification`, which no macOS API consumes) are counted as EXEMPT. The `MPMediaItemProperty*` keys are usable through `NowPlayingInfo::value`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MPErrorCode` | enum | `MPError.h` | ErrorCode |
| `MPErrorDomain` | constant | `MPError.h` | ERROR_DOMAIN |
| `MPMediaItemArtwork` | class | `MPMediaItem.h` | Artwork::{from_path, from_path_with_size, from_image_data, bounds, image_png_data}; NowPlayingInfoCenter::set_now_playing_info_with_artwork (imageCropRect not surfaced) |
| `MPMediaItemPropertyTitle` | constant | `MPMediaItem.h` | NowPlayingInfo::title |
| `MPMediaItemPropertyAlbumTitle` | constant | `MPMediaItem.h` | NowPlayingInfo::album_title |
| `MPMediaItemPropertyArtist` | constant | `MPMediaItem.h` | NowPlayingInfo::artist |
| `MPMediaItemPropertyPlaybackDuration` | constant | `MPMediaItem.h` | NowPlayingInfo::playback_duration |
| `MPMediaItemPropertyArtwork` | constant | `MPMediaItem.h` | NowPlayingInfoCenter::set_now_playing_info_with_artwork |
| `MPNowPlayingInfoCenter` | class | `MPNowPlayingInfoCenter.h` | NowPlayingInfoCenter::{default_center, set_now_playing_info, set_now_playing_info_with_artwork, now_playing_info, clear, set_playback_state, playback_state, supported_animated_artwork_keys} |
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
| `AVMediaSelectionOption (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | LanguageOption::from_av_media_selection_option_raw |
| `AVMediaSelectionGroup (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | LanguageOptionGroup::from_av_media_selection_group_raw |
| `MPContentItem` | class | `MPContentItem.h` | ContentItem::{new, identifier, title, set_title, subtitle, set_subtitle, artwork, set_artwork, playback_progress, set_playback_progress, is_streaming_content, set_streaming_content, is_explicit_content, set_explicit_content, is_container, set_container, is_playable, set_playable} |
| `MPMediaEntityPersistentID` | typedef | `MPMediaEntity.h` | MediaEntityPersistentId |
| `MPMediaEntityPropertyPersistentID` | constant | `MPMediaEntity.h` | constants::MEDIA_ENTITY_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemAnimatedArtwork` | class | `MPMediaItem.h` | AnimatedArtwork::from_files; NowPlayingInfo::{animated_artwork_1x1, animated_artwork_3x4} |
| `MPMediaType` | enum | `MPMediaItem.h` | MediaType |
| `MPMediaItemPropertyPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyMediaType` | constant | `MPMediaItem.h` | MediaType; constants::MEDIA_ITEM_MEDIA_TYPE; NowPlayingInfo::value |
| `MPMediaItemPropertyAlbumPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyArtistPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ARTIST_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyAlbumArtist` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_ARTIST; NowPlayingInfo::value |
| `MPMediaItemPropertyAlbumArtistPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_ARTIST_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyGenre` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_GENRE; NowPlayingInfo::value |
| `MPMediaItemPropertyGenrePersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_GENRE_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyComposer` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMPOSER; NowPlayingInfo::value |
| `MPMediaItemPropertyComposerPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMPOSER_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyAlbumTrackNumber` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_TRACK_NUMBER; NowPlayingInfo::value |
| `MPMediaItemPropertyAlbumTrackCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_TRACK_COUNT; NowPlayingInfo::value |
| `MPMediaItemPropertyDiscNumber` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DISC_NUMBER; NowPlayingInfo::value |
| `MPMediaItemPropertyDiscCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DISC_COUNT; NowPlayingInfo::value |
| `MPMediaItemPropertyIsExplicit` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_EXPLICIT; NowPlayingInfo::value |
| `MPMediaItemPropertyLyrics` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_LYRICS; NowPlayingInfo::value |
| `MPMediaItemPropertyIsCompilation` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_COMPILATION; NowPlayingInfo::value |
| `MPMediaItemPropertyReleaseDate` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_RELEASE_DATE; NowPlayingInfo::value |
| `MPMediaItemPropertyBeatsPerMinute` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_BEATS_PER_MINUTE; NowPlayingInfo::value |
| `MPMediaItemPropertyComments` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMMENTS; NowPlayingInfo::value |
| `MPMediaItemPropertyAssetURL` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ASSET_URL; NowPlayingInfo::value |
| `MPMediaItemPropertyIsCloudItem` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_CLOUD_ITEM; NowPlayingInfo::value |
| `MPMediaItemPropertyHasProtectedAsset` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_HAS_PROTECTED_ASSET; NowPlayingInfo::value |
| `MPMediaItemPropertyPodcastTitle` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PODCAST_TITLE; NowPlayingInfo::value |
| `MPMediaItemPropertyPodcastPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PODCAST_PERSISTENT_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyPlayCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PLAY_COUNT; NowPlayingInfo::value |
| `MPMediaItemPropertySkipCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_SKIP_COUNT; NowPlayingInfo::value |
| `MPMediaItemPropertyRating` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_RATING; NowPlayingInfo::value |
| `MPMediaItemPropertyLastPlayedDate` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_LAST_PLAYED_DATE; NowPlayingInfo::value |
| `MPMediaItemPropertyUserGrouping` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_USER_GROUPING; NowPlayingInfo::value |
| `MPMediaItemPropertyBookmarkTime` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_BOOKMARK_TIME; NowPlayingInfo::value |
| `MPMediaItemPropertyDateAdded` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DATE_ADDED; NowPlayingInfo::value |
| `MPMediaItemPropertyPlaybackStoreID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PLAYBACK_STORE_ID; NowPlayingInfo::value |
| `MPMediaItemPropertyIsPreorder` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_PREORDER; NowPlayingInfo::value |
| `MPNowPlayingInfoProperty1x1AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | constants::ANIMATED_ARTWORK_1X1; NowPlayingInfo::animated_artwork_1x1 |
| `MPNowPlayingInfoProperty3x4AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | constants::ANIMATED_ARTWORK_3X4; NowPlayingInfo::animated_artwork_3x4 |
| `MPLanguageOptionCharacteristicIsMainProgramContent` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_IS_MAIN_PROGRAM_CONTENT (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicIsAuxiliaryContent` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_IS_AUXILIARY_CONTENT (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_CONTAINS_ONLY_FORCED_SUBTITLES (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicTranscribesSpokenDialog` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_TRANSCRIBES_SPOKEN_DIALOG (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicDescribesMusicAndSound` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_MUSIC_AND_SOUND (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicEasyToRead` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_EASY_TO_READ (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicDescribesVideo` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_VIDEO (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicLanguageTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_LANGUAGE_TRANSLATION (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicDubbedTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DUBBED_TRANSLATION (translated by LanguageOption::new) |
| `MPLanguageOptionCharacteristicVoiceOverTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_VOICE_OVER_TRANSLATION (translated by LanguageOption::new) |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MPNowPlayingInfoPropertyAdTimeRanges` | constant | `MPNowPlayingInfoCenter.h` | The key is public on macOS, but its required value type MPAdTimeRange is explicitly unavailable on macOS, so safe Rust cannot construct the payload. | MPAdTimeRange is enclosed in `MP_UNAVAILABLE_BEGIN(watchos, macos)` in MPNowPlayingSession.h |
| `MPMediaPlaybackIsPreparedToPlayDidChangeNotification` | constant | `MPMediaPlayback.h` | Only the symbol name is exposed (constants::PLAYBACK_IS_PREPARED_TO_PLAY_DID_CHANGE_NOTIFICATION); the notification belongs to the deprecated iOS `MPMediaPlayback` surface. | `MP_DEPRECATED("Use AVPlayerViewController in AVKit.", ios(3.2, 9.0))` |
| `MPMediaPlaylistPropertyPersistentID` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_PERSISTENT_ID); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertyCloudGlobalID` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_CLOUD_GLOBAL_ID); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertyName` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_NAME); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertyPlaylistAttributes` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_PLAYLIST_ATTRIBUTES); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertySeedItems` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_SEED_ITEMS); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertyDescriptionText` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_DESCRIPTION_TEXT); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
| `MPMediaPlaylistPropertyAuthorDisplayName` | constant | `MPMediaPlaylist.h` | Only the symbol name is exposed (constants::PLAYLIST_AUTHOR_DISPLAY_NAME); `MPMediaPlaylist` is iOS-only, so no macOS API consumes the key. | `MPMediaPlaylist` is `MP_API(ios(3.0))` |
