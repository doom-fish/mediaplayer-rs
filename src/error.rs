use std::error::Error;
use std::fmt;

/// The framework error domain exported by `MediaPlayer.framework`.
pub const ERROR_DOMAIN: &str = "MPErrorDomain";

/// `MPErrorCode` values from `MPError.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    Unknown = 0,
    PermissionDenied = 1,
    CloudServiceCapabilityMissing = 2,
    NetworkConnectionFailed = 3,
    NotFound = 4,
    NotSupported = 5,
    Cancelled = 6,
    RequestTimedOut = 7,
}

impl ErrorCode {
    /// Convert a raw `MPErrorCode` integer into a Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::PermissionDenied,
            2 => Self::CloudServiceCapabilityMissing,
            3 => Self::NetworkConnectionFailed,
            4 => Self::NotFound,
            5 => Self::NotSupported,
            6 => Self::Cancelled,
            7 => Self::RequestTimedOut,
            _ => Self::Unknown,
        }
    }
}

/// Errors returned by mediaplayer operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaPlayerError {
    /// A required argument was null, empty, or otherwise invalid.
    InvalidArgument(String),
    /// The framework reported an error.
    Framework(String),
    /// The feature is not available on this macOS version or platform.
    NotAvailable(String),
    /// An unexpected error occurred.
    Unknown(String),
}

impl fmt::Display for MediaPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message)
            | Self::Framework(message)
            | Self::NotAvailable(message)
            | Self::Unknown(message) => f.write_str(message),
        }
    }
}

impl Error for MediaPlayerError {}
