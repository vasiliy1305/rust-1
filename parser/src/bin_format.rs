use std::io::{BufReader, BufWriter, Write};

use crate::error::{BinError, ParserError};
use crate::{FormatReader, FormatWriter, Status, TxType, YPBankRecord};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
const MAGIC_U32: u32 = u32::from_be_bytes(MAGIC);

pub struct YPBankBinFormat;

impl FormatReader for YPBankBinFormat {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<YPBankRecord>, ParserError> {
        let mut reader = BufReader::new(reader);
        let mut result = Vec::new();

        loop {
            let Some(record_size) = try_read_header(&mut reader)? else {
                break;
            };

            result.push(read_body(&mut reader, record_size)?);
        }

        Ok(result)
    }
}

impl FormatWriter for YPBankBinFormat {
    fn save<W: std::io::Write>(writer: &mut W, data: &[YPBankRecord]) -> Result<(), ParserError> {
        let mut writer = BufWriter::new(writer);
        for tx in data {
            save_tx(&mut writer, tx)?;
        }
        Ok(())
    }
}

fn tx_type_to_byte(tx_type: &TxType) -> u8 {
    match tx_type {
        TxType::DEPOSIT => 0,
        TxType::TRANSFER => 1,
        TxType::WITHDRAWAL => 2,
    }
}

fn status_to_byte(status: &Status) -> u8 {
    match status {
        Status::SUCCESS => 0,
        Status::FAILURE => 1,
        Status::PENDING => 2,
    }
}

fn save_tx<W: Write>(writer: &mut W, tx: &YPBankRecord) -> Result<(), ParserError> {
    let desc_byte = tx.description.as_bytes();
    let desc_len = desc_byte.len() as u32;
    let size: u32 = desc_len + // DESCRIPTION
                            8 + // TX_ID
                            1 + // TX_TYPE
                            8+// FROM_USER_ID
                            8+// TO_USER_ID
                            8+// AMOUNT
                            8+// TIMESTAMP
                            1+// STATUS
                            4; // DESC_LEN

    writer.write_all(&MAGIC)?;
    writer.write_all(&size.to_be_bytes())?;
    writer.write_all(&tx.tx_id.to_be_bytes())?;
    writer.write_all(&tx_type_to_byte(&tx.tx_type).to_be_bytes())?;
    writer.write_all(&tx.from_user_id.to_be_bytes())?;
    writer.write_all(&tx.to_user_id.to_be_bytes())?;
    writer.write_all(&tx.amount.to_be_bytes())?;
    writer.write_all(&tx.timestamp.to_be_bytes())?;
    writer.write_all(&status_to_byte(&tx.status).to_be_bytes())?;
    writer.write_all(&desc_len.to_be_bytes())?;
    writer.write_all(desc_byte)?;
    Ok(())
}

fn read_u32<R: std::io::Read>(reader: &mut R) -> Result<u32, std::io::Error> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_be_bytes(buffer))
}

fn read_u64<R: std::io::Read>(reader: &mut R) -> Result<u64, std::io::Error> {
    let mut buffer = [0u8; 8];
    reader.read_exact(&mut buffer)?;
    Ok(u64::from_be_bytes(buffer))
}

fn read_string<R: std::io::Read>(reader: &mut R, size: u32) -> Result<String, ParserError> {
    let mut buffer = vec![0u8; size as usize];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

fn read_u8<R: std::io::Read>(reader: &mut R) -> Result<u8, std::io::Error> {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer)?;
    Ok(u8::from_be_bytes(buffer))
}

fn try_read_header<R: std::io::Read>(reader: &mut R) -> Result<Option<u32>, ParserError> {
    let mut magic_buf = [0u8; 4];

    match reader.read_exact(&mut magic_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            return Ok(None);
        }
        Err(e) => return Err(e.into()),
    }

    let magic = u32::from_be_bytes(magic_buf);
    if magic != MAGIC_U32 {
        return Err(BinError::WrongMagic.into());
    }

    let record_size = read_u32(reader)?;
    Ok(Some(record_size))
}

