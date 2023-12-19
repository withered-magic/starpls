mod cursor;
pub mod unescape;

#[cfg(test)]
mod tests;

pub use crate::cursor::Cursor;

use self::LiteralKind::*;
use self::TokenKind::*;
use crate::cursor::EOF_CHAR;

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

    /// A sequence of whitespace characters.
    Whitespace,

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
    /// "not"
    Not,
    /// "or"
    Or,
    /// "pass"
    Pass,
    /// "return"
    Return,

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
    /// "!="
    Ne,
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
    Int { base: Base, empty_int: bool },
    Float { empty_exponent: bool },
    Str { terminated: bool },
    ByteStr { terminated: bool },
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
    // matches!()
    true
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(Eof, 0),
        };
        let token_kind = match first_char {
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
        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
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
            "for" => For,
            "if" => If,
            "in" => In,
            "lambda" => Lambda,
            "load" => Load,
            "not" => Not,
            "or" => Or,
            "pass" => Pass,
            "return" => Return,
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
}
