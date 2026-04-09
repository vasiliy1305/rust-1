use std::io::{BufRead, BufReader};
use std::str;

use crate::error::{CsvError, ParserError};
use crate::utils::trim_quotes;
use crate::{FormatReader, FormatWriter, Status, TxType, YPBankRecord};

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

pub struct YPBankCsvFormat;

impl FormatReader for YPBankCsvFormat {
    fn load<R: std::io::Read>(mut reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let mut reader = BufReader::new(reader);
        let mut result = Vec::<YPBankRecord>::new();

        let mut is_header = true;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if is_header {
                check_header(&line)?;
                is_header = false;
            } else {
                result.push(parse_line(&line)?);
            }
        }
        Ok(result)
    }
}

impl FormatWriter for YPBankCsvFormat {
    fn save<W: std::io::Write>(writer: &mut W, data: &[YPBankRecord]) -> Result<(), ParserError> {
        writeln!(writer, "{}", HEADER.join(","))?;
        for tx in data {
            writeln!(writer, "{}", transaction_to_str(tx))?;
        }
        writer.flush()?;
        Ok(())
    }
}

fn transaction_to_str(tx: &YPBankRecord) -> String {
    format!(
        "{},{},{},{},{},{},{},\"{}\"", // не совсем хорошо так делать
        tx.tx_id,
        tx.tx_type,
        tx.from_user_id,
        tx.to_user_id,
        tx.amount,
        tx.timestamp,
        tx.status,
        tx.description
    )
}

fn check_header(header: &str) -> Result<(), CsvError> {
    let actual_header: Vec<&str> = header.split(',').collect();

    if HEADER.len() != actual_header.len() {
        return Err(CsvError::InvalidLength {
            expected: HEADER.len(),
            actual: actual_header.len(),
        });
    }

    for (index, (expected, actual)) in HEADER.iter().zip(actual_header.iter()).enumerate() {
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

fn split_csv_line(line: &str) -> Result<Vec<String>, CsvError> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes {
                    if let Some('"') = chars.peek() {
                        current.push('"');
                        chars.next();
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            ',' if !in_quotes => {
                fields.push(current);
                current = String::new();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if in_quotes {
        return Err(CsvError::WrongColumn {
            index: 7,
            expected: "closed quoted string".to_string(),
            actual: line.to_string(),
        });
    }

    fields.push(current);
    Ok(fields)
}

fn parse_line(line: &str) -> Result<YPBankRecord, CsvError> {
    let parts: Vec<String> = split_csv_line(line)?;
    if parts.len() != HEADER.len() {
        return Err(CsvError::InvalidLength {
            expected: HEADER.len(),
            actual: parts.len(),
        });
    }

    let tx_id = parts[0].trim().parse::<u64>()?;

    let tx_type = match parts[1].trim() {
        "DEPOSIT" => TxType::DEPOSIT,
        "TRANSFER" => TxType::TRANSFER,
        "WITHDRAWAL" => TxType::WITHDRAWAL,
        _ => {
            return Err(CsvError::InvalidTxType {
                value: parts[1].trim().to_string(),
            });
        }
    };

    let from_user_id = parts[2].trim().parse::<u64>()?;
    let to_user_id = parts[3].trim().parse::<u64>()?;
    let amount = parts[4].trim().parse::<u64>()?;
    let timestamp = parts[5].trim().parse::<u64>()?;

    let status = match parts[6].trim() {
        "SUCCESS" => Status::SUCCESS,
        "FAILURE" => Status::FAILURE,
        "PENDING" => Status::PENDING,
        _ => {
            return Err(CsvError::InvalidStatus {
                value: parts[6].trim().to_string(),
            });
        }
    };

    let description = trim_quotes(&parts[7]);

    Ok(YPBankRecord {
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
    #[test]
    fn test_parse_line_with_comma_in_description() {
        let result = parse_line(
            r#"1002,TRANSFER,501,502,15000,1672534800000,FAILURE,"Payment for services, invoice123""#,
        );

        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.description, "Payment for services, invoice123");
    }

    #[test]
    fn test_transaction_to_str() {
        let tx = YPBankRecord {
            tx_id: 1000000000000000,
            tx_type: TxType::DEPOSIT,
            from_user_id: 0,
            to_user_id: 9223372036854775807,
            amount: 100,
            timestamp: 1633036860000,
            status: Status::FAILURE,
            description: "Record number 1".to_string(),
        };

        assert_eq!(transaction_to_str(&tx), "1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"".to_string())
    }

    // #[test]
    // fn test_load_save_data(){
    //     let mut file = File::open("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv").unwrap();
    //     let reader = BufReader::new(file);
    //     let data = CsvFormat::load(reader);

    //     println!("{:?}", data);
    // }

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
            "1000000000000017,WITHDRAWAL,9223372036854775807,0,1800,1633037880000,SUCCESS,\"Record number 18\"",
        );

        assert!(result.is_ok());

        let tx = result.unwrap();

        assert_eq!(
            tx,
            YPBankRecord {
                tx_id: 1000000000000017,
                tx_type: TxType::WITHDRAWAL,
                from_user_id: 9223372036854775807,
                to_user_id: 0,
                amount: 1800,
                timestamp: 1633037880000,
                status: Status::SUCCESS,
                description: "Record number 18".to_string(),
            }
        );

        let result = parse_line(
            "1000000000000017,WITHDRAWAL,9223372036854775807,0,1800,1633037880000,\"Record number 18\"",
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
