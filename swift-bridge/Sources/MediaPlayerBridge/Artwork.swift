import AppKit
import Foundation
import MediaPlayer

private let mpMaximumArtworkDimension = 16384.0

private func mpIsUsableArtworkSize(_ size: CGSize) -> Bool {
    size.width.isFinite && size.height.isFinite
        && size.width >= 1 && size.height >= 1
        && size.width <= mpMaximumArtworkDimension && size.height <= mpMaximumArtworkDimension
}

private func mpImage(_ image: NSImage, fittedTo requested: CGSize) -> NSImage {
    let natural = image.size
    guard mpIsUsableArtworkSize(requested), natural.width > 0, natural.height > 0, requested != natural else {
        return image
    }
    return NSImage(size: requested, flipped: false) { rect in
        let scale = min(rect.width / natural.width, rect.height / natural.height)
        let drawn = CGSize(width: natural.width * scale, height: natural.height * scale)
        let origin = CGPoint(x: rect.midX - drawn.width / 2, y: rect.midY - drawn.height / 2)
        image.draw(in: CGRect(origin: origin, size: drawn))
        return true
    }
}

private func mpArtwork(image: NSImage, width: Double, height: Double) -> UnsafeMutableRawPointer? {
    let boundsSize = width > 0 && height > 0 ? CGSize(width: width, height: height) : image.size
    let artwork = MPMediaItemArtwork(boundsSize: boundsSize) { requested in
        mpImage(image, fittedTo: requested)
    }
    return mpRetain(artwork)
}

@_cdecl("mp_artwork_new_from_path")
public func mp_artwork_new_from_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    guard let image = NSImage(contentsOfFile: String(cString: path)) else { return nil }
    return mpArtwork(image: image, width: 0, height: 0)
}

@_cdecl("mp_artwork_new_from_path_with_size")
public func mp_artwork_new_from_path_with_size(
    _ path: UnsafePointer<CChar>?,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    guard let image = NSImage(contentsOfFile: String(cString: path)) else { return nil }
    return mpArtwork(image: image, width: width, height: height)
}

@_cdecl("mp_artwork_new_from_data")
public func mp_artwork_new_from_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let bytes, length > 0 else { return nil }
    guard let image = NSImage(data: Data(bytes: bytes, count: length)), image.isValid else { return nil }
    return mpArtwork(image: image, width: width, height: height)
}

@_cdecl("mp_artwork_copy_png_data")
public func mp_artwork_copy_png_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ width: Double,
    _ height: Double,
    _ outLength: UnsafeMutablePointer<Int>?
) -> UnsafeMutablePointer<UInt8>? {
    let size = CGSize(width: width, height: height)
    guard let ptr, let outLength, mpIsUsableArtworkSize(size) else { return nil }
    let artwork: MPMediaItemArtwork = mpBorrow(ptr)
    guard let image = artwork.image(at: size), mpIsUsableArtworkSize(image.size) else { return nil }
    let imageSize = image.size
    let pixelsWide = Int(imageSize.width.rounded(.up))
    let pixelsHigh = Int(imageSize.height.rounded(.up))
    guard let rep = NSBitmapImageRep(
        bitmapDataPlanes: nil,
        pixelsWide: pixelsWide,
        pixelsHigh: pixelsHigh,
        bitsPerSample: 8,
        samplesPerPixel: 4,
        hasAlpha: true,
        isPlanar: false,
        colorSpaceName: .deviceRGB,
        bytesPerRow: 0,
        bitsPerPixel: 0
    ), let context = NSGraphicsContext(bitmapImageRep: rep) else {
        return nil
    }
    rep.size = imageSize
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = context
    image.draw(in: CGRect(origin: .zero, size: imageSize))
    NSGraphicsContext.restoreGraphicsState()
    guard let data = rep.representation(using: .png, properties: [:]), !data.isEmpty,
          let buffer = malloc(data.count)?.assumingMemoryBound(to: UInt8.self)
    else {
        return nil
    }
    data.copyBytes(to: buffer, count: data.count)
    outLength.pointee = data.count
    return buffer
}

@_cdecl("mp_bytes_free")
public func mp_bytes_free(_ bytes: UnsafeMutablePointer<UInt8>?) {
    free(bytes)
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

@_cdecl("mp_animated_artwork_new_from_files")
public func mp_animated_artwork_new_from_files(
    _ artworkID: UnsafePointer<CChar>?,
    _ previewImagePath: UnsafePointer<CChar>?,
    _ videoAssetPath: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 26.0, *),
          let artworkID,
          let previewImagePath,
          let videoAssetPath
    else {
        return nil
    }

    let previewPath = String(cString: previewImagePath)
    let videoPath = String(cString: videoAssetPath)
    guard let previewImage = NSImage(contentsOfFile: previewPath) else { return nil }
    let videoURL = URL(fileURLWithPath: videoPath)
    guard FileManager.default.fileExists(atPath: videoURL.path) else { return nil }

    let artwork = MPMediaItemAnimatedArtwork(
        artworkID: String(cString: artworkID),
        previewImageRequestHandler: { _, completion in
            completion(previewImage)
        },
        videoAssetFileURLRequestHandler: { _, completion in
            completion(videoURL)
        }
    )
    return mpRetain(artwork)
}

@_cdecl("mp_animated_artwork_release")
public func mp_animated_artwork_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    mpRelease(ptr)
}
