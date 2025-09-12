# JAML – Rust Configuration Library

**JAML** ist eine einfache, verschachtelte Konfigurationsbibliothek für Rust, ähnlich wie TOML oder YAML, aber mit Fokus auf **einfaches Parsen, Ändern und Speichern**.

---

## 📦 Installation

Füge JAML als Dependency in deinem Rust-Projekt hinzu:

```toml
[dependencies]
jaml = { git = "https://github.com/Satisfactorcraft/jaml.git" }
```

# Cargo zieht automatisch die Library vom GitHub-Repo.

🚀 Nutzung
```rs
use jaml::{Jaml, JamlValue};

fn main() {
    // Lade eine bestehende config.jaml oder erstelle eine neue
    let mut cfg = Jaml::load("config.jaml").unwrap_or(Jaml { data: indexmap::IndexMap::new() });

    // Werte setzen (verschachtelte Keys via Punkt-Notation)
    cfg.set_value("settings.ui.theme", JamlValue::String("dark".to_string()));
    cfg.set_value("settings.ui.font_size", JamlValue::Integer(14));
    cfg.set_value("settings.ui.languages", JamlValue::Array(vec![
        JamlValue::String("rust".to_string()),
        JamlValue::String("c".to_string()),
        JamlValue::String("python".to_string()),
    ]));

    // Werte abrufen
    if let Ok(theme) = cfg.get_value("settings.ui.theme") {
        println!("UI Theme: {:?}", theme);
    }

    if let Ok(languages) = cfg.get_value("settings.ui.languages") {
        println!("Programmiersprachen: {:?}", languages);
    }

    // Speichern
    cfg.save("config.jaml").unwrap();
    println!("config.jaml wurde erfolgreich gespeichert!");
}
```
🔹 Vorteile von JAML

Einfaches Parsen & Serialisieren von Konfigurationen

Verschachtelte Sections & Punkt-Notation für Keys

Arrays von Strings, Zahlen oder Booleans unterstützt

Direktes Ändern von Werten via .set_value()

Automatisches Laden & Speichern von .jaml Dateien

Ideal, wenn du schnelle Rust-Konfigurationen ohne komplexes YAML/TOML Setup brauchst

# 📁 Beispiel config.jaml
```
[category]
integer = 5
boolean = true

[category.subcategory]
string = dark
integer = 14
array = [string1, string2, string3]
```