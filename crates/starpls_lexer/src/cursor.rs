use std::str::Chars;

/// Sentinel value used to mark end-of-file.
pub(crate) const EOF_CHAR: char = '\0';

pub(crate) enum CursorState {
    BeforeLeadingSpaces,
    Dedenting {
        num_remaining: u32,
        consistent: bool,
    },
    AfterLeadingSpaces,
}

pub struct Cursor<'a> {
    pub(crate) state: CursorState,
    pub(crate) indents: Vec<u32>,
    chars: Chars<'a>,
    len_remaining: usize,
    input: &'a str,
    closers: Vec<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            state: CursorState::BeforeLeadingSpaces,
            chars: input.chars(),
            len_remaining: input.len(),
            input,
            indents: Vec::new(),
            closers: Vec::new(),
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

    pub(crate) fn reset_pos_within_token(&mut self) -> u32 {
        let pos_within_token = self.pos_within_token();
        self.len_remaining = self.chars.as_str().len();
        pos_within_token
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

    pub(crate) fn has_open_block(&self) -> bool {
        !self.closers.is_empty()
    }

    pub(crate) fn open_block(&mut self, closer: char) {
        self.closers.push(closer);
    }

    pub(crate) fn close_block(&mut self, closer: char) {
        if closer == self.closers.last().cloned().unwrap_or(EOF_CHAR) {
            self.closers.pop();
        }
    }
}
