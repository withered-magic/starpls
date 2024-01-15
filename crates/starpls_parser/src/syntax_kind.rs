use self::SyntaxKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens.
    ERROR,
    EOF,
    COMMENT,
    NEWLINE,
    WHITESPACE,
    INDENT,
    DEDENT,
    IDENT,
    INT,
    FLOAT,
    STRING,
    BYTES,
    TRUE,
    FALSE,
    NONE,
    AND,
    BREAK,
    CONTINUE,
    DEF,
    ELIF,
    ELSE,
    FOR,
    IF,
    IN,
    LAMBDA,
    LOAD,
    NOT,
    OR,
    PASS,
    RETURN,
    AS,
    ASSERT,
    ASYNC,
    AWAIT,
    CLASS,
    DEL,
    EXCEPT,
    FINALLY,
    FROM,
    GLOBAL,
    IMPORT,
    IS,
    NONLOCAL,
    RAISE,
    TRY,
    WHILE,
    WITH,
    YIELD,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    SLASH_SLASH,
    MOD,
    STAR_STAR,
    TILDE,
    AMPERSAND,
    BAR,
    CARET,
    LT_LT,
    GT_GT,
    DOT,
    COMMA,
    EQ,
    SEMI,
    COLON,
    OPEN_PAREN,
    CLOSE_PAREN,
    OPEN_BRACK,
    CLOSE_BRACK,
    OPEN_BRACE,
    CLOSE_BRACE,
    LT,
    GT,
    GE,
    LE,
    EQ_EQ,
    BANG,
    BANG_EQ,
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    SLASH_SLASH_EQ,
    MOD_EQ,
    AMPERSAND_EQ,
    BAR_EQ,
    CARET_EQ,
    LT_LT_EQ,
    GT_GT_EQ,
    ARROW,
    ELLIPSES,

    // Expressions.
    NAME,
    NAME_REF,
    LITERAL_EXPR,
    IF_EXPR,
    UNARY_EXPR,
    BINARY_EXPR,
    LAMBDA_EXPR,
    LIST_EXPR,
    LIST_COMP,
    DICT_EXPR,
    DICT_COMP,
    TUPLE_EXPR,
    PAREN_EXPR,
    DOT_EXPR,
    CALL_EXPR,
    INDEX_EXPR,
    SLICE_EXPR,

    // Statements.
    DEF_STMT,
    IF_STMT,
    FOR_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    PASS_STMT,
    ASSIGN_STMT,
    LOAD_STMT,

    // Types.
    NAMED_TYPE,
    NONE_TYPE,
    FUNCTION_TYPE,
    TYPE_REF,
    GENERIC_ARGUMENTS,
    PARAMETER_TYPES,
    TYPE_COMMENT,
    TYPE_COMMENT_PREFIX,
    TYPE_LIST,

    ARGUMENTS,
    SIMPLE_ARGUMENT,        // f(x)
    KEYWORD_ARGUMENT,       // f(kwarg=x)
    UNPACKED_LIST_ARGUMENT, // f(*x)
    UNPACKED_DICT_ARGUMENT, // f(**x)

    PARAMETERS,
    SIMPLE_PARAMETER,      // def f(x, y="default")
    ARGS_LIST_PARAMETER,   // def f(*, *args)
    KWARGS_LIST_PARAMETER, // def f(**kwargs)

    SUITE,
    LOOP_VARIABLES,
    COMP_CLAUSE_FOR,
    COMP_CLAUSE_IF,
    DICT_ENTRY,

    DIRECT_LOAD_ITEM,
    ALIASED_LOAD_ITEM,

    MODULE,
}

