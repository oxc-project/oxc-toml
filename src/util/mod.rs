use rowan::TextRange;

mod escape;

pub use escape::check_escape;

pub(crate) mod allowed_chars {
    pub(crate) fn comment(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            // Tab is 0x09, control chars are 0x00-0x1F and 0x7F (all ASCII)
            if b != b'\t' && (b < 0x20 || b == 0x7F) {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            // Check for disallowed control chars: 0x00-0x08, 0x0A-0x1F, 0x7F
            // Tab (0x09) is allowed
            if b != b'\t'
                && ((0x00..=0x08).contains(&b) || (0x0A..=0x1F).contains(&b) || b == 0x7F)
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            // Tab (0x09), LF (0x0A), CR (0x0D) are allowed
            if b != b'\t'
                && b != b'\n'
                && b != b'\r'
                && ((0x00..=0x08).contains(&b) || (0x0A..=0x1F).contains(&b) || b == 0x7F)
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            // Tab is 0x09, control chars are 0x00-0x1F and 0x7F
            if b != b'\t' && (b < 0x20 || b == 0x7F) {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() { Ok(()) } else { Err(err_indices) }
    }

    pub(crate) fn multi_line_string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            // Tab (0x09), LF (0x0A), CR (0x0D) are allowed
            if b != b'\t' && b != b'\n' && b != b'\r' && (b < 0x20 || b == 0x7F) {
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
