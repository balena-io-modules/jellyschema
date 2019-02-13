use std::{error, fmt};

use serde;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn message<S>(msg: S) -> Error
    where
        S: Into<String>,
    {
        Error { msg: msg.into() }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<T: serde::de::Error + 'static> From<T> for Error {
    fn from(e: T) -> Error {
        Error { msg: format!("{}", e) }
    }
}
