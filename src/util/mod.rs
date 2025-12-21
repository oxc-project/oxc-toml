use rowan::TextRange;

mod escape;

pub use escape::check_escape;

pub(crate) mod allowed_chars {
    pub(crate) fn comment(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && (('\u{0000}'..='\u{0008}').contains(&c)
                    || ('\u{000A}'..='\u{001F}').contains(&c)
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && c != '\n'
                && c != '\r'
                && (('\u{0000}'..='\u{0008}').contains(&c)
                    || ('\u{000A}'..='\u{001F}').contains(&c)
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn multi_line_string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c != '\n' && c != '\r' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }
}

pub fn overlaps(range: TextRange, other: TextRange) -> bool {
    range.contains_range(other)
        || other.contains_range(range)
        || range.contains(other.start())
        || range.contains(other.end())
        || other.contains(range.start())
        || other.contains(range.end())
}
