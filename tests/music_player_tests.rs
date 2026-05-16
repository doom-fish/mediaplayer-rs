use mediaplayer::MusicPlayer;

#[test]
fn music_player_is_explicitly_unavailable_on_macos() {
    assert!(!MusicPlayer::is_supported());

    let reason = MusicPlayer::unavailable_reason();
    assert!(reason.contains("MPMusicPlayer"), "unexpected reason: {reason}");

    let err = MusicPlayer::application_music_player().expect_err("MPMusicPlayer should be unavailable on macOS");
    assert!(err.to_string().contains("MPMusicPlayer"));
}
