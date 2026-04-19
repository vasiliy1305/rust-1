use crate::error::DescriptionError;

pub(crate) fn parse_description(value: &str) -> Result<String, DescriptionError> {
    let value = value.trim();

    if !value.starts_with('"') || !value.ends_with('"') {
        return Err(DescriptionError::InvalidFormat {
            value: value.to_string(),
        });
    }

    let inner = &value[1..value.len() - 1];

    let mut result = String::new();
    let mut chars = inner.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    return Err(DescriptionError::InvalidEscape {
                        value: format!("\\{}", other),
                    });
                }
                None => {
                    return Err(DescriptionError::InvalidEscape {
                        value: "\\".to_string(),
                    });
                }
            },
            _ => result.push(ch),
        }
    }

    Ok(result)
}

pub(crate) fn format_description(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");

    format!("\"{}\"", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DescriptionError;

    #[test]
    fn test_parse_description_ok() {
        let result = parse_description(r#""hello""#);
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_parse_description_with_escaped_quote() {
        let result = parse_description(r#""hello \"world\"""#);
        assert_eq!(result.unwrap(), r#"hello "world""#);
    }

    #[test]
    fn test_parse_description_with_escaped_backslash() {
        let result = parse_description(r#""path \\ temp""#);
        assert_eq!(result.unwrap(), r#"path \ temp"#);
    }

    #[test]
    fn test_parse_description_invalid_format() {
        let result = parse_description("hello");

        match result {
            Err(DescriptionError::InvalidFormat { value }) => {
                assert_eq!(value, "hello");
            }
            _ => panic!("ожидали DescriptionError::InvalidFormat"),
        }
    }

    #[test]
    fn test_parse_description_invalid_escape() {
        let result = parse_description(r#""hello \n""#);

        match result {
            Err(DescriptionError::InvalidEscape { value }) => {
                assert_eq!(value, r#"\n"#);
            }
            _ => panic!("ожидали DescriptionError::InvalidEscape"),
        }
    }

    #[test]
    fn test_format_description() {
        let result = format_description(r#"hello "world" \ test"#);
        assert_eq!(result, r#""hello \"world\" \\ test""#);
    }
}
