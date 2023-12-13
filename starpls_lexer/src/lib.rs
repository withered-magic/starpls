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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int,
    Float,
    Str { terminated: bool },
    ByteStr { terminated: bool },
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
            None => return Token::new(TokenKind::Eof, 0),
        };
        let token_kind = match first_char {
            'a'..='z' | 'A'..='Z' | '_' => self.ident_or_keyword(),
        };
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

    fn int_or_float(&mut self) -> TokenKind {
        // self.eat_while(|c| matches!())
    }
}
