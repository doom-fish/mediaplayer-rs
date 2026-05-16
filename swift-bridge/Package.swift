// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MediaPlayerBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "MediaPlayerBridge",
            type: .static,
            targets: ["MediaPlayerBridge"])
    ],
    targets: [
        .target(
            name: "MediaPlayerBridge",
            path: "Sources/MediaPlayerBridge",
            publicHeadersPath: "include")
    ]
)
