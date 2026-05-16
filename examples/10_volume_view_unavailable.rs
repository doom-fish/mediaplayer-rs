//! Smoke example for the explicit macOS-unavailable `MPVolumeView` wrapper.

use mediaplayer::VolumeView;

fn main() {
    assert!(!VolumeView::is_supported());
    println!("{}", VolumeView::unavailable_reason());
    let err = VolumeView::new().expect_err("MPVolumeView should be unavailable on macOS");
    println!("{err}");
}