#[macro_export]
macro_rules! T {
    [ident] => { $ crate :: SyntaxKind :: IDENT };
    ['('] => { $ crate :: SyntaxKind :: OPEN_PAREN };
    [')'] => { $ crate :: SyntaxKind :: CLOSE_PAREN };
    ['['] => { $ crate :: SyntaxKind :: OPEN_BRACK };
    [']'] => { $ crate :: SyntaxKind :: CLOSE_BRACK };
    ['{'] => { $ crate :: SyntaxKind :: OPEN_BRACE };
    ['}'] => { $ crate :: SyntaxKind :: CLOSE_BRACE };
    [if] => { $ crate :: SyntaxKind :: IF };
    [elif] => { $ crate :: SyntaxKind :: ELIF };
    [else] => { $ crate :: SyntaxKind :: ELSE };
    [+] => { $ crate :: SyntaxKind :: PLUS };
    [-] => { $ crate :: SyntaxKind :: MINUS };
    [~] => { $ crate :: SyntaxKind :: TILDE };
    [not] => { $ crate :: SyntaxKind :: NOT };
    [lambda] => { $ crate :: SyntaxKind :: LAMBDA };
    [return] => { $ crate :: SyntaxKind :: RETURN };
    [break] => { $ crate :: SyntaxKind :: BREAK };
    [continue] => { $ crate :: SyntaxKind :: CONTINUE };
    [pass] => { $ crate :: SyntaxKind :: PASS };
    [load] => { $ crate :: SyntaxKind :: LOAD };
    [def] => { $ crate :: SyntaxKind :: DEF };
    [for] => { $ crate :: SyntaxKind :: FOR };
    ['\n'] => { $ crate :: SyntaxKind :: NEWLINE };
    [;] => { $ crate :: SyntaxKind :: SEMI };
    [or] => { $ crate :: SyntaxKind :: OR };
    [and] => { $ crate :: SyntaxKind :: AND };
    [==] => { $ crate :: SyntaxKind :: EQ_EQ };
    [!=] => { $ crate :: SyntaxKind :: BANG_EQ };
    [<] => { $ crate :: SyntaxKind :: LT };
    [>] => { $ crate :: SyntaxKind :: GT };
    [<=] => { $ crate :: SyntaxKind :: LE };
    [>=] => { $ crate :: SyntaxKind :: GE };
    [in] => { $ crate :: SyntaxKind :: IN };
    [|] => { $ crate :: SyntaxKind :: BAR };
    [^] => { $ crate :: SyntaxKind :: CARET };
    [&] => { $ crate :: SyntaxKind :: AMPERSAND };
    [<<] => { $ crate :: SyntaxKind :: LT_LT };
    [>>] => { $ crate :: SyntaxKind :: GT_GT };
    [*] => { $ crate :: SyntaxKind :: STAR };
    [**] => { $ crate :: SyntaxKind :: STAR_STAR };
    [/] => { $ crate :: SyntaxKind :: SLASH };
    ["//"] => { $ crate :: SyntaxKind :: SLASH_SLASH };
    [%] => { $ crate :: SyntaxKind :: MOD };
    [True] => { $ crate :: SyntaxKind :: TRUE };
    [False] => { $ crate :: SyntaxKind :: FALSE };
    [None] => { $ crate :: SyntaxKind :: NONE };
    [.] => { $ crate :: SyntaxKind :: DOT };
    [,] => { $ crate :: SyntaxKind :: COMMA };
    [=] => { $ crate :: SyntaxKind :: EQ };
    [+=] => { $ crate :: SyntaxKind :: PLUS_EQ };
    [-=] => { $ crate :: SyntaxKind :: MINUS_EQ };
    [*=] => { $ crate :: SyntaxKind :: STAR_EQ };
    [/=] => { $ crate :: SyntaxKind :: SLASH_EQ };
    ["//="] => { $ crate :: SyntaxKind :: SLASH_SLASH_EQ };
    [%=] => { $ crate :: SyntaxKind :: MOD_EQ };
    [&=] => { $ crate :: SyntaxKind :: AMPERSAND_EQ };
    [|=] => { $ crate :: SyntaxKind :: BAR_EQ };
    [^=] => { $ crate :: SyntaxKind :: CARET_EQ };
    [<<=] => { $ crate :: SyntaxKind :: LT_LT_EQ };
    [>>=] => { $ crate :: SyntaxKind :: GT_GT_EQ };
    [:] => { $ crate :: SyntaxKind :: COLON };
}

