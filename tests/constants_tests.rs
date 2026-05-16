use mediaplayer::constants::{
    ANIMATED_ARTWORK_1X1, ANIMATED_ARTWORK_3X4,
    LANGUAGE_OPTION_CHARACTERISTIC_CONTAINS_ONLY_FORCED_SUBTITLES,
    LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_MUSIC_AND_SOUND,
    LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_VIDEO,
    LANGUAGE_OPTION_CHARACTERISTIC_DUBBED_TRANSLATION, LANGUAGE_OPTION_CHARACTERISTIC_EASY_TO_READ,
    LANGUAGE_OPTION_CHARACTERISTIC_IS_AUXILIARY_CONTENT,
    LANGUAGE_OPTION_CHARACTERISTIC_IS_MAIN_PROGRAM_CONTENT,
    LANGUAGE_OPTION_CHARACTERISTIC_LANGUAGE_TRANSLATION,
    LANGUAGE_OPTION_CHARACTERISTIC_TRANSCRIBES_SPOKEN_DIALOG,
    LANGUAGE_OPTION_CHARACTERISTIC_VOICE_OVER_TRANSLATION, MEDIA_ENTITY_PERSISTENT_ID,
    MEDIA_ITEM_ALBUM_ARTIST, MEDIA_ITEM_ALBUM_ARTIST_PERSISTENT_ID, MEDIA_ITEM_ALBUM_PERSISTENT_ID,
    MEDIA_ITEM_ALBUM_TRACK_COUNT, MEDIA_ITEM_ALBUM_TRACK_NUMBER, MEDIA_ITEM_ARTIST_PERSISTENT_ID,
    MEDIA_ITEM_ASSET_URL, MEDIA_ITEM_BEATS_PER_MINUTE, MEDIA_ITEM_BOOKMARK_TIME,
    MEDIA_ITEM_COMMENTS, MEDIA_ITEM_COMPOSER, MEDIA_ITEM_COMPOSER_PERSISTENT_ID,
    MEDIA_ITEM_DATE_ADDED, MEDIA_ITEM_DISC_COUNT, MEDIA_ITEM_DISC_NUMBER, MEDIA_ITEM_GENRE,
    MEDIA_ITEM_GENRE_PERSISTENT_ID, MEDIA_ITEM_HAS_PROTECTED_ASSET, MEDIA_ITEM_IS_CLOUD_ITEM,
    MEDIA_ITEM_IS_COMPILATION, MEDIA_ITEM_IS_EXPLICIT, MEDIA_ITEM_IS_PREORDER,
    MEDIA_ITEM_LAST_PLAYED_DATE, MEDIA_ITEM_LYRICS, MEDIA_ITEM_MEDIA_TYPE,
    MEDIA_ITEM_PERSISTENT_ID, MEDIA_ITEM_PLAYBACK_STORE_ID, MEDIA_ITEM_PLAY_COUNT,
    MEDIA_ITEM_PODCAST_PERSISTENT_ID, MEDIA_ITEM_PODCAST_TITLE, MEDIA_ITEM_RATING,
    MEDIA_ITEM_RELEASE_DATE, MEDIA_ITEM_SKIP_COUNT, MEDIA_ITEM_USER_GROUPING,
    PLAYBACK_IS_PREPARED_TO_PLAY_DID_CHANGE_NOTIFICATION, PLAYLIST_AUTHOR_DISPLAY_NAME,
    PLAYLIST_CLOUD_GLOBAL_ID, PLAYLIST_DESCRIPTION_TEXT, PLAYLIST_NAME, PLAYLIST_PERSISTENT_ID,
    PLAYLIST_PLAYLIST_ATTRIBUTES, PLAYLIST_SEED_ITEMS,
};
use mediaplayer::{MediaEntityPersistentId, MediaType};

