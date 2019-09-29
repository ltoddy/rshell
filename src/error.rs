use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum RShellError {
    // IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
}

impl From<io::Error> for RShellError {
    fn from(err: io::Error) -> Self {
        RShellError::Io(err)
    }
}

/// Result type for `rshell`
pub type Result<T> = std::result::Result<T, RShellError>;
