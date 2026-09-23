# mediaplayer-rs

Safe Rust bindings for Apple's `MediaPlayer.framework` on macOS.

> **Status:** covers the macOS-available now-playing, remote-command, language-option, and artwork APIs. Every `MPMediaItemProperty*` / `MPNowPlayingInfoProperty*` key in `constants` can be set through `NowPlayingInfo::value` and read back with `NowPlayingInfoCenter::now_playing_info`. The iOS-only `MPMediaLibrary`, `MPMediaQuery`, `MPMusicPlayer`, `MPMediaItem`, `MPMediaItemCollection`, `MPMediaPlaylist`, `MPVolumeView`, `MPSystemMusicPlayer`, and `MPPlayableContentDataSource` areas are explicit macOS-unavailable wrappers. The optional `async` feature adds `RemoteCommandStream`.

## Requirements

- macOS 12 or later (the Swift bridge's deployment target).
- Newer keys are version-gated: `MPNowPlayingInfoPropertyCreditsStartTime` needs macOS 13, the ISRC and exclude-from-suggestions keys need macOS 15, and animated artwork needs macOS 26. On older systems the typed builder skips them, and passing them to `NowPlayingInfo::value` makes `set_now_playing_info` return `InvalidArgument`.
- Remote-command handlers are delivered on the main queue, so the process must run the main run loop (an app, or `CFRunLoopRun` in a command-line tool) to receive commands.

## Async streams (`async` feature)

Enable with `features = ["async"]`:

```toml
mediaplayer = { version = "0.3", features = ["async"] }
```

| Stream type | Apple surface | macOS |
|---|---|---|
| `RemoteCommandStream` | `MPRemoteCommandCenter` command targets (20 commands) | ✅ |
| `NowPlayingItemChangeStream` | `MPMusicPlayerControllerNowPlayingItemDidChangeNotification` | `subscribe` returns `NotAvailable` |
| `PlaybackStateChangeStream` | `MPMusicPlayerControllerPlaybackStateDidChangeNotification` | `subscribe` returns `NotAvailable` |
| `VolumeChangeStream` | `MPMusicPlayerControllerVolumeDidChangeNotification` | `subscribe` returns `NotAvailable` |
| `MediaLibraryChangeStream` | `MPMediaLibraryDidChangeNotification` | `subscribe` returns `NotAvailable` |
| `NowPlayingSessionStream` | `MPNowPlayingSession` delegate | `subscribe` returns `NotAvailable` |

Apple marks the music-player, media-library and now-playing-session surfaces unavailable on macOS, so those streams could never deliver an event; their constructors return `MediaPlayerError::NotAvailable` instead of an idle stream. Stream capacities must be at least 1.

```rust,no_run
# #[cfg(feature = "async")]
# async fn run() -> Result<(), mediaplayer::MediaPlayerError> {
use mediaplayer::async_api::RemoteCommandStream;
use mediaplayer::remote_commands::Command;

let stream = RemoteCommandStream::subscribe(Command::Play, 16)?;
while let Some(event) = stream.next().await {
    println!("play at t={:.3}", event.timestamp);
}
# Ok(())
# }
```


## Quick start

```rust,no_run
use std::time::UNIX_EPOCH;

use mediaplayer::constants;
use mediaplayer::prelude::*;

fn main() -> Result<(), MediaPlayerError> {
    let subtitles = LanguageOption::new(
        LanguageOptionType::Legible,
        Some("en"),
        &["public.legible"],
        "English Subtitles",
        "subtitles-en",
    )?;

    let subtitle_group = LanguageOptionGroup::new(&[subtitles.clone()], Some(0), true)?;

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
        .media_type(NowPlayingMediaType::Audio)
        .value(constants::MEDIA_ITEM_COMPOSER, NowPlayingValue::String("Composer".into()))
        .value(constants::MEDIA_ITEM_ALBUM_TRACK_NUMBER, NowPlayingValue::Integer(3));

    center.set_now_playing_info(&info)?;
    center.set_playback_state(PlaybackState::Playing);

    if let Some(applied) = center.now_playing_info() {
        println!("composer = {:?}", applied.get(constants::MEDIA_ITEM_COMPOSER));
    }

    let remote = RemoteCommandCenter::shared();
    remote.play_command().set_enabled(true);
    remote.skip_forward_command().set_preferred_intervals(&[15.0, 30.0]);
    remote.change_playback_rate_command().set_supported_playback_rates(&[1.0, 1.5, 2.0]);

    let _play = remote.on_play(|_| HandlerStatus::Success)?;
    let _rating = remote.on_rating(|event| {
        println!("rating event = {:?}", event.rating);
        HandlerStatus::Success
    })?;

    center.clear();
    Ok(())
}
```

## Highlights

- **`NowPlayingInfoCenter`** — fluent `NowPlayingInfo` builder covering queue state, playback progress, language options, service identifiers, live-stream flags, and playback dates, plus `NowPlayingInfo::value(key, NowPlayingValue)` for any media-item or now-playing key in `constants` and `now_playing_info()` to read the current dictionary back.
  - Setters return `Result`. Strings with NUL bytes, unknown keys, invalid URLs, and object-valued keys (artwork, language options and animated artwork have typed setters) are `InvalidArgument`, and a rejected update leaves the current info untouched.
  - Readback reports keys by symbol name. `MPMediaEntityPropertyPersistentID` and `MPMediaItemPropertyPersistentID` share one dictionary key and read back as the latter, unsigned values that fit in `i64` read back as `Integer`, and objects read back as `Other` with their type name.
- **`LanguageOption` / `LanguageOptionGroup`** — wrappers for `MPNowPlayingInfoLanguageOption` and `MPNowPlayingInfoLanguageOptionGroup`; the `constants::LANGUAGE_OPTION_CHARACTERISTIC_*` names are translated to the SDK's characteristic values.
- **`RemoteCommandCenter`** — zero-cost command handles for base, skip-interval, feedback, rating, playback-rate, shuffle, repeat, and language-option commands. Registering a handler returns `Result<CommandToken, MediaPlayerError>`.
- **`CommandToken`** — RAII guard. Dropping it (on any thread) removes the target, and the handler block owns the closure's context, so the closure is freed only after MediaPlayer releases the block. Deliveries to one handler are serialized; a re-entrant delivery gets `HandlerStatus::CommandFailed`.
- **`Artwork`** — `MPMediaItemArtwork` from file paths or in-memory image data (`Artwork::from_image_data`), with a request handler that aspect-fits the image to the requested size. `image_png_data` returns the image the artwork produces for a size; MediaPlayer clamps requests larger than the artwork bounds to the bounds.
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
cargo run --example 13_content_item_smoke
cargo run --features async --example 14_async_streams
```

## Verification

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
