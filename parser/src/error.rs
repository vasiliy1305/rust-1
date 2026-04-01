use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("CSV Error: {0}")]
    Csv(#[from] CsvError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum CsvError {
    #[error("Invalid column count, expected '{expected}', got '{actual}'")]
    InvalidLength { expected: usize, actual: usize },

    #[error("Wrong column at index {index}: expected '{expected}', got '{actual}'")]
    WrongColumn {
        index: usize,
        expected: String,
        actual: String,
    },

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}
