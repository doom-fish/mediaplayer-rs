# mediaplayer-rs

Safe Rust bindings for Apple's `MediaPlayer.framework` on macOS — Now Playing info and Remote Commands.

> **Status:** v0.1.0 covers `MPNowPlayingInfoCenter` metadata, `MPRemoteCommandCenter` handler registration, and `MPMediaItemArtwork` creation from file paths.

## Quick start

```rust,no_run
use mediaplayer::prelude::*;

fn main() {
    let center = NowPlayingInfoCenter::default_center();

    let info = NowPlayingInfo::new()
        .title("My Song")
        .artist("doom-fish")
        .album_title("Tests")
        .playback_duration(300.0)
        .elapsed_playback_time(0.0)
        .playback_rate(1.0)
        .media_type(NowPlayingMediaType::Audio);

    center.set_now_playing_info(&info);
    center.set_playback_state(PlaybackState::Playing);

    let rcc = RemoteCommandCenter::shared();

    let _play = rcc.on_play(|_| HandlerStatus::Success);
    let _pause = rcc.on_pause(|_| HandlerStatus::Success);

    // … your playback loop …

    center.clear();   // or just drop `center`
}
```

## Highlights

- **`NowPlayingInfoCenter`** — fluent `NowPlayingInfo` builder, `set_now_playing_info`, `set_playback_state`, `clear()`, auto-clear on drop.
- **`RemoteCommandCenter`** — `add_handler` + per-command convenience (`on_play`, `on_pause`, `on_toggle_play_pause`, `on_next_track`, `on_previous_track`, `on_skip_forward`, `on_skip_backward`, `on_seek_forward`, `on_seek_backward`, `on_change_playback_position`).
- **`CommandToken`** — RAII guard that deregisters and frees the handler on drop.
- **`Artwork`** — `MPMediaItemArtwork` from a file path; optional explicit bounds `CGSize` via `apple-cf`.
- **`constants`** — now-playing info dictionary key strings for documentation reference.

## Availability

- `MPNowPlayingInfoCenter`, `MPRemoteCommandCenter` — macOS 10.12.2+
- `MPNowPlayingInfoCenter.playbackState` — macOS 10.12.2+
- `MPMediaItemArtwork(boundsSize:requestHandler:)` — macOS 10.12.2+

## Smoke example

```bash
cargo run --example 01_now_playing_smoke
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
