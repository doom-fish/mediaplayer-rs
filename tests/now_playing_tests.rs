use std::time::UNIX_EPOCH;

use mediaplayer::{
    LanguageOption, LanguageOptionGroup, LanguageOptionType, NowPlayingInfo,
    NowPlayingInfoCenter, NowPlayingMediaType, PlaybackState,
};

#[test]
fn now_playing_smoke_with_language_options() {
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

    center.set_now_playing_info(&info);
    center.set_playback_state(PlaybackState::Playing);
    assert_eq!(center.playback_state(), PlaybackState::Playing);

    let _animated_keys = center.supported_animated_artwork_keys();
    center.clear();
}
