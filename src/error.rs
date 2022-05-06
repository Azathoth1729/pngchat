//! Custom error type using in pngme
//!
//! [`Error<'a>`] is the type used for error definition
//! ```
//! # #[allow(dead_code)]
//! pub enum Error<'a> {
//!    Custom(&'a str),
//!    IO(std::io::Error),
//!    Fmt(std::fmt::Error),
//!    FromUtf8Error(std::string::FromUtf8Error),
//! }
//! ```

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error<'static>>;

/// Enum error type for pngme
#[derive(Debug)]
pub enum Error<'a> {
    /// Errors which can occur when other case
    Custom(&'a str),
    /// Errors which can occur when doing io operation
    IO(std::io::Error),
    /// Errors which can occur when formating
    Fmt(std::fmt::Error),
    /// Errors which can occur when attempting to interpret a sequence of `[u8]` as a string.
    FromUtf8Error(std::string::FromUtf8Error),
    /// Errors which can occur when attempting to interpret a sequence of `[u8]` as a str.
    Utf8Err(std::str::Utf8Error),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{}", s),
            Self::IO(e) => write!(f, "{}", e),
            Self::Fmt(e) => write!(f, "{}", e),
            Self::Utf8Err(e) => write!(f, "{}", e),
            Self::FromUtf8Error(e) => write!(f, "{}", e),
        }
    }
}

impl<'a> From<Error<'a>> for std::fmt::Error {
    fn from(_: Error) -> Self {
        std::fmt::Error
    }
}

impl<'a> From<std::str::Utf8Error> for Error<'a> {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Err(e)
    }
}

impl<'a> From<std::string::FromUtf8Error> for Error<'a> {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}

impl<'a> error::Error for Error<'a> {}
