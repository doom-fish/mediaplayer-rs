import Foundation
import MediaPlayer

private enum MPNowPlayingKey: Int32 {
    case title = 0
    case artist = 1
    case albumTitle = 2
    case playbackDuration = 3
    case elapsedPlaybackTime = 4
    case playbackRate = 5
    case defaultPlaybackRate = 6
    case playbackQueueIndex = 7
    case playbackQueueCount = 8
    case chapterNumber = 9
    case chapterCount = 10
    case isLiveStream = 11
    case collectionIdentifier = 12
    case externalContentIdentifier = 13
    case externalUserProfileIdentifier = 14
    case serviceIdentifier = 15
    case playbackProgress = 16
    case mediaType = 17
    case assetURL = 18
    case currentPlaybackDate = 19
    case creditsStartTime = 20
    case internationalStandardRecordingCode = 21
    case excludeFromSuggestions = 22
    case animatedArtwork1x1 = 23
    case animatedArtwork3x4 = 24
}

final class MPNowPlayingInfoBox: NSObject {
    var info: [String: Any] = [:]
}

private func mpNowPlayingDictionaryKey(_ rawValue: Int32) -> String? {
    guard let key = MPNowPlayingKey(rawValue: rawValue) else { return nil }

    switch key {
    case .title:
        return MPMediaItemPropertyTitle
    case .artist:
        return MPMediaItemPropertyArtist
    case .albumTitle:
        return MPMediaItemPropertyAlbumTitle
    case .playbackDuration:
        return MPMediaItemPropertyPlaybackDuration
    case .elapsedPlaybackTime:
        return MPNowPlayingInfoPropertyElapsedPlaybackTime
    case .playbackRate:
        return MPNowPlayingInfoPropertyPlaybackRate
    case .defaultPlaybackRate:
        return MPNowPlayingInfoPropertyDefaultPlaybackRate
    case .playbackQueueIndex:
        return MPNowPlayingInfoPropertyPlaybackQueueIndex
    case .playbackQueueCount:
        return MPNowPlayingInfoPropertyPlaybackQueueCount
    case .chapterNumber:
        return MPNowPlayingInfoPropertyChapterNumber
    case .chapterCount:
        return MPNowPlayingInfoPropertyChapterCount
    case .isLiveStream:
        return MPNowPlayingInfoPropertyIsLiveStream
    case .collectionIdentifier:
        return MPNowPlayingInfoCollectionIdentifier
    case .externalContentIdentifier:
        return MPNowPlayingInfoPropertyExternalContentIdentifier
    case .externalUserProfileIdentifier:
        return MPNowPlayingInfoPropertyExternalUserProfileIdentifier
    case .serviceIdentifier:
        if #available(macOS 10.13, *) {
            return MPNowPlayingInfoPropertyServiceIdentifier
        }
        return nil
    case .playbackProgress:
        return MPNowPlayingInfoPropertyPlaybackProgress
    case .mediaType:
        return MPNowPlayingInfoPropertyMediaType
    case .assetURL:
        return MPNowPlayingInfoPropertyAssetURL
    case .currentPlaybackDate:
        if #available(macOS 10.13.1, *) {
            return MPNowPlayingInfoPropertyCurrentPlaybackDate
        }
        return nil
    case .creditsStartTime:
        if #available(macOS 13.0, *) {
            return MPNowPlayingInfoPropertyCreditsStartTime
        }
        return nil
    case .internationalStandardRecordingCode:
        if #available(macOS 15.0, *) {
            return MPNowPlayingInfoPropertyInternationalStandardRecordingCode
        }
        return nil
    case .excludeFromSuggestions:
        if #available(macOS 15.0, *) {
            return MPNowPlayingInfoPropertyExcludeFromSuggestions
        }
        return nil
    case .animatedArtwork1x1:
        if #available(macOS 16.0, *) {
            return MPNowPlayingInfoProperty1x1AnimatedArtwork
        }
        return nil
    case .animatedArtwork3x4:
        if #available(macOS 16.0, *) {
            return MPNowPlayingInfoProperty3x4AnimatedArtwork
        }
        return nil
    }
}

private func mpCStringArray(
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    count: Int
) -> [String] {
    guard let values, count > 0 else { return [] }
    return UnsafeBufferPointer(start: values, count: count).compactMap { value in
        value.map { String(cString: $0) }
    }
}

private func mpObjectArray<T: AnyObject>(
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    count: Int,
    as _: T.Type = T.self
) -> [T] {
    guard let values, count > 0 else { return [] }
    return UnsafeBufferPointer(start: values, count: count).compactMap { value in
        value.map { mpBorrow($0, as: T.self) }
    }
}

@_cdecl("mp_now_playing_info_box_new")
public func mp_now_playing_info_box_new() -> UnsafeMutableRawPointer? {
    mpRetain(MPNowPlayingInfoBox())
}

@_cdecl("mp_now_playing_info_box_release")
public func mp_now_playing_info_box_release(_ info: UnsafeMutableRawPointer?) {
    guard let info else { return }
    mpRelease(info)
}

