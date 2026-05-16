import AVFoundation
import Foundation
import MediaPlayer

private enum MPRustMediaSelectionKind: Int32 {
    case audible = 0
    case legible = 1
    case other = 2
}

private final class MPRustMediaSelectionOption: AVMediaSelectionOption {
    private let rustKind: MPRustMediaSelectionKind
    private let rustExtendedLanguageTag: String?
    private let rustLocale: Locale?
    private let rustDisplayName: String
    private let rustPropertyList: NSString
    private let rustCharacteristics: Set<AVMediaCharacteristic>

    override var mediaType: AVMediaType {
        switch rustKind {
        case .audible:
            return .audio
        case .legible:
            return .subtitle
        case .other:
            return .video
        }
    }

    override var mediaSubTypes: [NSNumber] { [] }

    override func hasMediaCharacteristic(_ mediaCharacteristic: AVMediaCharacteristic) -> Bool {
        rustCharacteristics.contains(mediaCharacteristic)
    }

    override var isPlayable: Bool { true }

    override var extendedLanguageTag: String? { rustExtendedLanguageTag }

    override var locale: Locale? { rustLocale }

    override var commonMetadata: [AVMetadataItem] { [] }

    override var availableMetadataFormats: [String] { [] }

    override func metadata(forFormat _: String) -> [AVMetadataItem] { [] }

    override func associatedMediaSelectionOption(in _: AVMediaSelectionGroup) -> AVMediaSelectionOption? {
        nil
    }

    override func propertyList() -> Any { rustPropertyList }

    override func displayName(with _: Locale) -> String { rustDisplayName }

    override var displayName: String { rustDisplayName }

    init(
        kind: MPRustMediaSelectionKind,
        extendedLanguageTag: String?,
        displayName: String,
        identifier: String
    ) {
        self.rustKind = kind
        self.rustExtendedLanguageTag = extendedLanguageTag
        self.rustLocale = extendedLanguageTag.map(Locale.init(identifier:))
        self.rustDisplayName = displayName
        self.rustPropertyList = identifier as NSString

        var characteristics: Set<AVMediaCharacteristic> = []
        switch kind {
        case .audible:
            characteristics.insert(.audible)
        case .legible:
            characteristics.insert(.legible)
        case .other:
            break
        }
        self.rustCharacteristics = characteristics

        super.init()
    }

    required init?(coder _: NSCoder) {
        nil
    }

    override func copy(with _: NSZone? = nil) -> Any {
        self
    }
}

private final class MPRustMediaSelectionGroup: AVMediaSelectionGroup {
    private let rustOptions: [AVMediaSelectionOption]
    private let rustDefaultOption: AVMediaSelectionOption?
    private let rustAllowsEmptySelection: Bool

    override var options: [AVMediaSelectionOption] { rustOptions }

    override var defaultOption: AVMediaSelectionOption? { rustDefaultOption }

    override var allowsEmptySelection: Bool { rustAllowsEmptySelection }

    override func mediaSelectionOption(withPropertyList plist: Any) -> AVMediaSelectionOption? {
        let propertyList = plist as AnyObject
        return rustOptions.first { (($0.propertyList as AnyObject).isEqual(propertyList)) }
    }

    init(options: [AVMediaSelectionOption], defaultIndex: Int32, allowEmptySelection: Bool) {
        self.rustOptions = options
        if defaultIndex >= 0, Int(defaultIndex) < options.count {
            self.rustDefaultOption = options[Int(defaultIndex)]
        } else {
            self.rustDefaultOption = nil
        }
        self.rustAllowsEmptySelection = allowEmptySelection
        super.init()
    }

    required init?(coder _: NSCoder) {
        nil
    }

    override func copy(with _: NSZone? = nil) -> Any {
        self
    }
}

private func mpMediaSelectionOptions(
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    count: Int
) -> [AVMediaSelectionOption] {
    guard let values, count > 0 else { return [] }
    return UnsafeBufferPointer(start: values, count: count).compactMap { value in
        value.map { mpBorrow($0, as: AVMediaSelectionOption.self) }
    }
}

@_cdecl("mp_language_option_new_from_media_selection_option")
public func mp_language_option_new_from_media_selection_option(
    _ option: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let option else { return nil }
    let mediaSelectionOption: AVMediaSelectionOption = mpBorrow(option)
    guard let languageOption = mediaSelectionOption.makeNowPlayingInfoLanguageOption() else {
        return nil
    }
    return mpRetain(languageOption)
}

@_cdecl("mp_language_option_group_new_from_media_selection_group")
public func mp_language_option_group_new_from_media_selection_group(
    _ group: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let group else { return nil }
    let mediaSelectionGroup: AVMediaSelectionGroup = mpBorrow(group)
    return mpRetain(mediaSelectionGroup.makeNowPlayingInfoLanguageOptionGroup())
}

@_cdecl("mp_test_media_selection_option_new")
public func mp_test_media_selection_option_new(
    _ kind: Int32,
    _ languageTag: UnsafePointer<CChar>?,
    _ displayName: UnsafePointer<CChar>?,
    _ identifier: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let kind = MPRustMediaSelectionKind(rawValue: kind), let displayName, let identifier else {
        return nil
    }

    let option = MPRustMediaSelectionOption(
        kind: kind,
        extendedLanguageTag: languageTag.map { String(cString: $0) },
        displayName: String(cString: displayName),
        identifier: String(cString: identifier)
    )
    return mpRetain(option)
}

@_cdecl("mp_test_media_selection_option_release")
public func mp_test_media_selection_option_release(_ option: UnsafeMutableRawPointer?) {
    guard let option else { return }
    mpRelease(option)
}

@_cdecl("mp_test_media_selection_group_new")
public func mp_test_media_selection_group_new(
    _ options: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ defaultIndex: Int32,
    _ allowEmptySelection: Int32
) -> UnsafeMutableRawPointer? {
    let mediaSelectionOptions = mpMediaSelectionOptions(options, count: count)
    return mpRetain(
        MPRustMediaSelectionGroup(
            options: mediaSelectionOptions,
            defaultIndex: defaultIndex,
            allowEmptySelection: allowEmptySelection != 0
        )
    )
}

@_cdecl("mp_test_media_selection_group_release")
public func mp_test_media_selection_group_release(_ group: UnsafeMutableRawPointer?) {
    guard let group else { return }
    mpRelease(group)
}