#[test]
#[allow(clippy::too_many_lines)]
fn mediaplayer_exposes_public_media_symbol_constants() {
    let symbol_pairs = [
        (
            MEDIA_ENTITY_PERSISTENT_ID,
            "MPMediaEntityPropertyPersistentID",
        ),
        (MEDIA_ITEM_PERSISTENT_ID, "MPMediaItemPropertyPersistentID"),
        (MEDIA_ITEM_MEDIA_TYPE, "MPMediaItemPropertyMediaType"),
        (
            MEDIA_ITEM_ALBUM_PERSISTENT_ID,
            "MPMediaItemPropertyAlbumPersistentID",
        ),
        (
            MEDIA_ITEM_ARTIST_PERSISTENT_ID,
            "MPMediaItemPropertyArtistPersistentID",
        ),
        (MEDIA_ITEM_ALBUM_ARTIST, "MPMediaItemPropertyAlbumArtist"),
        (
            MEDIA_ITEM_ALBUM_ARTIST_PERSISTENT_ID,
            "MPMediaItemPropertyAlbumArtistPersistentID",
        ),
        (MEDIA_ITEM_GENRE, "MPMediaItemPropertyGenre"),
        (
            MEDIA_ITEM_GENRE_PERSISTENT_ID,
            "MPMediaItemPropertyGenrePersistentID",
        ),
        (MEDIA_ITEM_COMPOSER, "MPMediaItemPropertyComposer"),
        (
            MEDIA_ITEM_COMPOSER_PERSISTENT_ID,
            "MPMediaItemPropertyComposerPersistentID",
        ),
        (
            MEDIA_ITEM_ALBUM_TRACK_NUMBER,
            "MPMediaItemPropertyAlbumTrackNumber",
        ),
        (
            MEDIA_ITEM_ALBUM_TRACK_COUNT,
            "MPMediaItemPropertyAlbumTrackCount",
        ),
        (MEDIA_ITEM_DISC_NUMBER, "MPMediaItemPropertyDiscNumber"),
        (MEDIA_ITEM_DISC_COUNT, "MPMediaItemPropertyDiscCount"),
        (MEDIA_ITEM_IS_EXPLICIT, "MPMediaItemPropertyIsExplicit"),
        (MEDIA_ITEM_LYRICS, "MPMediaItemPropertyLyrics"),
        (
            MEDIA_ITEM_IS_COMPILATION,
            "MPMediaItemPropertyIsCompilation",
        ),
        (MEDIA_ITEM_RELEASE_DATE, "MPMediaItemPropertyReleaseDate"),
        (
            MEDIA_ITEM_BEATS_PER_MINUTE,
            "MPMediaItemPropertyBeatsPerMinute",
        ),
        (MEDIA_ITEM_COMMENTS, "MPMediaItemPropertyComments"),
        (MEDIA_ITEM_ASSET_URL, "MPMediaItemPropertyAssetURL"),
        (MEDIA_ITEM_IS_CLOUD_ITEM, "MPMediaItemPropertyIsCloudItem"),
        (
            MEDIA_ITEM_HAS_PROTECTED_ASSET,
            "MPMediaItemPropertyHasProtectedAsset",
        ),
        (MEDIA_ITEM_PODCAST_TITLE, "MPMediaItemPropertyPodcastTitle"),
        (
            MEDIA_ITEM_PODCAST_PERSISTENT_ID,
            "MPMediaItemPropertyPodcastPersistentID",
        ),
        (MEDIA_ITEM_PLAY_COUNT, "MPMediaItemPropertyPlayCount"),
        (MEDIA_ITEM_SKIP_COUNT, "MPMediaItemPropertySkipCount"),
        (MEDIA_ITEM_RATING, "MPMediaItemPropertyRating"),
        (
            MEDIA_ITEM_LAST_PLAYED_DATE,
            "MPMediaItemPropertyLastPlayedDate",
        ),
        (MEDIA_ITEM_USER_GROUPING, "MPMediaItemPropertyUserGrouping"),
        (MEDIA_ITEM_BOOKMARK_TIME, "MPMediaItemPropertyBookmarkTime"),
        (MEDIA_ITEM_DATE_ADDED, "MPMediaItemPropertyDateAdded"),
        (
            MEDIA_ITEM_PLAYBACK_STORE_ID,
            "MPMediaItemPropertyPlaybackStoreID",
        ),
        (MEDIA_ITEM_IS_PREORDER, "MPMediaItemPropertyIsPreorder"),
        (
            PLAYBACK_IS_PREPARED_TO_PLAY_DID_CHANGE_NOTIFICATION,
            "MPMediaPlaybackIsPreparedToPlayDidChangeNotification",
        ),
        (
            PLAYLIST_PERSISTENT_ID,
            "MPMediaPlaylistPropertyPersistentID",
        ),
        (
            PLAYLIST_CLOUD_GLOBAL_ID,
            "MPMediaPlaylistPropertyCloudGlobalID",
        ),
        (PLAYLIST_NAME, "MPMediaPlaylistPropertyName"),
        (
            PLAYLIST_PLAYLIST_ATTRIBUTES,
            "MPMediaPlaylistPropertyPlaylistAttributes",
        ),
        (PLAYLIST_SEED_ITEMS, "MPMediaPlaylistPropertySeedItems"),
        (
            PLAYLIST_DESCRIPTION_TEXT,
            "MPMediaPlaylistPropertyDescriptionText",
        ),
        (
            PLAYLIST_AUTHOR_DISPLAY_NAME,
            "MPMediaPlaylistPropertyAuthorDisplayName",
        ),
        (
            ANIMATED_ARTWORK_1X1,
            "MPNowPlayingInfoProperty1x1AnimatedArtwork",
        ),
        (
            ANIMATED_ARTWORK_3X4,
            "MPNowPlayingInfoProperty3x4AnimatedArtwork",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_IS_MAIN_PROGRAM_CONTENT,
            "MPLanguageOptionCharacteristicIsMainProgramContent",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_IS_AUXILIARY_CONTENT,
            "MPLanguageOptionCharacteristicIsAuxiliaryContent",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_CONTAINS_ONLY_FORCED_SUBTITLES,
            "MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_TRANSCRIBES_SPOKEN_DIALOG,
            "MPLanguageOptionCharacteristicTranscribesSpokenDialog",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_MUSIC_AND_SOUND,
            "MPLanguageOptionCharacteristicDescribesMusicAndSound",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_EASY_TO_READ,
            "MPLanguageOptionCharacteristicEasyToRead",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_VIDEO,
            "MPLanguageOptionCharacteristicDescribesVideo",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_LANGUAGE_TRANSLATION,
            "MPLanguageOptionCharacteristicLanguageTranslation",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_DUBBED_TRANSLATION,
            "MPLanguageOptionCharacteristicDubbedTranslation",
        ),
        (
            LANGUAGE_OPTION_CHARACTERISTIC_VOICE_OVER_TRANSLATION,
            "MPLanguageOptionCharacteristicVoiceOverTranslation",
        ),
    ];

    for (value, expected) in symbol_pairs {
        assert_eq!(value, expected);
    }
}

#[test]
fn media_type_bitflags_cover_audio_and_video_categories() {
    let _: MediaEntityPersistentId = 42;

    let spoken_word = MediaType::PODCAST | MediaType::AUDIOBOOK;
    assert!(spoken_word.contains(MediaType::PODCAST));
    assert!(MediaType::ANY_AUDIO.contains(MediaType::MUSIC));
    assert!(MediaType::ANY_AUDIO.contains(spoken_word));

    let video = MediaType::MOVIE | MediaType::TV_SHOW | MediaType::HOME_VIDEO;
    assert!(video.contains(MediaType::MOVIE));
    assert!(video.intersects(MediaType::ANY_VIDEO));
    assert!(MediaType::ANY.contains(video));

    let custom = MediaType::from_bits(MediaType::MUSIC.bits() | MediaType::VIDEO_PODCAST.bits());
    assert!(custom.contains(MediaType::MUSIC));
    assert!(custom.contains(MediaType::VIDEO_PODCAST));
    assert!(!custom.contains(MediaType::AUDIOBOOK));
    assert!(MediaType::NONE.is_empty());
}
