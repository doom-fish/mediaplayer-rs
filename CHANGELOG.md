# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Security

- Remote-command handler blocks now own their Rust context through `CallbackContext`. Dropping a `CommandToken` (on any thread) removes the target, and the closure is freed only when MediaPlayer releases the block, so a delivery that is already running can no longer use a freed closure. Deliveries to one handler are serialized, and a re-entrant delivery gets `HandlerStatus::CommandFailed`.
- `RemoteCommandStream` is built on the same registration. Its old Swift bridge captured the raw context pointer in the handler block.

### Fixed

- A failed handler registration returns `Err(MediaPlayerError::Framework)` instead of a token that does nothing, and the Swift side releases the context it was handed.
- The language option passed to command handlers is no longer retained without a matching release (it leaked when no context was registered).
- Now-playing strings containing NUL bytes return `InvalidArgument` instead of silently becoming empty strings.
- Animated-artwork availability checks use `#available(macOS 26.0, *)` instead of `macOS 16.0`.
- `LanguageOption::new` translates the `constants::LANGUAGE_OPTION_CHARACTERISTIC_*` names to the SDK's characteristic values.
- The artwork request handler aspect-fits the image to the requested size instead of always returning the full image.
- Swift conversions of framework enum values and indices clamp instead of trapping on unexpected values.

### Changed

- **Breaking:** `add_handler` and the `on_*` helpers return `Result<CommandToken, MediaPlayerError>`.
- **Breaking:** `NowPlayingInfoCenter::set_now_playing_info` and `set_now_playing_info_with_artwork` return `Result<(), MediaPlayerError>`. A rejected update leaves the current now-playing info untouched.
- **Breaking:** `RemoteCommandStream::subscribe` returns `Result` and rejects a capacity of 0. `NowPlayingItemChangeStream`, `PlaybackStateChangeStream`, `VolumeChangeStream`, `MediaLibraryChangeStream` and `NowPlayingSessionStream::subscribe` return `Err(MediaPlayerError::NotAvailable)`: Apple marks those notifications and the session delegate unavailable on macOS, so the streams could never deliver an event.
- **Breaking:** `NowPlayingInfo` has a new public `values` field.
- **Breaking:** the typed `NowPlayingInfo` builders for version-gated keys (`credits_start_time` before macOS 13, `international_standard_recording_code` and `exclude_from_suggestions` before macOS 15, the animated artwork before macOS 26) make `set_now_playing_info` return `Err(MediaPlayerError::NotAvailable)` instead of silently leaving the key out. `NowPlayingInfo::value` returns `NotAvailable` for those keys too, where it returned `InvalidArgument`.
- **Breaking:** an `asset_url` that isn't a valid URL makes `set_now_playing_info` return `InvalidArgument` instead of being silently left out.
- **Breaking:** the raw `ffi::mp_now_playing_info_box_set_*` setters for typed keys return a status code.
- **Breaking:** the raw `ffi::mp_remote_command_add_handler` takes a context release callback.
- **Breaking:** requires apple-cf 0.11, so `Artwork::bounds` returns the nested `CGRect { origin, size }`, and doom-fish-utils 0.4.1.
- `rust-version` is 1.82.

### Added

- `NowPlayingInfo::value(key, NowPlayingValue)` sets any media-item or now-playing key in `constants` (composer, genre, album artist, track and disc numbers, persistent IDs, ...), by symbol name or raw key value.
- `NowPlayingInfoCenter::now_playing_info` reads `nowPlayingInfo` back as a `BTreeMap<String, NowPlayingValue>`.
- `Artwork::from_image_data` creates artwork from in-memory image data, and `Artwork::image_png_data` returns the image the artwork produces for a size.

### Removed

- The notification and session-delegate Swift bridges behind the unavailable streams and the old stream bridge, together with `ffi::mp_notification_*`, `ffi::mp_stream_remote_command_*`, `ffi::mp_now_playing_session_stream_*`, `ffi::StreamEventCallback`, `ffi::ContextRefCallback`, the pinned command payload layout and `ffi::mp_verify_ffi_layout`.

## [0.4.3] - 2026-06-06

- Guarded the remote-command handler against panics crossing the FFI boundary, reference-counted the async stream context, pinned the command payload ABI, and dropped a placeholder header.

## [0.4.2] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.4.1] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.4.0] - 2026-05-19

### Added

- Completed the `MPNowPlayingSessionDelegate` event enum in `async_api` with explicit `DidChangeActive` and `DidChangeCanBecomeActive` cases for cross-platform callers.
- Documented the deprecated `MPMovie*` / `MPTimedMetadata` area and the iOS-only `MPMediaPickerControllerDelegate` surface as explicit audit exemptions.

## [0.3.3] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.3.1] - 2026-05-28

### Fixed

- **Sender-pointer leak in all async stream subscriptions** (`subscribe_notification`,
  `RemoteCommandStream::subscribe`, `NowPlayingSessionStream::subscribe`): the
  `Box<AsyncStreamSender<T>>` was leaked via `Box::into_raw` into the Swift bridge
  `ctx` pointer but never reconstituted. `SubscriptionHandle` now stores the sender
  pointer and a type-erased `free_sender` destructor; `Drop` calls `free_sender` after
  calling `unsubscribe` (removing the observer/target first so no callback can fire
  against the freed pointer). This closes the `BoundedAsyncStream` correctly and
  prevents `is_closed()` from permanently returning `false` after drop.
- **Defensive `deinit` on `MPRemoteCommandStreamBridge`** (Swift): added a `deinit`
  that calls `command.removeTarget(handlerToken)` so the handler is removed even if
  the bridge is released without going through `mp_stream_remote_command_unsubscribe`.
- **`SAFETY:` comments** added to every `unsafe {}` block in `src/async_api.rs`
  (callbacks, FFI call sites, `Box::from_raw` uses, and `SubscriptionHandle::drop`).
- **`doom-fish-utils` version constraint** widened from `"0.1"` to `">=0.1, <0.3"` per
  workspace versioning policy.

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