@_cdecl("mp_now_playing_info_box_set_string")
public func mp_now_playing_info_box_set_string(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: UnsafePointer<CChar>?
) {
    guard let info, let value, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = String(cString: value)
}

@_cdecl("mp_now_playing_info_box_set_double")
public func mp_now_playing_info_box_set_double(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: Double
) {
    guard let info, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = value
}

@_cdecl("mp_now_playing_info_box_set_u64")
public func mp_now_playing_info_box_set_u64(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: UInt64
) {
    guard let info, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = NSNumber(value: value)
}

@_cdecl("mp_now_playing_info_box_set_bool")
public func mp_now_playing_info_box_set_bool(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: Int32
) {
    guard let info, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = value != 0
}

@_cdecl("mp_now_playing_info_box_set_url")
public func mp_now_playing_info_box_set_url(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: UnsafePointer<CChar>?
) {
    guard let info, let value, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    if let url = URL(string: String(cString: value)) {
        box.info[key] = url
    }
}

@_cdecl("mp_now_playing_info_box_set_date_seconds")
public func mp_now_playing_info_box_set_date_seconds(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ value: Double
) {
    guard let info, let key = mpNowPlayingDictionaryKey(keyId) else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = Date(timeIntervalSince1970: value)
}

@_cdecl("mp_now_playing_info_box_set_artwork")
public func mp_now_playing_info_box_set_artwork(
    _ info: UnsafeMutableRawPointer?,
    _ artworkPtr: UnsafeMutableRawPointer?
) {
    guard let info, let artworkPtr else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    let artwork: MPMediaItemArtwork = mpBorrow(artworkPtr)
    box.info[MPMediaItemPropertyArtwork] = artwork
}

@_cdecl("mp_now_playing_info_box_set_animated_artwork")
public func mp_now_playing_info_box_set_animated_artwork(
    _ info: UnsafeMutableRawPointer?,
    _ keyId: Int32,
    _ artworkPtr: UnsafeMutableRawPointer?
) {
    guard #available(macOS 16.0, *), let info, let artworkPtr, let key = mpNowPlayingDictionaryKey(keyId) else {
        return
    }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    let artwork: MPMediaItemAnimatedArtwork = mpBorrow(artworkPtr)
    box.info[key] = artwork
}

@_cdecl("mp_now_playing_info_box_set_available_language_option_groups")
public func mp_now_playing_info_box_set_available_language_option_groups(
    _ info: UnsafeMutableRawPointer?,
    _ groups: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    guard let info else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    let values: [MPNowPlayingInfoLanguageOptionGroup] = mpObjectArray(groups, count: count)
    if !values.isEmpty {
        box.info[MPNowPlayingInfoPropertyAvailableLanguageOptions] = values
    }
}

@_cdecl("mp_now_playing_info_box_set_current_language_options")
public func mp_now_playing_info_box_set_current_language_options(
    _ info: UnsafeMutableRawPointer?,
    _ options: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    guard let info else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    let values: [MPNowPlayingInfoLanguageOption] = mpObjectArray(options, count: count)
    if !values.isEmpty {
        box.info[MPNowPlayingInfoPropertyCurrentLanguageOptions] = values
    }
}

@_cdecl("mp_now_playing_apply_info_box")
public func mp_now_playing_apply_info_box(_ info: UnsafeMutableRawPointer?) {
    guard let info else { return }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    MPNowPlayingInfoCenter.default().nowPlayingInfo = box.info
}

@_cdecl("mp_now_playing_clear")
public func mp_now_playing_clear() {
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nil
}

@_cdecl("mp_now_playing_set_playback_state")
public func mp_now_playing_set_playback_state(_ state: Int32) {
    guard #available(macOS 10.12.2, *) else { return }

    let playbackState: MPNowPlayingPlaybackState
    switch state {
    case 1:
        playbackState = .playing
    case 2:
        playbackState = .paused
    case 3:
        playbackState = .stopped
    case 4:
        playbackState = .interrupted
    default:
        playbackState = .unknown
    }

    MPNowPlayingInfoCenter.default().playbackState = playbackState
}

@_cdecl("mp_now_playing_get_playback_state")
public func mp_now_playing_get_playback_state() -> Int32 {
    guard #available(macOS 10.12.2, *) else { return 0 }
    return Int32(MPNowPlayingInfoCenter.default().playbackState.rawValue)
}

@_cdecl("mp_now_playing_copy_supported_animated_artwork_keys")
public func mp_now_playing_copy_supported_animated_artwork_keys() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 16.0, *) {
        return mpCString(MPNowPlayingInfoCenter.supportedAnimatedArtworkKeys.joined(separator: "\n"))
    }
    return mpCString("")
}