fn read_body<R: std::io::Read>(
    reader: &mut R,
    record_size: u32,
) -> Result<YPBankRecord, ParserError> {
    let tx_id = read_u64(reader)?;
    let tx_type = match read_u8(reader)? {
        0 => TxType::DEPOSIT,
        1 => TxType::TRANSFER,
        2 => TxType::WITHDRAWAL,
        value => {
            return Err(ParserError::Bin(BinError::WrongTxType { value }));
        }
    };
    let from_user_id = read_u64(reader)?;
    let to_user_id = read_u64(reader)?;
    let amount = read_u64(reader)?;
    let timestamp = read_u64(reader)?;
    let status = match read_u8(reader)? {
        0 => Status::SUCCESS,
        1 => Status::FAILURE,
        2 => Status::PENDING,
        value => {
            return Err(ParserError::Bin(BinError::WrongStatus { value }));
        }
    };
    let desc_len = read_u32(reader)?;

    let expected_size = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_len;

    if record_size != expected_size {
        return Err(BinError::WrongRecordSize {
            expected: expected_size,
            actual: record_size,
        }
        .into());
    }
    let description = read_string(reader, desc_len)?;

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

    fn sample_record() -> YPBankRecord {
        YPBankRecord {
            tx_id: 1001,
            tx_type: TxType::TRANSFER,
            from_user_id: 501,
            to_user_id: 502,
            amount: 15000,
            timestamp: 1672534800000,
            status: Status::FAILURE,
            description: "Payment".to_string(),
        }
    }

    #[test]
    fn test_save_and_load_bin_roundtrip() {
        let data = vec![sample_record()];

        let mut out = Vec::new();
        YPBankBinFormat::save(&mut out, &data).unwrap();

        let loaded = YPBankBinFormat::load(out.as_slice()).unwrap();

        assert_eq!(loaded, data);
    }

    #[test]
    fn test_load_bin_wrong_magic() {
        let bytes = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let result = YPBankBinFormat::load(bytes.as_slice());

        match result {
            Err(ParserError::Bin(BinError::WrongMagic)) => {}
            _ => panic!("ожидали BinError::WrongMagic"),
        }
    }

    #[test]
    fn test_load_bin_wrong_tx_type() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&(46u32).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.push(9);
        bytes.extend_from_slice(&(0u64).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.extend_from_slice(&(100u64).to_be_bytes());
        bytes.extend_from_slice(&(123u64).to_be_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&(0u32).to_be_bytes());

        let result = YPBankBinFormat::load(bytes.as_slice());

        match result {
            Err(ParserError::Bin(BinError::WrongTxType { value })) => {
                assert_eq!(value, 9);
            }
            _ => panic!("ожидали BinError::WrongTxType"),
        }
    }

    #[test]
    fn test_load_bin_wrong_status() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&(46u32).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&(0u64).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.extend_from_slice(&(100u64).to_be_bytes());
        bytes.extend_from_slice(&(123u64).to_be_bytes());
        bytes.push(9);
        bytes.extend_from_slice(&(0u32).to_be_bytes());

        let result = YPBankBinFormat::load(bytes.as_slice());

        match result {
            Err(ParserError::Bin(BinError::WrongStatus { value })) => {
                assert_eq!(value, 9);
            }
            _ => panic!("ожидали BinError::WrongStatus"),
        }
    }

    #[test]
    fn test_load_bin_wrong_record_size() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&(999u32).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&(0u64).to_be_bytes());
        bytes.extend_from_slice(&(1u64).to_be_bytes());
        bytes.extend_from_slice(&(100u64).to_be_bytes());
        bytes.extend_from_slice(&(123u64).to_be_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&(0u32).to_be_bytes());

        let result = YPBankBinFormat::load(bytes.as_slice());

        match result {
            Err(ParserError::Bin(BinError::WrongRecordSize { expected, actual })) => {
                assert_eq!(expected, 46);
                assert_eq!(actual, 999);
            }
            _ => panic!("ожидали BinError::WrongRecordSize"),
        }
    }
}
