//! Create an `MPContentItem`, assign artwork, and print the configured metadata.

use mediaplayer::{Artwork, ContentItem};

fn main() {
    let artwork =
        Artwork::from_path("tests/fixtures/cover.png").expect("fixture artwork should load");
    let item = ContentItem::new("demo-track").expect("content item should be created");

    item.set_title(Some("Demo Track"))
        .expect("title should be set");
    item.set_subtitle(Some("doom-fish"))
        .expect("subtitle should be set");
    item.set_artwork(Some(&artwork));
    item.set_playback_progress(0.25);
    item.set_streaming_content(true);
    item.set_playable(true);

    println!("identifier = {}", item.identifier());
    println!("title = {:?}", item.title());
    println!("subtitle = {:?}", item.subtitle());
    println!("streaming = {}", item.is_streaming_content());
    println!("playable = {}", item.is_playable());
    println!("content item = {item:?}");
}
