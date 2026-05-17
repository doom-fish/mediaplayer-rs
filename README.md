# mediaplayer-rs

Safe Rust bindings for Apple's `MediaPlayer.framework` on macOS.

> **Status:** v0.3.0 adds a Tier-2 async stream module (`async` feature) wrapping six notification / delegate / command-target surfaces as executor-agnostic `BoundedAsyncStream<T>` event streams.  v0.2.0 covers the macOS-available now-playing, remote-command, language-option, and artwork APIs, and exposes explicit macOS-unavailable wrappers for the iOS-only `MPMediaLibrary`, `MPMediaQuery`, `MPMusicPlayer`, `MPMediaItem`, `MPMediaItemCollection`, `MPMediaPlaylist`, `MPVolumeView`, `MPSystemMusicPlayer`, and `MPPlayableContentDataSource` areas.

## Async streams (`async` feature)

Enable with `features = ["async"]`:

```toml
mediaplayer = { version = "0.3", features = ["async"] }
```

| Stream type | Apple surface |
|---|---|
| `NowPlayingItemChangeStream` | `MPMusicPlayerControllerNowPlayingItemDidChangeNotification` |
| `PlaybackStateChangeStream` | `MPMusicPlayerControllerPlaybackStateDidChangeNotification` |
| `VolumeChangeStream` | `MPMusicPlayerControllerVolumeDidChangeNotification` |
| `MediaLibraryChangeStream` | `MPMediaLibraryDidChangeNotification` |
| `RemoteCommandStream` | `MPRemoteCommandCenter` command targets (20 commands) |
| `NowPlayingSessionStream` | `MPNowPlayingSession` delegate (stub; unavailable on macOS) |

```rust,no_run
# #[cfg(feature = "async")]
# async fn run() {
use mediaplayer::async_api::RemoteCommandStream;
use mediaplayer::remote_commands::Command;

let stream = RemoteCommandStream::subscribe(Command::Play, 16);
while let Some(event) = stream.next().await {
    println!("play at t={:.3}", event.timestamp);
}
# }
```


## Quick start

```rust,no_run
use std::time::UNIX_EPOCH;

use mediaplayer::prelude::*;

fn main() {
    let subtitles = LanguageOption::new(
        LanguageOptionType::Legible,
        Some("en"),
        &["public.legible"],
        "English Subtitles",
        "subtitles-en",
    )
    .unwrap();

    let subtitle_group = LanguageOptionGroup::new(&[subtitles.clone()], Some(0), true).unwrap();

    let center = NowPlayingInfoCenter::default_center();
    let info = NowPlayingInfo::new()
        .title("My Song")
        .artist("doom-fish")
        .album_title("Tests")
        .playback_duration(300.0)
        .elapsed_playback_time(0.0)
        .playback_rate(1.0)
        .default_playback_rate(1.0)
        .playback_queue_index(0)
        .playback_queue_count(1)
        .available_language_option_groups(vec![subtitle_group])
        .current_language_options(vec![subtitles])
        .current_playback_date(UNIX_EPOCH)
        .media_type(NowPlayingMediaType::Audio);

    center.set_now_playing_info(&info);
    center.set_playback_state(PlaybackState::Playing);

    let remote = RemoteCommandCenter::shared();
    remote.play_command().set_enabled(true);
    remote.skip_forward_command().set_preferred_intervals(&[15.0, 30.0]);
    remote.change_playback_rate_command().set_supported_playback_rates(&[1.0, 1.5, 2.0]);

    let _play = remote.on_play(|_| HandlerStatus::Success);
    let _rating = remote.on_rating(|event| {
        println!("rating event = {:?}", event.rating);
        HandlerStatus::Success
    });

    center.clear();
}
```

## Highlights

- **`NowPlayingInfoCenter`** — fluent `NowPlayingInfo` builder covering queue state, playback progress, language options, service identifiers, live-stream flags, and playback dates.
- **`LanguageOption` / `LanguageOptionGroup`** — wrappers for `MPNowPlayingInfoLanguageOption` and `MPNowPlayingInfoLanguageOptionGroup`.
- **`RemoteCommandCenter`** — zero-cost command handles for base, skip-interval, feedback, rating, playback-rate, shuffle, repeat, and language-option commands.
- **`CommandToken`** — RAII guard that deregisters closures on drop.
- **`Artwork`** — `MPMediaItemArtwork` from file paths, plus bounds inspection.
- **Explicit macOS stubs** — `MediaLibrary`, `MediaQuery`, `MusicPlayer`, `MediaItem`, `MediaItemCollection`, `MediaPlaylist`, `VolumeView`, `SystemMusicPlayer`, and `PlayableContentDataSource` all report the Apple availability reason instead of failing mysteriously.

## Example matrix

```bash
cargo run --example 01_now_playing_smoke
cargo run --example 02_remote_command_center_smoke
cargo run --example 03_artwork_smoke
cargo run --example 04_media_library_unavailable
cargo run --example 05_media_query_unavailable
cargo run --example 06_music_player_unavailable
cargo run --example 07_media_item_unavailable
cargo run --example 08_media_item_collection_unavailable
cargo run --example 09_media_playlist_unavailable
cargo run --example 10_volume_view_unavailable
cargo run --example 11_system_music_player_unavailable
cargo run --example 12_playable_content_data_source_unavailable
```

## Verification

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
