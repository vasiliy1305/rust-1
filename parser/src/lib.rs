pub mod bin_format;
pub mod csv_format;
pub mod error;
pub mod txt_format;
pub(crate) mod utils;

use crate::error::ParserError;

/// Тип банковской операции
#[derive(Debug, PartialEq)]
pub enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

impl std::fmt::Display for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxType::DEPOSIT => write!(f, "DEPOSIT"),
            TxType::TRANSFER => write!(f, "TRANSFER"),
            TxType::WITHDRAWAL => write!(f, "WITHDRAWAL"),
        }
    }
}

impl std::str::FromStr for TxType {
    type Err = crate::error::TxtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::DEPOSIT),
            "TRANSFER" => Ok(TxType::TRANSFER),
            "WITHDRAWAL" => Ok(TxType::WITHDRAWAL),
            _ => Err(crate::error::TxtError::WrongValue {
                // привести ошибки к адекватному виду
                value: s.to_string(),
            }),
        }
    }
}



/// Статус банковской операции
#[derive(Debug, PartialEq)]
pub enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

impl std::str::FromStr for Status {
    type Err = crate::error::TxtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Status::SUCCESS),
            "FAILURE" => Ok(Status::FAILURE),
            "PENDING" => Ok(Status::PENDING),
            _ => Err(crate::error::TxtError::WrongValue {
                // привести ошибки к адекватному виду
                value: s.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::SUCCESS => write!(f, "SUCCESS"),
            Status::FAILURE => write!(f, "FAILURE"),
            Status::PENDING => write!(f, "PENDING"),
        }
    }
}


/// Банковская операция (данные)
#[derive(Debug, PartialEq)]
pub struct YPBankRecord {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}

/// Универсальный интерфейс чтения банковских операций из формата
pub trait FormatReader {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<YPBankRecord>, ParserError>;
}


/// Универсальный интерфейс записи банковских операций из формата
pub trait FormatWriter {
    fn save<W: std::io::Write>(writer: &mut W, data: &[YPBankRecord]) -> Result<(), ParserError>;
}
