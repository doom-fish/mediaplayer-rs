import Foundation
import MediaPlayer
#if canImport(AVFoundation)
import AVFoundation
#endif

// ─────────────────────────────────────────────────────────────────────────────
// Shared callback type used by every async-stream bridge function.
// kind  : i32 — event discriminant
// payload: opaque pointer to event data (bridge-specific; may be nil)
// ctx   : opaque Rust AsyncStreamSender<T> pointer (never nil)
// ─────────────────────────────────────────────────────────────────────────────
public typealias MPStreamCallback = @convention(c) (
    Int32,
    UnsafeRawPointer?,
    UnsafeMutableRawPointer
) -> Void

// ─────────────────────────────────────────────────────────────────────────────
// MARK: - Notification streams
//
// Notification kind discriminants (must match NotificationKind in Rust):
//   0 = MPMusicPlayerControllerNowPlayingItemDidChange
//   1 = MPMusicPlayerControllerPlaybackStateDidChange
//   2 = MPMusicPlayerControllerVolumeDidChange
//   3 = MPMediaLibraryDidChange
// ─────────────────────────────────────────────────────────────────────────────

private func notificationName(for kind: Int32) -> Notification.Name? {
    switch kind {
    case 0:
        return Notification.Name("MPMusicPlayerControllerNowPlayingItemDidChangeNotification")
    case 1:
        return Notification.Name("MPMusicPlayerControllerPlaybackStateDidChangeNotification")
    case 2:
        return Notification.Name("MPMusicPlayerControllerVolumeDidChangeNotification")
    case 3:
        return Notification.Name("MPMediaLibraryDidChangeNotification")
    default:
        return nil
    }
}

/// Holds an NSNotificationCenter token and forwards each notification to Rust.
final class MPNotificationStreamBridge: NSObject {
    let callback: MPStreamCallback
    let ctx: UnsafeMutableRawPointer
    let kind: Int32
    var observer: NSObjectProtocol?

    init(kind: Int32, callback: MPStreamCallback, ctx: UnsafeMutableRawPointer) {
        self.kind = kind
        self.callback = callback
        self.ctx = ctx
        super.init()
    }

    deinit {
        if let observer = observer {
            NotificationCenter.default.removeObserver(observer)
        }
    }
}

/// Registers an NSNotificationCenter observer and streams each notification
/// to Rust via `callback(kind, nil, ctx)`.
///
/// Returns an opaque handle (retained `MPNotificationStreamBridge`), or nil
/// if `kind` is unknown. Pass the handle to `mp_notification_unsubscribe`
/// to cancel.
@_cdecl("mp_notification_subscribe")
public func mp_notification_subscribe(
    _ kind: Int32,
    _ callback: MPStreamCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let name = notificationName(for: kind) else { return nil }
    let bridge = MPNotificationStreamBridge(kind: kind, callback: callback, ctx: ctx)
    bridge.observer = NotificationCenter.default.addObserver(
        forName: name,
        object: nil,
        queue: .main
    ) { [weak bridge] _ in
        guard let bridge = bridge else { return }
        bridge.callback(bridge.kind, nil, bridge.ctx)
    }
    return mpRetain(bridge)
}

/// Cancels the notification subscription represented by `handle` and releases
/// the bridge object.
@_cdecl("mp_notification_unsubscribe")
public func mp_notification_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    // The bridge's deinit removes the observer automatically.
    mpRelease(handle)
}

// ─────────────────────────────────────────────────────────────────────────────
// MARK: - Remote command streams
//
// Wraps MPRemoteCommandCenter targets via the existing
// mp_remote_command_add_handler / mp_remote_command_remove_handler FFI but
// with an async-stream–oriented callback that auto-returns `.success`.
//
// Command-event payload is encoded as a stack-allocated struct and passed via
// UnsafeRawPointer so Rust can read it without heap allocation.
// ─────────────────────────────────────────────────────────────────────────────

/// Plain-old-data view of a remote command event passed across the FFI boundary.
/// All fields are always present; unused ones are set to NaN / -1 / 0.
private struct MPStreamCommandPayload {
    var commandId: Int32
    var timestamp: Double
    var extra: Double        // skip interval OR playback position
    var seekType: Int32      // -1 = not a seek event
    var rating: Double
    var playbackRate: Double
    var negative: Int32      // -1 = not a feedback event
    var shuffleType: Int32
    var repeatType: Int32
    var preservesShuffleMode: Int32
    var preservesRepeatMode: Int32
    var languageOptionSetting: Int32
    // Note: LanguageOption pointer is intentionally omitted to keep payload
    // POD-safe for stack passing; callers that need it can use the
    // full-fat CommandToken API instead.
}

/// Holds one remote-command handler token and forwards events to Rust.
final class MPRemoteCommandStreamBridge: NSObject {
    let command: MPRemoteCommand
    let callback: MPStreamCallback
    let ctx: UnsafeMutableRawPointer
    let commandId: Int32
    var handlerToken: Any?

    init(command: MPRemoteCommand, commandId: Int32,
         callback: MPStreamCallback, ctx: UnsafeMutableRawPointer) {
        self.command = command
        self.commandId = commandId
        self.callback = callback
        self.ctx = ctx
        super.init()
    }

