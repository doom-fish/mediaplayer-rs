//! Example: async streams smoke test.
//!
//! Demonstrates subscribing to `RemoteCommandStream` and shows that the
//! notification and `NowPlayingSessionStream` constructors report
//! `NotAvailable` on macOS.  Each remote-command subscription is created and
//! immediately dropped to verify the subscribe → drop path works on a
//! headless macOS machine.
//!
//! Run with:
//! ```
//! cargo run --features async --example 14_async_streams
//! ```

#![cfg(feature = "async")]

fn main() {
    #[cfg(feature = "async")]
    run();
    #[cfg(not(feature = "async"))]
    {
        eprintln!("Rerun with --features async");
        std::process::exit(1);
    }
}

#[cfg(feature = "async")]
fn run() {
    use mediaplayer::async_api::{
        MediaLibraryChangeStream, NowPlayingItemChangeStream, NowPlayingSessionStream,
        PlaybackStateChangeStream, RemoteCommandStream, VolumeChangeStream,
    };
    use mediaplayer::remote_commands::Command;

    println!("=== mediaplayer async_api smoke test ===");

    // ── Notification streams ────────────────────────────────────────────────
    let unavailable = [
        (
            "NowPlayingItemChangeStream",
            NowPlayingItemChangeStream::subscribe(8).err(),
        ),
        (
            "PlaybackStateChangeStream",
            PlaybackStateChangeStream::subscribe(8).err(),
        ),
        ("VolumeChangeStream", VolumeChangeStream::subscribe(8).err()),
        (
            "MediaLibraryChangeStream",
            MediaLibraryChangeStream::subscribe(8).err(),
        ),
        (
            "NowPlayingSessionStream",
            NowPlayingSessionStream::subscribe(8).err(),
        ),
    ];
    for (name, error) in unavailable {
        let error = error.expect("stream should be unavailable on macOS");
        println!("[ok] {name:<27} {error}");
    }

    // ── RemoteCommandStream ─────────────────────────────────────────────────
    let commands = [
        Command::Play,
        Command::Pause,
        Command::Stop,
        Command::TogglePlayPause,
        Command::NextTrack,
        Command::PreviousTrack,
        Command::SkipForward,
        Command::SkipBackward,
        Command::SeekForward,
        Command::SeekBackward,
    ];
    for cmd in commands {
        let s = RemoteCommandStream::subscribe(cmd, 16).expect("remote command should subscribe");
        // try_next on an idle stream should return None
        assert!(
            s.try_next().is_none(),
            "unexpected buffered event for {cmd:?}"
        );
        println!("[ok] RemoteCommandStream({cmd:?})  buffered={}", s.buffered_count());
    }

    println!("=== all streams created and dropped cleanly — PASS ===");
}
