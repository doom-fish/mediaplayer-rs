//! Load fixture artwork, including animated artwork, and print the resulting handles.

use mediaplayer::{AnimatedArtwork, Artwork};

fn main() {
    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork
        .bounds()
        .expect("artwork bounds should be available");
    println!("artwork bounds = {bounds:?}");

    let animated = AnimatedArtwork::from_files(
        "cover-loop",
        "tests/fixtures/cover.png",
        "tests/fixtures/animated-artwork.mov",
    )
    .expect("animated artwork should be created");
    println!("animated artwork = {animated:?}");
}
