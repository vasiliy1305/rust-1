use std::io::{BufReader, Error};

use crate::{LoadData, ParserError, Transaction};
use thiserror::Error;

const HEADER: [&str; 8] = [
    "TX_ID",
    "TX_TYPE",
    "FROM_USER_ID",
    "TO_USER_ID",
    "AMOUNT",
    "TIMESTAMP",
    "STATUS",
    "DESCRIPTION",
];

// csv_format.rs — логика и типы для csv формата.
#[derive(Debug, Error)]
pub enum CsvError {
    #[error("Invalid column count, expected '{expected}', got '{actual}'")]
    InvalidLength { expected: usize, actual: usize },

    #[error("wrong column at index {index}: expected '{expected}', got '{actual}'")]
    WrongColumn {
        index: usize,
        expected: String,
        actual: String,
    },

    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

struct CsvFormat;

impl LoadData for CsvFormat {
    fn load<R: std::io::Read>(mut reader: R) -> Result<Vec<Transaction>, ParserError> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let mut resualt = Vec::<Transaction>::new();

        let mut is_header = true;
        for line in content.lines() {
            if is_header {
                check_header(line)?;
                is_header = false;
            } else {
                resualt.push(parse_line(line)?);
            }
        }
        Ok(resualt)
    }
}

fn check_header(header: &str) -> Result<(), CsvError> {
    let expected_header: [&str; 8] = [
        "TX_ID",
        "TX_TYPE",
        "FROM_USER_ID",
        "TO_USER_ID",
        "AMOUNT",
        "TIMESTAMP",
        "STATUS",
        "DESCRIPTION",
    ];

    let actual_header: Vec<&str> = header.split(',').collect();

    if expected_header.len() != actual_header.len() {
        return Err(CsvError::InvalidLength {
            expected: expected_header.len(),
            actual: actual_header.len(),
        });
    }

    for (index, (expected, actual)) in expected_header.iter().zip(actual_header.iter()).enumerate()
    {
        if expected.trim() != actual.trim() {
            return Err(CsvError::WrongColumn {
                index: index,
                expected: expected.to_string(),
                actual: actual.to_string(),
            });
        }
    }
    Ok(())
}

fn parse_line(line: &str) -> Result<Transaction, CsvError> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != HEADER.len() {
        return Err(CsvError::InvalidLength {
            expected: HEADER.len(),
            actual: parts.len(),
        });
    }

    let tx_id = parts[0].trim().parse::<u64>()?;

    let tx_type = match parts[1].trim() {
        "DEPOSIT" => crate::TxType::DEPOSIT,
        "TRANSFER" => crate::TxType::TRANSFER,
        "WITHDRAWAL" => crate::TxType::WITHDRAWAL,
        _ => {
            return Err(CsvError::WrongColumn {
                index: 1,
                expected: "DEPOSIT, TRANSFER, WITHDRAWAL".to_string(),
                actual: parts[1].trim().to_string(),
            });
        }
    };

    let from_user_id = parts[2].trim().parse::<u64>()?;
    let to_user_id = parts[3].trim().parse::<u64>()?;
    let amount = parts[4].trim().parse::<u64>()?;
    let timestamp = parts[5].trim().parse::<u64>()?;

    let status = match parts[6].trim() {
        "SUCCESS" => crate::Status::SUCCESS,
        "FAILURE" => crate::Status::FAILURE,
        "PENDING" => crate::Status::PENDING,
        _ => {
            return Err(CsvError::WrongColumn {
                index: 6,
                expected: "SUCCESS, FAILURE, PENDING".to_string(),
                actual: parts[6].trim().to_string(),
            });
        }
    };

    let description = parts[7].to_string();

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_header() {
        assert!(
            check_header(
                "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
            )
            .is_ok()
        );

        assert!(
            check_header(
                "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID, AMOUNT,TIMESTAMP ,STATUS,  DESCRIPTION"
            )
            .is_ok()
        );

        assert!(
            check_header("TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS").is_err()
        );

        assert!(
            check_header(
                "TX_ID,qwerty,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
            )
            .is_err()
        );
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line(
            "1000000000000017,WITHDRAWAL,9223372036854775807,0,1800,1633037880000,SUCCESS,Record number 18",
        );

        assert!(result.is_ok());

        let tx = result.unwrap();

        assert_eq!(
            tx,
            Transaction {
                tx_id: 1000000000000017,
                tx_type: crate::TxType::WITHDRAWAL,
                from_user_id: 9223372036854775807,
                to_user_id: 0,
                amount: 1800,
                timestamp: 1633037880000,
                status: crate::Status::SUCCESS,
                description: "Record number 18".to_string(),
            }
        );

        let result = parse_line(
            "1000000000000017,WITHDRAWAL,9223372036854775807,0,1800,1633037880000,Record number 18",
        );

        match result {
            Err(CsvError::InvalidLength { expected, actual }) => {
                assert_eq!(expected, 8);
                assert_eq!(actual, 7);
            }
            _ => panic!("ожидали CsvError::InvalidLength"),
        }
    }
}
