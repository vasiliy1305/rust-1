pub mod csv_format;
pub mod error;

use error::ParserError;
// use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}

pub trait LoadData {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<Transaction>, ParserError>;
}

pub trait SaveData {
    fn save<W: std::io::Write>(writer: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError>;
}
