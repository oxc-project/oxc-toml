//! Declaration of the syntax tokens and lexer implementation.

#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

use crate::lexer::LexerToken;

/// Enum containing all the tokens in a syntax tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE = 0,
    NEWLINE,
    COMMENT,
    IDENT,
    /// Not part of the regular TOML syntax, only used to allow
    /// glob patterns in keys.
    IDENT_WITH_GLOB,
    PERIOD,
    COMMA,
    EQ,
    STRING,
    MULTI_LINE_STRING,
    STRING_LITERAL,
    MULTI_LINE_STRING_LITERAL,
    INTEGER,
    INTEGER_HEX,
    INTEGER_OCT,
    INTEGER_BIN,
    FLOAT,
    BOOL,
    DATE_TIME_OFFSET,
    DATE_TIME_LOCAL,
    DATE,
    TIME,
    BRACKET_START,
    BRACKET_END,
    BRACE_START,
    BRACE_END,
    ERROR,

    // composite types
    KEY,                // e.g.: parent.child
    VALUE,              // e.g.: "2"
    TABLE_HEADER,       // e.g.: [table]
    TABLE_ARRAY_HEADER, // e.g.: [[table]]
    ENTRY,              // e.g.: key = "value"
    ARRAY,              // e.g.: [ 1, 2 ]
    INLINE_TABLE,       // e.g.: { key = "value" }

    ROOT, // root node
}

// Type aliases for tree types
pub use crate::tree::{Element as SyntaxElement, Node as SyntaxNode, Token as SyntaxToken};

// Helper functions for lexing
fn lex_string(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut escaped = false;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if b == b'\\' {
            escaped = !escaped;
            i += 1;
            continue;
        }

        if b == b'"' && !escaped {
            return Some(i + 1);
        }

        escaped = false;
        i += 1;
    }
    None
}

fn lex_multi_line_string(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut quote_count = 0;
    let mut escaped = false;

    // As the string can contain ",
    // we can end up with more than 3 "-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    while i < bytes.len() {
        let b = bytes[i];

        if quotes_found {
            if b != b'"' {
                if quote_count >= 6 {
                    return None;
                }
                return Some(i);
            } else {
                quote_count += 1;
                i += 1;
                continue;
            }
        }
        i += 1;

        if b == b'\\' {
            escaped = true;
            continue;
        }

        if b == b'"' && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }

        escaped = false;
    }

    // End of input
    if quotes_found {
        if quote_count >= 6 {
            return None;
        }
        Some(i)
    } else {
        None
    }
}

fn lex_string_literal(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'\'' {
            return Some(i + 1);
        }
    }
    None
}

fn lex_multi_line_string_literal(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut quote_count = 0;

    // As the string can contain ',
    // we can end up with more than 3 '-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    while i < bytes.len() {
        let b = bytes[i];

        if quotes_found {
            if b != b'\'' {
                return Some(i);
            } else {
                if quote_count > 4 {
                    return None;
                }

                quote_count += 1;
                i += 1;
                continue;
            }
        }
        i += 1;

        if b == b'\'' {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }
    }

    // End of input
    if quotes_found { Some(i) } else { None }
}

// Helper functions for matching patterns
fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

fn is_ident_with_glob_char(b: u8) -> bool {
    is_ident_char(b) || b == b'*' || b == b'?'
}

