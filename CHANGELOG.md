# Changelog

## [0.3.0] - 2026-05-27

### Added

- **`async` Cargo feature** — new `src/async_api.rs` module exposing six
  executor-agnostic `BoundedAsyncStream<T>`-backed event streams:
  - `NowPlayingItemChangeStream` — wraps `MPMusicPlayerControllerNowPlayingItemDidChangeNotification`
  - `PlaybackStateChangeStream` — wraps `MPMusicPlayerControllerPlaybackStateDidChangeNotification`
  - `VolumeChangeStream` — wraps `MPMusicPlayerControllerVolumeDidChangeNotification`
  - `MediaLibraryChangeStream` — wraps `MPMediaLibraryDidChangeNotification`
  - `RemoteCommandStream` — wraps `MPRemoteCommandCenter` command targets (all 20 commands);
    each event auto-acknowledges with `.success`
  - `NowPlayingSessionStream` — `MPNowPlayingSession` delegate stub (API-unavailable on macOS;
    stream is open but idle)
- Swift bridge file `AsyncStream.swift` with `mp_notification_subscribe/unsubscribe`,
  `mp_stream_remote_command_subscribe/unsubscribe`, and
  `mp_now_playing_session_stream_subscribe/unsubscribe` C-cdecl entry points.
- `src/ffi/async_stream.rs` with Rust `extern "C"` declarations for the new bridge functions.
- `examples/14_async_streams.rs` — smoke test for all six streams (exits 0 on headless macOS).
- `tests/async_stream_tests.rs` — 15 tests covering subscribe → try\_next → drop for each stream type.
- `doom-fish-utils` path-dependency (feature-gated behind `async`) providing the
  `BoundedAsyncStream<T>` / `AsyncStreamSender<T>` primitives.
- `pollster` dev-dependency for executor-agnostic example test patterns.

## [0.2.1] - 2026-05-16

### Added

- `ContentItem` wrapper for `MPContentItem` with metadata, artwork, progress, and playback-state setters/getters.
- `AnimatedArtwork::from_files` plus `NowPlayingInfo::{animated_artwork_1x1, animated_artwork_3x4}` for `MPMediaItemAnimatedArtwork` and the animated now-playing artwork keys.
- Raw-pointer bridge helpers `LanguageOption::from_av_media_selection_option_raw` and `LanguageOptionGroup::from_av_media_selection_group_raw` for the MediaPlayer AVFoundation language-option categories.
- `MediaType` bitflags, `MediaEntityPersistentId`, and symbolic constant exports covering the remaining `MPMediaEntity`, `MPMediaItem`, `MPMediaPlaylist`, `MPMediaPlayback`, and `MPLanguageOptionCharacteristic*` surfaces.
- Coverage-closing tests for content items, animated artwork, AV media-selection conversions, and constant smoke coverage, plus `examples/13_content_item_smoke.rs`.

## [0.2.0] - 2026-05-16

### Added

- `LanguageOption` and `LanguageOptionGroup` wrappers for `MPNowPlayingInfoLanguageOption` and `MPNowPlayingInfoLanguageOptionGroup`.
- Expanded `NowPlayingInfo` coverage for default playback rate, queue/chapter metadata, live-stream state, service/profile identifiers, playback progress, playback date, credits start time, ISRC, and exclude-from-suggestions.
- `NowPlayingInfoCenter::supported_animated_artwork_keys()` for the macOS 16+ class property.
- Command-handle wrappers for `MPRemoteCommand`, `MPSkipIntervalCommand`, `MPFeedbackCommand`, `MPRatingCommand`, `MPChangePlaybackRateCommand`, `MPChangePlaybackPositionCommand`, `MPChangeShuffleModeCommand`, and `MPChangeRepeatModeCommand`.
- Extended remote-command events with rating, playback-rate, feedback, language-option, repeat, and shuffle payloads.
- `Artwork::bounds()` plus `Clone` support for retained Objective-C wrappers.
- Explicit macOS-unavailable wrapper modules, Swift bridge files, examples, and tests for `MPMediaLibrary`, `MPMediaQuery`, `MPMusicPlayer`, `MPMediaItem`, `MPMediaItemCollection`, `MPMediaPlaylist`, `MPVolumeView`, `MPSystemMusicPlayer`, and `MPPlayableContentDataSource`.
- Integration tests for every logical area and numbered examples `01` through `12`.
- `COVERAGE.md` auditing the framework surface and documenting implemented, partial, and skipped areas.

## [0.1.0] - 2025-07-14

### Added

- `NowPlayingInfoCenter` wrapper for `MPNowPlayingInfoCenter.default()` with `set_now_playing_info`, `set_playback_state`, `playback_state`, `clear()`, and auto-clear on drop.
- `NowPlayingInfo` fluent builder covering title, artist, album title, playback duration, elapsed playback time, playback rate, media type, external content identifier, and asset URL.
- `NowPlayingMediaType` enum (`None`, `Audio`, `Video`) mapping `MPNowPlayingInfoMediaType`.
- `PlaybackState` enum (`Unknown`, `Playing`, `Paused`, `Stopped`, `Interrupted`) mapping `MPNowPlayingPlaybackState`.
- `RemoteCommandCenter` wrapper for `MPRemoteCommandCenter.shared()` with `add_handler` and per-command convenience methods: `on_play`, `on_pause`, `on_stop`, `on_toggle_play_pause`, `on_next_track`, `on_previous_track`, `on_skip_forward`, `on_skip_backward`, `on_seek_forward`, `on_seek_backward`, `on_change_playback_position`.
- `CommandToken` RAII guard that deregisters the remote command handler and frees the Rust closure on drop.
- `CommandEvent` with `command`, `timestamp`, `skip_interval`, `seek_type`, and `position` fields.
- `HandlerStatus` enum (`Success`, `NoSuchContent`, `NoActionableNowPlayingItem`, `DeviceNotFound`, `CommandFailed`) mapping `MPRemoteCommandHandlerStatus`.
- `SeekType` enum (`BeginSeeking`, `EndSeeking`) mapping `MPSeekCommandEventType`.
- `Artwork` wrapping `MPMediaItemArtwork` from a file path, with optional explicit `CGSize` bounds via `apple-cf`.
- `constants` module with now-playing info dictionary key strings.
- Swift bridge (`MediaPlayerBridge`) built with `swift build --triple` into `OUT_DIR`.
- Smoke example `examples/01_now_playing_smoke.rs`.
