use mediaplayer::MediaLibrary;

#[test]
fn media_library_is_explicitly_unavailable_on_macos() {
    assert!(!MediaLibrary::is_supported());

    let reason = MediaLibrary::unavailable_reason();
    assert!(reason.contains("MPMediaLibrary"), "unexpected reason: {reason}");

    let err = MediaLibrary::default_media_library().expect_err("MPMediaLibrary should be unavailable on macOS");
    assert!(err.to_string().contains("MPMediaLibrary"));
}
