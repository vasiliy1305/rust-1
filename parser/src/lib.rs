pub mod csv_format;
pub mod error;
pub mod txt_format;

use std::fmt::write;

use error::ParserError;
// use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

impl std::fmt::Display for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxType::DEPOSIT => {
                write!(f, "DEPOSIT")
            }
            TxType::TRANSFER => {
                write!(f, "TRANSFER")
            }
            TxType::WITHDRAWAL => {
                write!(f, "WITHDRAWAL")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::SUCCESS => {
                write!(f, "SUCCESS")
            }
            Status::FAILURE => {
                write!(f, "FAILURE")
            }
            Status::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
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

fn trim_quotes(s: &str) -> &str {
    let s = s.trim();
    let s = s.strip_prefix('"').unwrap_or(s);
    let s = s.strip_suffix('"').unwrap_or(s);
    s
}
