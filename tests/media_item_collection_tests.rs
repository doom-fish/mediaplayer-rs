use mediaplayer::MediaItemCollection;

#[test]
fn media_item_collection_is_explicitly_unavailable_on_macos() {
    assert!(!MediaItemCollection::is_supported());

    let reason = MediaItemCollection::unavailable_reason();
    assert!(reason.contains("MPMediaItemCollection"), "unexpected reason: {reason}");

    let err = MediaItemCollection::collection_with_items().expect_err("MPMediaItemCollection should be unavailable on macOS");
    assert!(err.to_string().contains("MPMediaItemCollection"));
}
