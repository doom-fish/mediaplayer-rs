use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, UNIX_EPOCH};

use mediaplayer::constants as keys;
use mediaplayer::{
    LanguageOption, LanguageOptionGroup, LanguageOptionType, MediaPlayerError, NowPlayingInfo,
    NowPlayingInfoCenter, NowPlayingMediaType, NowPlayingValue, PlaybackState,
};

static SHARED_CENTER: Mutex<()> = Mutex::new(());

fn exclusive_center() -> MutexGuard<'static, ()> {
    SHARED_CENTER.lock().unwrap_or_else(PoisonError::into_inner)
}

fn text(value: &str) -> NowPlayingValue {
    NowPlayingValue::String(value.to_string())
}

fn as_read_back(value: &NowPlayingValue) -> NowPlayingValue {
    match value {
        NowPlayingValue::UnsignedInteger(number) => i64::try_from(*number).map_or(
            NowPlayingValue::UnsignedInteger(*number),
            NowPlayingValue::Integer,
        ),
        other => other.clone(),
    }
}

#[test]
fn now_playing_smoke_with_language_options() {
    let _center_lock = exclusive_center();
    let subtitles = LanguageOption::new(
        LanguageOptionType::Legible,
        Some("en"),
        &["public.legible"],
        "English Subtitles",
        "subtitles-en",
    )
    .expect("language option should be created");
    let alternate_subtitles = LanguageOption::new(
        LanguageOptionType::Legible,
        Some("sv"),
        &["public.legible"],
        "Swedish Subtitles",
        "subtitles-sv",
    )
    .expect("alternate language option should be created");
    let group = LanguageOptionGroup::new(&[subtitles.clone(), alternate_subtitles], Some(0), true)
        .expect("language option group should be created");

    assert_eq!(subtitles.language_option_type(), LanguageOptionType::Legible);
    assert_eq!(subtitles.language_tag().as_deref(), Some("en"));
    assert_eq!(subtitles.display_name().as_deref(), Some("English Subtitles"));
    assert_eq!(subtitles.identifier().as_deref(), Some("subtitles-en"));
    assert_eq!(group.count(), 2);
    assert_eq!(group.default_language_option_index(), Some(0));
    assert!(group.allow_empty_selection());

    let center = NowPlayingInfoCenter::default_center();
    let info = NowPlayingInfo::new()
        .title("Smoke Test Song")
        .artist("doom-fish")
        .album_title("Crate Tests")
        .playback_duration(300.0)
        .elapsed_playback_time(12.0)
        .playback_rate(1.0)
        .default_playback_rate(1.0)
        .playback_queue_index(1)
        .playback_queue_count(3)
        .chapter_number(0)
        .chapter_count(1)
        .live_stream(false)
        .available_language_option_groups(vec![group])
        .current_language_options(vec![subtitles])
        .collection_identifier("album-1")
        .external_content_identifier("track-1")
        .external_user_profile_identifier("user-1")
        .service_identifier("service")
        .playback_progress(0.5)
        .media_type(NowPlayingMediaType::Audio)
        .asset_url("https://example.com/audio.mp3")
        .current_playback_date(UNIX_EPOCH)
        .credits_start_time(250.0)
        .international_standard_recording_code("TESTCODE12345")
        .exclude_from_suggestions(true);

    center
        .set_now_playing_info(&info)
        .expect("now-playing info should apply");
    center.set_playback_state(PlaybackState::Playing);
    assert_eq!(center.playback_state(), PlaybackState::Playing);

    let applied = center
        .now_playing_info()
        .expect("now-playing info should read back");
    assert_eq!(applied.get(keys::TITLE), Some(&text("Smoke Test Song")));
    assert_eq!(applied.get(keys::ARTIST), Some(&text("doom-fish")));
    assert_eq!(
        applied.get(keys::PLAYBACK_QUEUE_COUNT),
        Some(&NowPlayingValue::Integer(3))
    );
    assert_eq!(
        applied.get(keys::CURRENT_PLAYBACK_DATE),
        Some(&NowPlayingValue::Date(UNIX_EPOCH))
    );
    assert!(matches!(
        applied.get(keys::AVAILABLE_LANGUAGE_OPTIONS),
        Some(NowPlayingValue::Other(_))
    ));
    assert!(matches!(
        applied.get(keys::CURRENT_LANGUAGE_OPTIONS),
        Some(NowPlayingValue::Other(_))
    ));

    let _animated_keys = center.supported_animated_artwork_keys();
    center.clear();
    assert!(center.now_playing_info().is_none());
}

