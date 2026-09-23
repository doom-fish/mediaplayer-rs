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
        if #available(macOS 26.0, *) {
            return MPNowPlayingInfoProperty1x1AnimatedArtwork
        }
        return nil
    case .animatedArtwork3x4:
        if #available(macOS 26.0, *) {
            return MPNowPlayingInfoProperty3x4AnimatedArtwork
        }
        return nil
    }
}

private func mpNamedNowPlayingKeys() -> [(name: String, key: String)] {
    var keys: [(name: String, key: String)] = [
        ("MPMediaEntityPropertyPersistentID", MPMediaEntityPropertyPersistentID),
        ("MPMediaItemPropertyPersistentID", MPMediaItemPropertyPersistentID),
        ("MPMediaItemPropertyMediaType", MPMediaItemPropertyMediaType),
        ("MPMediaItemPropertyTitle", MPMediaItemPropertyTitle),
        ("MPMediaItemPropertyArtist", MPMediaItemPropertyArtist),
        ("MPMediaItemPropertyAlbumTitle", MPMediaItemPropertyAlbumTitle),
        ("MPMediaItemPropertyAlbumPersistentID", MPMediaItemPropertyAlbumPersistentID),
        ("MPMediaItemPropertyArtistPersistentID", MPMediaItemPropertyArtistPersistentID),
        ("MPMediaItemPropertyAlbumArtist", MPMediaItemPropertyAlbumArtist),
        ("MPMediaItemPropertyAlbumArtistPersistentID", MPMediaItemPropertyAlbumArtistPersistentID),
        ("MPMediaItemPropertyGenre", MPMediaItemPropertyGenre),
        ("MPMediaItemPropertyGenrePersistentID", MPMediaItemPropertyGenrePersistentID),
        ("MPMediaItemPropertyComposer", MPMediaItemPropertyComposer),
        ("MPMediaItemPropertyComposerPersistentID", MPMediaItemPropertyComposerPersistentID),
        ("MPMediaItemPropertyPlaybackDuration", MPMediaItemPropertyPlaybackDuration),
        ("MPMediaItemPropertyAlbumTrackNumber", MPMediaItemPropertyAlbumTrackNumber),
        ("MPMediaItemPropertyAlbumTrackCount", MPMediaItemPropertyAlbumTrackCount),
        ("MPMediaItemPropertyDiscNumber", MPMediaItemPropertyDiscNumber),
        ("MPMediaItemPropertyDiscCount", MPMediaItemPropertyDiscCount),
        ("MPMediaItemPropertyArtwork", MPMediaItemPropertyArtwork),
        ("MPMediaItemPropertyIsExplicit", MPMediaItemPropertyIsExplicit),
        ("MPMediaItemPropertyLyrics", MPMediaItemPropertyLyrics),
        ("MPMediaItemPropertyIsCompilation", MPMediaItemPropertyIsCompilation),
        ("MPMediaItemPropertyReleaseDate", MPMediaItemPropertyReleaseDate),
        ("MPMediaItemPropertyBeatsPerMinute", MPMediaItemPropertyBeatsPerMinute),
        ("MPMediaItemPropertyComments", MPMediaItemPropertyComments),
        ("MPMediaItemPropertyAssetURL", MPMediaItemPropertyAssetURL),
        ("MPMediaItemPropertyIsCloudItem", MPMediaItemPropertyIsCloudItem),
        ("MPMediaItemPropertyHasProtectedAsset", MPMediaItemPropertyHasProtectedAsset),
        ("MPMediaItemPropertyPodcastTitle", MPMediaItemPropertyPodcastTitle),
        ("MPMediaItemPropertyPodcastPersistentID", MPMediaItemPropertyPodcastPersistentID),
        ("MPMediaItemPropertyPlayCount", MPMediaItemPropertyPlayCount),
        ("MPMediaItemPropertySkipCount", MPMediaItemPropertySkipCount),
        ("MPMediaItemPropertyRating", MPMediaItemPropertyRating),
        ("MPMediaItemPropertyLastPlayedDate", MPMediaItemPropertyLastPlayedDate),
        ("MPMediaItemPropertyUserGrouping", MPMediaItemPropertyUserGrouping),
        ("MPMediaItemPropertyBookmarkTime", MPMediaItemPropertyBookmarkTime),
        ("MPMediaItemPropertyDateAdded", MPMediaItemPropertyDateAdded),
        ("MPMediaItemPropertyPlaybackStoreID", MPMediaItemPropertyPlaybackStoreID),
        ("MPMediaItemPropertyIsPreorder", MPMediaItemPropertyIsPreorder),
        ("MPNowPlayingInfoPropertyElapsedPlaybackTime", MPNowPlayingInfoPropertyElapsedPlaybackTime),
        ("MPNowPlayingInfoPropertyPlaybackRate", MPNowPlayingInfoPropertyPlaybackRate),
        ("MPNowPlayingInfoPropertyDefaultPlaybackRate", MPNowPlayingInfoPropertyDefaultPlaybackRate),
        ("MPNowPlayingInfoPropertyPlaybackQueueIndex", MPNowPlayingInfoPropertyPlaybackQueueIndex),
        ("MPNowPlayingInfoPropertyPlaybackQueueCount", MPNowPlayingInfoPropertyPlaybackQueueCount),
        ("MPNowPlayingInfoPropertyChapterNumber", MPNowPlayingInfoPropertyChapterNumber),
        ("MPNowPlayingInfoPropertyChapterCount", MPNowPlayingInfoPropertyChapterCount),
        ("MPNowPlayingInfoPropertyIsLiveStream", MPNowPlayingInfoPropertyIsLiveStream),
        ("MPNowPlayingInfoPropertyAvailableLanguageOptions", MPNowPlayingInfoPropertyAvailableLanguageOptions),
        ("MPNowPlayingInfoPropertyCurrentLanguageOptions", MPNowPlayingInfoPropertyCurrentLanguageOptions),
        ("MPNowPlayingInfoCollectionIdentifier", MPNowPlayingInfoCollectionIdentifier),
        ("MPNowPlayingInfoPropertyExternalContentIdentifier", MPNowPlayingInfoPropertyExternalContentIdentifier),
        ("MPNowPlayingInfoPropertyExternalUserProfileIdentifier", MPNowPlayingInfoPropertyExternalUserProfileIdentifier),
        ("MPNowPlayingInfoPropertyServiceIdentifier", MPNowPlayingInfoPropertyServiceIdentifier),
        ("MPNowPlayingInfoPropertyPlaybackProgress", MPNowPlayingInfoPropertyPlaybackProgress),
        ("MPNowPlayingInfoPropertyMediaType", MPNowPlayingInfoPropertyMediaType),
        ("MPNowPlayingInfoPropertyAssetURL", MPNowPlayingInfoPropertyAssetURL),
        ("MPNowPlayingInfoPropertyCurrentPlaybackDate", MPNowPlayingInfoPropertyCurrentPlaybackDate),
    ]
    if #available(macOS 13.0, *) {
        keys.append(("MPNowPlayingInfoPropertyCreditsStartTime", MPNowPlayingInfoPropertyCreditsStartTime))
    }
    if #available(macOS 15.0, *) {
        keys.append(("MPNowPlayingInfoPropertyInternationalStandardRecordingCode", MPNowPlayingInfoPropertyInternationalStandardRecordingCode))
        keys.append(("MPNowPlayingInfoPropertyExcludeFromSuggestions", MPNowPlayingInfoPropertyExcludeFromSuggestions))
    }
    if #available(macOS 26.0, *) {
        keys.append(("MPNowPlayingInfoProperty1x1AnimatedArtwork", MPNowPlayingInfoProperty1x1AnimatedArtwork))
        keys.append(("MPNowPlayingInfoProperty3x4AnimatedArtwork", MPNowPlayingInfoProperty3x4AnimatedArtwork))
    }
    return keys
}

