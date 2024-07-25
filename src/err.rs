use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use std::fmt::{self, Formatter};
use std::io;

#[derive(Debug)]
pub enum DocError {
    SerializationError(serde_json::Error),
    FileError(io::Error),
    FontLoadError(google_fonts::FontError),
    FontParseError(StringError),
}

impl std::error::Error for DocError {}

impl Display for DocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DocError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            DocError::FileError(err) => write!(f, "File error: {}", err),
            DocError::FontLoadError(err) => write!(f, "Font load error: {}", err),
            DocError::FontParseError(err) => write!(f, "Font parse error: {}", err),
        }
    }
}

impl From<serde_json::Error> for DocError {
    fn from(err: serde_json::Error) -> DocError {
        DocError::SerializationError(err)
    }
}

impl From<io::Error> for DocError {
    fn from(err: io::Error) -> DocError {
        DocError::FileError(err)
    }
}

impl From<google_fonts::FontError> for DocError {
    fn from(err: google_fonts::FontError) -> DocError {
        DocError::FontLoadError(err)
    }
}

impl From<&str> for DocError {
    fn from(msg: &str) -> DocError {
        DocError::FontParseError(StringError::new(msg))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringError {
    msg: String,
}
impl StringError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
impl Display for StringError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for StringError {}

impl From<&str> for StringError {
    fn from(msg: &str) -> StringError {
        StringError::new(msg)
    }
}
