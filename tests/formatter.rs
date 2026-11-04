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

fn expand_options() -> Options {
    Options { column_width: 40, inline_table_auto_expand: true, ..Options::default() }
}

fn format_checked(source: &str, options: &Options) -> String {
    let first = format(&parse(source), options.clone());
    assert_eq!(format(&parse(&first), options.clone()), first, "not idempotent: {source:?}");
    assert_eq!(
        toml::from_str::<toml::Value>(&first).unwrap(),
        toml::from_str::<toml::Value>(source).unwrap(),
        "semantics changed: {source:?}"
    );
    first
}

#[test]
fn test_inline_table_auto_expand_basic() {
    assert_eq!(
        format_checked(
            "t = { name = \"alpha\", version = \"1.2.3\", edition = \"2024\" }\n",
            &expand_options()
        ),
        "t = {\n  name = \"alpha\",\n  version = \"1.2.3\",\n  edition = \"2024\"\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_width_boundary() {
    let at_limit = "t = { key = \"aaaaaaaaaaaaaaaaaaaaaaaa\" }\n";
    assert_eq!(at_limit.trim_end().chars().count(), 40);
    assert_eq!(format_checked(at_limit, &expand_options()), at_limit);
    assert_eq!(
        format_checked("t = { key = \"aaaaaaaaaaaaaaaaaaaaaaaaa\" }\n", &expand_options()),
        "t = {\n  key = \"aaaaaaaaaaaaaaaaaaaaaaaaa\"\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_collapses_when_fitting() {
    assert_eq!(
        format_checked("t = {\n  a = 1,\n  b = 2\n}\n", &expand_options()),
        "t = { a = 1, b = 2 }\n"
    );
}

#[test]
fn test_inline_table_auto_expand_outermost_first() {
    assert_eq!(
        format_checked(
            "t = { name = \"aaaaaaaaaaaa\", list = [1, 2], flag = true }\n",
            &expand_options()
        ),
        "t = {\n  name = \"aaaaaaaaaaaa\",\n  list = [1, 2],\n  flag = true\n}\n"
    );
}

const NESTED_TABLE: &str =
    "t = { name = \"a\", deep = { x = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\", y = 1 } }\n";

#[test]
fn test_inline_table_auto_expand_nested_table() {
    assert_eq!(
        format_checked(NESTED_TABLE, &expand_options()),
        "t = {\n  name = \"a\",\n  deep = {\n    x = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\",\n    y = 1\n  }\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_array_of_tables() {
    assert_eq!(
        format_checked(
            "deps = [{ name = \"a\", version = \"1\" }, { name = \"bbbbbbbbbbbbbbbbbbbb\", version = \"2.0.0\" }]\n",
            &expand_options()
        ),
        "deps = [\n  { name = \"a\", version = \"1\" },\n  {\n    name = \"bbbbbbbbbbbbbbbbbbbb\",\n    version = \"2.0.0\"\n  },\n]\n"
    );
}

#[test]
fn test_inline_table_auto_expand_nested_arrays() {
    assert_eq!(
        format_checked(
            "m = [[1, 2], [3, 4], [\"aaaaaaaaaa\", \"bbbbbbbbbb\"]]\n",
            &expand_options()
        ),
        "m = [\n  [1, 2],\n  [3, 4],\n  [\"aaaaaaaaaa\", \"bbbbbbbbbb\"],\n]\n"
    );
}

#[test]
fn test_inline_table_auto_expand_array_in_table() {
    let source = "t = { list = [\"aaaaaaaaaa\", \"bbbbbbbbbb\", \"cccccccccc\"], n = 1 }\n";
    assert_eq!(
        format_checked(source, &expand_options()),
        "t = {\n  list = [\n    \"aaaaaaaaaa\",\n    \"bbbbbbbbbb\",\n    \"cccccccccc\",\n  ],\n  n = 1\n}\n"
    );
    assert_eq!(
        format_checked(source, &Options { array_auto_expand: false, ..expand_options() }),
        "t = {\n  list = [\"aaaaaaaaaa\", \"bbbbbbbbbb\", \"cccccccccc\"],\n  n = 1\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_without_inline_table_expand() {
    assert_eq!(
        format_checked(NESTED_TABLE, &Options { inline_table_expand: false, ..expand_options() }),
        "t = {\n  name = \"a\",\n  deep = { x = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\", y = 1 }\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_counts_trailing_comment() {
    assert_eq!(
        format_checked(
            "t = { a = \"xxxxxxxx\", b = \"yyyy\" } # comment here\n",
            &expand_options()
        ),
        "t = {\n  a = \"xxxxxxxx\",\n  b = \"yyyy\"\n} # comment here\n"
    );
}

#[test]
fn test_inline_table_auto_expand_reorder() {
    assert_eq!(
        format_checked(
            "t = { zeta = \"zzzzzzzzzz\", alpha = \"aaaaaaaaaa\", mid = \"mmmmmmmmmm\" }\n",
            &Options { reorder_inline_tables: true, ..expand_options() }
        ),
        "t = {\n  alpha = \"aaaaaaaaaa\",\n  mid = \"mmmmmmmmmm\",\n  zeta = \"zzzzzzzzzz\"\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_comment_layout_rechecks_entries() {
    assert_eq!(
        format_checked(
            "t = { a = 1, # c\n  b = { x = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\", y = 2 } }\n",
            &expand_options()
        ),
        "t = {\n  a = 1, # c\n  b = {\n    x = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\",\n    y = 2\n  }\n}\n"
    );
}

#[test]
fn test_inline_table_auto_expand_edge_cases() {
    assert_eq!(format_checked("t = {}\n", &expand_options()), "t = {}\n");
    let long = "t = { name = \"alpha\", version = \"1.2.3\", edition = \"2024\" }\n";
    assert_eq!(format_checked(long, &Options { column_width: 40, ..Options::default() }), long);
}
