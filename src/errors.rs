//! This module has the Error type used throught the entire crate.

use std::{fmt::Display, path::StripPrefixError};

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    // fs errors
    IoError(std::io::Error),
    InvalidPathPrefix,
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "IO error: {error}"),
            Error::InvalidPathPrefix => write!(f, "Invalid path prefix: cannot get relative path"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<StripPrefixError> for Error {
    fn from(_: StripPrefixError) -> Self {
        Self::InvalidPathPrefix
    }
}
