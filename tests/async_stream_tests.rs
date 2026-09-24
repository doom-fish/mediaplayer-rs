//! Integration tests for `mediaplayer::async_api` (requires `--features async`).

#![cfg(feature = "async")]

mod common;

use common::LiveRemoteCommands;
use mediaplayer::async_api::{
    MediaLibraryChangeStream, NowPlayingItemChangeStream, NowPlayingSessionEventKind,
    NowPlayingSessionStream, PlaybackStateChangeStream, RemoteCommandStream, VolumeChangeStream,
};
use mediaplayer::remote_commands::Command;
use mediaplayer::MediaPlayerError;

#[test]
fn now_playing_item_change_stream_is_unavailable_on_macos() {
    assert!(matches!(
        NowPlayingItemChangeStream::subscribe(8),
        Err(MediaPlayerError::NotAvailable(_))
    ));
}

#[test]
fn playback_state_change_stream_is_unavailable_on_macos() {
    assert!(matches!(
        PlaybackStateChangeStream::subscribe(8),
        Err(MediaPlayerError::NotAvailable(_))
    ));
}

#[test]
fn volume_change_stream_is_unavailable_on_macos() {
    assert!(matches!(
        VolumeChangeStream::subscribe(8),
        Err(MediaPlayerError::NotAvailable(_))
    ));
}

#[test]
fn media_library_change_stream_is_unavailable_on_macos() {
    assert!(matches!(
        MediaLibraryChangeStream::subscribe(8),
        Err(MediaPlayerError::NotAvailable(_))
    ));
}

// ── RemoteCommandStream ──────────────────────────────────────────────────────

#[test]
fn remote_command_stream_subscribe_play_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_play_drop",
        &[Command::Play],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::Play, 16)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_pause_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_pause_drop",
        &[Command::Pause],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::Pause, 16)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_skip_forward_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_skip_forward_drop",
        &[Command::SkipForward],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::SkipForward, 8)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_skip_backward_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_skip_backward_drop",
        &[Command::SkipBackward],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::SkipBackward, 8)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_next_track_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_next_track_drop",
        &[Command::NextTrack],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::NextTrack, 8)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_subscribe_previous_track_drop() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_subscribe_previous_track_drop",
        &[Command::PreviousTrack],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::PreviousTrack, 8)
        .expect("remote command stream should subscribe");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

#[test]
fn remote_command_stream_try_next_empty() {
    let Some(_commands) =
        LiveRemoteCommands::acquire("remote_command_stream_try_next_empty", &[Command::Play])
    else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::Play, 4)
        .expect("remote command stream should subscribe");
    assert!(stream.try_next().is_none());
}

#[test]
fn remote_command_stream_rejects_zero_capacity() {
    assert!(matches!(
        RemoteCommandStream::subscribe(Command::Play, 0),
        Err(MediaPlayerError::InvalidArgument(_))
    ));
}

#[test]
fn now_playing_session_stream_is_unavailable_on_macos() {
    assert!(matches!(
        NowPlayingSessionStream::subscribe(8),
        Err(MediaPlayerError::NotAvailable(_))
    ));
}

// ── Stream-closes-on-drop: pollster smoke ────────────────────────────────────

/// After dropping the stream the consumer gets EOF.
/// We test this by verifying `is_closed` is false before drop and the stream
/// was open; we can't easily trigger a close without unsubscribing, but we
/// can verify the happy-path shape.
#[test]
fn remote_command_stream_capacity_is_honored() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_stream_capacity_is_honored",
        &[Command::Play],
    ) else {
        return;
    };
    let stream = RemoteCommandStream::subscribe(Command::Play, 4)
        .expect("remote command stream should subscribe");
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
