use std::sync::{Mutex, MutexGuard, PoisonError};

use apple_cf::cg::CGSize;
use mediaplayer::constants as keys;
use mediaplayer::{
    AnimatedArtwork, Artwork, MediaPlayerError, NowPlayingInfo, NowPlayingInfoCenter,
    NowPlayingValue,
};

static SHARED_CENTER: Mutex<()> = Mutex::new(());

fn exclusive_center() -> MutexGuard<'static, ()> {
    SHARED_CENTER.lock().unwrap_or_else(PoisonError::into_inner)
}

fn fixture_bytes() -> Vec<u8> {
    std::fs::read("tests/fixtures/cover.png").expect("fixture artwork should be readable")
}

fn png_dimensions(png: &[u8]) -> (u32, u32) {
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "not a PNG");
    assert_eq!(&png[12..16], b"IHDR");
    let read = |range: std::ops::Range<usize>| {
        u32::from_be_bytes(png[range].try_into().expect("IHDR field is four bytes"))
    };
    (read(16..20), read(20..24))
}

#[test]
fn artwork_loads_fixture_and_reports_bounds() {
    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork
        .bounds()
        .expect("artwork bounds should be available");
    assert!(bounds.size.width > 0.0);
    assert!(bounds.size.height > 0.0);

    let cloned = artwork.clone();
    drop(artwork);

    let cloned_bounds = cloned
        .bounds()
        .expect("cloned artwork bounds should be available");
    assert!((bounds.size.width - cloned_bounds.size.width).abs() < f64::EPSILON);
    assert!((bounds.size.height - cloned_bounds.size.height).abs() < f64::EPSILON);
}

#[test]
fn animated_artwork_can_be_created_and_applied() {
    let _center_lock = exclusive_center();
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

    center
        .set_now_playing_info(&info)
        .expect("animated artwork should apply");
    let applied = center
        .now_playing_info()
        .expect("now-playing info should read back");
    assert!(matches!(
        applied.get(keys::ANIMATED_ARTWORK_1X1),
        Some(NowPlayingValue::Other(_))
    ));
    assert!(matches!(
        applied.get(keys::ANIMATED_ARTWORK_3X4),
        Some(NowPlayingValue::Other(_))
    ));
    let _supported_keys = center.supported_animated_artwork_keys();
    center.clear();
}

#[test]
fn artwork_decodes_in_memory_image_data() {
    let natural =
        Artwork::from_image_data(&fixture_bytes(), None).expect("fixture bytes should decode");
    let natural_bounds = natural
        .bounds()
        .expect("artwork bounds should be available");
    assert_eq!(natural_bounds.size, CGSize::new(1.0, 1.0));

    let sized = Artwork::from_image_data(&fixture_bytes(), Some(CGSize::new(600.0, 400.0)))
        .expect("fixture bytes should decode with explicit bounds");
    let sized_bounds = sized.bounds().expect("artwork bounds should be available");
    assert_eq!(sized_bounds.size, CGSize::new(600.0, 400.0));
}

#[test]
fn artwork_rejects_unusable_image_data() {
    assert!(matches!(
        Artwork::from_image_data(&[], None),
        Err(MediaPlayerError::InvalidArgument(_))
    ));
    assert!(matches!(
        Artwork::from_image_data(b"definitely not an image", None),
        Err(MediaPlayerError::Framework(_))
    ));
    for size in [
        CGSize::new(0.0, 10.0),
        CGSize::new(10.0, f64::NAN),
        CGSize::new(20_000.0, 10.0),
    ] {
        assert!(matches!(
            Artwork::from_image_data(&fixture_bytes(), Some(size)),
            Err(MediaPlayerError::InvalidArgument(_))
        ));
    }
}

#[test]
fn artwork_images_honour_the_requested_size() {
    let bounds = CGSize::new(600.0, 400.0);
    let in_memory = Artwork::from_image_data(&fixture_bytes(), Some(bounds))
        .expect("fixture bytes should decode");
    let from_file = Artwork::from_path_with_size("tests/fixtures/cover.png", bounds)
        .expect("fixture artwork should load");

    for artwork in [&in_memory, &from_file] {
        let square = artwork
            .image_png_data(CGSize::new(64.0, 64.0))
            .expect("artwork should render at 64x64");
        assert_eq!(png_dimensions(&square), (64, 64));

        let wide = artwork
            .image_png_data(CGSize::new(128.0, 32.0))
            .expect("artwork should render at 128x32");
        assert_eq!(png_dimensions(&wide), (128, 32));

        let oversized = artwork
            .image_png_data(CGSize::new(1000.0, 1000.0))
            .expect("artwork should render oversized requests at its bounds");
        assert_eq!(png_dimensions(&oversized), (600, 400));
    }
}

#[test]
fn artwork_image_requests_validate_the_size() {
    let artwork =
        Artwork::from_image_data(&fixture_bytes(), None).expect("fixture bytes should decode");
    for size in [
        CGSize::new(0.0, 64.0),
        CGSize::new(64.0, -1.0),
        CGSize::new(f64::INFINITY, 64.0),
        CGSize::new(64.0, 16_385.0),
    ] {
        assert!(matches!(
            artwork.image_png_data(size),
            Err(MediaPlayerError::InvalidArgument(_))
        ));
    }
}

#[test]
fn in_memory_artwork_applies_to_now_playing() {
    let _center_lock = exclusive_center();
    let artwork = Artwork::from_image_data(&fixture_bytes(), Some(CGSize::new(300.0, 300.0)))
        .expect("fixture bytes should decode");
    let center = NowPlayingInfoCenter::default_center();
    center
        .set_now_playing_info_with_artwork(&NowPlayingInfo::new().title("Artwork"), Some(&artwork))
        .expect("artwork should apply");
    let applied = center
        .now_playing_info()
        .expect("now-playing info should read back");
    assert!(matches!(
        applied.get(keys::ARTWORK),
        Some(NowPlayingValue::Other(_))
    ));
    center.clear();
}
