import AppKit
import Foundation
import MediaPlayer

@_cdecl("mp_artwork_new_from_path")
public func mp_artwork_new_from_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    let filePath = String(cString: path)
    guard let image = NSImage(contentsOfFile: filePath) else { return nil }
    guard #available(macOS 10.12.2, *) else { return nil }

    let artwork = MPMediaItemArtwork(boundsSize: image.size) { _ in image }
    return mpRetain(artwork)
}

@_cdecl("mp_artwork_new_from_path_with_size")
public func mp_artwork_new_from_path_with_size(
    _ path: UnsafePointer<CChar>?,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    let filePath = String(cString: path)
    guard let image = NSImage(contentsOfFile: filePath) else { return nil }
    guard #available(macOS 10.12.2, *) else { return nil }

    let boundsSize = CGSize(width: width, height: height)
    let artwork = MPMediaItemArtwork(boundsSize: boundsSize) { _ in image }
    return mpRetain(artwork)
}

@_cdecl("mp_artwork_copy_bounds")
public func mp_artwork_copy_bounds(
    _ ptr: UnsafeMutableRawPointer?,
    _ originX: UnsafeMutablePointer<Double>?,
    _ originY: UnsafeMutablePointer<Double>?,
    _ width: UnsafeMutablePointer<Double>?,
    _ height: UnsafeMutablePointer<Double>?
) -> Int32 {
    guard let ptr else { return 0 }
    let artwork: MPMediaItemArtwork = mpBorrow(ptr)
    let bounds = artwork.bounds
    originX?.pointee = bounds.origin.x
    originY?.pointee = bounds.origin.y
    width?.pointee = bounds.size.width
    height?.pointee = bounds.size.height
    return 1
}

@_cdecl("mp_artwork_release")
public func mp_artwork_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    mpRelease(ptr)
}
