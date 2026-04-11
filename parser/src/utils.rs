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
