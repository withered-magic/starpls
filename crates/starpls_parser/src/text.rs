use crate::{step::Step, syntax_kind::SyntaxKind, Input, Output};
use starpls_lexer::{
    unescape::{unescape_byte_string, unescape_string, EscapeError},
    LiteralKind, Token, TokenKind,
};
use std::mem;

pub enum StrStep<'a> {
    Start { kind: SyntaxKind },
    Finish,
    Token { kind: SyntaxKind, text: &'a str },
    Error { message: String, pos: usize },
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: &'static str,
    pub start: usize,
    pub end: usize,
}

pub struct StrWithTokens<'a> {
    input: &'a str,
    token_kinds: Vec<SyntaxKind>,
    token_start: Vec<u32>,
    lexer_errors: Vec<LexerError>,
}

impl<'a> StrWithTokens<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut current_start = 0;
        let mut str_with_tokens = Self {
            input,
            token_kinds: Vec::new(),
            token_start: Vec::new(),
            lexer_errors: Vec::new(),
        };

        for Token { kind, len } in starpls_lexer::tokenize(input) {
            str_with_tokens.token_kinds.push(kind.into());
            str_with_tokens.token_start.push(current_start);

            // Collect any potential errors from the lexer.
            collect_token_lexer_errors(
                input,
                &mut str_with_tokens.lexer_errors,
                current_start as usize,
                (current_start + len) as usize,
                kind,
            );

            current_start += len;
        }

        // Push an extra entry to the `token_start` array to allow easily getting the length of the final token.
        str_with_tokens.token_start.push(input.len() as u32);
        str_with_tokens
    }

    pub fn to_input(&self) -> Input {
        Input {
            tokens: self
                .token_kinds
                .iter()
                .filter(|kind| !kind.is_trivia_token())
                .cloned()
                .collect(),
        }
    }

    pub fn build_with_trivia(&self, output: Output, sink: &mut dyn FnMut(StrStep<'_>)) {
        let mut builder = Builder {
            state: BuilderState::Init,
            str_with_tokens: self,
            sink,
            pos: 0,
        };

        // Defer to the builder to handle each step type.
        for step in output.steps {
            match step {
                Step::Start { kind } => builder.start(kind),
                Step::Finish => builder.finish(),
                Step::Token { kind } => builder.token(kind),
                Step::Error { message } => builder.error(message),
            }
        }

        // The builder defers its last finish step to be manually handled by us here.
        // Consume any remaining trivia tokens, then finally emit the "finish" step.
        match builder.state {
            BuilderState::PendingFinish => {
                builder.eat_trivia_tokens();
                (builder.sink)(StrStep::Finish)
            }
            BuilderState::Init | BuilderState::Normal => unreachable!(),
        }
    }

    pub fn kind(&self, pos: usize) -> SyntaxKind {
        self.token_kinds[pos]
    }

    pub fn token_text(&self, pos: usize) -> &str {
        &self.input[self.token_start[pos] as usize..self.token_start[pos + 1] as usize]
    }

    pub fn token_pos(&self, pos: usize) -> u32 {
        self.token_start[pos]
    }

    pub fn token_range(&self, pos: usize) -> (u32, u32) {
        assert!(pos < self.token_start.len());
        // If we have an error at EOF, then `pos` is equal to `self.token_start.len() - 1`, and we need to
        // explicitly avoid being out of bounds.
        let end = if pos == self.token_start.len() - 1 {
            pos
        } else {
            pos + 1
        };
        (self.token_start[pos], self.token_start[end])
    }

    pub fn len(&self) -> usize {
        self.token_kinds.len()
    }

    pub fn lexer_errors(&self) -> impl Iterator<Item = &LexerError> {
        self.lexer_errors.iter()
    }
}

