# Changelog

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
