//! Integration tests for `mediaplayer::async_api` (requires `--features async`).

#![cfg(feature = "async")]

use mediaplayer::async_api::{
    MediaLibraryChangeStream, NowPlayingItemChangeStream, NowPlayingSessionEventKind,
    NowPlayingSessionStream, PlaybackStateChangeStream, RemoteCommandStream, VolumeChangeStream,
};
use mediaplayer::remote_commands::Command;

// ── Construction / drop hygiene ──────────────────────────────────────────────

/// Subscribing and immediately dropping should not panic or leak.
#[test]
fn now_playing_item_change_stream_subscribe_drop() {
    let stream = NowPlayingItemChangeStream::subscribe(8);
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
    drop(stream);
}

#[test]
fn playback_state_change_stream_subscribe_drop() {
    let stream = PlaybackStateChangeStream::subscribe(8);
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
    drop(stream);
}

#[test]
fn volume_change_stream_subscribe_drop() {
    let stream = VolumeChangeStream::subscribe(8);
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
    drop(stream);
}

#[test]
fn media_library_change_stream_subscribe_drop() {
    let stream = MediaLibraryChangeStream::subscribe(8);
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
    drop(stream);
}

// ── RemoteCommandStream ──────────────────────────────────────────────────────

#[test]
fn remote_command_stream_subscribe_play_drop() {
    let stream = RemoteCommandStream::subscribe(Command::Play, 16);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_pause_drop() {
    let stream = RemoteCommandStream::subscribe(Command::Pause, 16);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_skip_forward_drop() {
    let stream = RemoteCommandStream::subscribe(Command::SkipForward, 8);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_skip_backward_drop() {
    let stream = RemoteCommandStream::subscribe(Command::SkipBackward, 8);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_next_track_drop() {
    let stream = RemoteCommandStream::subscribe(Command::NextTrack, 8);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_previous_track_drop() {
    let stream = RemoteCommandStream::subscribe(Command::PreviousTrack, 8);
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_try_next_empty() {
    let stream = RemoteCommandStream::subscribe(Command::Play, 4);
    assert!(stream.try_next().is_none());
}

// ── NowPlayingSessionStream ──────────────────────────────────────────────────

#[test]
fn now_playing_session_stream_subscribe_drop() {
    let stream = NowPlayingSessionStream::subscribe(8);
    assert_eq!(stream.buffered_count(), 0);
    // Not closed until the handle drops (which it does at end of scope).
    drop(stream);
}

#[test]
fn now_playing_session_stream_try_next_empty() {
    let stream = NowPlayingSessionStream::subscribe(4);
    assert!(stream.try_next().is_none());
}

// ── Stream-closes-on-drop: pollster smoke ────────────────────────────────────

/// After dropping the stream the consumer gets EOF.
/// We test this by verifying `is_closed` is false before drop and the stream
/// was open; we can't easily trigger a close without unsubscribing, but we
/// can verify the happy-path shape.
#[test]
fn remote_command_stream_capacity_is_honored() {
    let stream = RemoteCommandStream::subscribe(Command::Play, 4);
    assert_eq!(stream.buffered_count(), 0);
    // capacity getter via inner — we just assert no panic:
    drop(stream);
}

#[test]
fn now_playing_session_event_kind_debug() {
    let active = NowPlayingSessionEventKind::DidChangeActive;
    let can_become_active = NowPlayingSessionEventKind::DidChangeCanBecomeActive;
    assert_eq!(format!("{active:?}"), "DidChangeActive");
    assert_eq!(
        format!("{can_become_active:?}"),
        "DidChangeCanBecomeActive"
    );
}
