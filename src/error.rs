use std::io;
use std::string::FromUtf8Error;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum RShellError {
    // IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    // UTF-8 error
    #[fail(display = "Invalid input(not UTF-8). {}", _0)]
    InvalidUTF8(#[cause] FromUtf8Error),
}

impl From<io::Error> for RShellError {
    fn from(err: io::Error) -> Self {
        RShellError::Io(err)
    }
}

impl From<FromUtf8Error> for RShellError {
    fn from(err: FromUtf8Error) -> Self {
        RShellError::InvalidUTF8(err)
    }
}

/// Result type for `rshell`
pub type Result<T> = std::result::Result<T, RShellError>;
