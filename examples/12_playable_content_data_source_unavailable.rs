//! Smoke example for the explicit macOS-unavailable `MPPlayableContentDataSource` wrapper.

use mediaplayer::PlayableContentDataSource;

fn main() {
    assert!(!PlayableContentDataSource::is_supported());
    println!("{}", PlayableContentDataSource::unavailable_reason());
    let err = PlayableContentDataSource::new().expect_err("MPPlayableContentDataSource should be unavailable on macOS");
    println!("{err}");
}
