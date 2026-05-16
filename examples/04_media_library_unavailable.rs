//! Smoke example for the explicit macOS-unavailable `MPMediaLibrary` wrapper.

use mediaplayer::MediaLibrary;

fn main() {
    assert!(!MediaLibrary::is_supported());
    println!("{}", MediaLibrary::unavailable_reason());
    let err = MediaLibrary::default_media_library().expect_err("MPMediaLibrary should be unavailable on macOS");
    println!("{err}");
}
