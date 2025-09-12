use std::fs;
use std::io::Write;
use indexmap::IndexMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JamlError {
    #[error("Syntaxfehler in Zeile {0}")]
    Syntax(usize),
    #[error("I/O Fehler: {0}")]
    Io(String),
    #[error("Key '{0}' nicht gefunden")]
    KeyNotFound(String),
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
            JamlValue::Section(s) => s,
            _ => panic!("Value is not a Section"),
        }
    }
}

/// Hauptstruktur für JAML-Daten
#[derive(Debug, Clone)]
pub struct Jaml {
    pub data: IndexMap<String, JamlValue>,
}

impl Jaml {
    /// Datei laden
    pub fn load(path: &str) -> Result<Self, JamlError> {
        let content = fs::read_to_string(path)
            .map_err(|e| JamlError::Io(format!("Fehler beim Lesen: {}", e)))?;
        Ok(Jaml { data: parse(&content)? })
    }

    /// Datei speichern
    pub fn save(&self, path: &str) -> Result<(), JamlError> {
        let content = serialize(&self.data);
        let mut file = fs::File::create(path)
            .map_err(|e| JamlError::Io(format!("Fehler beim Erstellen: {}", e)))?;
        file.write_all(content.as_bytes())
            .map_err(|e| JamlError::Io(format!("Fehler beim Schreiben: {}", e)))
    }

    /// Setzt einen Wert (nested via "settings.ui.theme")
    pub fn set_value(&mut self, path: &str, value: JamlValue) {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &mut self.data;
        for i in 0..parts.len() {
            let key = parts[i].to_string();
            if i == parts.len() - 1 {
                current.insert(key, value.clone());
            } else {
                current = current.entry(key)
                    .or_insert_with(|| JamlValue::Section(IndexMap::new()))
                    .as_section_mut();
            }
        }
    }

    /// Holt einen Wert (nested via "settings.ui.theme")
    pub fn get_value(&self, path: &str) -> Result<&JamlValue, JamlError> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.data;
        for i in 0..parts.len() {
            let key = parts[i];
            match current.get(key) {
                Some(JamlValue::Section(s)) if i < parts.len() - 1 => current = s,
                Some(v) if i == parts.len() - 1 => return Ok(v),
                _ => return Err(JamlError::KeyNotFound(path.to_string())),
            }
        }
        Err(JamlError::KeyNotFound(path.to_string()))
    }
}

/// --- Öffentliche Parser / Serializer ---
pub fn parse(input: &str) -> Result<IndexMap<String, JamlValue>, JamlError> {
    let mut result = IndexMap::new();
    let mut current_section: Option<Vec<String>> = None;

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        if line.starts_with('[') && line.ends_with(']') {
            let path: Vec<String> = line[1..line.len()-1].split('.').map(|s| s.to_string()).collect();
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

pub fn serialize(map: &IndexMap<String, JamlValue>) -> String {
    fn ser_inner(map: &IndexMap<String, JamlValue>, prefix: Option<String>, out: &mut String) {
        for (k, v) in map {
            if !matches!(v, JamlValue::Section(_)) {
                let val = match v {
                    JamlValue::String(s) => s.clone(),
                    JamlValue::Integer(n) => n.to_string(),
                    JamlValue::Boolean(b) => b.to_string(),
                    JamlValue::Array(arr) => {
                        let items: Vec<String> = arr.iter().map(|v| match v {
                            JamlValue::String(s) => s.clone(),
                            JamlValue::Integer(n) => n.to_string(),
                            JamlValue::Boolean(b) => b.to_string(),
                            _ => panic!("Nested Arrays/Sections nicht erlaubt"),
                        }).collect();
                        format!("[{}]", items.join(", "))
                    }
                    _ => panic!("Unbekannter Typ"),
                };
                out.push_str(&format!("{} = {}\n", k, val));
            }
        }
        for (k, v) in map {
            if let JamlValue::Section(s) = v {
                let section_name = match &prefix { Some(p) => format!("{}.{}", p, k), None => k.clone() };
                out.push_str(&format!("\n[{}]\n", section_name));
                ser_inner(s, Some(section_name), out);
            }
        }
    }
    let mut out = String::new();
    ser_inner(map, None, &mut out);
    out
}

/// --- Interne Hilfsfunktionen ---
fn insert_nested_section<'a>(map: &'a mut IndexMap<String, JamlValue>, path: &[String]) -> &'a mut IndexMap<String, JamlValue> {
    let mut current = map;
    for key in path {
        current = current.entry(key.clone())
            .or_insert_with(|| JamlValue::Section(IndexMap::new()))
            .as_section_mut();
    }
    current
}

fn get_nested_section_mut<'a>(map: &'a mut IndexMap<String, JamlValue>, path: &[String]) -> &'a mut IndexMap<String, JamlValue> {
    let mut current = map;
    for key in path {
        current = match current.get_mut(key) {
            Some(JamlValue::Section(s)) => s,
            _ => panic!("Section {} existiert nicht", key),
        };
    }
    current
}

fn parse_value(raw: &str) -> Result<JamlValue, JamlError> {
    if raw == "true" { Ok(JamlValue::Boolean(true)) }
    else if raw == "false" { Ok(JamlValue::Boolean(false)) }
    else if let Ok(n) = raw.parse::<i64>() { Ok(JamlValue::Integer(n)) }
    else if raw.starts_with('[') && raw.ends_with(']') {
        let inner = &raw[1..raw.len()-1];
        let elements = inner.split(',').map(|s| parse_value(s.trim())).collect::<Result<Vec<_>, _>>()?;
        Ok(JamlValue::Array(elements))
    } else { Ok(JamlValue::String(raw.to_string())) }
}
