use mediaplayer::PlayableContentDataSource;

#[test]
fn playable_content_data_source_is_explicitly_unavailable_on_macos() {
    assert!(!PlayableContentDataSource::is_supported());

    let reason = PlayableContentDataSource::unavailable_reason();
    assert!(reason.contains("MPPlayableContentDataSource"), "unexpected reason: {reason}");

    let err = PlayableContentDataSource::new().expect_err("MPPlayableContentDataSource should be unavailable on macOS");
    assert!(err.to_string().contains("MPPlayableContentDataSource"));
}
