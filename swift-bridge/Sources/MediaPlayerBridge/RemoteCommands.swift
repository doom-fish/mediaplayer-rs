import MediaPlayer
import Foundation

// MARK: - Callback type
//
// The Rust side registers a C function pointer + opaque refcon.
// Swift calls it when a remote command event fires.
// Return value: MPRemoteCommandHandlerStatus raw value (0=success, 200=failed, …).

public typealias MPCommandCallback = @convention(c) (
    UnsafeMutableRawPointer?,   // refcon
    Int32,                       // command_id (mirrors Rust Command repr)
    Double,                      // event timestamp
    Double,                      // extra: skip interval (commands 6/7) or position (command 10)
    Int32                        // seek_type raw value (commands 8/9; 0=begin, 1=end)
) -> Int32

// MARK: - Token box
//
// Holds the MPRemoteCommand and the opaque handler token returned by
// addTarget(handler:) so we can call removeTarget(_:) later.

final class MPCommandHandlerBox: NSObject {
    let command: MPRemoteCommand
    let handlerToken: Any

    init(command: MPRemoteCommand, handlerToken: Any) {
        self.command = command
        self.handlerToken = handlerToken
        super.init()
    }
}

// MARK: - Command lookup

private func mpRemoteCommand(for id: Int32) -> MPRemoteCommand? {
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
    default: return nil
    }
}

// MARK: - Bridge functions

@_cdecl("mp_remote_command_add_handler")
public func mp_remote_command_add_handler(
    _ commandId: Int32,
    _ callback: MPCommandCallback?,
    _ refcon: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let callback, let command = mpRemoteCommand(for: commandId) else { return nil }

    let token = command.addTarget { event in
        let ts = event.timestamp
        var extra: Double = 0.0
        var seekType: Int32 = 0

        if let skip = event as? MPSkipIntervalCommandEvent {
            extra = skip.interval
        } else if let seek = event as? MPSeekCommandEvent {
            seekType = Int32(seek.type.rawValue)
        } else if let pos = event as? MPChangePlaybackPositionCommandEvent {
            extra = pos.positionTime
        }

        let rawStatus = callback(refcon, commandId, ts, extra, seekType)
        return MPRemoteCommandHandlerStatus(rawValue: Int(rawStatus)) ?? .commandFailed
    }

    let box = MPCommandHandlerBox(command: command, handlerToken: token)
    return mpRetain(box)
}

@_cdecl("mp_remote_command_remove_handler")
public func mp_remote_command_remove_handler(_ tokenPtr: UnsafeMutableRawPointer?) {
    guard let tokenPtr else { return }
    let box: MPCommandHandlerBox = mpBorrow(tokenPtr)
    box.command.removeTarget(box.handlerToken)
}

@_cdecl("mp_command_token_release")
public func mp_command_token_release(_ tokenPtr: UnsafeMutableRawPointer?) {
    guard let tokenPtr else { return }
    mpRelease(tokenPtr)
}