private func mpObjectValuedNowPlayingKeys() -> Set<String> {
    var keys: Set<String> = [
        MPMediaItemPropertyArtwork,
        MPNowPlayingInfoPropertyAvailableLanguageOptions,
        MPNowPlayingInfoPropertyCurrentLanguageOptions,
    ]
    if #available(macOS 26.0, *) {
        keys.insert(MPNowPlayingInfoProperty1x1AnimatedArtwork)
        keys.insert(MPNowPlayingInfoProperty3x4AnimatedArtwork)
    }
    return keys
}

private func mpResolveNowPlayingKey(_ name: String) -> String? {
    let keys = mpNamedNowPlayingKeys()
    if let entry = keys.first(where: { $0.name == name }) {
        return entry.key
    }
    return keys.first(where: { $0.key == name })?.key
}

private func mpNowPlayingKeyName(_ key: String) -> String {
    if key == MPMediaItemPropertyPersistentID {
        return "MPMediaItemPropertyPersistentID"
    }
    return mpNamedNowPlayingKeys().first(where: { $0.key == key })?.name ?? key
}

private let mpLanguageOptionCharacteristics: [String: String] = [
    "MPLanguageOptionCharacteristicIsMainProgramContent": MPLanguageOptionCharacteristicIsMainProgramContent,
    "MPLanguageOptionCharacteristicIsAuxiliaryContent": MPLanguageOptionCharacteristicIsAuxiliaryContent,
    "MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles": MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles,
    "MPLanguageOptionCharacteristicTranscribesSpokenDialog": MPLanguageOptionCharacteristicTranscribesSpokenDialog,
    "MPLanguageOptionCharacteristicDescribesMusicAndSound": MPLanguageOptionCharacteristicDescribesMusicAndSound,
    "MPLanguageOptionCharacteristicEasyToRead": MPLanguageOptionCharacteristicEasyToRead,
    "MPLanguageOptionCharacteristicDescribesVideo": MPLanguageOptionCharacteristicDescribesVideo,
    "MPLanguageOptionCharacteristicLanguageTranslation": MPLanguageOptionCharacteristicLanguageTranslation,
    "MPLanguageOptionCharacteristicDubbedTranslation": MPLanguageOptionCharacteristicDubbedTranslation,
    "MPLanguageOptionCharacteristicVoiceOverTranslation": MPLanguageOptionCharacteristicVoiceOverTranslation,
]