fn is_hex_digit(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

// Lexer implementation for SyntaxKind
impl<'source> LexerToken<'source> for SyntaxKind {
    fn lex(input: &'source str) -> Option<(Self, usize)> {
        let bytes = input.as_bytes();
        let first = *bytes.first()?;

        // Try to match tokens in order of priority

        // Single character tokens
        match first {
            b'.' => return Some((SyntaxKind::PERIOD, 1)),
            b',' => return Some((SyntaxKind::COMMA, 1)),
            b'=' => return Some((SyntaxKind::EQ, 1)),
            b'[' => return Some((SyntaxKind::BRACKET_START, 1)),
            b']' => return Some((SyntaxKind::BRACKET_END, 1)),
            b'{' => return Some((SyntaxKind::BRACE_START, 1)),
            b'}' => return Some((SyntaxKind::BRACE_END, 1)),
            _ => {}
        }

        // Whitespace
        if is_whitespace(first) {
            let len = bytes.iter().take_while(|&&b| is_whitespace(b)).count();
            return Some((SyntaxKind::WHITESPACE, len));
        }

        // Newline
        if first == b'\n' {
            let len = bytes.iter().take_while(|&&b| b == b'\n').count();
            return Some((SyntaxKind::NEWLINE, len));
        }
        if first == b'\r' && bytes.len() >= 2 && bytes[1] == b'\n' {
            let mut len = 0;
            let mut i = 0;
            while i + 1 < bytes.len() && bytes[i] == b'\r' && bytes[i + 1] == b'\n' {
                len += 2;
                i += 2;
            }
            if len > 0 {
                return Some((SyntaxKind::NEWLINE, len));
            }
        }

        // Comment
        if first == b'#' {
            let len = bytes.iter().take_while(|&&b| b != b'\n' && b != b'\r').count();
            return Some((SyntaxKind::COMMENT, len));
        }

        // Multi-line strings (must check before single quote/double quote)
        if bytes.len() >= 3
            && &bytes[..3] == b"\"\"\""
            && let Some(len) = lex_multi_line_string(&input[3..])
        {
            return Some((SyntaxKind::MULTI_LINE_STRING, 3 + len));
        }
        if bytes.len() >= 3
            && &bytes[..3] == b"'''"
            && let Some(len) = lex_multi_line_string_literal(&input[3..])
        {
            return Some((SyntaxKind::MULTI_LINE_STRING_LITERAL, 3 + len));
        }

        // String
        if first == b'"'
            && let Some(len) = lex_string(&input[1..])
        {
            return Some((SyntaxKind::STRING, 1 + len));
        }

        // String literal
        if first == b'\''
            && let Some(len) = lex_string_literal(&input[1..])
        {
            return Some((SyntaxKind::STRING_LITERAL, 1 + len));
        }

        // Boolean
        if input.starts_with("true") {
            return Some((SyntaxKind::BOOL, 4));
        }
        if input.starts_with("false") {
            return Some((SyntaxKind::BOOL, 5));
        }

        // Numbers and dates (complex matching)
        if first.is_ascii_digit() || first == b'+' || first == b'-' {
            // Try date/time first (they are more specific)
            if let Some(len) = try_lex_datetime(input) {
                return Some(len);
            }

            // Try float keywords
            if input.starts_with("nan") || input.starts_with("+nan") || input.starts_with("-nan") {
                let len = if first == b'+' || first == b'-' { 4 } else { 3 };
                return Some((SyntaxKind::FLOAT, len));
            }
            if input.starts_with("inf") || input.starts_with("+inf") || input.starts_with("-inf") {
                let len = if first == b'+' || first == b'-' { 4 } else { 3 };
                return Some((SyntaxKind::FLOAT, len));
            }

            // Try integers with different bases
            if bytes.len() >= 2 && bytes[0] == b'0' && bytes[1] == b'x' {
                let len =
                    2 + bytes[2..].iter().take_while(|&&b| is_hex_digit(b) || b == b'_').count();
                if len > 2 {
                    return Some((SyntaxKind::INTEGER_HEX, len));
                }
            }
            if bytes.len() >= 2 && bytes[0] == b'0' && bytes[1] == b'o' {
                let len = 2 + bytes[2..]
                    .iter()
                    .take_while(|&&b| (b'0'..=b'7').contains(&b) || b == b'_')
                    .count();
                if len > 2 {
                    return Some((SyntaxKind::INTEGER_OCT, len));
                }
            }
            if bytes.len() >= 2 && bytes[0] == b'0' && bytes[1] == b'b' {
                let len = 2 + bytes[2..]
                    .iter()
                    .take_while(|&&b| b == b'0' || b == b'1' || b == b'_')
                    .count();
                if len > 2 {
                    return Some((SyntaxKind::INTEGER_BIN, len));
                }
            }

            // Try float or integer
            if let Some((kind, len)) = try_lex_number(input) {
                return Some((kind, len));
            }
        }

        // Identifier (lower priority than keywords)
        if first.is_ascii_alphanumeric() || first == b'_' || first == b'-' {
            let len = bytes.iter().take_while(|&&b| is_ident_char(b)).count();
            return Some((SyntaxKind::IDENT, len));
        }

        // Identifier with glob
        if first == b'*' || first == b'?' {
            let len = bytes.iter().take_while(|&&b| is_ident_with_glob_char(b)).count();
            return Some((SyntaxKind::IDENT_WITH_GLOB, len));
        }

        None
    }
}

fn try_lex_number(input: &str) -> Option<(SyntaxKind, usize)> {
    let mut i = 0;
    let bytes = input.as_bytes();

    // Optional sign
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }

    // Integer part
    let start = i;
    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
        i += 1;
    }
    if i == start {
        return None;
    }

    // Check for float indicators
    let mut is_float = false;

    // Decimal point
    if i < bytes.len() && bytes[i] == b'.' {
        // Make sure it's not just a trailing period
        if i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            is_float = true;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
                i += 1;
            }
        }
    }

    // Exponent
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        is_float = true;
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let exp_start = i;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
            i += 1;
        }
        // Must have at least one digit in exponent
        if i == exp_start {
            return None;
        }
    }

    if i > 0 {
        Some((if is_float { SyntaxKind::FLOAT } else { SyntaxKind::INTEGER }, i))
    } else {
        None
    }
}

