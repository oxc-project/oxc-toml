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
            let bytes = rest.as_bytes();
            let ws_len = bytes.iter().take_while(|&&b| b == b' ' || b == b'\t').count();
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
            let hex_bytes = &input.as_bytes()[2..6];
            if hex_bytes.iter().all(|&b| b.is_ascii_hexdigit() || b == b'_') {
                return Some((Unicode, 6));
            }
        }

        // Unicode escape \UXXXXXXXX
        if input.starts_with("\\U") && input.len() >= 10 {
            let hex_bytes = &input.as_bytes()[2..10];
            if hex_bytes.iter().all(|&b| b.is_ascii_hexdigit() || b == b'_') {
                return Some((UnicodeLarge, 10));
            }
        }

        // Unknown escape sequence
        if input.starts_with('\\') && input.len() >= 2 {
            return Some((Unknown, 2));
        }

        // Unescaped character - need to handle UTF-8 here
        let bytes = input.as_bytes();
        if let Some(&first_byte) = bytes.first() {
            // Determine UTF-8 sequence length from first byte
            let len = if first_byte < 0x80 {
                1 // ASCII
            } else if first_byte < 0xE0 {
                2 // 2-byte sequence
            } else if first_byte < 0xF0 {
                3 // 3-byte sequence
            } else {
                4 // 4-byte sequence
            };
            return Some((UnEscaped, len));
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
