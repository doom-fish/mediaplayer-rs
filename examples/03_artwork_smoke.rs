//! Load a tiny PNG fixture into `MPMediaItemArtwork` and print its bounds.

use mediaplayer::Artwork;

fn main() {
    let artwork = Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let bounds = artwork.bounds().expect("artwork bounds should be available");
    println!("artwork bounds = {bounds:?}");
}