fn try_lex_datetime(input: &str) -> Option<(SyntaxKind, usize)> {
    // Simplified datetime matching - try to match date/time patterns
    // This is a simplified version - the full regex patterns are complex

    // Try to match time first (HH:MM:SS)
    if let Some(len) = try_match_time(input) {
        return Some((SyntaxKind::TIME, len));
    }

    // Try to match date (YYYY-MM-DD)
    if let Some(date_len) = try_match_date(input) {
        // Check if followed by time
        if date_len < input.len() {
            let rest = &input[date_len..];
            if (rest.starts_with('T') || rest.starts_with('t') || rest.starts_with(' '))
                && let Some(time_len) = try_match_time(&rest[1..])
            {
                let total = date_len + 1 + time_len;
                // Check for timezone offset
                if total < input.len() {
                    let tz_rest = &input[total..];
                    if tz_rest.starts_with('Z') || tz_rest.starts_with('z') {
                        return Some((SyntaxKind::DATE_TIME_OFFSET, total + 1));
                    }
                    if let Some(tz_len) = try_match_timezone(tz_rest) {
                        return Some((SyntaxKind::DATE_TIME_OFFSET, total + tz_len));
                    }
                }
                return Some((SyntaxKind::DATE_TIME_LOCAL, total));
            }
        }
        return Some((SyntaxKind::DATE, date_len));
    }

    None
}

fn try_match_date(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.len() < 10 {
        return None;
    }

    // YYYY-MM-DD with validation
    if !bytes[0].is_ascii_digit()
        || !bytes[1].is_ascii_digit()
        || !bytes[2].is_ascii_digit()
        || !bytes[3].is_ascii_digit()
        || bytes[4] != b'-'
        || !bytes[5].is_ascii_digit()
        || !bytes[6].is_ascii_digit()
        || bytes[7] != b'-'
        || !bytes[8].is_ascii_digit()
        || !bytes[9].is_ascii_digit()
    {
        return None;
    }

    // Parse year, month, day
    let year = (bytes[0] - b'0') as u32 * 1000
        + (bytes[1] - b'0') as u32 * 100
        + (bytes[2] - b'0') as u32 * 10
        + (bytes[3] - b'0') as u32;
    let month = (bytes[5] - b'0') as u32 * 10 + (bytes[6] - b'0') as u32;
    let day = (bytes[8] - b'0') as u32 * 10 + (bytes[9] - b'0') as u32;

    // Validate month (01-12)
    if !(1..=12).contains(&month) {
        return None;
    }

    // Validate day based on month
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Check for leap year
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400) {
                29
            } else {
                28
            }
        }
        _ => return None,
    };

    if day < 1 || day > max_day {
        return None;
    }

    Some(10)
}

fn try_match_time(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.len() < 8 {
        return None;
    }

    // HH:MM:SS with validation
    if !bytes[0].is_ascii_digit()
        || !bytes[1].is_ascii_digit()
        || bytes[2] != b':'
        || !bytes[3].is_ascii_digit()
        || !bytes[4].is_ascii_digit()
        || bytes[5] != b':'
        || !bytes[6].is_ascii_digit()
        || !bytes[7].is_ascii_digit()
    {
        return None;
    }

    let hour = (bytes[0] - b'0') as u32 * 10 + (bytes[1] - b'0') as u32;
    let minute = (bytes[3] - b'0') as u32 * 10 + (bytes[4] - b'0') as u32;
    let second = (bytes[6] - b'0') as u32 * 10 + (bytes[7] - b'0') as u32;

    // Validate ranges: hour 00-23, minute 00-59, second 00-59
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    let mut len = 8;
    // Optional fractional seconds
    if len < bytes.len() && (bytes[len] == b'.' || bytes[len] == b',') {
        let frac_start = len + 1;
        len += 1;
        while len < bytes.len() && bytes[len].is_ascii_digit() {
            len += 1;
        }
        // Must have at least one digit after the decimal point
        if len == frac_start {
            return None;
        }
    }
    Some(len)
}

fn try_match_timezone(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.len() < 6 {
        return None;
    }

    // +HH:MM or -HH:MM with validation
    if !(bytes[0] == b'+' || bytes[0] == b'-')
        || !bytes[1].is_ascii_digit()
        || !bytes[2].is_ascii_digit()
        || bytes[3] != b':'
        || !bytes[4].is_ascii_digit()
        || !bytes[5].is_ascii_digit()
    {
        return None;
    }

    let hour = (bytes[1] - b'0') as u32 * 10 + (bytes[2] - b'0') as u32;
    let minute = (bytes[4] - b'0') as u32 * 10 + (bytes[5] - b'0') as u32;

    // Validate timezone offset: hour 00-23, minute 00-59
    if hour > 23 || minute > 59 {
        return None;
    }

    Some(6)
}
