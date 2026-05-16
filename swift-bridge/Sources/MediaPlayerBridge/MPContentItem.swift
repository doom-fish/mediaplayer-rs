import Foundation
import MediaPlayer

@_cdecl("mp_content_item_new")
public func mp_content_item_new(_ identifier: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.12.2, *), let identifier else { return nil }
    return mpRetain(MPContentItem(identifier: String(cString: identifier)))
}

@_cdecl("mp_content_item_release")
public func mp_content_item_release(_ item: UnsafeMutableRawPointer?) {
    guard let item else { return }
    mpRelease(item)
}

@_cdecl("mp_content_item_copy_identifier")
public func mp_content_item_copy_identifier(_ item: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let item else { return nil }
    let contentItem: MPContentItem = mpBorrow(item)
    return mpCString(contentItem.identifier)
}

@_cdecl("mp_content_item_copy_title")
public func mp_content_item_copy_title(_ item: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let item else { return nil }
    let contentItem: MPContentItem = mpBorrow(item)
    guard let title = contentItem.title else { return nil }
    return mpCString(title)
}

@_cdecl("mp_content_item_set_title")
public func mp_content_item_set_title(_ item: UnsafeMutableRawPointer?, _ title: UnsafePointer<CChar>?) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.title = title.map { String(cString: $0) }
}

@_cdecl("mp_content_item_copy_subtitle")
public func mp_content_item_copy_subtitle(_ item: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let item else { return nil }
    let contentItem: MPContentItem = mpBorrow(item)
    guard let subtitle = contentItem.subtitle else { return nil }
    return mpCString(subtitle)
}

@_cdecl("mp_content_item_set_subtitle")
public func mp_content_item_set_subtitle(_ item: UnsafeMutableRawPointer?, _ subtitle: UnsafePointer<CChar>?) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.subtitle = subtitle.map { String(cString: $0) }
}

@_cdecl("mp_content_item_copy_artwork")
public func mp_content_item_copy_artwork(_ item: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let item else { return nil }
    let contentItem: MPContentItem = mpBorrow(item)
    guard let artwork = contentItem.artwork else { return nil }
    return mpRetain(artwork)
}

@_cdecl("mp_content_item_set_artwork")
public func mp_content_item_set_artwork(_ item: UnsafeMutableRawPointer?, _ artwork: UnsafeMutableRawPointer?) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.artwork = artwork.map { mpBorrow($0, as: MPMediaItemArtwork.self) }
}

@_cdecl("mp_content_item_get_playback_progress")
public func mp_content_item_get_playback_progress(_ item: UnsafeMutableRawPointer?) -> Float {
    guard let item else { return 0 }
    let contentItem: MPContentItem = mpBorrow(item)
    return contentItem.playbackProgress
}

@_cdecl("mp_content_item_set_playback_progress")
public func mp_content_item_set_playback_progress(_ item: UnsafeMutableRawPointer?, _ playbackProgress: Float) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.playbackProgress = playbackProgress
}

@_cdecl("mp_content_item_is_streaming_content")
public func mp_content_item_is_streaming_content(_ item: UnsafeMutableRawPointer?) -> Int32 {
    guard let item else { return 0 }
    let contentItem: MPContentItem = mpBorrow(item)
    return contentItem.isStreamingContent ? 1 : 0
}

@_cdecl("mp_content_item_set_streaming_content")
public func mp_content_item_set_streaming_content(_ item: UnsafeMutableRawPointer?, _ streamingContent: Int32) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.isStreamingContent = streamingContent != 0
}

@_cdecl("mp_content_item_is_explicit_content")
public func mp_content_item_is_explicit_content(_ item: UnsafeMutableRawPointer?) -> Int32 {
    guard let item else { return 0 }
    let contentItem: MPContentItem = mpBorrow(item)
    return contentItem.isExplicitContent ? 1 : 0
}

@_cdecl("mp_content_item_set_explicit_content")
public func mp_content_item_set_explicit_content(_ item: UnsafeMutableRawPointer?, _ explicitContent: Int32) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.isExplicitContent = explicitContent != 0
}

@_cdecl("mp_content_item_is_container")
public func mp_content_item_is_container(_ item: UnsafeMutableRawPointer?) -> Int32 {
    guard let item else { return 0 }
    let contentItem: MPContentItem = mpBorrow(item)
    return contentItem.isContainer ? 1 : 0
}

@_cdecl("mp_content_item_set_container")
public func mp_content_item_set_container(_ item: UnsafeMutableRawPointer?, _ container: Int32) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.isContainer = container != 0
}

@_cdecl("mp_content_item_is_playable")
public func mp_content_item_is_playable(_ item: UnsafeMutableRawPointer?) -> Int32 {
    guard let item else { return 0 }
    let contentItem: MPContentItem = mpBorrow(item)
    return contentItem.isPlayable ? 1 : 0
}

@_cdecl("mp_content_item_set_playable")
public func mp_content_item_set_playable(_ item: UnsafeMutableRawPointer?, _ playable: Int32) {
    guard let item else { return }
    let contentItem: MPContentItem = mpBorrow(item)
    contentItem.isPlayable = playable != 0
}
