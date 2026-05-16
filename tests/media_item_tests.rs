use mediaplayer::MediaItem;

#[test]
fn media_item_is_explicitly_unavailable_on_macos() {
    assert!(!MediaItem::is_supported());

    let reason = MediaItem::unavailable_reason();
    assert!(reason.contains("MPMediaItem"), "unexpected reason: {reason}");

    let err = MediaItem::example_instance().expect_err("MPMediaItem should be unavailable on macOS");
    assert!(err.to_string().contains("MPMediaItem"));
}
