//! Smoke example for the explicit macOS-unavailable `MPMediaItem` wrapper.

use mediaplayer::MediaItem;

fn main() {
    assert!(!MediaItem::is_supported());
    println!("{}", MediaItem::unavailable_reason());
    let err = MediaItem::example_instance().expect_err("MPMediaItem should be unavailable on macOS");
    println!("{err}");
}
