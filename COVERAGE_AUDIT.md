# mediaplayer coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 121
VERIFIED: 120
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 100.00%

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
| `AVMediaSelectionOption (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | LanguageOption::from_av_media_selection_option_raw |
| `AVMediaSelectionGroup (MPNowPlayingInfoLanguageOptionAdditions)` | category | `AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h` | LanguageOptionGroup::from_av_media_selection_group_raw |
| `MPContentItem` | class | `MPContentItem.h` | ContentItem::{new, identifier, title, set_title, subtitle, set_subtitle, artwork, set_artwork, playback_progress, set_playback_progress, is_streaming_content, set_streaming_content, is_explicit_content, set_explicit_content, is_container, set_container, is_playable, set_playable} |
| `MPMediaEntityPersistentID` | typedef | `MPMediaEntity.h` | MediaEntityPersistentId |
| `MPMediaEntityPropertyPersistentID` | constant | `MPMediaEntity.h` | constants::MEDIA_ENTITY_PERSISTENT_ID |
| `MPMediaItemAnimatedArtwork` | class | `MPMediaItem.h` | AnimatedArtwork::from_files; NowPlayingInfo::{animated_artwork_1x1, animated_artwork_3x4} |
| `MPMediaType` | enum | `MPMediaItem.h` | MediaType |
| `MPMediaItemPropertyPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PERSISTENT_ID |
| `MPMediaItemPropertyMediaType` | constant | `MPMediaItem.h` | MediaType; constants::MEDIA_ITEM_MEDIA_TYPE |
| `MPMediaItemPropertyAlbumPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_PERSISTENT_ID |
| `MPMediaItemPropertyArtistPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ARTIST_PERSISTENT_ID |
| `MPMediaItemPropertyAlbumArtist` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_ARTIST |
| `MPMediaItemPropertyAlbumArtistPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_ARTIST_PERSISTENT_ID |
| `MPMediaItemPropertyGenre` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_GENRE |
| `MPMediaItemPropertyGenrePersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_GENRE_PERSISTENT_ID |
| `MPMediaItemPropertyComposer` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMPOSER |
| `MPMediaItemPropertyComposerPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMPOSER_PERSISTENT_ID |
| `MPMediaItemPropertyAlbumTrackNumber` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_TRACK_NUMBER |
| `MPMediaItemPropertyAlbumTrackCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ALBUM_TRACK_COUNT |
| `MPMediaItemPropertyDiscNumber` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DISC_NUMBER |
| `MPMediaItemPropertyDiscCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DISC_COUNT |
| `MPMediaItemPropertyIsExplicit` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_EXPLICIT |
| `MPMediaItemPropertyLyrics` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_LYRICS |
| `MPMediaItemPropertyIsCompilation` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_COMPILATION |
| `MPMediaItemPropertyReleaseDate` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_RELEASE_DATE |
| `MPMediaItemPropertyBeatsPerMinute` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_BEATS_PER_MINUTE |
| `MPMediaItemPropertyComments` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_COMMENTS |
| `MPMediaItemPropertyAssetURL` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_ASSET_URL |
| `MPMediaItemPropertyIsCloudItem` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_CLOUD_ITEM |
| `MPMediaItemPropertyHasProtectedAsset` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_HAS_PROTECTED_ASSET |
| `MPMediaItemPropertyPodcastTitle` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PODCAST_TITLE |
| `MPMediaItemPropertyPodcastPersistentID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PODCAST_PERSISTENT_ID |
| `MPMediaItemPropertyPlayCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PLAY_COUNT |
| `MPMediaItemPropertySkipCount` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_SKIP_COUNT |
| `MPMediaItemPropertyRating` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_RATING |
| `MPMediaItemPropertyLastPlayedDate` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_LAST_PLAYED_DATE |
| `MPMediaItemPropertyUserGrouping` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_USER_GROUPING |
| `MPMediaItemPropertyBookmarkTime` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_BOOKMARK_TIME |
| `MPMediaItemPropertyDateAdded` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_DATE_ADDED |
| `MPMediaItemPropertyPlaybackStoreID` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_PLAYBACK_STORE_ID |
| `MPMediaItemPropertyIsPreorder` | constant | `MPMediaItem.h` | constants::MEDIA_ITEM_IS_PREORDER |
| `MPMediaPlaybackIsPreparedToPlayDidChangeNotification` | constant | `MPMediaPlayback.h` | constants::PLAYBACK_IS_PREPARED_TO_PLAY_DID_CHANGE_NOTIFICATION |
| `MPMediaPlaylistPropertyPersistentID` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_PERSISTENT_ID |
| `MPMediaPlaylistPropertyCloudGlobalID` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_CLOUD_GLOBAL_ID |
| `MPMediaPlaylistPropertyName` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_NAME |
| `MPMediaPlaylistPropertyPlaylistAttributes` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_PLAYLIST_ATTRIBUTES |
| `MPMediaPlaylistPropertySeedItems` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_SEED_ITEMS |
| `MPMediaPlaylistPropertyDescriptionText` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_DESCRIPTION_TEXT |
| `MPMediaPlaylistPropertyAuthorDisplayName` | constant | `MPMediaPlaylist.h` | constants::PLAYLIST_AUTHOR_DISPLAY_NAME |
| `MPNowPlayingInfoProperty1x1AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | constants::ANIMATED_ARTWORK_1X1; NowPlayingInfo::animated_artwork_1x1 |
| `MPNowPlayingInfoProperty3x4AnimatedArtwork` | constant | `MPNowPlayingInfoCenter.h` | constants::ANIMATED_ARTWORK_3X4; NowPlayingInfo::animated_artwork_3x4 |
| `MPLanguageOptionCharacteristicIsMainProgramContent` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_IS_MAIN_PROGRAM_CONTENT |
| `MPLanguageOptionCharacteristicIsAuxiliaryContent` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_IS_AUXILIARY_CONTENT |
| `MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_CONTAINS_ONLY_FORCED_SUBTITLES |
| `MPLanguageOptionCharacteristicTranscribesSpokenDialog` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_TRANSCRIBES_SPOKEN_DIALOG |
| `MPLanguageOptionCharacteristicDescribesMusicAndSound` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_MUSIC_AND_SOUND |
| `MPLanguageOptionCharacteristicEasyToRead` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_EASY_TO_READ |
| `MPLanguageOptionCharacteristicDescribesVideo` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_VIDEO |
| `MPLanguageOptionCharacteristicLanguageTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_LANGUAGE_TRANSLATION |
| `MPLanguageOptionCharacteristicDubbedTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_DUBBED_TRANSLATION |
| `MPLanguageOptionCharacteristicVoiceOverTranslation` | constant | `MPNowPlayingInfoLanguageOption.h` | constants::LANGUAGE_OPTION_CHARACTERISTIC_VOICE_OVER_TRANSLATION |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MPNowPlayingInfoPropertyAdTimeRanges` | constant | `MPNowPlayingInfoCenter.h` | The key is public on macOS, but its required value type MPAdTimeRange is explicitly unavailable on macOS, so safe Rust cannot construct the payload. | MPNowPlayingInfoPropertyAdTimeRanges MP_API(..., macos(13.0)); MPAdTimeRange MP_UNAVAILABLE(watchos, macos) in MPNowPlayingSession.h |
