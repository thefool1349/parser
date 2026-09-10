use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ParserError {
    FileError(IoError),
    InvalidCsv,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::FileError(msg) => write!(f, "File Error: {}", msg),
            ParserError::InvalidCsv => write!(f, "Invalid Csv file."),
        }
    }
}

impl From<IoError> for ParserError {
    fn from(value: IoError) -> Self {
        ParserError::FileError(value)
    }
}
