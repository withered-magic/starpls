use crate::{
    cursor::{Cursor, CursorState},
    LiteralKind::*,
    TokenKind::*,
};

mod cursor;

pub mod unescape;

#[cfg(test)]
mod tests;

/// Parsed token.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A comment.
    Comment,

    /// A newline character.
    Newline,

    /// A sequence of non-newline whitespace characters.
    Whitespace,

    /// An increase in the indentation level.
    Indent,

    /// A decrease in the indentation level.
    Dedent { consistent: bool },

    /// An identifier.
    Ident,

    /// A literal.
    Literal { kind: LiteralKind },

    // Keywords:
    /// "and"
    And,
    /// "break"
    Break,
    /// "continue"
    Continue,
    /// "def"
    Def,
    /// "elif"
    Elif,
    /// "else"
    Else,
    /// "False"
    False,
    /// "for"
    For,
    /// "if"
    If,
    /// "in"
    In,
    /// "lambda"
    Lambda,
    /// "load"
    Load,
    /// "None"
    NoneKw,
    /// "not"
    Not,
    /// "or"
    Or,
    /// "pass"
    Pass,
    /// "return"
    Return,
    /// "True"
    True,

    // Reserved tokens:
    /// "as"
    As,
    /// "assert"
    Assert,
    /// "async"
    Async,
    /// "await"
    Await,
    /// "class"
    Class,
    /// "del"
    Del,
    /// "except"
    Except,
    /// "finally"
    Finally,
    /// "from"
    From,
    /// "global"
    Global,
    /// "import"
    Import,
    /// "is"
    Is,
    /// "nonlocal"
    Nonlocal,
    /// "raise"
    Raise,
    /// "try"
    Try,
    /// "while"
    While,
    /// "with"
    With,
    /// "yield"
    Yield,

    // Symbols:
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "//"
    SlashSlash,
    /// "%"
    Mod,
    /// "**"
    StarStar,
    /// "~"
    Tilde,
    /// "&"
    Ampersand,
    /// "|"
    Bar,
    /// "^"
    Caret,
    /// "<<"
    LtLt,
    /// ">>"
    GtGt,
    /// "."
    Dot,
    /// ","
    Comma,
    /// "="
    Eq,
    /// ";"
    Semi,
    /// ":"
    Colon,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "["
    OpenBrack,
    /// "]"
    CloseBrack,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// ">="
    Ge,
    /// "<="
    Le,
    /// "=="
    EqEq,
    /// "!"
    Bang,
    /// "!="
    BangEq,
    /// "+="
    PlusEq,
    /// "-="
    MinusEq,
    /// "*="
    StarEq,
    /// "/="
    SlashEq,
    /// "//="
    SlashSlashEq,
    /// "%="
    ModEq,
    /// "&="
    AmpersandEq,
    /// "|="
    BarEq,
    /// "^="
    CaretEq,
    /// "<<="
    LtLtEq,
    /// ">>="
    GtGtEq,

    /// Unknown token, unrecognized by the lexer.
    Unknown,

    /// End of input.
    Eof,
}

// Enum representing the literal types supported by the lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// Decimal, hexadecimal, and octal integers, e.g. `0`, `123`, `0x7f`, `0o755`.
    Int { base: Base, empty_int: bool },
    /// Floating-point numbers with optional exponents, e.g. `0.0`, `1.1e-10`.
    Float { empty_exponent: bool },
    /// Strings delimited with either single or double quotes, e.g. `"hello"`, `'hello'`.
    Str {
        terminated: bool,
        triple_quoted: bool,
    },
    /// Raw strings, e.g. `r'hello'`, `r"hello"`.
    RawStr {
        terminated: bool,
        triple_quoted: bool,
    },
    /// Byte strings delimited with either single or double quotes, e.g. `b"hello"`, `b'hello'`.
    ByteStr {
        terminated: bool,
        triple_quoted: bool,
    },
    /// Raw byte strings, e.g. `rb'hello'`, `rb"hello"`.
    RawByteStr {
        terminated: bool,
        triple_quoted: bool,
    },
}

