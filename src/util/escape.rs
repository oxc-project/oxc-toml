use crate::lexer::{Lexer, LexerToken};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Escape {
    Backspace,
    Tab,
    Newline,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Quote,
    Backslash,
    Unicode,
    UnicodeLarge,
    Unknown,
    UnEscaped,
}
use Escape::*;

impl<'source> LexerToken<'source> for Escape {
    fn lex(input: &'source str) -> Option<(Self, usize)> {
        if input.starts_with("\\b") {
            return Some((Backspace, 2));
        }
        if input.starts_with("\\t") {
            return Some((Tab, 2));
        }
        if input.starts_with("\\n") {
            return Some((LineFeed, 2));
        }
        if input.starts_with("\\f") {
            return Some((FormFeed, 2));
        }
        if input.starts_with("\\r") {
            return Some((CarriageReturn, 2));
        }
        if input.starts_with("\\\"") {
            return Some((Quote, 2));
        }
        if input.starts_with("\\\\") {
            return Some((Backslash, 2));
        }

        // Newline escape: backslash followed by optional whitespace and newline
        if let Some(rest) = input.strip_prefix('\\') {
            let ws_len: usize =
                rest.chars().take_while(|&c| c == ' ' || c == '\t').map(|c| c.len_utf8()).sum();
            let after_ws = &rest[ws_len..];
            if after_ws.starts_with('\n') {
                return Some((Newline, 1 + ws_len + 1));
            }
            if after_ws.starts_with("\r\n") {
                return Some((Newline, 1 + ws_len + 2));
            }
        }

        // Unicode escape \uXXXX
        if input.starts_with("\\u") && input.len() >= 6 {
            let hex_chars = &input[2..6];
            if hex_chars.chars().all(|c| c.is_ascii_hexdigit() || c == '_') {
                return Some((Unicode, 6));
            }
        }

        // Unicode escape \UXXXXXXXX
        if input.starts_with("\\U") && input.len() >= 10 {
            let hex_chars = &input[2..10];
            if hex_chars.chars().all(|c| c.is_ascii_hexdigit() || c == '_') {
                return Some((UnicodeLarge, 10));
            }
        }

        // Unknown escape sequence
        if input.starts_with('\\') && input.len() >= 2 {
            return Some((Unknown, 2));
        }

        // Unescaped character
        if let Some(c) = input.chars().next() {
            return Some((UnEscaped, c.len_utf8()));
        }

        None
    }
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

    if invalid.is_empty() { Ok(()) } else { Err(invalid) }
}
