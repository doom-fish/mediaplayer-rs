//! Smoke example for the explicit macOS-unavailable `MPMediaPlaylist` wrapper.

use mediaplayer::MediaPlaylist;

fn main() {
    assert!(!MediaPlaylist::is_supported());
    println!("{}", MediaPlaylist::unavailable_reason());
    let err = MediaPlaylist::playlist_named().expect_err("MPMediaPlaylist should be unavailable on macOS");
    println!("{err}");
}