impl SyntaxKind {
    pub fn is_trivia_token(&self) -> bool {
        matches!(*self, WHITESPACE | COMMENT)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            *self,
            AND | BREAK
                | CONTINUE
                | DEF
                | ELIF
                | ELSE
                | FOR
                | IF
                | IN
                | LAMBDA
                | LOAD
                | NOT
                | OR
                | PASS
                | RETURN
        )
    }
}

impl From<starpls_lexer::TokenKind> for SyntaxKind {
    fn from(value: starpls_lexer::TokenKind) -> Self {
        match value {
            starpls_lexer::TokenKind::Comment => COMMENT,
            starpls_lexer::TokenKind::Newline => NEWLINE,
            starpls_lexer::TokenKind::Whitespace => WHITESPACE,
            starpls_lexer::TokenKind::Indent => INDENT,
            starpls_lexer::TokenKind::Dedent { .. } => DEDENT,
            starpls_lexer::TokenKind::Ident => IDENT,
            starpls_lexer::TokenKind::Literal { kind } => match kind {
                starpls_lexer::LiteralKind::Int { .. } => INT,
                starpls_lexer::LiteralKind::Float { .. } => FLOAT,
                starpls_lexer::LiteralKind::Str { .. } => STRING,
                starpls_lexer::LiteralKind::RawStr { .. } => STRING,
                starpls_lexer::LiteralKind::ByteStr { .. } => BYTES,
                starpls_lexer::LiteralKind::RawByteStr { .. } => BYTES,
            },
            starpls_lexer::TokenKind::And => AND,
            starpls_lexer::TokenKind::Break => BREAK,
            starpls_lexer::TokenKind::Continue => CONTINUE,
            starpls_lexer::TokenKind::Def => DEF,
            starpls_lexer::TokenKind::Elif => ELIF,
            starpls_lexer::TokenKind::Else => ELSE,
            starpls_lexer::TokenKind::False => FALSE,
            starpls_lexer::TokenKind::For => FOR,
            starpls_lexer::TokenKind::If => IF,
            starpls_lexer::TokenKind::In => IN,
            starpls_lexer::TokenKind::Lambda => LAMBDA,
            starpls_lexer::TokenKind::Load => LOAD,
            starpls_lexer::TokenKind::NoneKw => NONE,
            starpls_lexer::TokenKind::Not => NOT,
            starpls_lexer::TokenKind::Or => OR,
            starpls_lexer::TokenKind::Pass => PASS,
            starpls_lexer::TokenKind::Return => RETURN,
            starpls_lexer::TokenKind::True => TRUE,
            starpls_lexer::TokenKind::As => AS,
            starpls_lexer::TokenKind::Assert => ASSERT,
            starpls_lexer::TokenKind::Async => ASYNC,
            starpls_lexer::TokenKind::Await => AWAIT,
            starpls_lexer::TokenKind::Class => CLASS,
            starpls_lexer::TokenKind::Del => DEL,
            starpls_lexer::TokenKind::Except => EXCEPT,
            starpls_lexer::TokenKind::Finally => FINALLY,
            starpls_lexer::TokenKind::From => FROM,
            starpls_lexer::TokenKind::Global => GLOBAL,
            starpls_lexer::TokenKind::Import => IMPORT,
            starpls_lexer::TokenKind::Is => IS,
            starpls_lexer::TokenKind::Nonlocal => NONLOCAL,
            starpls_lexer::TokenKind::Raise => RAISE,
            starpls_lexer::TokenKind::Try => TRY,
            starpls_lexer::TokenKind::While => WHILE,
            starpls_lexer::TokenKind::With => WITH,
            starpls_lexer::TokenKind::Yield => YIELD,
            starpls_lexer::TokenKind::Plus => PLUS,
            starpls_lexer::TokenKind::Minus => MINUS,
            starpls_lexer::TokenKind::Star => STAR,
            starpls_lexer::TokenKind::Slash => SLASH,
            starpls_lexer::TokenKind::SlashSlash => SLASH_SLASH,
            starpls_lexer::TokenKind::Mod => MOD,
            starpls_lexer::TokenKind::StarStar => STAR_STAR,
            starpls_lexer::TokenKind::Tilde => TILDE,
            starpls_lexer::TokenKind::Ampersand => AMPERSAND,
            starpls_lexer::TokenKind::Bar => BAR,
            starpls_lexer::TokenKind::Caret => CARET,
            starpls_lexer::TokenKind::LtLt => LT_LT,
            starpls_lexer::TokenKind::GtGt => GT_GT,
            starpls_lexer::TokenKind::Dot => DOT,
            starpls_lexer::TokenKind::Comma => COMMA,
            starpls_lexer::TokenKind::Eq => EQ,
            starpls_lexer::TokenKind::Semi => SEMI,
            starpls_lexer::TokenKind::Colon => COLON,
            starpls_lexer::TokenKind::OpenParen => OPEN_PAREN,
            starpls_lexer::TokenKind::CloseParen => CLOSE_PAREN,
            starpls_lexer::TokenKind::OpenBrack => OPEN_BRACK,
            starpls_lexer::TokenKind::CloseBrack => CLOSE_BRACK,
            starpls_lexer::TokenKind::OpenBrace => OPEN_BRACE,
            starpls_lexer::TokenKind::CloseBrace => CLOSE_BRACE,
            starpls_lexer::TokenKind::Lt => LT,
            starpls_lexer::TokenKind::Gt => GT,
            starpls_lexer::TokenKind::Ge => GE,
            starpls_lexer::TokenKind::Le => LE,
            starpls_lexer::TokenKind::EqEq => EQ_EQ,
            starpls_lexer::TokenKind::Bang => BANG,
            starpls_lexer::TokenKind::BangEq => BANG_EQ,
            starpls_lexer::TokenKind::PlusEq => PLUS_EQ,
            starpls_lexer::TokenKind::MinusEq => MINUS_EQ,
            starpls_lexer::TokenKind::StarEq => STAR_EQ,
            starpls_lexer::TokenKind::SlashEq => SLASH_EQ,
            starpls_lexer::TokenKind::SlashSlashEq => SLASH_SLASH_EQ,
            starpls_lexer::TokenKind::ModEq => MOD_EQ,
            starpls_lexer::TokenKind::AmpersandEq => AMPERSAND_EQ,
            starpls_lexer::TokenKind::BarEq => BAR_EQ,
            starpls_lexer::TokenKind::CaretEq => CARET_EQ,
            starpls_lexer::TokenKind::LtLtEq => LT_LT_EQ,
            starpls_lexer::TokenKind::GtGtEq => GT_GT_EQ,
            starpls_lexer::TokenKind::Arrow => ARROW,
            starpls_lexer::TokenKind::Ellipses => ELLIPSES,
            starpls_lexer::TokenKind::Unknown => ERROR,
            starpls_lexer::TokenKind::Eof => EOF,
        }
    }
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(value: u16) -> Self {
        assert!(value <= MODULE as u16);
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        kind as u16
    }
}

/// A bitset of `SyntaxKind`s. Only `SyntaxKind`s corresponding to lexer tokens should be added to a `SyntaxKindSet`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKindSet(u128);

impl SyntaxKindSet {
    pub const fn new(kinds: &[SyntaxKind]) -> SyntaxKindSet {
        let mut inner = 0;
        let mut i = 0;
        while i < kinds.len() {
            inner |= 1 << kinds[i] as u16;
            i += 1;
        }
        SyntaxKindSet(inner)
    }

    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & 1 << kind as usize > 0
    }

    pub const fn union(&self, other: SyntaxKindSet) -> SyntaxKindSet {
        SyntaxKindSet(self.0 | other.0)
    }
}
