use mediaplayer::SystemMusicPlayer;

#[test]
fn system_music_player_is_explicitly_unavailable_on_macos() {
    assert!(!SystemMusicPlayer::is_supported());

    let reason = SystemMusicPlayer::unavailable_reason();
    assert!(reason.contains("MPSystemMusicPlayer"), "unexpected reason: {reason}");

    let err = SystemMusicPlayer::shared().expect_err("MPSystemMusicPlayer should be unavailable on macOS");
    assert!(err.to_string().contains("MPSystemMusicPlayer"));
}
