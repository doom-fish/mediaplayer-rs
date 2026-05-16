import MediaPlayer
import AppKit

// MARK: - MPMediaItemArtwork bridge
//
// Creates an MPMediaItemArtwork wrapping a loaded NSImage from a file path.
// The boundsSize is set to the image's natural size; the request handler
// always returns the same NSImage regardless of the requested CGSize.
//
// Availability: MPMediaItemArtwork(boundsSize:requestHandler:) requires macOS 10.12.2+.

@_cdecl("mp_artwork_new_from_path")
public func mp_artwork_new_from_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    let filePath = String(cString: path)
    guard let image = NSImage(contentsOfFile: filePath) else { return nil }
    if #available(macOS 10.12.2, *) {
        let size = image.size
        let artwork = MPMediaItemArtwork(boundsSize: size) { _ in image }
        return mpRetain(artwork)
    }
    return nil
}

// Variant that accepts an explicit bounds size (width, height as doubles)
// matching apple_cf::cg::CGSize layout.
@_cdecl("mp_artwork_new_from_path_with_size")
public func mp_artwork_new_from_path_with_size(
    _ path: UnsafePointer<CChar>?,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    let filePath = String(cString: path)
    guard let image = NSImage(contentsOfFile: filePath) else { return nil }
    if #available(macOS 10.12.2, *) {
        let boundsSize = CGSize(width: width, height: height)
        let artwork = MPMediaItemArtwork(boundsSize: boundsSize) { _ in image }
        return mpRetain(artwork)
    }
    return nil
}

@_cdecl("mp_artwork_release")
public func mp_artwork_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    mpRelease(ptr)
}
