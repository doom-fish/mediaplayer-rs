//! Smoke example for the explicit macOS-unavailable `MPMediaItemCollection` wrapper.

use mediaplayer::MediaItemCollection;

fn main() {
    assert!(!MediaItemCollection::is_supported());
    println!("{}", MediaItemCollection::unavailable_reason());
    let err = MediaItemCollection::collection_with_items().expect_err("MPMediaItemCollection should be unavailable on macOS");
    println!("{err}");
}
