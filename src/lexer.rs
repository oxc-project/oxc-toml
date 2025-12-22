//! Manual lexer implementation to replace logos dependency

use std::ops::Range;

pub struct Lexer<'source, Token> {
    source: &'source str,
    pos: usize,
    current_span: Range<usize>,
    _phantom: std::marker::PhantomData<Token>,
}

impl<'source, Token> Lexer<'source, Token> {
    pub fn new(source: &'source str) -> Self {
        Self { source, pos: 0, current_span: 0..0, _phantom: std::marker::PhantomData }
    }

    pub fn span(&self) -> Range<usize> {
        self.current_span.clone()
    }

    pub fn slice(&self) -> &'source str {
        &self.source[self.current_span.clone()]
    }

    pub fn remainder(&self) -> &'source str {
        &self.source[self.pos..]
    }
}

impl<'source, Token: LexerToken<'source>> Iterator for Lexer<'source, Token> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.source.len() {
            return None;
        }

        let start = self.pos;
        let remainder = &self.source[self.pos..];

        if let Some((token, len)) = Token::lex(remainder) {
            self.pos += len;
            self.current_span = start..self.pos;
            Some(Ok(token))
        } else {
            // Error case - consume one UTF-8 character
            let bytes = remainder.as_bytes();
            let first_byte = *bytes.first()?;
            // Determine UTF-8 sequence length from first byte
            let char_len = if first_byte < 0x80 {
                1 // ASCII
            } else if first_byte < 0xE0 {
                2 // 2-byte sequence
            } else if first_byte < 0xF0 {
                3 // 3-byte sequence
            } else {
                4 // 4-byte sequence
            };
            self.pos += char_len;
            self.current_span = start..self.pos;
            Some(Err(()))
        }
    }
}

pub trait LexerToken<'source>: Sized {
    /// Try to match a token at the beginning of the input.
    /// Returns the token and the number of bytes consumed.
    fn lex(input: &'source str) -> Option<(Self, usize)>;
}
