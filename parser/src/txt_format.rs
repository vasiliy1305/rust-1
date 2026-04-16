use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::error::{ParserError, TxtError};
use crate::utils::{format_description, parse_description};
use crate::{FormatReader, FormatWriter, Status, TxType, YPBankRecord};

pub struct YPBankTxtFormat;

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
    // все равно осталась проблемма дублирования полей (не совсем понял по спецификации евляется ли это ошибкой)
    fn build(self) -> Result<YPBankRecord, TxtError> {
        Ok(YPBankRecord {
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

    fn is_empty(&self) -> bool {
        self.tx_id.is_none()
            && self.tx_type.is_none()
            && self.from_user_id.is_none()
            && self.to_user_id.is_none()
            && self.amount.is_none()
            && self.timestamp.is_none()
            && self.status.is_none()
            && self.description.is_none()
    }
}

impl FormatReader for YPBankTxtFormat {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let reader = BufReader::new(reader);
        let mut result = Vec::<YPBankRecord>::new();
        let mut tx = TransactionDraft::default();

        for line in reader.lines() {
            let line = line?;
            let key_value = get_line_type(&line)?;

            match key_value {
                TxtLineType::Comment => {} // просто игнорируем
                TxtLineType::Data(value) => match value {
                    TxtKeyValue::TxId(id) => tx.tx_id = Some(id),
                    TxtKeyValue::TxType(tx_type) => tx.tx_type = Some(tx_type),
                    TxtKeyValue::Amount(amount) => tx.amount = Some(amount),
                    TxtKeyValue::FromUserId(from) => tx.from_user_id = Some(from),
                    TxtKeyValue::ToUserId(to) => tx.to_user_id = Some(to),
                    TxtKeyValue::Description(desk) => tx.description = Some(desk),
                    TxtKeyValue::Status(status) => tx.status = Some(status),
                    TxtKeyValue::Timestamp(ts) => tx.timestamp = Some(ts),
                },
                TxtLineType::Empty => {
                    if !tx.is_empty() {
                        result.push(tx.build()?);
                        tx = TransactionDraft::default();
                    }
                }
            }
        }

        if !tx.is_empty() {
            result.push(tx.build()?);
        }

        Ok(result)
    }
}

impl FormatWriter for YPBankTxtFormat {
    fn save<W: std::io::Write>(writer: &mut W, data: &[YPBankRecord]) -> Result<(), ParserError> {
        for tx in data {
            writeln!(
                writer,
                "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}\n",
                tx.tx_id,
                tx.tx_type,
                tx.from_user_id,
                tx.to_user_id,
                tx.amount,
                tx.timestamp,
                tx.status,
                format_description(&tx.description)
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
    TxId(u64),
    TxType(TxType),
    FromUserId(u64),
    ToUserId(u64),
    Amount(u64),
    Timestamp(u64),
    Status(Status),
    Description(String),
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
            "TX_ID" => Ok(TxtLineType::Data(TxtKeyValue::TxId(value.parse::<u64>()?))),

            "TX_TYPE" => Ok(TxtLineType::Data(TxtKeyValue::TxType(TxType::from_str(
                value,
            )?))),

            "FROM_USER_ID" => Ok(TxtLineType::Data(TxtKeyValue::FromUserId(
                value.parse::<u64>()?,
            ))),

            "TO_USER_ID" => Ok(TxtLineType::Data(TxtKeyValue::ToUserId(
                value.parse::<u64>()?,
            ))),

            "AMOUNT" => Ok(TxtLineType::Data(TxtKeyValue::Amount(
                value.parse::<u64>()?,
            ))),
            "TIMESTAMP" => Ok(TxtLineType::Data(TxtKeyValue::Timestamp(
                value.parse::<u64>()?,
            ))),

            "STATUS" => Ok(TxtLineType::Data(TxtKeyValue::Status(Status::from_str(
                value,
            )?))),

            "DESCRIPTION" => Ok(TxtLineType::Data(TxtKeyValue::Description(
                parse_description(value)?,
            ))),

            _ => Err(TxtError::WrongKey {
                key: key.to_string(),
            }),
        }
    }
}

#[cfg(test)]
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
            TxtLineType::Data(TxtKeyValue::TxId(1234567890123456))
        );

        let line = get_line_type("TX_TYPE: DEPOSIT");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TxType(TxType::DEPOSIT))
        );

        let line = get_line_type("TX_TYPE: TRANSFER");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TxType(TxType::TRANSFER))
        );

        let line = get_line_type("TX_TYPE: WITHDRAWAL");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::TxType(TxType::WITHDRAWAL))
        );

        let line = get_line_type("FROM_USER_ID: 123");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::FromUserId(123))
        );

        let line = get_line_type("TO_USER_ID: 9876543210987654");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::ToUserId(9876543210987654))
        );

        let line = get_line_type("AMOUNT: 10000");
        assert!(!line.is_err());
        assert_eq!(line.unwrap(), TxtLineType::Data(TxtKeyValue::Amount(10000)));

        let line = get_line_type("TIMESTAMP: 1633036800000");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Timestamp(1633036800000))
        );

        let line = get_line_type("STATUS: SUCCESS");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Status(Status::SUCCESS))
        );

        let line = get_line_type("STATUS: FAILURE");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Status(Status::FAILURE))
        );

        let line = get_line_type("STATUS: PENDING");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Status(Status::PENDING))
        );

        let line = get_line_type("DESCRIPTION: \"Terminal deposit\"");
        assert!(!line.is_err());
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Description("Terminal deposit".to_string()))
        );
    }

    #[test]
    fn test_status() {
        let line = get_line_type("STATUS: SUCCESS");
        assert_eq!(
            line.unwrap(),
            TxtLineType::Data(TxtKeyValue::Status(Status::SUCCESS))
        );
    }

    #[test]
    fn test_get_line_type_wrong_key() {
        let result = get_line_type("BAD_KEY: 123");

        match result {
            Err(TxtError::WrongKey { key }) => {
                assert_eq!(key, "BAD_KEY");
            }
            _ => panic!("ожидали TxtError::WrongKey"),
        }
    }

    #[test]
    fn test_get_line_type_wrong_parts() {
        let result = get_line_type("TX_ID 123");

        match result {
            Err(TxtError::WrongPartsSize { expected, actual }) => {
                assert_eq!(expected, 2);
                assert_eq!(actual, 1);
            }
            _ => panic!("ожидали TxtError::WrongPartsSize"),
        }
    }

    #[test]
    fn test_load_txt_ok() {
        let input = r#"
# Record 1
TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: "Terminal deposit"

# Record 2
TX_ID: 2312321321321321
TX_TYPE: TRANSFER
FROM_USER_ID: 1231231231231231
TO_USER_ID: 9876543210987654
AMOUNT: 1000
TIMESTAMP: 1633056800000
STATUS: FAILURE
DESCRIPTION: "User transfer"
"#;

        let result = YPBankTxtFormat::load(input.as_bytes()).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].tx_id, 1234567890123456);
        assert_eq!(result[0].status, Status::SUCCESS);
        assert_eq!(result[1].tx_type, TxType::TRANSFER);
        assert_eq!(result[1].description, "User transfer");
    }

    #[test]
    fn test_load_txt_missing_field() {
        let input = r#"
TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
"#;

        let result = YPBankTxtFormat::load(input.as_bytes());

        match result {
            Err(ParserError::Txt(TxtError::MissingField { field })) => {
                assert_eq!(field, "DESCRIPTION");
            }
            _ => panic!("ожидали TxtError::MissingField"),
        }
    }

    #[test]
    fn test_save_txt_ok() {
        let data = vec![YPBankRecord {
            tx_id: 123,
            tx_type: TxType::DEPOSIT,
            from_user_id: 0,
            to_user_id: 456,
            amount: 1000,
            timestamp: 111111,
            status: Status::SUCCESS,
            description: "Terminal deposit".to_string(),
        }];

        let mut out = Vec::new();
        YPBankTxtFormat::save(&mut out, &data).unwrap();

        let output = String::from_utf8(out).unwrap();

        assert_eq!(
            output,
            "TX_ID: 123\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 0\n\
TO_USER_ID: 456\n\
AMOUNT: 1000\n\
TIMESTAMP: 111111\n\
STATUS: SUCCESS\n\
DESCRIPTION: \"Terminal deposit\"\n\n"
        );
    }
}

// DESCRIPTION: "Terminal deposit"
