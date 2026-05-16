import MediaPlayer
import Foundation

// MARK: - mp_now_playing_set_info
//
// Builds the nowPlayingInfo dictionary from individual C parameters and
// pushes it to MPNowPlayingInfoCenter.default().
//
// Convention for absent optional values:
//   * String fields   — pass NULL
//   * Double fields   — pass a negative value (< 0) to omit
//   * Int32 fields    — pass -1 to omit
//   * artwork pointer — pass NULL

@_cdecl("mp_now_playing_set_info")
public func mp_now_playing_set_info(
    _ title: UnsafePointer<CChar>?,
    _ artist: UnsafePointer<CChar>?,
    _ albumTitle: UnsafePointer<CChar>?,
    _ playbackDuration: Double,
    _ elapsedPlaybackTime: Double,
    _ playbackRate: Double,
    _ mediaType: Int32,
    _ contentId: UnsafePointer<CChar>?,
    _ assetUrl: UnsafePointer<CChar>?,
    _ artworkPtr: UnsafeMutableRawPointer?
) {
    var info: [String: Any] = [:]

    if let t = title      { info[MPMediaItemPropertyTitle]     = String(cString: t) }
    if let a = artist     { info[MPMediaItemPropertyArtist]    = String(cString: a) }
    if let al = albumTitle { info[MPMediaItemPropertyAlbumTitle] = String(cString: al) }

    if playbackDuration >= 0 {
        info[MPMediaItemPropertyPlaybackDuration] = playbackDuration
    }
    if elapsedPlaybackTime >= 0 {
        info[MPNowPlayingInfoPropertyElapsedPlaybackTime] = elapsedPlaybackTime
    }
    if playbackRate >= 0 {
        info[MPNowPlayingInfoPropertyPlaybackRate] = playbackRate
    }
    if mediaType >= 0 {
        info[MPNowPlayingInfoPropertyMediaType] = UInt(mediaType)
    }
    if let cid = contentId {
        info[MPNowPlayingInfoPropertyExternalContentIdentifier] = String(cString: cid)
    }
    if let rawUrl = assetUrl {
        let urlStr = String(cString: rawUrl)
        if let url = URL(string: urlStr) {
            info[MPNowPlayingInfoPropertyAssetURL] = url
        }
    }
    if let artPtr = artworkPtr {
        let artwork: MPMediaItemArtwork = mpBorrow(artPtr)
        info[MPMediaItemPropertyArtwork] = artwork
    }

    MPNowPlayingInfoCenter.default().nowPlayingInfo = info
}

// MARK: - Clear

@_cdecl("mp_now_playing_clear")
public func mp_now_playing_clear() {
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nil
}

// MARK: - Playback state

@_cdecl("mp_now_playing_set_playback_state")
public func mp_now_playing_set_playback_state(_ state: Int32) {
    if #available(macOS 10.12.2, *) {
        let s: MPNowPlayingPlaybackState
        switch state {
        case 1:  s = .playing
        case 2:  s = .paused
        case 3:  s = .stopped
        case 4:  s = .interrupted
        default: s = .unknown
        }
        MPNowPlayingInfoCenter.default().playbackState = s
    }
}

@_cdecl("mp_now_playing_get_playback_state")
public func mp_now_playing_get_playback_state() -> Int32 {
    if #available(macOS 10.12.2, *) {
        return Int32(MPNowPlayingInfoCenter.default().playbackState.rawValue)
    }
    return 0
}
