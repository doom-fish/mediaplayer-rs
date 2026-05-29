//! ABI layout assertions for the `#[repr(C)]` payload shared with the Swift bridge.
//!
//! `RawCommandPayload` (in `src/async_api.rs`) is read directly from the raw
//! bytes of the Swift `@frozen struct MPStreamCommandPayload`. If their size,
//! alignment or field offsets ever drift apart, the data marshalling silently
//! corrupts (over-read / garbage). The compile-time `const _` asserts next to
//! `RawCommandPayload` pin the Rust side; this test asks the Swift bridge to
//! confirm *its* `MemoryLayout` agrees, catching a real cross-language ABI bug
//! at `cargo test` time rather than as runtime garbage.

use mediaplayer::ffi::mp_verify_ffi_layout;

/// Cross-language ABI check: a `false` return means the Rust `RawCommandPayload`
/// and the Swift `MPStreamCommandPayload` layouts genuinely disagree.
#[test]
fn ffi_command_payload_layout_matches_swift() {
    // SAFETY: `mp_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { mp_verify_ffi_layout() };
    assert!(
        matches,
        "Swift MPStreamCommandPayload layout disagrees with Rust RawCommandPayload (ABI mismatch)"
    );
}
