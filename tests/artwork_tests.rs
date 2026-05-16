use mediaplayer::Artwork;

#[test]
fn artwork_loads_fixture_and_reports_bounds() {
    let artwork = Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork.bounds().expect("artwork bounds should be available");
    assert!(bounds.width > 0.0);
    assert!(bounds.height > 0.0);

    let cloned = artwork.clone();
    drop(artwork);

    let cloned_bounds = cloned.bounds().expect("cloned artwork bounds should be available");
    assert!((bounds.width - cloned_bounds.width).abs() < f64::EPSILON);
    assert!((bounds.height - cloned_bounds.height).abs() < f64::EPSILON);
}
