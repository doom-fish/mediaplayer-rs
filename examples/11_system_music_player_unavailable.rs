//! Smoke example for the explicit macOS-unavailable `MPSystemMusicPlayer` wrapper.

use mediaplayer::SystemMusicPlayer;

fn main() {
    assert!(!SystemMusicPlayer::is_supported());
    println!("{}", SystemMusicPlayer::unavailable_reason());
    let err = SystemMusicPlayer::shared().expect_err("MPSystemMusicPlayer should be unavailable on macOS");
    println!("{err}");
}
