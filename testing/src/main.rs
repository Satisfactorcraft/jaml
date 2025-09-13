use jaml::{Jaml, JamlValue};

fn main() {
    // Versuche, config.jaml zu laden. Wenn nicht vorhanden, erstelle leere Struktur.
    let mut cfg = Jaml::load("config.jaml").unwrap_or(Jaml { data: indexmap::IndexMap::new() });

    // Beispiel: Wert setzen
    cfg.set_value("settings.ui.theme", JamlValue::String("dark".to_string()));
    cfg.set_value("settings.ui.font_size", JamlValue::Integer(14));
    cfg.set_value("settings.ui.languages", JamlValue::Array(vec![
        JamlValue::String("rust".to_string()),
        JamlValue::String("c++".to_string()),
        JamlValue::String("python".to_string()),
    ]));

    cfg.set_value("settings.refresh_rate", JamlValue::Integer(144));
    cfg.set_value("settings.dark_mode", JamlValue::Boolean(true));

    // Beispiel: Wert abfragen
    if let Ok(theme) = cfg.get_value("settings.ui.theme") {
        println!("UI Theme: {:?}", theme);
    }

    if let Ok(languages) = cfg.get_value("settings.ui.languages") {
        println!("Programmiersprachen: {:?}", languages);
    }

    // Speichern
    if let Err(e) = cfg.save("config.jaml") {
        eprintln!("Fehler beim Speichern: {}", e);
    } else {
        println!("config.jaml wurde erfolgreich gespeichert!");
    }

    // Optional: komplette Struktur anzeigen
    println!("\n=== Aktuelle Konfiguration ===\n{:#?}", cfg.data);
}
