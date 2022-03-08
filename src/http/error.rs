use std::convert::From;
use std::fmt::Display;

// handle error
pub enum Error {
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
    IO(String),
    Utf8(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            Self::IO(e) => e,
            Self::Utf8(e) => e,
        };
        write!(f, "Error: {}", message)
    }
}

// convert io error to enum Error
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Utf8(err.to_string())
    }
}