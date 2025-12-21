use oxc_toml::formatter::{format, Options};

#[test]
fn test_basic_formatting() {
    const SOURCE: &str = "value=1\n[table]\nstring='some string'";
    let formatted = format(SOURCE, Options::default());
    
    // Should add spaces around =
    assert!(formatted.contains("value = 1"));
    assert!(formatted.contains("string = 'some string'"));
}

#[test]
fn test_complex_toml() {
    const SOURCE: &str = r#"
[package]
name="test"
version="1.0.0"

[dependencies]
foo="1.0"
bar  =   "2.0"
"#;
    let formatted = format(SOURCE, Options::default());
    
    // Should normalize spacing
    assert!(formatted.contains("name = \"test\""));
    assert!(formatted.contains("version = \"1.0.0\""));
    assert!(formatted.contains("foo = \"1.0\""));
    assert!(formatted.contains("bar = \"2.0\""));
}

#[test]
fn test_formatter_preserves_comments() {
    const SOURCE: &str = "# Comment\nvalue=1";
    let formatted = format(SOURCE, Options::default());
    
    assert!(formatted.contains("# Comment"));
    assert!(formatted.contains("value = 1"));
}
