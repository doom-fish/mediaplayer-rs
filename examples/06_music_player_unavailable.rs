//! Smoke example for the explicit macOS-unavailable `MPMusicPlayer` wrapper.

use mediaplayer::MusicPlayer;

fn main() {
    assert!(!MusicPlayer::is_supported());
    println!("{}", MusicPlayer::unavailable_reason());
    let err = MusicPlayer::application_music_player().expect_err("MPMusicPlayer should be unavailable on macOS");
    println!("{err}");
}