#[test]
#[allow(clippy::too_many_lines)]
fn every_media_item_and_now_playing_key_round_trips() {
    let _center_lock = exclusive_center();
    let released = NowPlayingValue::Date(UNIX_EPOCH + Duration::from_secs(1_000_000_000));
    let values = [
        (
            keys::MEDIA_ITEM_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(u64::MAX - 7),
        ),
        (
            keys::MEDIA_ITEM_MEDIA_TYPE,
            NowPlayingValue::UnsignedInteger(1),
        ),
        (keys::TITLE, text("Title")),
        (keys::ARTIST, text("Artist")),
        (keys::ALBUM_TITLE, text("Album")),
        (
            keys::MEDIA_ITEM_ALBUM_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(11),
        ),
        (
            keys::MEDIA_ITEM_ARTIST_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(12),
        ),
        (keys::MEDIA_ITEM_ALBUM_ARTIST, text("Album Artist")),
        (
            keys::MEDIA_ITEM_ALBUM_ARTIST_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(13),
        ),
        (keys::MEDIA_ITEM_GENRE, text("Genre")),
        (
            keys::MEDIA_ITEM_GENRE_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(14),
        ),
        (keys::MEDIA_ITEM_COMPOSER, text("Composer")),
        (
            keys::MEDIA_ITEM_COMPOSER_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(15),
        ),
        (keys::PLAYBACK_DURATION, NowPlayingValue::Double(300.5)),
        (
            keys::MEDIA_ITEM_ALBUM_TRACK_NUMBER,
            NowPlayingValue::Integer(3),
        ),
        (
            keys::MEDIA_ITEM_ALBUM_TRACK_COUNT,
            NowPlayingValue::Integer(12),
        ),
        (keys::MEDIA_ITEM_DISC_NUMBER, NowPlayingValue::Integer(1)),
        (keys::MEDIA_ITEM_DISC_COUNT, NowPlayingValue::Integer(2)),
        (keys::MEDIA_ITEM_IS_EXPLICIT, NowPlayingValue::Bool(true)),
        (keys::MEDIA_ITEM_LYRICS, text("Lyrics")),
        (
            keys::MEDIA_ITEM_IS_COMPILATION,
            NowPlayingValue::Bool(false),
        ),
        (keys::MEDIA_ITEM_RELEASE_DATE, released.clone()),
        (
            keys::MEDIA_ITEM_BEATS_PER_MINUTE,
            NowPlayingValue::Integer(120),
        ),
        (keys::MEDIA_ITEM_COMMENTS, text("Comments")),
        (
            keys::MEDIA_ITEM_ASSET_URL,
            NowPlayingValue::Url("file:///tmp/song.m4a".to_string()),
        ),
        (keys::MEDIA_ITEM_IS_CLOUD_ITEM, NowPlayingValue::Bool(false)),
        (
            keys::MEDIA_ITEM_HAS_PROTECTED_ASSET,
            NowPlayingValue::Bool(false),
        ),
        (keys::MEDIA_ITEM_PODCAST_TITLE, text("Podcast")),
        (
            keys::MEDIA_ITEM_PODCAST_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(16),
        ),
        (keys::MEDIA_ITEM_PLAY_COUNT, NowPlayingValue::Integer(4)),
        (keys::MEDIA_ITEM_SKIP_COUNT, NowPlayingValue::Integer(5)),
        (keys::MEDIA_ITEM_RATING, NowPlayingValue::Integer(3)),
        (keys::MEDIA_ITEM_LAST_PLAYED_DATE, released.clone()),
        (keys::MEDIA_ITEM_USER_GROUPING, text("Grouping")),
        (keys::MEDIA_ITEM_BOOKMARK_TIME, NowPlayingValue::Double(7.5)),
        (keys::MEDIA_ITEM_DATE_ADDED, released.clone()),
        (keys::MEDIA_ITEM_PLAYBACK_STORE_ID, text("store-id")),
        (keys::MEDIA_ITEM_IS_PREORDER, NowPlayingValue::Bool(false)),
        (keys::ELAPSED_PLAYBACK_TIME, NowPlayingValue::Double(1.5)),
        (keys::PLAYBACK_RATE, NowPlayingValue::Double(1.0)),
        (keys::DEFAULT_PLAYBACK_RATE, NowPlayingValue::Double(1.0)),
        (
            keys::PLAYBACK_QUEUE_INDEX,
            NowPlayingValue::UnsignedInteger(0),
        ),
        (
            keys::PLAYBACK_QUEUE_COUNT,
            NowPlayingValue::UnsignedInteger(3),
        ),
        (keys::CHAPTER_NUMBER, NowPlayingValue::UnsignedInteger(0)),
        (keys::CHAPTER_COUNT, NowPlayingValue::UnsignedInteger(2)),
        (keys::IS_LIVE_STREAM, NowPlayingValue::Bool(false)),
        (keys::COLLECTION_IDENTIFIER, text("collection")),
        (keys::EXTERNAL_CONTENT_IDENTIFIER, text("content")),
        (keys::EXTERNAL_USER_PROFILE_IDENTIFIER, text("profile")),
        (keys::SERVICE_IDENTIFIER, text("service")),
        (keys::PLAYBACK_PROGRESS, NowPlayingValue::Double(0.25)),
        (keys::MEDIA_TYPE, NowPlayingValue::UnsignedInteger(1)),
        (
            keys::ASSET_URL,
            NowPlayingValue::Url("https://example.com/audio.mp3".to_string()),
        ),
        (keys::CURRENT_PLAYBACK_DATE, released),
        (keys::CREDITS_START_TIME, NowPlayingValue::Double(250.0)),
        (
            keys::INTERNATIONAL_STANDARD_RECORDING_CODE,
            text("TESTCODE12345"),
        ),
        (keys::EXCLUDE_FROM_SUGGESTIONS, NowPlayingValue::Bool(true)),
    ];
    let info = values
        .iter()
        .fold(NowPlayingInfo::new(), |info, (key, value)| {
            info.value(*key, value.clone())
        });

    let center = NowPlayingInfoCenter::default_center();
    center
        .set_now_playing_info(&info)
        .expect("every media-item and now-playing key should be accepted");
    let applied = center
        .now_playing_info()
        .expect("now-playing info should read back");

    for (key, value) in &values {
        assert_eq!(applied.get(*key), Some(&as_read_back(value)), "{key}");
    }
    assert_eq!(applied.len(), values.len());
}