let MP_NOW_PLAYING_OK: Int32 = 0
let MP_NOW_PLAYING_UNKNOWN_KEY: Int32 = 1
let MP_NOW_PLAYING_OBJECT_KEY: Int32 = 2
let MP_NOW_PLAYING_INVALID_VALUE: Int32 = 3

@_cdecl("mp_now_playing_info_box_set_named")
public func mp_now_playing_info_box_set_named(
    _ info: UnsafeMutableRawPointer?,
    _ keyName: UnsafePointer<CChar>?,
    _ kind: Int32,
    _ stringValue: UnsafePointer<CChar>?,
    _ doubleValue: Double,
    _ int64Value: Int64,
    _ uint64Value: UInt64
) -> Int32 {
    guard let info, let keyName else { return MP_NOW_PLAYING_INVALID_VALUE }
    guard let key = mpResolveNowPlayingKey(String(cString: keyName)) else {
        return MP_NOW_PLAYING_UNKNOWN_KEY
    }
    guard !mpObjectValuedNowPlayingKeys().contains(key) else { return MP_NOW_PLAYING_OBJECT_KEY }
    let value: Any
    switch kind {
    case 0:
        guard let stringValue else { return MP_NOW_PLAYING_INVALID_VALUE }
        value = String(cString: stringValue)
    case 1:
        value = NSNumber(value: doubleValue)
    case 2:
        value = NSNumber(value: int64Value)
    case 3:
        value = NSNumber(value: uint64Value)
    case 4:
        value = NSNumber(value: int64Value != 0)
    case 5:
        value = Date(timeIntervalSince1970: doubleValue)
    case 6:
        guard let stringValue, let url = URL(string: String(cString: stringValue)) else {
            return MP_NOW_PLAYING_INVALID_VALUE
        }
        value = url
    default:
        return MP_NOW_PLAYING_INVALID_VALUE
    }
    let box: MPNowPlayingInfoBox = mpBorrow(info)
    box.info[key] = value
    return MP_NOW_PLAYING_OK
}

final class MPNowPlayingSnapshot: NSObject {
    let entries: [(name: String, value: Any)]

    init(info: [String: Any]) {
        entries = info
            .map { (name: mpNowPlayingKeyName($0.key), value: $0.value) }
            .sorted { $0.name < $1.name }
    }

    func entry(_ index: Int) -> Any? {
        guard index >= 0, index < entries.count else { return nil }
        return entries[index].value
    }
}

