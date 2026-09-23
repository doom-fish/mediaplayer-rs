//! Load fixture artwork, including animated artwork, and print the resulting handles.

use apple_cf::cg::CGSize;
use mediaplayer::{AnimatedArtwork, Artwork};

fn main() {
    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork
        .bounds()
        .expect("artwork bounds should be available");
    println!("artwork bounds = {bounds:?}");

    let bytes =
        std::fs::read("tests/fixtures/cover.png").expect("fixture artwork should be readable");
    let in_memory = Artwork::from_image_data(&bytes, Some(CGSize::new(600.0, 600.0)))
        .expect("in-memory artwork should decode");
    let thumbnail = in_memory
        .image_png_data(CGSize::new(64.0, 64.0))
        .expect("artwork should render a thumbnail");
    println!(
        "in-memory artwork thumbnail = {} PNG bytes",
        thumbnail.len()
    );

    let animated = AnimatedArtwork::from_files(
        "cover-loop",
        "tests/fixtures/cover.png",
        "tests/fixtures/animated-artwork.mov",
    )
    .expect("animated artwork should be created");
    println!("animated artwork = {animated:?}");
}
