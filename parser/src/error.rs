use core::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("CSV Error: {0}")]
    Csv(#[from] CsvError),

    #[error("TXT Error: {0}")]
    Txt(#[from] TxtError),

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

#[derive(Debug, Error)]
pub enum TxtError {
    #[error("Wrong parts number expected = '{expected}', actual = '{actual}'")]
    WrongPartsSize { expected: usize, actual: usize },

    #[error("Wrong key = '{key}'")]
    WrongKey { key: String },

    #[error("Wrong value = '{value}'")]
    WrongValue { value: String },

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("MissingField field = '{field}'")]
    MissingField { field: String },
}
