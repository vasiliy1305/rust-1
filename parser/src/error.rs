use thiserror::Error;


/// Общие ошибки для всех форматов
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("CSV Error: {0}")]
    Csv(#[from] CsvError),

    #[error("TXT Error: {0}")]
    Txt(#[from] TxtError),

    #[error("Binary Error: {0}")]
    Bin(#[from] BinError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Rarce From Utf Error: {0}")]
    UTF(#[from] std::string::FromUtf8Error),
}


/// ошибки специфичные для CSV формата
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

    #[error("Invalid TxType {value}")]
    InvalidTxType { value: String },

    #[error("Description error: {0}")]
    Description(#[from] DescriptionError),

    #[error("Invalid Status {value}")]
    InvalidStatus { value: String },

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Unclosed quoted field in CSV line: {line}")]
    UnclosedQuotedField { line: String },
}


/// ошибки специфичные для TXT формата
#[derive(Debug, Error)]
pub enum TxtError {
    #[error("Wrong parts number expected = '{expected}', actual = '{actual}'")]
    WrongPartsSize { expected: usize, actual: usize },

    #[error("Wrong key = '{key}'")]
    WrongKey { key: String },

    #[error("Wrong value = '{value}'")]
    WrongValue { value: String },

    #[error("Description error: {0}")]
    Description(#[from] DescriptionError),

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("MissingField field = '{field}'")]
    MissingField { field: String },
}


/// ошибки специфичные для Binary формата
#[derive(Debug, Error)]
pub enum BinError {
    #[error("Wrong magic")]
    WrongMagic,

    // #[error("Wrong key = '{key}'")]
    // WrongKey { key: String },
    #[error("Wrong tx type = '{value}'")]
    WrongTxType { value: u8 },

    #[error("Wrong status = '{value}'")]
    WrongStatus { value: u8 },

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    // #[error("MissingField field = '{field}'")]
    // MissingField { field: String },
    #[error("Wrong record size: expected '{expected}', got '{actual}'")]
    WrongRecordSize { expected: u32, actual: u32 },
}

#[derive(Debug, Error)]
pub enum DescriptionError {
    #[error("Invalid description format: '{value}'")]
    InvalidFormat { value: String },

    #[error("Invalid description escape: '{value}'")]
    InvalidEscape { value: String },
}
