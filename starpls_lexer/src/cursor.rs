use std::str::Chars;

/// Sentinel value used to mark end-of-file.
pub(crate) const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    chars: Chars<'a>,
    len_remaining: usize,
    input: &'a str,
    is_line_start: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
            input,
            is_line_start: true,
        }
    }

    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    pub(crate) fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len()
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn str_until_pos_within_token(&self) -> &str {
        let start = self.input.len() - self.len_remaining;
        &self.input[start..start + self.pos_within_token() as usize]
    }
}
