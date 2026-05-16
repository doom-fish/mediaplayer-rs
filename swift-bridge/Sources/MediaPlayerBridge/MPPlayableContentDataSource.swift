import Foundation

@_cdecl("mp_playable_content_data_source_is_supported")
public func mp_playable_content_data_source_is_supported() -> Int32 {
    0
}

@_cdecl("mp_playable_content_data_source_copy_unavailable_reason")
public func mp_playable_content_data_source_copy_unavailable_reason() -> UnsafeMutablePointer<CChar>? {
    mpCString("MPPlayableContentDataSource is unavailable on macOS; Apple marks this MediaPlayer API iOS/tvOS-only.")
}
