use std::io::{BufRead, BufReader, Read};

use crate::error::{ParserError, BinError};
use crate::{LoadData, SaveData, Status, Transaction, TxType};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
const MAGIC_U32: u32 = u32::from_be_bytes(MAGIC);

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

fn read_string<R: std::io::Read>(reader: &mut R, size: usize) -> Result<String, ParserError> {
let mut buffer = vec![0u8; size];
reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

fn read_u8<R: std::io::Read>(reader: &mut R) -> Result<u8, std::io::Error> {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer)?;
    Ok(u8::from_be_bytes(buffer))
}

fn read_header<R: std::io::Read>(reader: &mut R) ->Result<u32, BinError>
{
    let magic = read_u32(reader)?;
    if magic != MAGIC_U32 {
        Err(BinError::WrongMagic)
    }else {
        Ok(read_u32(reader)?)
    }
}

fn read_body<R: std::io::Read>(reader: &mut R) ->Result<Transaction, BinError>
{
    let tx_id = read_u64(reader)?;
    let tx_type = match read_u8(reader)? {
        0 => {TxType::DEPOSIT},
        1 => {TxType::TRANSFER},
        2 => {TxType::WITHDRAWAL},
        value => {return Err(BinError::WrongTxType { value: value });}
    };
    let from_user_id = read_u64(reader)?;
    let to_user_id = read_u64(reader)?;
    let amount = read_u64(reader)?;
    let timestamp = read_u64(reader)?;
    let status = match read_u8(reader)? {
        0 => {Status::SUCCESS},
        1 => {Status::FAILURE},
        2 => {Status::PENDING},
        value => {return Err(BinError::WrongStatus  { value: value });}
    };
    let desk_len = read_u32(reader)?;
}


// | `TX_ID` | 8 байт | беззнаковое 64-битное | Уникальный идентификатор транзакции. |
// | `TX_TYPE` | 1 байт | перечисление (0 = DEPOSIT, 1 = TRANSFER, 2 = WITHDRAWAL) | |
// | `FROM_USER_ID` | 8 байт | беззнаковое 64-битное | Счёт отправителя; `0` для DEPOSIT. |
// | `TO_USER_ID` | 8 байт | беззнаковое 64-битное | Счёт получателя; `0` для WITHDRAWAL. |
// | `AMOUNT` | 8 байт | знаковое 64-битное | Сумма в наименьшей денежной единице (центах). Положительное значение для зачислений, отрицательное для списаний. |
// | `TIMESTAMP` | 8 байт | беззнаковое 64-битное | Время выполнения транзакции в миллисекундах от эпохи Unix. |
// | `STATUS` | 1 байт | перечисление (0 = SUCCESS, 1 = FAILURE, 2 = PENDING) | |
// | `DESC_LEN` | 4 байта | беззнаковое 32-битное | Длина следующего описания в кодировке UTF-8. |
// | `DESCRIPTION` | `DESC_LEN` байт | UTF-8 | Необязательное текстовое описание. Если описание отсутствует, `DESC_LEN` равен `0`. |