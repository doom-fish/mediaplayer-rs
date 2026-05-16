use mediaplayer::{Artwork, ContentItem};

#[test]
fn content_item_round_trips_metadata() {
    let item = ContentItem::new("episode-1").expect("content item should be created");
    assert_eq!(item.identifier(), "episode-1");

    item.set_title(Some("Episode One"))
        .expect("title should be set");
    item.set_subtitle(Some("doom-fish"))
        .expect("subtitle should be set");

    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    item.set_artwork(Some(&artwork));
    item.set_playback_progress(0.5);
    item.set_streaming_content(true);
    item.set_explicit_content(false);
    item.set_container(false);
    item.set_playable(true);

    assert_eq!(item.title().as_deref(), Some("Episode One"));
    assert_eq!(item.subtitle().as_deref(), Some("doom-fish"));
    assert!((item.playback_progress() - 0.5).abs() < f32::EPSILON);
    assert!(item.is_streaming_content());
    assert!(!item.is_explicit_content());
    assert!(!item.is_container());
    assert!(item.is_playable());

    let returned_artwork = item
        .artwork()
        .expect("content item artwork should round-trip");
    assert!(returned_artwork.bounds().is_some());

    let cloned = item.clone();
    drop(item);

    assert_eq!(cloned.identifier(), "episode-1");
    assert_eq!(cloned.title().as_deref(), Some("Episode One"));
    cloned.set_title(None).expect("title should clear");
    cloned.set_subtitle(None).expect("subtitle should clear");
    assert_eq!(cloned.title(), None);
    assert_eq!(cloned.subtitle(), None);
}
