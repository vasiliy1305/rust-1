pub mod csv_format;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParserError {
    #[error("csv error: {0}")]
    Csv(#[from] csv_format::CsvError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

use std::io::Read;

#[derive(Debug, PartialEq)]
enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Debug, PartialEq)]
enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug, PartialEq)]
struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}

trait LoadData {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<Transaction>, ParserError>;
}

trait SaveData {
    fn save<W: std::io::Write>(writer: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError>;
}
