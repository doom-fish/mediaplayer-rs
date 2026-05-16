import Foundation
import MediaPlayer

public typealias MPCommandCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int32,
    Double,
    Double,
    Int32,
    Double,
    Double,
    Int32,
    Int32,
    Int32,
    Int32,
    Int32,
    UnsafeMutableRawPointer?,
    Int32
) -> Int32

final class MPCommandHandlerBox: NSObject {
    let command: MPRemoteCommand
    let handlerToken: Any

    init(command: MPRemoteCommand, handlerToken: Any) {
        self.command = command
        self.handlerToken = handlerToken
        super.init()
    }
}

private func mpRemoteCommand(for id: Int32) -> MPRemoteCommand? {
    let center = MPRemoteCommandCenter.shared()
    switch id {
    case 0: return center.playCommand
    case 1: return center.pauseCommand
    case 2: return center.stopCommand
    case 3: return center.togglePlayPauseCommand
    case 4: return center.nextTrackCommand
    case 5: return center.previousTrackCommand
    case 6: return center.skipForwardCommand
    case 7: return center.skipBackwardCommand
    case 8: return center.seekForwardCommand
    case 9: return center.seekBackwardCommand
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

private func mpJoinedNumbers(_ values: [NSNumber]) -> UnsafeMutablePointer<CChar>? {
    mpCString(values.map { String(describing: $0.doubleValue) }.joined(separator: "\n"))
}

@_cdecl("mp_remote_command_add_handler")
public func mp_remote_command_add_handler(
    _ commandId: Int32,
    _ callback: MPCommandCallback?,
    _ refcon: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let callback, let command = mpRemoteCommand(for: commandId) else { return nil }

    let token = command.addTarget { event in
        var extra = Double.nan
        var seekType: Int32 = -1
        var rating = Double.nan
        var playbackRate = Double.nan
        var negative: Int32 = -1
        var shuffleType: Int32 = -1
        var repeatType: Int32 = -1
        var preservesShuffleMode: Int32 = -1
        var preservesRepeatMode: Int32 = -1
        var languageOptionPtr: UnsafeMutableRawPointer?
        var languageOptionSetting: Int32 = -1

        if let skipEvent = event as? MPSkipIntervalCommandEvent {
            extra = skipEvent.interval
        }
        if let seekEvent = event as? MPSeekCommandEvent {
            seekType = Int32(seekEvent.type.rawValue)
        }
        if let positionEvent = event as? MPChangePlaybackPositionCommandEvent {
            extra = positionEvent.positionTime
        }
        if let ratingEvent = event as? MPRatingCommandEvent {
            rating = Double(ratingEvent.rating)
        }
        if let playbackRateEvent = event as? MPChangePlaybackRateCommandEvent {
            playbackRate = Double(playbackRateEvent.playbackRate)
        }
        if let feedbackEvent = event as? MPFeedbackCommandEvent {
            negative = feedbackEvent.isNegative ? 1 : 0
        }
        if let shuffleEvent = event as? MPChangeShuffleModeCommandEvent {
            shuffleType = Int32(shuffleEvent.shuffleType.rawValue)
            preservesShuffleMode = shuffleEvent.preservesShuffleMode ? 1 : 0
        }
        if let repeatEvent = event as? MPChangeRepeatModeCommandEvent {
            repeatType = Int32(repeatEvent.repeatType.rawValue)
            preservesRepeatMode = repeatEvent.preservesRepeatMode ? 1 : 0
        }
        if let languageEvent = event as? MPChangeLanguageOptionCommandEvent {
            languageOptionPtr = mpRetain(languageEvent.languageOption)
            languageOptionSetting = Int32(languageEvent.setting.rawValue)
        }

        let rawStatus = callback(
            refcon,
            commandId,
            event.timestamp,
            extra,
            seekType,
            rating,
            playbackRate,
            negative,
            shuffleType,
            repeatType,
            preservesShuffleMode,
            preservesRepeatMode,
            languageOptionPtr,
            languageOptionSetting
        )
        return MPRemoteCommandHandlerStatus(rawValue: Int(rawStatus)) ?? .commandFailed
    }

    return mpRetain(MPCommandHandlerBox(command: command, handlerToken: token))
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

@_cdecl("mp_remote_command_is_enabled")
public func mp_remote_command_is_enabled(_ commandId: Int32) -> Int32 {
    guard let command = mpRemoteCommand(for: commandId) else { return 0 }
    return command.isEnabled ? 1 : 0
}

@_cdecl("mp_remote_command_set_enabled")
public func mp_remote_command_set_enabled(_ commandId: Int32, _ enabled: Int32) {
    guard let command = mpRemoteCommand(for: commandId) else { return }
    command.isEnabled = enabled != 0
}

@_cdecl("mp_skip_command_copy_preferred_intervals")
public func mp_skip_command_copy_preferred_intervals(_ commandId: Int32) -> UnsafeMutablePointer<CChar>? {
    guard let command = mpRemoteCommand(for: commandId) as? MPSkipIntervalCommand else {
        return mpCString("")
    }
    return mpJoinedNumbers(command.preferredIntervals)
}

@_cdecl("mp_skip_command_set_preferred_intervals")
public func mp_skip_command_set_preferred_intervals(
    _ commandId: Int32,
    _ intervals: UnsafePointer<Double>?,
    _ count: Int
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPSkipIntervalCommand else { return }
    guard let intervals, count > 0 else {
        command.preferredIntervals = []
        return
    }

    let values = UnsafeBufferPointer(start: intervals, count: count).map { NSNumber(value: $0) }
    command.preferredIntervals = values
}

@_cdecl("mp_feedback_command_is_active")
public func mp_feedback_command_is_active(_ commandId: Int32) -> Int32 {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else { return 0 }
    return command.isActive ? 1 : 0
}

@_cdecl("mp_feedback_command_set_active")
public func mp_feedback_command_set_active(_ commandId: Int32, _ active: Int32) {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else { return }
    command.isActive = active != 0
}

@_cdecl("mp_feedback_command_copy_localized_title")
public func mp_feedback_command_copy_localized_title(_ commandId: Int32) -> UnsafeMutablePointer<CChar>? {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else {
        return mpCString("")
    }
    return mpCString(command.localizedTitle)
}

@_cdecl("mp_feedback_command_set_localized_title")
public func mp_feedback_command_set_localized_title(
    _ commandId: Int32,
    _ title: UnsafePointer<CChar>?
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else { return }
    command.localizedTitle = title.map(String.init(cString:)) ?? ""
}

@_cdecl("mp_feedback_command_copy_localized_short_title")
public func mp_feedback_command_copy_localized_short_title(
    _ commandId: Int32
) -> UnsafeMutablePointer<CChar>? {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else {
        return mpCString("")
    }
    return mpCString(command.localizedShortTitle)
}

@_cdecl("mp_feedback_command_set_localized_short_title")
public func mp_feedback_command_set_localized_short_title(
    _ commandId: Int32,
    _ title: UnsafePointer<CChar>?
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPFeedbackCommand else { return }
    command.localizedShortTitle = title.map(String.init(cString:)) ?? ""
}

@_cdecl("mp_rating_command_get_minimum_rating")
public func mp_rating_command_get_minimum_rating(_ commandId: Int32) -> Double {
    guard let command = mpRemoteCommand(for: commandId) as? MPRatingCommand else { return 0 }
    return Double(command.minimumRating)
}

@_cdecl("mp_rating_command_set_minimum_rating")
public func mp_rating_command_set_minimum_rating(_ commandId: Int32, _ rating: Double) {
    guard let command = mpRemoteCommand(for: commandId) as? MPRatingCommand else { return }
    command.minimumRating = Float(rating)
}

@_cdecl("mp_rating_command_get_maximum_rating")
public func mp_rating_command_get_maximum_rating(_ commandId: Int32) -> Double {
    guard let command = mpRemoteCommand(for: commandId) as? MPRatingCommand else { return 0 }
    return Double(command.maximumRating)
}

@_cdecl("mp_rating_command_set_maximum_rating")
public func mp_rating_command_set_maximum_rating(_ commandId: Int32, _ rating: Double) {
    guard let command = mpRemoteCommand(for: commandId) as? MPRatingCommand else { return }
    command.maximumRating = Float(rating)
}

@_cdecl("mp_change_playback_rate_copy_supported_rates")
public func mp_change_playback_rate_copy_supported_rates(
    _ commandId: Int32
) -> UnsafeMutablePointer<CChar>? {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangePlaybackRateCommand else {
        return mpCString("")
    }
    return mpJoinedNumbers(command.supportedPlaybackRates)
}

@_cdecl("mp_change_playback_rate_set_supported_rates")
public func mp_change_playback_rate_set_supported_rates(
    _ commandId: Int32,
    _ rates: UnsafePointer<Double>?,
    _ count: Int
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangePlaybackRateCommand else { return }
    guard let rates, count > 0 else {
        command.supportedPlaybackRates = []
        return
    }

    let values = UnsafeBufferPointer(start: rates, count: count).map { NSNumber(value: Float($0)) }
    command.supportedPlaybackRates = values
}

@_cdecl("mp_change_shuffle_mode_get_current_shuffle_type")
public func mp_change_shuffle_mode_get_current_shuffle_type(_ commandId: Int32) -> Int32 {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangeShuffleModeCommand else {
        return 0
    }
    return Int32(command.currentShuffleType.rawValue)
}

@_cdecl("mp_change_shuffle_mode_set_current_shuffle_type")
public func mp_change_shuffle_mode_set_current_shuffle_type(
    _ commandId: Int32,
    _ shuffleType: Int32
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangeShuffleModeCommand,
          let shuffle = MPShuffleType(rawValue: Int(shuffleType))
    else {
        return
    }
    command.currentShuffleType = shuffle
}

@_cdecl("mp_change_repeat_mode_get_current_repeat_type")
public func mp_change_repeat_mode_get_current_repeat_type(_ commandId: Int32) -> Int32 {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangeRepeatModeCommand else {
        return 0
    }
    return Int32(command.currentRepeatType.rawValue)
}

@_cdecl("mp_change_repeat_mode_set_current_repeat_type")
public func mp_change_repeat_mode_set_current_repeat_type(
    _ commandId: Int32,
    _ repeatType: Int32
) {
    guard let command = mpRemoteCommand(for: commandId) as? MPChangeRepeatModeCommand,
          let repeatMode = MPRepeatType(rawValue: Int(repeatType))
    else {
        return
    }
    command.currentRepeatType = repeatMode
}