@_cdecl("mp_language_option_new")
public func mp_language_option_new(
    _ optionType: Int32,
    _ languageTag: UnsafePointer<CChar>?,
    _ characteristics: UnsafePointer<UnsafePointer<CChar>?>?,
    _ characteristicsCount: Int,
    _ displayName: UnsafePointer<CChar>?,
    _ identifier: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let optionType = MPNowPlayingInfoLanguageOptionType(rawValue: UInt(optionType)),
          let displayName,
          let identifier
    else {
        return nil
    }

    let option = MPNowPlayingInfoLanguageOption(
        type: optionType,
        languageTag: languageTag.map { String(cString: $0) } ?? "",
        characteristics: mpCStringArray(characteristics, count: characteristicsCount),
        displayName: String(cString: displayName),
        identifier: String(cString: identifier)
    )
    return mpRetain(option)
}

@_cdecl("mp_language_option_release")
public func mp_language_option_release(_ option: UnsafeMutableRawPointer?) {
    guard let option else { return }
    mpRelease(option)
}

@_cdecl("mp_language_option_get_type")
public func mp_language_option_get_type(_ option: UnsafeMutableRawPointer?) -> Int32 {
    guard let option else { return -1 }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    return Int32(languageOption.languageOptionType.rawValue)
}

@_cdecl("mp_language_option_copy_language_tag")
public func mp_language_option_copy_language_tag(
    _ option: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let option else { return nil }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    guard let tag = languageOption.languageTag else { return nil }
    return mpCString(tag)
}

@_cdecl("mp_language_option_copy_characteristics")
public func mp_language_option_copy_characteristics(
    _ option: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let option else { return nil }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    return mpCString((languageOption.languageOptionCharacteristics ?? []).joined(separator: "\n"))
}

@_cdecl("mp_language_option_copy_display_name")
public func mp_language_option_copy_display_name(
    _ option: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let option else { return nil }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    if let displayName = languageOption.displayName {
        return mpCString(displayName)
    }
    return nil
}

@_cdecl("mp_language_option_copy_identifier")
public func mp_language_option_copy_identifier(
    _ option: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let option else { return nil }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    if let identifier = languageOption.identifier {
        return mpCString(identifier)
    }
    return nil
}

@_cdecl("mp_language_option_is_automatic_legible")
public func mp_language_option_is_automatic_legible(_ option: UnsafeMutableRawPointer?) -> Int32 {
    guard let option else { return 0 }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    return languageOption.isAutomaticLegibleLanguageOption() ? 1 : 0
}

@_cdecl("mp_language_option_is_automatic_audible")
public func mp_language_option_is_automatic_audible(_ option: UnsafeMutableRawPointer?) -> Int32 {
    guard let option else { return 0 }
    let languageOption: MPNowPlayingInfoLanguageOption = mpBorrow(option)
    return languageOption.isAutomaticAudibleLanguageOption() ? 1 : 0
}

@_cdecl("mp_language_option_group_new")
public func mp_language_option_group_new(
    _ options: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ defaultIndex: Int32,
    _ allowEmptySelection: Int32
) -> UnsafeMutableRawPointer? {
    let languageOptions: [MPNowPlayingInfoLanguageOption] = mpObjectArray(options, count: count)
    guard !languageOptions.isEmpty else { return nil }

    let defaultOption: MPNowPlayingInfoLanguageOption?
    if defaultIndex >= 0, Int(defaultIndex) < languageOptions.count {
        defaultOption = languageOptions[Int(defaultIndex)]
    } else {
        defaultOption = nil
    }

    let group = MPNowPlayingInfoLanguageOptionGroup(
        languageOptions: languageOptions,
        defaultLanguageOption: defaultOption,
        allowEmptySelection: allowEmptySelection != 0
    )
    return mpRetain(group)
}

@_cdecl("mp_language_option_group_release")
public func mp_language_option_group_release(_ group: UnsafeMutableRawPointer?) {
    guard let group else { return }
    mpRelease(group)
}

@_cdecl("mp_language_option_group_get_count")
public func mp_language_option_group_get_count(_ group: UnsafeMutableRawPointer?) -> Int {
    guard let group else { return 0 }
    let languageOptionGroup: MPNowPlayingInfoLanguageOptionGroup = mpBorrow(group)
    return languageOptionGroup.languageOptions.count
}

@_cdecl("mp_language_option_group_get_default_index")
public func mp_language_option_group_get_default_index(_ group: UnsafeMutableRawPointer?) -> Int32 {
    guard let group else { return -1 }
    let languageOptionGroup: MPNowPlayingInfoLanguageOptionGroup = mpBorrow(group)
    guard let defaultOption = languageOptionGroup.defaultLanguageOption else { return -1 }

    if let index = languageOptionGroup.languageOptions.firstIndex(where: { option in
        option === defaultOption || option.identifier == defaultOption.identifier
    }) {
        return Int32(index)
    }

    return -1
}

@_cdecl("mp_language_option_group_allows_empty_selection")
public func mp_language_option_group_allows_empty_selection(
    _ group: UnsafeMutableRawPointer?
) -> Int32 {
    guard let group else { return 0 }
    let languageOptionGroup: MPNowPlayingInfoLanguageOptionGroup = mpBorrow(group)
    return languageOptionGroup.allowEmptySelection ? 1 : 0
}
