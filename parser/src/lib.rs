pub mod csv_format;
use std::io::Read;

enum TX_TYPE {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

enum STATUS {
    SUCCESS,
    FAILURE,
    PENDING,
}
struct Transaction {
    tx_id: u64,
    tx_type: TX_TYPE,
    from_user_id: u64,
    to_user_id: u64,
    timestamp: u64,
    status: STATUS,
    description: String,
}

enum CsvError {
    InvalidLength {
        expected: usize,
        actual: usize,
    },
    WrongColumn {
        index: usize,
        expected: String,
        actual: String,
    },
}
enum ParserError {
    Csv(CsvError),
}

trait LoadData {
    fn load<R: std::io::Read>(reader: R) -> Result<Vec<Transaction>, ParserError>;
}

trait SaveData {
    fn save<W: std::io::Write>(writer: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError>;
}
