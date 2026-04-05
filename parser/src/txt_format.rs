use std::io::{BufRead, BufReader};

// txt_format.rs — логика и типы для текстового формата.
use crate::error::{ParserError, TxtError};
use crate::{LoadData, SaveData, Status, Transaction, TxType, trim_quotes};

pub struct TxtFormat;

#[derive(Default)]
struct TransactionDraft {
    tx_id: Option<u64>,
    tx_type: Option<TxType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<u64>,
    timestamp: Option<u64>,
    status: Option<Status>,
    description: Option<String>,
}

impl TransactionDraft {
    fn build(self) -> Result<Transaction, TxtError> {
        Ok(Transaction {
            tx_id: self.tx_id.ok_or(TxtError::MissingField {
                field: "TX_ID".into(),
            })?,
            tx_type: self.tx_type.ok_or(TxtError::MissingField {
                field: "TX_TYPE".into(),
            })?,
            from_user_id: self.from_user_id.ok_or(TxtError::MissingField {
                field: "FROM_USER_ID".into(),
            })?,
            to_user_id: self.to_user_id.ok_or(TxtError::MissingField {
                field: "TO_USER_ID".into(),
            })?,
            amount: self.amount.ok_or(TxtError::MissingField {
                field: "AMOUNT".into(),
            })?,
            timestamp: self.timestamp.ok_or(TxtError::MissingField {
                field: "TIMESTAMP".into(),
            })?,
            status: self.status.ok_or(TxtError::MissingField {
                field: "STATUS".into(),
            })?,
            description: self.description.ok_or(TxtError::MissingField {
                field: "DESCRIPTION".into(),
            })?,
        })
    }
}

impl LoadData for TxtFormat {
    fn load<R: std::io::Read>(mut reader: R) -> Result<Vec<Transaction>, ParserError> {
        let reader = BufReader::new(reader);

        let mut resualt = Vec::<Transaction>::new();

        let mut tx = TransactionDraft::default();

        for line in reader.lines() {
            let line = line?;
            let key_value = get_line_type(&line)?;
            match key_value {
                TxtLineType::Comment => {} // просто игнорируем
                TxtLineType::Data(value) => match value {
                    TxtKeyValue::TX_ID(id) => tx.tx_id = Some(id),
                    TxtKeyValue::TX_TYPE(tx_type) => tx.tx_type = Some(tx_type),
                    TxtKeyValue::AMOUNT(amount) => tx.amount = Some(amount),
                    TxtKeyValue::FROM_USER_ID(from) => tx.from_user_id = Some(from),
                    TxtKeyValue::TO_USER_ID(to) => tx.to_user_id = Some(to),
                    TxtKeyValue::DESCRIPTION(desk) => tx.description = Some(desk),
                    TxtKeyValue::STATUS(status) => tx.status = Some(status),
                    TxtKeyValue::TIMESTAMP(ts) => tx.timestamp = Some(ts),
                },
                TxtLineType::Empty => {
                    // что если несколько пустых строк подрят или пустая строка на первом месте? можно ввести флаг на этот случай
                    resualt.push(tx.build()?);
                    tx = TransactionDraft::default();
                }
            }
        }
        Ok(resualt)
    }
}

