use mediaplayer::MediaPlaylist;

#[test]
fn media_playlist_is_explicitly_unavailable_on_macos() {
    assert!(!MediaPlaylist::is_supported());

    let reason = MediaPlaylist::unavailable_reason();
    assert!(reason.contains("MPMediaPlaylist"), "unexpected reason: {reason}");

    let err = MediaPlaylist::playlist_named().expect_err("MPMediaPlaylist should be unavailable on macOS");
    assert!(err.to_string().contains("MPMediaPlaylist"));
}