private func mpSnapshotKind(_ value: Any) -> Int32 {
    if value is String {
        return 0
    }
    if let number = value as? NSNumber {
        if CFGetTypeID(number) == CFBooleanGetTypeID() {
            return 4
        }
        if CFNumberIsFloatType(number) {
            return 1
        }
        return String(cString: number.objCType) == "Q" ? 3 : 2
    }
    if value is Date {
        return 5
    }
    if value is URL {
        return 6
    }
    return 7
}

@_cdecl("mp_now_playing_info_snapshot")
public func mp_now_playing_info_snapshot() -> UnsafeMutableRawPointer? {
    guard let info = MPNowPlayingInfoCenter.default().nowPlayingInfo else { return nil }
    return mpRetain(MPNowPlayingSnapshot(info: info))
}

@_cdecl("mp_now_playing_snapshot_release")
public func mp_now_playing_snapshot_release(_ snapshot: UnsafeMutableRawPointer?) {
    guard let snapshot else { return }
    mpRelease(snapshot)
}

@_cdecl("mp_now_playing_snapshot_count")
public func mp_now_playing_snapshot_count(_ snapshot: UnsafeMutableRawPointer?) -> Int {
    guard let snapshot else { return 0 }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    return box.entries.count
}

@_cdecl("mp_now_playing_snapshot_copy_key")
public func mp_now_playing_snapshot_copy_key(
    _ snapshot: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot else { return nil }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    guard index >= 0, index < box.entries.count else { return nil }
    return mpCString(box.entries[index].name)
}

@_cdecl("mp_now_playing_snapshot_kind")
public func mp_now_playing_snapshot_kind(_ snapshot: UnsafeMutableRawPointer?, _ index: Int) -> Int32 {
    guard let snapshot else { return -1 }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    guard let value = box.entry(index) else { return -1 }
    return mpSnapshotKind(value)
}

@_cdecl("mp_now_playing_snapshot_copy_string")
public func mp_now_playing_snapshot_copy_string(
    _ snapshot: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot else { return nil }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    guard let value = box.entry(index) else { return nil }
    if let string = value as? String {
        return mpCString(string)
    }
    if let url = value as? URL {
        return mpCString(url.absoluteString)
    }
    return mpCString(String(describing: type(of: value)))
}

@_cdecl("mp_now_playing_snapshot_double")
public func mp_now_playing_snapshot_double(_ snapshot: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let snapshot else { return .nan }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    switch box.entry(index) {
    case let number as NSNumber:
        return number.doubleValue
    case let date as Date:
        return date.timeIntervalSince1970
    default:
        return .nan
    }
}

@_cdecl("mp_now_playing_snapshot_int64")
public func mp_now_playing_snapshot_int64(_ snapshot: UnsafeMutableRawPointer?, _ index: Int) -> Int64 {
    guard let snapshot else { return 0 }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    return (box.entry(index) as? NSNumber)?.int64Value ?? 0
}

@_cdecl("mp_now_playing_snapshot_uint64")
public func mp_now_playing_snapshot_uint64(_ snapshot: UnsafeMutableRawPointer?, _ index: Int) -> UInt64 {
    guard let snapshot else { return 0 }
    let box: MPNowPlayingSnapshot = mpBorrow(snapshot)
    return (box.entry(index) as? NSNumber)?.uint64Value ?? 0
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
    guard #available(macOS 26.0, *), let info, let artworkPtr, let key = mpNowPlayingDictionaryKey(keyId) else {
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
    return Int32(clamping: MPNowPlayingInfoCenter.default().playbackState.rawValue)
}

@_cdecl("mp_now_playing_copy_supported_animated_artwork_keys")
public func mp_now_playing_copy_supported_animated_artwork_keys() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 26.0, *) {
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
    guard let rawOptionType = UInt(exactly: optionType),
          let optionType = MPNowPlayingInfoLanguageOptionType(rawValue: rawOptionType),
          let displayName,
          let identifier
    else {
        return nil
    }

    let option = MPNowPlayingInfoLanguageOption(
        type: optionType,
        languageTag: languageTag.map { String(cString: $0) } ?? "",
        characteristics: mpCStringArray(characteristics, count: characteristicsCount).map {
            mpLanguageOptionCharacteristics[$0] ?? $0
        },
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
    return Int32(clamping: languageOption.languageOptionType.rawValue)
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
        return Int32(clamping: index)
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