    deinit {
        if let token = handlerToken {
            command.removeTarget(token)
        }
    }
}

private func mpRemoteCommandForStream(_ id: Int32) -> MPRemoteCommand? {
    let center = MPRemoteCommandCenter.shared()
    switch id {
    case 0:  return center.playCommand
    case 1:  return center.pauseCommand
    case 2:  return center.stopCommand
    case 3:  return center.togglePlayPauseCommand
    case 4:  return center.nextTrackCommand
    case 5:  return center.previousTrackCommand
    case 6:  return center.skipForwardCommand
    case 7:  return center.skipBackwardCommand
    case 8:  return center.seekForwardCommand
    case 9:  return center.seekBackwardCommand
    case 10: return center.changePlaybackPositionCommand
    case 11: return center.enableLanguageOptionCommand
    case 12: return center.disableLanguageOptionCommand
    case 13: return center.changePlaybackRateCommand
    case 14: return center.changeRepeatModeCommand
    case 15: return center.changeShuffleModeCommand
    case 16: return center.ratingCommand
    case 17: return center.likeCommand
    case 18: return center.dislikeCommand
    case 19: return center.bookmarkCommand
    default: return nil
    }
}

/// Subscribes to a single remote command and streams events to Rust.
///
/// Each event is forwarded via `callback(0, &payload, ctx)` and the handler
/// automatically returns `.success` to the system.
///
/// Returns an opaque handle or nil if `commandId` is unknown.
@_cdecl("mp_stream_remote_command_subscribe")
public func mp_stream_remote_command_subscribe(
    _ commandId: Int32,
    _ callback: MPStreamCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let command = mpRemoteCommandForStream(commandId) else { return nil }
    let bridge = MPRemoteCommandStreamBridge(
        command: command, commandId: commandId, callback: callback, ctx: ctx)
    bridge.handlerToken = command.addTarget { event in
        var payload = MPStreamCommandPayload(
            commandId: commandId,
            timestamp: event.timestamp,
            extra: Double.nan,
            seekType: -1,
            rating: Double.nan,
            playbackRate: Double.nan,
            negative: -1,
            shuffleType: -1,
            repeatType: -1,
            preservesShuffleMode: -1,
            preservesRepeatMode: -1,
            languageOptionSetting: -1
        )
        if let e = event as? MPSkipIntervalCommandEvent   { payload.extra = e.interval }
        if let e = event as? MPChangePlaybackPositionCommandEvent { payload.extra = e.positionTime }
        if let e = event as? MPSeekCommandEvent           { payload.seekType = Int32(e.type.rawValue) }
        if let e = event as? MPRatingCommandEvent         { payload.rating = Double(e.rating) }
        if let e = event as? MPChangePlaybackRateCommandEvent { payload.playbackRate = Double(e.playbackRate) }
        if let e = event as? MPFeedbackCommandEvent       { payload.negative = e.isNegative ? 1 : 0 }
        if let e = event as? MPChangeShuffleModeCommandEvent {
            payload.shuffleType = Int32(e.shuffleType.rawValue)
            payload.preservesShuffleMode = e.preservesShuffleMode ? 1 : 0
        }
        if let e = event as? MPChangeRepeatModeCommandEvent {
            payload.repeatType = Int32(e.repeatType.rawValue)
            payload.preservesRepeatMode = e.preservesRepeatMode ? 1 : 0
        }
        if let e = event as? MPChangeLanguageOptionCommandEvent {
            payload.languageOptionSetting = Int32(e.setting.rawValue)
        }
        withUnsafeBytes(of: &payload) { raw in
            // kind = 0 for all remote-command events (type encoded in payload.commandId)
            callback(0, raw.baseAddress.map { UnsafeRawPointer($0) }, ctx)
        }
        return .success
    }
    return mpRetain(bridge)
}

/// Cancels the remote-command subscription represented by `handle`.
@_cdecl("mp_stream_remote_command_unsubscribe")
public func mp_stream_remote_command_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let bridge: MPRemoteCommandStreamBridge = mpBorrow(handle)
    if let token = bridge.handlerToken {
        bridge.command.removeTarget(token)
    }
    mpRelease(handle)
}

// ─────────────────────────────────────────────────────────────────────────────
// MARK: - MPNowPlayingSession delegate stream  (macOS only)
//
// Event kind discriminants (must match NowPlayingSessionEventKind in Rust):
//   0 = ActiveMediaPlaybackTargetChanged
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// MARK: - MPNowPlayingSession delegate stream
//
// MPNowPlayingSession is only available on iOS 16.0+ and tvOS 14.0+; it is
// explicitly marked unavailable on macOS.  The subscribe function always
// returns nil so the Rust side creates an immediately-closed stream.
// ─────────────────────────────────────────────────────────────────────────────

/// Stub — always returns nil; MPNowPlayingSession is unavailable on macOS.
@_cdecl("mp_now_playing_session_stream_subscribe")
public func mp_now_playing_session_stream_subscribe(
    _ callback: MPStreamCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    return nil
}

/// No-op teardown (subscribe always returns nil so no handle exists).
@_cdecl("mp_now_playing_session_stream_unsubscribe")
public func mp_now_playing_session_stream_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    mpRelease(handle)
}
