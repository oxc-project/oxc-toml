use logos::{Lexer, Logos};

/// Escaping based on:
///
/// \b         - backspace       (U+0008)
/// \t         - tab             (U+0009)
/// \n         - linefeed        (U+000A)
/// \f         - form feed       (U+000C)
/// \r         - carriage return (U+000D)
/// \"         - quote           (U+0022)
/// \\         - backslash       (U+005C)
/// \uXXXX     - unicode         (U+XXXX)
/// \UXXXXXXXX - unicode         (U+XXXXXXXX)
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Escape {
    #[token(r#"\b"#, priority = 5)]
    Backspace,

    #[token(r#"\t"#, priority = 5)]
    Tab,

    #[regex(r#"(\\\s*\n)|(\\\s*\r\n)"#, priority = 5)]
    Newline,

    #[token(r#"\n"#, priority = 5)]
    LineFeed,

    #[token(r#"\f"#, priority = 5)]
    FormFeed,

    #[token(r#"\r"#, priority = 5)]
    CarriageReturn,

    #[token(r#"\""#, priority = 5)]
    Quote,

    #[token(r#"\\"#, priority = 5)]
    Backslash,

    // Same thing repeated 4 times, but the {n} repetition syntax is not supported by Logos
    #[regex(r#"\\u[0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_]"#, priority = 5)]
    Unicode,

    // Same thing repeated 8 times, but the {n} repetition syntax is not supported by Logos
    #[regex(r#"\\U[0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_]"#, priority = 5)]
    UnicodeLarge,

    #[regex(r#"\\."#, priority = 4)]
    Unknown,

    UnEscaped,
}
use Escape::*;

/// Escape values in a given string.
pub fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '\u{0008}' => escaped.push_str(r#"\b"#),
            '\u{0009}' => escaped.push_str(r#"\t"#),
            '\u{000A}' => escaped.push_str(r#"\n"#),
            '\u{000C}' => escaped.push_str(r#"\f"#),
            '\u{000D}' => escaped.push_str(r#"\r"#),
            '\u{0022}' => escaped.push_str(r#"\""#),
            '\u{005C}' => escaped.push_str(r#"\\"#),
            _ => {
                escaped.push(c);
            }
        }
    }

    escaped
}

/// Unescape all supported escape sequences.
///
/// If it fails, the index of failure is returned.
pub fn unescape(s: &str) -> Result<String, usize> {
    let mut new_s = String::with_capacity(s.len());
    let mut lexer: Lexer<Escape> = Lexer::new(s);

    while let Some(t) = lexer.next() {
        let t = t.unwrap_or(UnEscaped);
        match t {
            Backspace => new_s += "\u{0008}",
            Tab => new_s += "\u{0009}",
            LineFeed => new_s += "\u{000A}",
            FormFeed => new_s += "\u{000C}",
            CarriageReturn => new_s += "\u{000D}",
            Quote => new_s += "\u{0022}",
            Backslash => new_s += "\u{005C}",
            Newline => {}
            Unicode => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            UnicodeLarge => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            Unknown => return Err(lexer.span().end),
            UnEscaped => {
                new_s += lexer.slice();
            }
        }
    }

    Ok(new_s + lexer.remainder())
}

/// Same as unescape, but doesn't create a new
/// unescaped string, and returns all invalid escape indices.
pub fn check_escape(s: &str) -> Result<(), Vec<usize>> {
    let mut lexer: Lexer<Escape> = Lexer::new(s);
    let mut invalid = Vec::new();

    while let Some(t) = lexer.next() {
        let t = t.unwrap_or(UnEscaped);
        match t {
            Backspace => {}
            Tab => {}
            LineFeed => {}
            FormFeed => {}
            CarriageReturn => {}
            Quote => {}
            Backslash => {}
            Newline => {}
            Unicode => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            UnicodeLarge => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            Unknown => invalid.push(lexer.span().start),
            UnEscaped => {}
        }
    }

    if invalid.is_empty() {
        Ok(())
    } else {
        Err(invalid)
    }
}
