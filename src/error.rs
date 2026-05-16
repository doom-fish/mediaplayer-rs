use std::error::Error;
use std::fmt;

/// Errors returned by mediaplayer operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaPlayerError {
    /// A required argument was null, empty, or otherwise invalid.
    InvalidArgument(String),
    /// The framework reported an error.
    Framework(String),
    /// The feature is not available on this macOS version.
    NotAvailable(String),
    /// An unexpected error occurred.
    Unknown(String),
}

impl fmt::Display for MediaPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m)
            | Self::Framework(m)
            | Self::NotAvailable(m)
            | Self::Unknown(m) => f.write_str(m),
        }
    }
}

impl Error for MediaPlayerError {}
