use mediaplayer::VolumeView;

#[test]
fn volume_view_is_explicitly_unavailable_on_macos() {
    assert!(!VolumeView::is_supported());

    let reason = VolumeView::unavailable_reason();
    assert!(reason.contains("MPVolumeView"), "unexpected reason: {reason}");

    let err = VolumeView::new().expect_err("MPVolumeView should be unavailable on macOS");
    assert!(err.to_string().contains("MPVolumeView"));
}
