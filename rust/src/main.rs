fn main() {
    let data = r#"
name = Jan
age = 22

[settings]
dark_mode = true
refresh_rate = 144

[settings.ui]
font_size = 14
theme = dark
languages = [rust, c, python]
"#;

    let parsed = jaml::parse(data).unwrap();
    println!("Parsed:\n{:#?}", parsed);

    let serialized = jaml::serialize(&parsed);
    println!("\nSerialized:\n{}", serialized);
}
