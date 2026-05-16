import Foundation

@_cdecl("mp_system_music_player_is_supported")
public func mp_system_music_player_is_supported() -> Int32 {
    0
}

@_cdecl("mp_system_music_player_copy_unavailable_reason")
public func mp_system_music_player_copy_unavailable_reason() -> UnsafeMutablePointer<CChar>? {
    mpCString("MPSystemMusicPlayer is unavailable on macOS; Apple marks this MediaPlayer API iOS/tvOS-only.")
}
