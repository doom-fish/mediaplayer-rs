//! Example: async streams smoke test.
//!
//! Demonstrates subscribing to `RemoteCommandStream`,
//! `NowPlayingItemChangeStream`, and `NowPlayingSessionStream`.  Each
//! subscription is created and immediately dropped to verify the
//! subscribe → drop → EOF path works on a headless macOS machine.
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
    {
        let s = NowPlayingItemChangeStream::subscribe(8);
        println!(
            "[ok] NowPlayingItemChangeStream  buffered={} closed={}",
            s.buffered_count(),
            s.is_closed()
        );
    }
    {
        let s = PlaybackStateChangeStream::subscribe(8);
        println!(
            "[ok] PlaybackStateChangeStream   buffered={} closed={}",
            s.buffered_count(),
            s.is_closed()
        );
    }
    {
        let s = VolumeChangeStream::subscribe(8);
        println!(
            "[ok] VolumeChangeStream          buffered={} closed={}",
            s.buffered_count(),
            s.is_closed()
        );
    }
    {
        let s = MediaLibraryChangeStream::subscribe(8);
        println!(
            "[ok] MediaLibraryChangeStream    buffered={} closed={}",
            s.buffered_count(),
            s.is_closed()
        );
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
        let s = RemoteCommandStream::subscribe(cmd, 16);
        // try_next on an idle stream should return None
        assert!(
            s.try_next().is_none(),
            "unexpected buffered event for {cmd:?}"
        );
        println!("[ok] RemoteCommandStream({cmd:?})  buffered={}", s.buffered_count());
    }

    // ── NowPlayingSessionStream ─────────────────────────────────────────────
    {
        let s = NowPlayingSessionStream::subscribe(8);
        println!(
            "[ok] NowPlayingSessionStream     buffered={} closed={}",
            s.buffered_count(),
            s.is_closed()
        );
    }

    println!("=== all streams created and dropped cleanly — PASS ===");
}
