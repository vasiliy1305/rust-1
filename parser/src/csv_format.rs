// csv_format.rs — логика и типы для csv формата.
#[derive(Debug, PartialEq)]
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

struct CsvFormat;

fn check_header(header: &str) -> Result<(), CsvError> {
    let expected_header = [
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_header() {
        // полностью коректный заголовок
        assert_eq!(
            check_header(
                "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
            ),
            Ok(())
        );

        //проверка что пробелы не влияют, обычно это не ошибка
        assert_eq!(
            check_header(
                "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID, AMOUNT,TIMESTAMP ,STATUS,  DESCRIPTION"
            ),
            Ok(())
        );

        // не коректная дли на заголовка - можно поменять на спец ошибку #todo
        assert_eq!(
            check_header("TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS"),
            Err(CsvError::InvalidLength {
                expected: 8,
                actual: 7
            })
        );

        // не коректные имена #todo
        assert_eq!(
            check_header(
                "TX_ID,qwerty,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
            ),
            Err(CsvError::WrongColumn {
                index: 1,
                expected: "TX_TYPE".to_string(),
                actual: "qwerty".to_string()
            })
        );
    }
}
