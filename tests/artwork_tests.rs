use mediaplayer::{AnimatedArtwork, Artwork, NowPlayingInfo, NowPlayingInfoCenter};

#[test]
fn artwork_loads_fixture_and_reports_bounds() {
    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork
        .bounds()
        .expect("artwork bounds should be available");
    assert!(bounds.width > 0.0);
    assert!(bounds.height > 0.0);

    let cloned = artwork.clone();
    drop(artwork);

    let cloned_bounds = cloned
        .bounds()
        .expect("cloned artwork bounds should be available");
    assert!((bounds.width - cloned_bounds.width).abs() < f64::EPSILON);
    assert!((bounds.height - cloned_bounds.height).abs() < f64::EPSILON);
}

#[test]
fn animated_artwork_can_be_created_and_applied() {
    let animated = AnimatedArtwork::from_files(
        "cover-loop",
        "tests/fixtures/cover.png",
        "tests/fixtures/animated-artwork.mov",
    )
    .expect("animated artwork should be created from local files");
    let cloned = animated.clone();

    let center = NowPlayingInfoCenter::default_center();
    let info = NowPlayingInfo::new()
        .title("Animated Artwork Demo")
        .animated_artwork_1x1(animated)
        .animated_artwork_3x4(cloned);

    center.set_now_playing_info(&info);
    let _supported_keys = center.supported_animated_artwork_keys();
    center.clear();
}
