use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JamlError {
    #[error("Syntaxfehler in Zeile {0}")]
    Syntax(usize),
    #[error("Unerwartetes Ende der Eingabe")]
    UnexpectedEOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JamlValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<JamlValue>),
    Section(HashMap<String, JamlValue>),
}

pub fn parse(input: &str) -> Result<HashMap<String, JamlValue>, JamlError> {
    let mut result = HashMap::new();
    let mut current_section: Option<String> = None;

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            // Neue Section
            let section_name = &line[1..line.len()-1];
            current_section = Some(section_name.to_string());
            result
                .entry(section_name.to_string())
                .or_insert(JamlValue::Section(HashMap::new()));
        } else if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = parse_value(value.trim())?;
            if let Some(section) = &current_section {
                if let Some(JamlValue::Section(map)) = result.get_mut(section) {
                    map.insert(key, value);
                }
            } else {
                result.insert(key, value);
            }
        } else {
            return Err(JamlError::Syntax(line_no + 1));
        }
    }

    Ok(result)
}

fn parse_value(raw: &str) -> Result<JamlValue, JamlError> {
    if raw == "true" {
        return Ok(JamlValue::Boolean(true));
    }
    if raw == "false" {
        return Ok(JamlValue::Boolean(false));
    }
    if let Ok(num) = raw.parse::<i64>() {
        return Ok(JamlValue::Integer(num));
    }
    if raw.starts_with('[') && raw.ends_with(']') {
        let inner = &raw[1..raw.len()-1];
        let elements = inner
            .split(',')
            .map(|s| parse_value(s.trim()))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(JamlValue::Array(elements));
    }
    Ok(JamlValue::String(raw.to_string()))
}
