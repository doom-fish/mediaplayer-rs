//! Smoke test: set Now Playing metadata, register remote command handlers,
//! wait briefly, then clear state before exit.

use std::time::Duration;

use mediaplayer::prelude::*;

fn main() {
    println!("== mediaplayer smoke ==");

    // ── Now Playing ──────────────────────────────────────────────────────────

    let center = NowPlayingInfoCenter::default_center();

    let info = NowPlayingInfo::new()
        .title("Smoke Test Song")
        .artist("doom-fish")
        .album_title("Crate Tests")
        .playback_duration(300.0)
        .elapsed_playback_time(0.0)
        .playback_rate(1.0)
        .media_type(NowPlayingMediaType::Audio);

    center.set_now_playing_info(&info);
    center.set_playback_state(PlaybackState::Playing);

    println!(
        "playback state = {:?}",
        center.playback_state()
    );

    // ── Remote Commands ──────────────────────────────────────────────────────

    let rcc = RemoteCommandCenter::shared();

    let _play_token = rcc.on_play(|event| {
        println!("▶️  play received  ts={:.3}", event.timestamp);
        HandlerStatus::Success
    });

    let _pause_token = rcc.on_pause(|event| {
        println!("⏸  pause received  ts={:.3}", event.timestamp);
        HandlerStatus::Success
    });

    let _toggle_token = rcc.on_toggle_play_pause(|event| {
        println!("⏯  toggle play/pause  ts={:.3}", event.timestamp);
        HandlerStatus::Success
    });

    let _next_token = rcc.on_next_track(|event| {
        println!("⏭  next track  ts={:.3}", event.timestamp);
        HandlerStatus::Success
    });

    let _prev_token = rcc.on_previous_track(|event| {
        println!("⏮  previous track  ts={:.3}", event.timestamp);
        HandlerStatus::Success
    });

    println!("Now Playing set. Remote command handlers registered.");
    println!("Waiting 1 s for any system delivery…");
    std::thread::sleep(Duration::from_secs(1));

    // ── Cleanup ───────────────────────────────────────────────────────────────
    // Tokens dropped here → handlers deregistered automatically.
    // center dropped here → nowPlayingInfo = nil automatically.

    center.clear();
    println!("✅ mediaplayer now playing OK");
}
