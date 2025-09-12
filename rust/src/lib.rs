use indexmap::IndexMap;
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
    Section(IndexMap<String, JamlValue>),
}

impl JamlValue {
    fn as_section_mut(&mut self) -> &mut IndexMap<String, JamlValue> {
        match self {
            JamlValue::Section(inner) => inner,
            _ => panic!("Value is not a Section"),
        }
    }
}

pub fn parse(input: &str) -> Result<IndexMap<String, JamlValue>, JamlError> {
    let mut result = IndexMap::new();
    let mut current_section: Option<Vec<String>> = None;

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            let section_name = &line[1..line.len()-1];
            let path: Vec<String> = section_name.split('.').map(|s| s.to_string()).collect();
            current_section = Some(path.clone());
            insert_nested_section(&mut result, &path);
        } else if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = parse_value(value.trim())?;
            if let Some(section) = &current_section {
                let section_map = get_nested_section_mut(&mut result, section);
                section_map.insert(key, value);
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

// Section anlegen (inkl. nested)
fn insert_nested_section<'a>(
    map: &'a mut IndexMap<String, JamlValue>,
    path: &[String],
) -> &'a mut IndexMap<String, JamlValue> {
    let mut current = map;
    for key in path {
        current = current
            .entry(key.clone())
            .or_insert_with(|| JamlValue::Section(IndexMap::new()))
            .as_section_mut();
    }
    current
}

// Mutable Referenz auf nested Section
fn get_nested_section_mut<'a>(
    map: &'a mut IndexMap<String, JamlValue>,
    path: &[String],
) -> &'a mut IndexMap<String, JamlValue> {
    let mut current = map;
    for key in path {
        current = match current.get_mut(key) {
            Some(JamlValue::Section(inner)) => inner,
            _ => panic!("Section {} existiert nicht", key),
        };
    }
    current
}

// Serializer: Rust-Struktur → JAML-String
pub fn serialize(map: &IndexMap<String, JamlValue>) -> String {
    fn serialize_inner(
        map: &IndexMap<String, JamlValue>,
        prefix: Option<String>,
        out: &mut String,
    ) {
        // Zuerst direkte Keys (nicht Sections)
        for (key, value) in map {
            if !matches!(value, JamlValue::Section(_)) {
                let val_str = match value {
                    JamlValue::String(s) => s.clone(),
                    JamlValue::Integer(n) => n.to_string(),
                    JamlValue::Boolean(b) => b.to_string(),
                    JamlValue::Array(arr) => {
                        let items: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                JamlValue::String(s) => s.clone(),
                                JamlValue::Integer(n) => n.to_string(),
                                JamlValue::Boolean(b) => b.to_string(),
                                _ => panic!("Verschachtelte Arrays nicht unterstützt"),
                            })
                            .collect();
                        format!("[{}]", items.join(", "))
                    }
                    _ => panic!("Unbekannter Typ"),
                };
                out.push_str(&format!("{} = {}\n", key, val_str));
            }
        }

        // Dann Sections
        for (key, value) in map {
            if let JamlValue::Section(inner) = value {
                let section_name = match &prefix {
                    Some(p) => format!("{}.{}", p, key),
                    None => key.clone(),
                };
                out.push_str(&format!("\n[{}]\n", section_name));
                serialize_inner(inner, Some(section_name), out);
            }
        }
    }

    let mut out = String::new();
    serialize_inner(map, None, &mut out);
    out
}