impl SaveData for TxtFormat {
    fn save<W: std::io::Write>(writer: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError> {
        for tx in data {
            writeln!(
                writer,
                "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: \"{}\"\nDESCRIPTION: {}\n",
                tx.tx_id,
                tx.tx_type,
                tx.from_user_id,
                tx.to_user_id,
                tx.amount,
                tx.timestamp,
                tx.status,
                tx.description
            )?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum TxtLineType {
    Empty,
    Comment,
    Data(TxtKeyValue),
}

#[derive(Debug, PartialEq)]
enum TxtKeyValue {
    TX_ID(u64),
    TX_TYPE(TxType),
    FROM_USER_ID(u64),
    TO_USER_ID(u64),
    AMOUNT(u64),
    TIMESTAMP(u64),
    STATUS(Status),
    DESCRIPTION(String),
}

fn get_line_type(line: &str) -> Result<TxtLineType, TxtError> {
    // получилось масивно и не красиво((
    let s = line.trim();
    if s.is_empty() {
        Ok(TxtLineType::Empty)
    } else if s.starts_with('#') {
        Ok(TxtLineType::Comment)
    } else {
        let (key, value) = s.split_once(':').map(|(k, v)| (k.trim(), v.trim())).ok_or(
            TxtError::WrongPartsSize {
                expected: 2,
                actual: 1,
            },
        )?;

        match key {
            "TX_ID" => Ok(TxtLineType::Data(TxtKeyValue::TX_ID(value.parse::<u64>()?))),

            "TX_TYPE" => match value {
                "DEPOSIT" => Ok(TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::DEPOSIT))),
                "TRANSFER" => Ok(TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::TRANSFER))),
                "WITHDRAWAL" => Ok(TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::WITHDRAWAL))),
                _ => Err(TxtError::WrongValue {
                    value: value.to_string(),
                }),
            },
            "FROM_USER_ID" => Ok(TxtLineType::Data(TxtKeyValue::FROM_USER_ID(
                value.parse::<u64>()?,
            ))),

            "TO_USER_ID" => Ok(TxtLineType::Data(TxtKeyValue::TO_USER_ID(
                value.parse::<u64>()?,
            ))),

            "AMOUNT" => Ok(TxtLineType::Data(TxtKeyValue::AMOUNT(
                value.parse::<u64>()?,
            ))),
            "TIMESTAMP" => Ok(TxtLineType::Data(TxtKeyValue::TIMESTAMP(
                value.parse::<u64>()?,
            ))),

            "STATUS" => match value {
                "SUCCESS" => Ok(TxtLineType::Data(TxtKeyValue::STATUS(Status::SUCCESS))),
                "FAILURE" => Ok(TxtLineType::Data(TxtKeyValue::STATUS(Status::FAILURE))),
                "PENDING" => Ok(TxtLineType::Data(TxtKeyValue::STATUS(Status::PENDING))),
                _ => Err(TxtError::WrongValue {
                    value: value.to_string(),
                }),
            },

            "DESCRIPTION" => Ok(TxtLineType::Data(TxtKeyValue::DESCRIPTION(
                trim_quotes(value).to_string(),
            ))),

            _ => Err(TxtError::WrongKey {
                key: value.to_string(),
            }),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_get_line_type() {
        let line = get_line_type(" ");
        assert!(!line.is_err());
        assert_eq!(line.unwrap(), TxtLineType::Empty);

        let line = get_line_type("#qwerty");
        assert!(!line.is_err());
        assert_eq!(line.unwrap(), TxtLineType::Comment);

        let line = get_line_type("TX_ID: 1234567890123456");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TX_ID(1234567890123456))
        );

        let line = get_line_type("TX_TYPE: DEPOSIT");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::DEPOSIT))
        );

        let line = get_line_type("TX_TYPE: TRANSFER");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::TRANSFER))
        );

        let line = get_line_type("TX_TYPE: WITHDRAWAL");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TX_TYPE(TxType::WITHDRAWAL))
        );

        let line = get_line_type("FROM_USER_ID: 123");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::FROM_USER_ID(123))
        );

        let line = get_line_type("TO_USER_ID: 9876543210987654");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TO_USER_ID(9876543210987654))
        );

        let line = get_line_type("AMOUNT: 10000");
        assert!(!line.is_err());
        assert_eq!(line.unwrap(), TxtLineType::Data(TxtKeyValue::AMOUNT(10000)));

        let line = get_line_type("TIMESTAMP: 1633036800000");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TIMESTAMP(1633036800000))
        );

        let line = get_line_type("STATUS: SUCCESS");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::STATUS(Status::SUCCESS))
        );

        let line = get_line_type("STATUS: FAILURE");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::STATUS(Status::FAILURE))
        );

        let line = get_line_type("STATUS: PENDING");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::STATUS(Status::PENDING))
        );

        let line = get_line_type("DESCRIPTION: \"Terminal deposit\"");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::DESCRIPTION("Terminal deposit".to_string()))
        );
    }
}

// DESCRIPTION: "Terminal deposit"