/// The base of an integer literal, as specified by its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with "0o".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != Eof {
            Some(token)
        } else {
            None
        }
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

pub fn is_non_newline_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        macro_rules! augmented_assign {
            ($assign_tok:ident, $op_tok:ident) => {
                if self.first() == '=' {
                    self.bump();
                    $assign_tok
                } else {
                    $op_tok
                }
            };
        }

        // Check if we're at the beginning of the line or in the middle of emitting DEDENT tokens.
        loop {
            match self.state {
                CursorState::BeforeLeadingSpaces => {
                    // Consume tabs and spaces to determine the indentation level. We consider tabs equivalent to four spaces.
                    let mut indent = 0;
                    self.eat_while(|c| {
                        match c {
                            ' ' => {
                                indent += 1;
                            }
                            '\t' => {
                                indent += 4;
                            }
                            _ => return false,
                        }
                        true
                    });

                    let last_indent = self.indents.last().cloned().unwrap_or(0);

                    // If we are at an equal indentation level, or are currently at a newline or EOF, we can just
                    // emit the whitespace that we consumed.
                    if indent == last_indent || self.first() == '\n' || self.is_eof() {
                        self.state = CursorState::AfterLeadingSpaces;
                        let pos_within_token = self.reset_pos_within_token();
                        if pos_within_token == 0 {
                            continue;
                        }
                        return Token::new(Whitespace, pos_within_token);
                    } else if indent > last_indent {
                        // If we are at a greater indentation level, push it on the stack and return an INDENT token.
                        self.indents.push(indent);
                        self.state = CursorState::AfterLeadingSpaces;
                        return Token::new(Indent, self.reset_pos_within_token());
                    } else {
                        // If we are at a lower indentation level, pop levels off the stack until the top
                        // level is less than or equal to the current indentation level.
                        let mut num_remaining = 0;
                        loop {
                            self.indents.pop();
                            num_remaining += 1;
                            let last_indent = self.indents.last().cloned().unwrap_or(0);
                            if last_indent <= indent {
                                self.state = CursorState::Dedenting {
                                    num_remaining,
                                    consistent: last_indent == indent,
                                };
                                break;
                            }
                        }
                    }
                }
                CursorState::Dedenting {
                    ref mut num_remaining,
                    consistent,
                } => {
                    let token = if *num_remaining == 1 {
                        self.state = CursorState::AfterLeadingSpaces;
                        Token::new(Dedent { consistent }, self.reset_pos_within_token())
                    } else {
                        *num_remaining -= 1;
                        Token::new(Dedent { consistent: true }, 0)
                    };
                    return token;
                }
                CursorState::AfterLeadingSpaces => break,
            }
        }

        let first_char = match self.bump() {
            Some(c) => c,
            None => {
                let token = if self.indents.is_empty() {
                    Token::new(Eof, 0)
                } else {
                    self.indents.pop();
                    Token::new(Dedent { consistent: true }, 0)
                };
                return token;
            }
        };

        let token_kind = match first_char {
            // Skip emitting newlines if we currently have an opened parenthesis, bracket, or brace.
            c if is_whitespace(c) => {
                if self.has_open_block() {
                    self.eat_while(is_whitespace);
                    Whitespace
                } else if c == '\n' {
                    self.state = CursorState::BeforeLeadingSpaces;
                    Newline
                } else {
                    self.eat_while(is_non_newline_whitespace);
                    Whitespace
                }
            }

            // Comments start with a `#` and continue until a newline character.
            '#' => {
                self.eat_while(|c| c != '\n');
                Comment
            }

            // One-character tokens.
            ',' => Comma,
            ';' => Semi,
            ':' => Colon,
            '(' => {
                self.open_block(')');
                OpenParen
            }
            ')' => {
                self.close_block(')');
                CloseParen
            }
            '[' => {
                self.open_block(']');
                OpenBrack
            }
            ']' => {
                self.close_block(']');
                CloseBrack
            }
            '{' => {
                self.open_block('}');
                OpenBrace
            }
            '}' => {
                self.close_block('}');
                CloseBrace
            }
            '~' => Tilde,

            // One-character operators and their corresponding augmented assignments.
            '+' => augmented_assign!(PlusEq, Plus),
            '-' => augmented_assign!(MinusEq, Minus),
            '%' => augmented_assign!(ModEq, Mod),
            '&' => augmented_assign!(AmpersandEq, Ampersand),
            '|' => augmented_assign!(BarEq, Bar),
            '^' => augmented_assign!(CaretEq, Caret),

            '=' => augmented_assign!(EqEq, Eq),
            '!' => augmented_assign!(BangEq, Bang),

            // Less-than or less-than-equal comparison operators, or left-shift and its augmented assignment.
            '<' => match (self.first(), self.second()) {
                ('<', '=') => {
                    self.bump();
                    self.bump();
                    LtLtEq
                }
                ('<', _) => {
                    self.bump();
                    LtLt
                }
                ('=', _) => {
                    self.bump();
                    Le
                }
                _ => Lt,
            },

            // Greater-than or greater-than-equal comparison operators, or right-shift and its augmented assignment.
            '>' => match (self.first(), self.second()) {
                ('>', '=') => {
                    self.bump();
                    self.bump();
                    GtGtEq
                }
                ('>', _) => {
                    self.bump();
                    GtGt
                }
                ('=', _) => {
                    self.bump();
                    Ge
                }
                _ => Gt,
            },

            // Normal and floored division, plus their augmented assignments.
            '/' => match (self.first(), self.second()) {
                ('=', _) => {
                    self.bump();
                    SlashEq
                }
                ('/', '=') => {
                    self.bump();
                    self.bump();
                    SlashSlashEq
                }
                ('/', _) => {
                    self.bump();
                    SlashSlash
                }
                _ => Slash,
            },

            // Multiplication and its augmented assignment, or the "keywords arguments" operator.
            '*' => match self.first() {
                '=' => {
                    self.bump();
                    StarEq
                }
                '*' => {
                    self.bump();
                    StarStar
                }
                _ => Star,
            },

            // Raw string literal, raw byte string literal, or identifier.
            'r' => {
                let is_raw_byte_string = if self.first() == 'b' {
                    self.bump();
                    true
                } else {
                    false
                };
                match self.first() {
                    closing_quote @ ('"' | '\'') => {
                        self.bump();
                        let (terminated, triple_quoted) = self.string(closing_quote);
                        Literal {
                            kind: if is_raw_byte_string {
                                RawByteStr {
                                    terminated,
                                    triple_quoted,
                                }
                            } else {
                                RawStr {
                                    terminated,
                                    triple_quoted,
                                }
                            },
                        }
                    }
                    _ => self.ident_or_keyword(),
                }
            }

            // Byte string literal, raw byte string literal, or identifier.
            'b' => {
                let is_raw_byte_string = if self.first() == 'r' {
                    self.bump();
                    true
                } else {
                    false
                };
                match self.first() {
                    closing_quote @ ('"' | '\'') => {
                        self.bump();
                        let (terminated, triple_quoted) = self.string(closing_quote);
                        Literal {
                            kind: if is_raw_byte_string {
                                RawByteStr {
                                    terminated,
                                    triple_quoted,
                                }
                            } else {
                                ByteStr {
                                    terminated,
                                    triple_quoted,
                                }
                            },
                        }
                    }
                    _ => self.ident_or_keyword(),
                }
            }

            // Single-, double-, or triple-quoted string literal.
            closing_quote @ ('"' | '\'') => {
                let (terminated, triple_quoted) = self.string(closing_quote);
                Literal {
                    kind: Str {
                        terminated,
                        triple_quoted,
                    },
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => self.ident_or_keyword(),

            // Numerical literal starting with a digit.
            c @ '0'..='9' => {
                let literal_kind = self.number(c);
                TokenKind::Literal { kind: literal_kind }
            }

            // Float literals can start with a dot.
            '.' => {
                if self.eat_decimal_digits() {
                    let mut empty_exponent = false;
                    if matches!(self.first(), 'e' | 'E') {
                        self.bump();
                        empty_exponent = !self.eat_exponent();
                    }
                    TokenKind::Literal {
                        kind: Float { empty_exponent },
                    }
                } else {
                    Dot
                }
            }

            _ => Unknown,
        };

        Token::new(token_kind, self.reset_pos_within_token())
    }

    fn ident_or_keyword(&mut self) -> TokenKind {
        self.eat_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
        match self.str_until_pos_within_token() {
            "and" => And,
            "break" => Break,
            "continue" => Continue,
            "def" => Def,
            "elif" => Elif,
            "else" => Else,
            "False" => False,
            "for" => For,
            "if" => If,
            "in" => In,
            "lambda" => Lambda,
            "load" => Load,
            "None" => NoneKw,
            "not" => Not,
            "or" => Or,
            "pass" => Pass,
            "return" => Return,
            "True" => True,
            "as" => As,
            "assert" => Assert,
            "async" => Async,
            "await" => Await,
            "class" => Class,
            "del" => Del,
            "except" => Except,
            "finally" => Finally,
            "from" => From,
            "global" => Global,
            "import" => Import,
            "is" => Is,
            "nonlocal" => Nonlocal,
            "raise" => Raise,
            "try" => Try,
            "while" => While,
            "with" => With,
            "yield" => Yield,
            _ => Ident,
        }
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;
        if first_digit == '0' {
            // Attempt to parse encoding base.
            match self.first() {
                'o' | 'O' => {
                    base = Base::Octal;
                    self.bump();
                    return Int {
                        base,
                        empty_int: !self.eat_octal_digits(),
                    };
                }
                'x' | 'X' => {
                    base = Base::Hexadecimal;
                    self.bump();
                    return Int {
                        base,
                        empty_int: !self.eat_hexadecimal_digits(),
                    };
                }
                // TODO(withered-magic): Decimal int literals don't allow leading zeros.
                '0'..='9' | '.' | 'e' | 'E' => {}
                _ => {
                    return Int {
                        base,
                        empty_int: false,
                    }
                }
            }
        }
        self.eat_decimal_digits();
        match self.first() {
            '.' => {
                let mut empty_exponent = false;
                self.bump();
                self.eat_decimal_digits();
                if matches!(self.first(), 'e' | 'E') {
                    self.bump();
                    empty_exponent = !self.eat_exponent();
                }
                Float { empty_exponent }
            }
            'e' | 'E' => {
                self.bump();
                Float {
                    empty_exponent: !self.eat_exponent(),
                }
            }
            _ => Int {
                base,
                empty_int: false,
            },
        }
    }

    fn eat_octal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '0'..='7' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_exponent(&mut self) -> bool {
        if matches!(self.first(), '+' | '-') {
            self.bump();
        }
        self.eat_decimal_digits()
    }

    fn string(&mut self, closing_quote: char) -> (bool, bool) {
        let triple_quoted = if self.first() == closing_quote && self.second() == closing_quote {
            self.bump();
            self.bump();
            true
        } else {
            false
        };

        let mut closing_streak = 0;
        while let Some(c) = self.bump() {
            match c {
                c if c == closing_quote => {
                    closing_streak += 1;
                    if !triple_quoted || closing_streak == 3 {
                        return (true, triple_quoted);
                    }
                }
                '\\' if self.first() == '\\' || self.first() == closing_quote => {
                    // Bump again to skip the escaped character.
                    self.bump();
                }
                _ => {
                    closing_streak = 0;
                }
            }
        }
        // End-of-file was reached.
        (false, triple_quoted)
    }
}