#[test]
fn raw_key_values_and_aliases_read_back_by_symbol_name() {
    let _center_lock = exclusive_center();
    let before_epoch = UNIX_EPOCH - Duration::from_secs(86_400);
    let info = NowPlayingInfo::new()
        .value("composer", text("Composer"))
        .value(
            keys::MEDIA_ENTITY_PERSISTENT_ID,
            NowPlayingValue::UnsignedInteger(42),
        )
        .value(
            keys::MEDIA_ITEM_RELEASE_DATE,
            NowPlayingValue::Date(before_epoch),
        );

    let center = NowPlayingInfoCenter::default_center();
    center
        .set_now_playing_info(&info)
        .expect("raw keys and aliases should be accepted");
    let applied = center
        .now_playing_info()
        .expect("now-playing info should read back");

    assert_eq!(
        applied.get(keys::MEDIA_ITEM_COMPOSER),
        Some(&text("Composer"))
    );
    assert_eq!(
        applied.get(keys::MEDIA_ITEM_PERSISTENT_ID),
        Some(&NowPlayingValue::Integer(42))
    );
    assert_eq!(
        applied.get(keys::MEDIA_ITEM_RELEASE_DATE),
        Some(&NowPlayingValue::Date(before_epoch))
    );
}

#[test]
fn rejected_now_playing_info_leaves_the_current_info_untouched() {
    let _center_lock = exclusive_center();
    let center = NowPlayingInfoCenter::default_center();
    center
        .set_now_playing_info(&NowPlayingInfo::new().title("Kept"))
        .expect("baseline info should apply");

    let rejected = [
        NowPlayingInfo::new().title("Nul\0Title"),
        NowPlayingInfo::new().value(keys::MEDIA_ITEM_GENRE, text("Nul\0Genre")),
        NowPlayingInfo::new().value("NotANowPlayingKey", text("value")),
        NowPlayingInfo::new().value(keys::PLAYLIST_NAME, text("playlist")),
        NowPlayingInfo::new().value(keys::ASSET_URL, NowPlayingValue::Url(String::new())),
        NowPlayingInfo::new().asset_url(""),
        NowPlayingInfo::new().value(keys::TITLE, NowPlayingValue::Other("NSObject".to_string())),
        NowPlayingInfo::new().value(keys::ARTWORK, text("artwork")),
        NowPlayingInfo::new().value(keys::AVAILABLE_LANGUAGE_OPTIONS, text("options")),
        NowPlayingInfo::new().value(keys::CURRENT_LANGUAGE_OPTIONS, text("options")),
        NowPlayingInfo::new().value(keys::ANIMATED_ARTWORK_1X1, text("artwork")),
        NowPlayingInfo::new().value(keys::ANIMATED_ARTWORK_3X4, text("artwork")),
    ];
    for info in &rejected {
        let result = center.set_now_playing_info(&info.clone().artist("Replaced"));
        assert!(
            matches!(result, Err(MediaPlayerError::InvalidArgument(_))),
            "{info:?} returned {result:?}"
        );
        let applied = center
            .now_playing_info()
            .expect("baseline info should still be set");
        assert_eq!(applied.get(keys::TITLE), Some(&text("Kept")), "{info:?}");
        assert_eq!(applied.get(keys::ARTIST), None, "{info:?}");
    }
}

#[test]
fn language_option_characteristic_constants_map_to_sdk_values() {
    let characteristics = [
        keys::LANGUAGE_OPTION_CHARACTERISTIC_IS_MAIN_PROGRAM_CONTENT,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_IS_AUXILIARY_CONTENT,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_CONTAINS_ONLY_FORCED_SUBTITLES,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_TRANSCRIBES_SPOKEN_DIALOG,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_MUSIC_AND_SOUND,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_EASY_TO_READ,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_DESCRIBES_VIDEO,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_LANGUAGE_TRANSLATION,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_DUBBED_TRANSLATION,
        keys::LANGUAGE_OPTION_CHARACTERISTIC_VOICE_OVER_TRANSLATION,
    ];
    let option = LanguageOption::new(
        LanguageOptionType::Legible,
        Some("en"),
        &characteristics,
        "English",
        "subtitles-en",
    )
    .expect("language option should be created");
    let resolved = option.characteristics();

    assert_eq!(resolved.len(), characteristics.len());
    assert_eq!(resolved[0], "public.main-program-content");
    for (name, value) in characteristics.iter().zip(&resolved) {
        assert!(value.starts_with("public."), "{name} resolved to {value}");
    }
}
