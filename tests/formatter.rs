use oxc_toml::{Options, format, parse};

#[test]
fn test_basic_formatting() {
    const SOURCE: &str = "value=1\n[table]\nstring='some string'";
    let formatted = format(&parse(SOURCE), Options::default());

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
    let formatted = format(&parse(SOURCE), Options::default());

    // Should normalize spacing
    assert!(formatted.contains("name = \"test\""));
    assert!(formatted.contains("version = \"1.0.0\""));
    assert!(formatted.contains("foo = \"1.0\""));
    assert!(formatted.contains("bar = \"2.0\""));
}

#[test]
fn test_formatter_preserves_comments() {
    const SOURCE: &str = "# Comment\nvalue=1";
    let formatted = format(&parse(SOURCE), Options::default());

    assert!(formatted.contains("# Comment"));
    assert!(formatted.contains("value = 1"));
}

// https://github.com/oxc-project/oxc/issues/22348
#[test]
fn test_bare_key_starting_with_digit() {
    let cases = [
        "A = \"\"\n1_B = \"\"\n",
        "1B = \"\"\n",
        "1-B = \"\"\n",
        "34-11 = 23\n",
        "10e3 = \"false float\"\n",
    ];
    for input in cases {
        let formatted = format(&parse(input), Options::default());
        assert_eq!(formatted, input, "input: {input:?}");
    }
}

#[test]
fn test_unknown_escape_with_multibyte_char() {
    assert!(!parse("a = \"\\é\"\n").errors.is_empty());
}

#[test]
fn test_inline_table_with_comment() {
    let format = |source: &str| format(&parse(source), Options::default());

    // A comment would swallow the rest of a joined line, so one entry per line
    assert_eq!(format("t = { a = 1 # c\n, b = 2 }\n"), "t = {\n  a = 1, # c\n  b = 2\n}\n");
    // A nested array keeps its comment itself, the table stays valid TOML 1.0
    assert_eq!(format("a = { b = [ 1, # c\n 2 ] }\n"), "a = { b = [\n  1, # c\n  2,\n] }\n");
}
