import Foundation

@_cdecl("mp_volume_view_is_supported")
public func mp_volume_view_is_supported() -> Int32 {
    0
}

@_cdecl("mp_volume_view_copy_unavailable_reason")
public func mp_volume_view_copy_unavailable_reason() -> UnsafeMutablePointer<CChar>? {
    mpCString("MPVolumeView is unavailable on macOS; Apple marks this MediaPlayer API iOS/tvOS-only.")
}
