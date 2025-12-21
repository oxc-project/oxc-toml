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