fn collect_token_lexer_errors(
    input: &str,
    lexer_errors: &mut Vec<LexerError>,
    token_start: usize,
    token_end: usize,
    kind: TokenKind,
) {
    if let TokenKind::Literal { kind } = kind {
        let (terminated, triple_quoted, is_string, raw) = match kind {
            LiteralKind::Str {
                terminated,
                triple_quoted,
            } => (terminated, triple_quoted, true, false),
            LiteralKind::RawStr {
                terminated,
                triple_quoted,
            } => (terminated, triple_quoted, true, true),
            LiteralKind::ByteStr {
                terminated,
                triple_quoted,
            } => (terminated, triple_quoted, false, false),
            LiteralKind::RawByteStr {
                terminated,
                triple_quoted,
            } => (terminated, triple_quoted, false, true),
            _ => return,
        };

        let mut prefix_strip_len = if triple_quoted { 3 } else { 1 };
        let suffix_strip_len = if terminated { prefix_strip_len } else { 0 };
        prefix_strip_len += match (is_string, raw) {
            (true, true) => 1,   // r""
            (true, false) => 0,  // ""
            (false, true) => 2,  // rb""
            (false, false) => 1, // b""
        };

        let contents_start = token_start + prefix_strip_len;
        let contents = &input[contents_start..token_end - suffix_strip_len];

        if is_string {
            unescape_string(contents, raw, triple_quoted, &mut |range, res| {
                if let Err(err) = res {
                    lexer_errors.push(LexerError {
                        start: contents_start + range.start,
                        end: contents_start + range.end,
                        message: escape_error_as_message(err),
                    })
                }
            })
        } else {
            unescape_byte_string(contents, &mut |range, res| {
                if let Err(err) = res {
                    lexer_errors.push(LexerError {
                        start: contents_start + range.start,
                        end: contents_start + range.end,
                        message: escape_error_as_message(err),
                    })
                }
            })
        }
    }
}

fn escape_error_as_message(err: EscapeError) -> &'static str {
    match err {
        EscapeError::LoneSlash => "lone slash in escape sequence",
        EscapeError::InvalidEscape => "invalid escape sequence",
        EscapeError::InvalidOctalEscape => "invalid octal escape sequence: value too large",
        EscapeError::EmptyHexadecimalEscape => "invalid Unicode escape sequence: empty",
        EscapeError::TooShortHexadecimalEscape => "invalid hexadecimal escape sequence: too short",
        EscapeError::InvalidHexadecimalEscape => {
            "invalid hexadecimal escape sequence: value too large"
        }
        EscapeError::EmptyUnicodeEscape => "empty unicode escape sequence",
        EscapeError::TooShort16BitUnicodeEscape => "unicode escape sequence is too short",
        EscapeError::TooShort32BitUnicodeEscape => "unicode escape sequence is too short",
        EscapeError::TooLong32BitUnicodeEscape => "unicode escape sequence is too long",
        EscapeError::LoneSurrogateUnicodeEscape => "unicode escape sequence is a lone surrogate",
        EscapeError::OutOfRangeUnicodeEscape => "unicode escape sequence is out of range",
    }
}

enum BuilderState {
    Init,
    Normal,
    PendingFinish,
}

struct Builder<'a, 'b> {
    state: BuilderState,
    str_with_tokens: &'a StrWithTokens<'a>,
    sink: &'b mut dyn FnMut(StrStep<'_>),
    pos: usize,
}

impl Builder<'_, '_> {
    fn start(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, BuilderState::Normal) {
            BuilderState::Init => {
                (self.sink)(StrStep::Start { kind });
                return;
            }
            BuilderState::Normal => (),
            BuilderState::PendingFinish => (self.sink)(StrStep::Finish),
        }
        self.eat_trivia_tokens();
        (self.sink)(StrStep::Start { kind })
    }

    fn finish(&mut self) {
        match mem::replace(&mut self.state, BuilderState::PendingFinish) {
            BuilderState::Init => unreachable!(),
            BuilderState::Normal => (),
            BuilderState::PendingFinish => (self.sink)(StrStep::Finish),
        }
    }

    fn token(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, BuilderState::Normal) {
            BuilderState::Init => unreachable!(),
            BuilderState::Normal => (),
            BuilderState::PendingFinish => (self.sink)(StrStep::Finish),
        }
        self.eat_trivia_tokens();
        self.do_token(kind)
    }

    fn error(&mut self, message: String) {
        (self.sink)(StrStep::Error {
            message,
            pos: self.pos,
        })
    }

    fn eat_trivia_tokens(&mut self) {
        while self.pos < self.str_with_tokens.len() {
            let kind = self.str_with_tokens.kind(self.pos);
            if !kind.is_trivia_token() {
                break;
            }
            self.do_token(kind);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind) {
        let text = self.str_with_tokens.token_text(self.pos);
        (self.sink)(StrStep::Token { kind, text });
        self.pos += 1;
    }
}
