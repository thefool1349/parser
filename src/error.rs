use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ParserError {
    FileError(IoError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::FileError(msg) => write!(f, "File Error: {}", msg),
        }
    }
}

impl From<IoError> for ParserError {
    fn from(value: IoError) -> Self {
        ParserError::FileError(value)
    }
}
