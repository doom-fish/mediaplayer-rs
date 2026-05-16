import Foundation

@_cdecl("mp_media_query_is_supported")
public func mp_media_query_is_supported() -> Int32 {
    0
}

@_cdecl("mp_media_query_copy_unavailable_reason")
public func mp_media_query_copy_unavailable_reason() -> UnsafeMutablePointer<CChar>? {
    mpCString("MPMediaQuery is unavailable on macOS; Apple marks this MediaPlayer API iOS/tvOS-only.")
}
