use std::{
    fmt::{Debug, Write},
    marker::PhantomData,
    str::Chars,
};

pub use rowan::{
    ast::{AstNode, AstPtr},
    Direction,
};

use crate::{
    StarlarkLanguage,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxNodeChildren, SyntaxToken, TextSize, T,
};

pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<StarlarkLanguage>;

/// A trait that allows converting between untyped `SyntaxToken`s and typed AST tokens.
pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// A macro for defining AST nodes. The `AstNode` trait is automatically implemented.
macro_rules! ast_node {
    (
        $(#[doc = $doc:expr])*$node:ident => $kind:ident
        $(child $($child:ident -> $child_node:ident),+;)*
        $(child_token $($child_token:ident -> $child_token_kind:ident),+;)*
        $(children $($children:ident -> $children_node:ident),+;)*
    ) => {
        $(#[doc = $doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $node {
            pub(crate) syntax: SyntaxNode,
        }

        impl AstNode for $node {
            type Language = StarlarkLanguage;

            fn can_cast(kind: SyntaxKind) -> bool {
                kind == $kind
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }

        impl $node {
        $($(
            pub fn $child(&self) -> Option<$child_node> {
                child(&self.syntax)
            }
        )+)*

        $($(
            pub fn $child_token(&self) -> Option<SyntaxToken> {
                token(self.syntax(), $child_token_kind)
            }
        )+)*

        $($(
            pub fn $children(&self) -> AstChildren<$children_node> {
                AstChildren::new(&self.syntax)
            }
        )+)*
        }
    };
}

/// A macro for defining AST tokens. The `AstToken` trait is automatically implemented.
macro_rules! ast_token {
    (
        $(#[doc = $doc:expr])*$token:ident => $kind:ident
    ) => {
        $(#[doc = $doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $token {
            pub(crate) syntax: SyntaxToken,
        }

        impl AstToken for $token {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == $kind
            }

            fn cast(syntax: SyntaxToken) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxToken {
                &self.syntax
            }
        }
    };
}

pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    phantom: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            phantom: PhantomData,
        }
    }
}

impl<N: AstNode<Language = StarlarkLanguage>> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}

fn child<N: AstNode<Language = StarlarkLanguage>>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

fn children<N: AstNode<Language = StarlarkLanguage>>(
    parent: &SyntaxNode,
) -> impl Iterator<Item = N> {
    parent.children().filter_map(N::cast)
}

fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|element| element.into_token())
        .find(|token| token.kind() == kind)
}

ast_node! {
    /// A Starlark module. This is typically the root of the AST.
    Module => MODULE
    children statements -> Statement;
}

impl Module {
    pub fn doc(&self) -> Option<String> {
        self.syntax()
            .children()
            .next()
            .and_then(LiteralExpr::cast)
            .and_then(|expr| match expr.kind() {
                LiteralKind::String(s) => Some(s),
                _ => None,
            })
    }

    pub fn type_ignore_comment_positions(&self) -> impl Iterator<Item = TextSize> {
        self.syntax()
            .descendants()
            .filter_map(IgnoreType::cast)
            .map(|node| node.syntax().text_range().start())
    }
}

/// A statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Def(DefStmt),
    If(IfStmt),
    For(ForStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Pass(PassStmt),
    Assign(AssignStmt),
    Load(LoadStmt),
    Expr(Expression),
}

impl AstNode for Statement {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            DEF_STMT
                | IF_STMT
                | FOR_STMT
                | RETURN_STMT
                | BREAK_STMT
                | CONTINUE_STMT
                | PASS_STMT
                | ASSIGN_STMT
                | LOAD_STMT
        ) || Expression::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            DEF_STMT => Self::Def(DefStmt { syntax }),
            IF_STMT => Self::If(IfStmt { syntax }),
            FOR_STMT => Self::For(ForStmt { syntax }),
            RETURN_STMT => Self::Return(ReturnStmt { syntax }),
            BREAK_STMT => Self::Break(BreakStmt { syntax }),
            CONTINUE_STMT => Self::Continue(ContinueStmt { syntax }),
            PASS_STMT => Self::Pass(PassStmt { syntax }),
            ASSIGN_STMT => Self::Assign(AssignStmt { syntax }),
            LOAD_STMT => Self::Load(LoadStmt { syntax }),
            kind if Expression::can_cast(kind) => {
                Self::Expr(Expression::cast(syntax).expect("failed to cast as Expression"))
            }
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Def(DefStmt { syntax }) => syntax,
            Self::If(IfStmt { syntax }) => syntax,
            Self::For(ForStmt { syntax }) => syntax,
            Self::Return(ReturnStmt { syntax }) => syntax,
            Self::Break(BreakStmt { syntax }) => syntax,
            Self::Continue(ContinueStmt { syntax }) => syntax,
            Self::Pass(PassStmt { syntax }) => syntax,
            Self::Assign(AssignStmt { syntax }) => syntax,
            Self::Load(LoadStmt { syntax }) => syntax,
            Self::Expr(expr) => expr.syntax(),
        }
    }
}

ast_node! {
    /// A function definition.
    DefStmt => DEF_STMT
    child parameters -> Parameters;
    child suite -> Suite;
    child name -> Name;
}

impl DefStmt {
    pub fn spec(&self) -> Option<FunctionType> {
        self.suite()
            .and_then(|suite| suite.type_comment())
            .and_then(|type_comment| type_comment.function_type())
    }

    pub fn doc(&self) -> Option<String> {
        self.suite()
            .and_then(|suite| suite.syntax().children().find_map(LiteralExpr::cast))
            .and_then(|expr| match expr.kind() {
                LiteralKind::String(token) => Some(token),
                _ => None,
            })
    }
}

ast_node! {
    /// An `if` statement.
    IfStmt => IF_STMT
    child test -> Expression;
    child if_suite -> Suite;
    child elif_stmt -> IfStmt;
}

impl IfStmt {
    pub fn else_suite(&self) -> Option<Suite> {
        children(self.syntax()).nth(1)
    }
}

ast_node! {
    /// A `for` statement.
    ForStmt => FOR_STMT
    child suite -> Suite;
    child iterable -> Expression;
    child targets -> LoopVariables;
}

ast_node! {
    ReturnStmt => RETURN_STMT
    child expr -> Expression;
}

ast_node! {
    BreakStmt => BREAK_STMT
}

ast_node! {
    ContinueStmt => CONTINUE_STMT
}

ast_node! {
    PassStmt => PASS_STMT
}

ast_node! {
    AssignStmt => ASSIGN_STMT
    child lhs -> Expression;
}

impl AssignStmt {
    pub fn rhs(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }

    pub fn assign_op_info(&self) -> Option<(SyntaxToken, AssignOp)> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(|token| {
                let op = match token.kind() {
                    T![=] => AssignOp::Normal,
                    T![+=] => AssignOp::Arith(ArithAssignOp::Add),
                    T![-=] => AssignOp::Arith(ArithAssignOp::Sub),
                    T![*=] => AssignOp::Arith(ArithAssignOp::Mul),
                    T![/=] => AssignOp::Arith(ArithAssignOp::Div),
                    T!["//="] => AssignOp::Arith(ArithAssignOp::Flr),
                    T![%=] => AssignOp::Arith(ArithAssignOp::Mod),
                    T![&=] => AssignOp::Bitwise(BitwiseAssignOp::And),
                    T![|=] => AssignOp::Bitwise(BitwiseAssignOp::Or),
                    T![>>=] => AssignOp::Bitwise(BitwiseAssignOp::Shl),
                    T![<<=] => AssignOp::Bitwise(BitwiseAssignOp::Shr),
                    T![^=] => AssignOp::Bitwise(BitwiseAssignOp::Xor),
                    _ => return None,
                };
                Some((token, op))
            })
    }

    pub fn type_comment(&self) -> Option<TypeComment> {
        self.syntax
            .siblings_with_tokens(Direction::Next)
            .take_while(|el| !matches!(el.kind(), T![;] | T!['\n']))
            .filter_map(|el| el.into_node())
            .find_map(TypeComment::cast)
    }
}

ast_node! {
    LoadStmt => LOAD_STMT
    child module -> LoadModule;
    children items -> LoadItem;
}

ast_node! {
    LoadModule => LOAD_MODULE
    child_token name -> STRING;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Name(NameRef),
    Literal(LiteralExpr),
    If(IfExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Lambda(LambdaExpr),
    List(ListExpr),
    ListComp(ListComp),
    Dict(DictExpr),
    DictComp(DictComp),
    Tuple(TupleExpr),
    Paren(ParenExpr),
    Dot(DotExpr),
    Call(CallExpr),
    Index(IndexExpr),
    Slice(SliceExpr),
}

impl AstNode for Expression {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            NAME_REF
                | LITERAL_EXPR
                | IF_EXPR
                | UNARY_EXPR
                | BINARY_EXPR
                | LAMBDA_EXPR
                | LIST_EXPR
                | LIST_COMP
                | DICT_EXPR
                | DICT_COMP
                | TUPLE_EXPR
                | PAREN_EXPR
                | DOT_EXPR
                | CALL_EXPR
                | INDEX_EXPR
                | SLICE_EXPR
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            NAME_REF => Self::Name(NameRef { syntax }),
            LITERAL_EXPR => Self::Literal(LiteralExpr { syntax }),
            IF_EXPR => Self::If(IfExpr { syntax }),
            UNARY_EXPR => Self::Unary(UnaryExpr { syntax }),
            BINARY_EXPR => Self::Binary(BinaryExpr { syntax }),
            LAMBDA_EXPR => Self::Lambda(LambdaExpr { syntax }),
            LIST_EXPR => Self::List(ListExpr { syntax }),
            LIST_COMP => Self::ListComp(ListComp { syntax }),
            DICT_EXPR => Self::Dict(DictExpr { syntax }),
            DICT_COMP => Self::DictComp(DictComp { syntax }),
            TUPLE_EXPR => Self::Tuple(TupleExpr { syntax }),
            PAREN_EXPR => Self::Paren(ParenExpr { syntax }),
            DOT_EXPR => Self::Dot(DotExpr { syntax }),
            CALL_EXPR => Self::Call(CallExpr { syntax }),
            INDEX_EXPR => Self::Index(IndexExpr { syntax }),
            SLICE_EXPR => Self::Slice(SliceExpr { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expression::Name(NameRef { syntax }) => syntax,
            Expression::Literal(LiteralExpr { syntax }) => syntax,
            Expression::If(IfExpr { syntax }) => syntax,
            Expression::Unary(UnaryExpr { syntax }) => syntax,
            Expression::Binary(BinaryExpr { syntax }) => syntax,
            Expression::Lambda(LambdaExpr { syntax }) => syntax,
            Expression::List(ListExpr { syntax }) => syntax,
            Expression::ListComp(ListComp { syntax }) => syntax,
            Expression::Dict(DictExpr { syntax }) => syntax,
            Expression::DictComp(DictComp { syntax }) => syntax,
            Expression::Tuple(TupleExpr { syntax }) => syntax,
            Expression::Paren(ParenExpr { syntax }) => syntax,
            Expression::Dot(DotExpr { syntax }) => syntax,
            Expression::Call(CallExpr { syntax }) => syntax,
            Expression::Index(IndexExpr { syntax }) => syntax,
            Expression::Slice(SliceExpr { syntax }) => syntax,
        }
    }
}

impl From<NameRef> for Expression {
    fn from(value: NameRef) -> Self {
        Self::Name(value)
    }
}

impl From<DotExpr> for Expression {
    fn from(value: DotExpr) -> Self {
        Self::Dot(value)
    }
}

ast_node! {
    Name => NAME
    child_token name -> IDENT;
}

ast_node! {
    NameRef => NAME_REF
    child_token name -> IDENT;
}

ast_node! {
    LiteralExpr => LITERAL_EXPR
}

impl LiteralExpr {
    pub fn token(&self) -> SyntaxToken {
        self.syntax
            .children_with_tokens()
            .find(|el| !el.kind().is_trivia_token())
            .and_then(|el| el.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();
        if let Some(lit) = Int::cast(token.clone()) {
            return LiteralKind::Int(lit);
        }
        if let Some(lit) = Float::cast(token.clone()) {
            return LiteralKind::Float(lit);
        }
        if let Some(lit) = String::cast(token.clone()) {
            return LiteralKind::String(lit);
        }
        if let Some(lit) = Bytes::cast(token.clone()) {
            return LiteralKind::Bytes(lit);
        }
        match token.kind() {
            T![True] => LiteralKind::Bool(true),
            T![False] => LiteralKind::Bool(false),
            T![None] => LiteralKind::None,
            _ => unreachable!(),
        }
    }
}

ast_node! {
    IfExpr => IF_EXPR
    child if_expr -> Expression;
}

impl IfExpr {
    pub fn test(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }

    pub fn else_expr(&self) -> Option<Expression> {
        children(self.syntax()).nth(2)
    }
}

ast_node! {
    UnaryExpr => UNARY_EXPR
    child expr -> Expression;
}

impl UnaryExpr {
    pub fn unary_op_info(&self) -> Option<(SyntaxToken, UnaryOp)> {
        self.syntax
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(|token| {
                let op = match token.kind() {
                    T![+] => UnaryOp::Arith(UnaryArithOp::Add),
                    T![-] => UnaryOp::Arith(UnaryArithOp::Sub),
                    T![~] => UnaryOp::Inv,
                    T![not] => UnaryOp::Not,
                    _ => return None,
                };
                Some((token, op))
            })
    }
}

ast_node! {
    BinaryExpr => BINARY_EXPR
    child lhs -> Expression;
}

impl BinaryExpr {
    pub fn rhs(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }

    pub fn binary_op_info(&self) -> Option<(SyntaxToken, BinaryOp)> {
        self.syntax
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(|token| {
                let op = match token.kind() {
                    T![+] => BinaryOp::Arith(ArithOp::Add),
                    T![-] => BinaryOp::Arith(ArithOp::Sub),
                    T![*] => BinaryOp::Arith(ArithOp::Mul),
                    T![/] => BinaryOp::Arith(ArithOp::Div),
                    T!["//"] => BinaryOp::Arith(ArithOp::Flr),
                    T![%] => BinaryOp::Arith(ArithOp::Mod),
                    T![&] => BinaryOp::Bitwise(BitwiseOp::And),
                    T![|] => BinaryOp::Bitwise(BitwiseOp::Or),
                    T![^] => BinaryOp::Bitwise(BitwiseOp::Xor),
                    T![<<] => BinaryOp::Bitwise(BitwiseOp::Shl),
                    T![>>] => BinaryOp::Bitwise(BitwiseOp::Shr),
                    T![==] => BinaryOp::Cmp(CmpOp::Eq),
                    T![!=] => BinaryOp::Cmp(CmpOp::Ne),
                    T![<] => BinaryOp::Cmp(CmpOp::Lt),
                    T![>] => BinaryOp::Cmp(CmpOp::Gt),
                    T![<=] => BinaryOp::Cmp(CmpOp::Le),
                    T![>=] => BinaryOp::Cmp(CmpOp::Ge),
                    T![and] => BinaryOp::Logic(LogicOp::And),
                    T![or] => BinaryOp::Logic(LogicOp::Or),
                    T![in] => BinaryOp::MemberOp(MemberOp::In),
                    T![not] => BinaryOp::MemberOp(MemberOp::NotIn),
                    _ => return None,
                };
                Some((token, op))
            })
    }
}

ast_node! {
    LambdaExpr => LAMBDA_EXPR
    child parameters -> Parameters;
    child body -> Expression;
}

ast_node! {
    ListExpr => LIST_EXPR
    children elements -> Expression;
}

ast_node! {
    ListComp => LIST_COMP
    child expr -> Expression;
    children comp_clauses -> CompClause;
}

ast_node! {
    DictExpr => DICT_EXPR
    children entries -> DictEntry;
}

ast_node! {
    DictComp => DICT_COMP
    child entry -> DictEntry;
    children comp_clauses -> CompClause;
}

ast_node! {
    TupleExpr => TUPLE_EXPR
    children elements -> Expression;
}

ast_node! {
    ParenExpr => PAREN_EXPR
    child expr -> Expression;
}

ast_node! {
    DotExpr => DOT_EXPR
    child expr -> Expression;
    child field -> Name;
}

ast_node! {
    CallExpr => CALL_EXPR
    child callee -> Expression;
    child arguments -> Arguments;
}

ast_node! {
    IndexExpr => INDEX_EXPR
    child lhs -> Expression;
}

impl IndexExpr {
    pub fn index(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }
}

ast_node! {
    SliceExpr => SLICE_EXPR
    child expr -> Expression;
}

impl SliceExpr {
    pub fn start(&self) -> Option<Expression> {
        // Take all expressions until the first ":", which should just consist of "expr" (at index 0)
        // and "start" (at index 1).
        self.syntax()
            .children_with_tokens()
            .take_while(|el| el.kind() != T![:])
            .filter_map(|el| el.into_node())
            .filter_map(Expression::cast)
            .nth(1)
    }

    pub fn end(&self) -> Option<Expression> {
        // Skip all children until the first colon, consume the colon, then take all children until the second colon.
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.kind() != T![:])
            .skip(1)
            .take_while(|el| el.kind() != T![:])
            .filter_map(|el| el.into_node())
            .find_map(Expression::cast)
    }

    pub fn step(&self) -> Option<Expression> {
        // Skip all children until the second colon.
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.kind() != T![:])
            .skip(1)
            .skip_while(|el| el.kind() != T![:])
            .skip(1)
            .filter_map(|el| el.into_node())
            .find_map(Expression::cast)
    }
}

ast_node! {
    Suite => SUITE
    children statements -> Statement;
}

impl Suite {
    /// Only call this if you know the parent suite belongs to a function.
    pub fn type_comment(&self) -> Option<TypeComment> {
        self.syntax().first_child().and_then(TypeComment::cast)
    }
}

pub enum Argument {
    Simple(SimpleArgument),
    Keyword(KeywordArgument),
    UnpackedList(UnpackedListArgument),
    UnpackedDict(UnpackedDictArgument),
}

impl AstNode for Argument {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SIMPLE_ARGUMENT | KEYWORD_ARGUMENT | UNPACKED_LIST_ARGUMENT | UNPACKED_DICT_ARGUMENT
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            SIMPLE_ARGUMENT => Self::Simple(SimpleArgument { syntax }),
            KEYWORD_ARGUMENT => Self::Keyword(KeywordArgument { syntax }),
            UNPACKED_LIST_ARGUMENT => Self::UnpackedList(UnpackedListArgument { syntax }),
            UNPACKED_DICT_ARGUMENT => Self::UnpackedDict(UnpackedDictArgument { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Simple(SimpleArgument { syntax }) => syntax,
            Self::Keyword(KeywordArgument { syntax }) => syntax,
            Self::UnpackedList(UnpackedListArgument { syntax }) => syntax,
            Self::UnpackedDict(UnpackedDictArgument { syntax }) => syntax,
        }
    }
}

ast_node! {
    Arguments => ARGUMENTS
    children arguments -> Argument;
}

ast_node! {
    SimpleArgument => SIMPLE_ARGUMENT
    child expr -> Expression;
}

ast_node! {
    KeywordArgument => KEYWORD_ARGUMENT
    child expr -> Expression;
    child name -> Name;
}

ast_node! {
    UnpackedListArgument => UNPACKED_LIST_ARGUMENT
    child expr -> Expression;
}

ast_node! {
    UnpackedDictArgument => UNPACKED_DICT_ARGUMENT
    child expr -> Expression;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parameter {
    Simple(SimpleParameter),
    ArgsList(ArgsListParameter),
    KwargsDict(KwargsDictParameter),
}

impl Parameter {
    pub fn type_comment(&self) -> Option<TypeComment> {
        self.syntax()
            .siblings_with_tokens(Direction::Next)
            .skip(1)
            .take_while(|el| el.kind() != CLOSE_PAREN && !Self::can_cast(el.kind()))
            .filter_map(|el| el.into_node())
            .find_map(TypeComment::cast)
    }

    pub fn name(&self) -> Option<std::string::String> {
        match self {
            Parameter::Simple(param) => param.name(),
            Parameter::ArgsList(param) => param.name(),
            Parameter::KwargsDict(param) => param.name(),
        }
        .and_then(|name| name.name())
        .map(|token| token.text().to_string())
    }
}

impl AstNode for Parameter {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SIMPLE_PARAMETER | ARGS_LIST_PARAMETER | KWARGS_DICT_PARAMETER
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            SIMPLE_PARAMETER => Self::Simple(SimpleParameter { syntax }),
            ARGS_LIST_PARAMETER => Self::ArgsList(ArgsListParameter { syntax }),
            KWARGS_DICT_PARAMETER => Self::KwargsDict(KwargsDictParameter { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Simple(SimpleParameter { syntax }) => syntax,
            Self::ArgsList(ArgsListParameter { syntax }) => syntax,
            Self::KwargsDict(KwargsDictParameter { syntax }) => syntax,
        }
    }
}

ast_node! {
    Parameters => PARAMETERS
    children parameters -> Parameter;
}

ast_node! {
    SimpleParameter => SIMPLE_PARAMETER
    child default -> Expression;
    child name -> Name;
}

ast_node! {
    ArgsListParameter => ARGS_LIST_PARAMETER
    child name -> Name;
}

ast_node! {
    KwargsDictParameter => KWARGS_DICT_PARAMETER
    child name -> Name;
}

ast_node! {
    LoopVariables => LOOP_VARIABLES
    children exprs -> Expression;
}

ast_node! {
    DictEntry => DICT_ENTRY
    child key -> Expression;
}

impl DictEntry {
    pub fn value(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }
}

pub enum CompClause {
    For(CompClauseFor),
    If(CompClauseIf),
}

impl AstNode for CompClause {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, COMP_CLAUSE_FOR | COMP_CLAUSE_IF)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            COMP_CLAUSE_FOR => CompClause::For(CompClauseFor { syntax }),
            COMP_CLAUSE_IF => CompClause::If(CompClauseIf { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::For(CompClauseFor { syntax }) => syntax,
            Self::If(CompClauseIf { syntax }) => syntax,
        }
    }
}

ast_node! {
    CompClauseFor => COMP_CLAUSE_FOR
    child iterable -> Expression;
    child targets -> LoopVariables;
}

ast_node! {
    CompClauseIf => COMP_CLAUSE_IF
    child test -> Expression;
}

pub enum LoadItem {
    Direct(DirectLoadItem),
    Aliased(AliasedLoadItem),
}

impl AstNode for LoadItem {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, DIRECT_LOAD_ITEM | ALIASED_LOAD_ITEM)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            DIRECT_LOAD_ITEM => LoadItem::Direct(DirectLoadItem { syntax }),
            ALIASED_LOAD_ITEM => LoadItem::Aliased(AliasedLoadItem { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            LoadItem::Direct(DirectLoadItem { syntax }) => syntax,
            LoadItem::Aliased(AliasedLoadItem { syntax }) => syntax,
        }
    }
}

ast_node! {
    DirectLoadItem => DIRECT_LOAD_ITEM
    child_token name -> STRING;
}

ast_node! {
    AliasedLoadItem => ALIASED_LOAD_ITEM
    child alias -> Name;
    child_token name -> STRING;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Arith(ArithOp),
    Bitwise(BitwiseOp),
    Cmp(CmpOp),
    Logic(LogicOp),
    MemberOp(MemberOp),
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Arith(op) => std::fmt::Display::fmt(op, f),
            BinaryOp::Bitwise(op) => std::fmt::Display::fmt(op, f),
            BinaryOp::Cmp(op) => std::fmt::Display::fmt(op, f),
            BinaryOp::Logic(op) => std::fmt::Display::fmt(op, f),
            BinaryOp::MemberOp(op) => std::fmt::Display::fmt(op, f),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Flr,
    Mod,
}

impl std::fmt::Display for ArithOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ArithOp::Add => "+",
            ArithOp::Sub => "-",
            ArithOp::Mul => "*",
            ArithOp::Div => "/",
            ArithOp::Flr => "//",
            ArithOp::Mod => "%",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitwiseOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl std::fmt::Display for BitwiseOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BitwiseOp::And => "&",
            BitwiseOp::Or => "|",
            BitwiseOp::Xor => "^",
            BitwiseOp::Shl => "<<",
            BitwiseOp::Shr => ">>",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl std::fmt::Display for CmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Gt => ">",
            CmpOp::Le => "<=",
            CmpOp::Ge => ">=",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
}

impl std::fmt::Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogicOp::And => "and",
            LogicOp::Or => "or",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberOp {
    In,
    NotIn,
}

impl std::fmt::Display for MemberOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MemberOp::In => "in",
            MemberOp::NotIn => "not in",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssignOp {
    Normal,
    Arith(ArithAssignOp),
    Bitwise(BitwiseAssignOp),
}

impl std::fmt::Display for AssignOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignOp::Normal => f.write_char('='),
            AssignOp::Arith(op) => std::fmt::Display::fmt(op, f),
            AssignOp::Bitwise(op) => std::fmt::Display::fmt(op, f),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithAssignOp {
    Add,
    Sub,
    Mul,
    Div,
    Flr,
    Mod,
}

impl std::fmt::Display for ArithAssignOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ArithAssignOp::Add => "+=",
            ArithAssignOp::Sub => "-=",
            ArithAssignOp::Mul => "*=",
            ArithAssignOp::Div => "/=",
            ArithAssignOp::Flr => "//=",
            ArithAssignOp::Mod => "%=",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitwiseAssignOp {
    And,
    Or,
    Shl,
    Shr,
    Xor,
}

impl std::fmt::Display for BitwiseAssignOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BitwiseAssignOp::And => "&=",
            BitwiseAssignOp::Or => "|=",
            BitwiseAssignOp::Shl => "<<=",
            BitwiseAssignOp::Shr => ">>=",
            BitwiseAssignOp::Xor => "^=",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Arith(UnaryArithOp),
    Inv,
    Not,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Arith(op) => std::fmt::Display::fmt(op, f),
            UnaryOp::Inv => f.write_char('~'),
            UnaryOp::Not => f.write_char('!'),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryArithOp {
    Add,
    Sub,
}

impl std::fmt::Display for UnaryArithOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            UnaryArithOp::Add => '+',
            UnaryArithOp::Sub => '-',
        })
    }
}

ast_token! {
    Int => INT
}

impl Int {
    pub fn value(&self) -> Option<u64> {
        let text = self.syntax.text();
        if let Some(stripped) = text.strip_prefix('0') {
            return Some(if text.len() == 1 {
                0
            } else {
                u64::from_str_radix(stripped, 8).ok()?
            });
        }
        if let Some(stripped) = text.strip_prefix("0x") {
            return u64::from_str_radix(stripped, 16).ok();
        }
        text.parse::<u64>().ok()
    }
}

ast_token! {
    Float => FLOAT
}

ast_token! {
    String => STRING
}

impl String {
    pub fn value_and_offset(&self) -> Option<(Box<str>, u32)> {
        let mut cursor = Cursor::new(self.text());
        let mut is_raw = false;
        let mut is_bytes = false;

        // Determine the string's prefix.
        // "r" -> raw string
        // "b" -> bytes
        // "rb", "br" -> raw bytes
        match cursor.first() {
            Some('r') => {
                is_raw = true;
                cursor.bump();
                if let Some('b') = cursor.first() {
                    is_bytes = true;
                    cursor.bump();
                }
            }
            Some('b') => {
                is_bytes = true;
                cursor.bump();
                if let Some('r') = cursor.first() {
                    is_raw = true;
                    cursor.bump();
                }
            }
            None => return None,
            _ => {}
        }

        // Determine the opening quote, whether the string literal is triple
        // quoted, and if it's terminated.
        let suffix = match cursor.first() {
            Some('\'') => {
                cursor.bump();
                match (cursor.first(), cursor.second()) {
                    (Some('\''), Some('\'')) => {
                        cursor.bump();
                        cursor.bump();
                        "'''"
                    }
                    _ => "'",
                }
            }
            Some('"') => {
                cursor.bump();
                match (cursor.first(), cursor.second()) {
                    (Some('"'), Some('"')) => {
                        cursor.bump();
                        cursor.bump();
                        "\"\"\""
                    }
                    _ => "\"",
                }
            }
            _ => return None,
        };

        if is_bytes || !cursor.text().ends_with(suffix) {
            return None;
        }

        let mut ok = true;
        let mut s = std::string::String::new();
        starpls_lexer::unescape::unescape_string(
            &cursor.text()[..cursor.text().len() - suffix.len()],
            is_raw,
            suffix.len() == 3,
            &mut |_, res| match res {
                Ok(c) => s.push(c),
                Err(_) => ok = false,
            },
        );

        ok.then(|| {
            (
                s.into_boxed_str(),
                (self.text().len() - cursor.text().len()) as u32,
            )
        })
    }

    pub fn value(&self) -> Option<Box<str>> {
        self.value_and_offset().map(|(value, _)| value)
    }
}

ast_token! {
    Bytes => BYTES
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Int(Int),
    Float(Float),
    String(String),
    Bytes(Bytes),
    Bool(bool),
    None,
}

ast_node! {
    TypeComment => TYPE_COMMENT
    child body -> TypeCommentBody;
}

impl TypeComment {
    pub fn type_(&self) -> Option<Type> {
        self.body().and_then(|body| body.type_())
    }

    pub fn function_type(&self) -> Option<FunctionType> {
        self.body().and_then(|body| body.function_type())
    }
}

ast_node! {
    TypeCommentBody => TYPE_COMMENT_BODY
    child type_ -> Type;
    child function_type -> FunctionType;
    child ignore -> IgnoreType;
}

ast_node! {
    TypeList => TYPE_LIST
    children types -> Type;
}

pub enum Type {
    PathType(PathType),
    UnionType(UnionType),
    NoneType(NoneType),
    EllipsisType(EllipsisType),
}

impl AstNode for Type {
    type Language = StarlarkLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, PATH_TYPE | UNION_TYPE | NONE_TYPE | ELLIPSIS_TYPE)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            PATH_TYPE => Self::PathType(PathType { syntax }),
            UNION_TYPE => Self::UnionType(UnionType { syntax }),
            NONE_TYPE => Self::NoneType(NoneType { syntax }),
            ELLIPSIS_TYPE => Self::EllipsisType(EllipsisType { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Type::PathType(type_) => type_.syntax(),
            Type::UnionType(type_) => type_.syntax(),
            Type::NoneType(type_) => type_.syntax(),
            Type::EllipsisType(type_) => type_.syntax(),
        }
    }
}

ast_node! {
    PathType => PATH_TYPE
    child generic_arguments -> GenericArguments;
    children segments -> PathSegment;
}

ast_node! {
    UnionType => UNION_TYPE
    children types -> Type;
}

ast_node! {
    NoneType => NONE_TYPE
}

ast_node! {
    EllipsisType => ELLIPSIS_TYPE
}

ast_node! {
    GenericArguments => GENERIC_ARGUMENTS
    children types -> Type;
}

ast_node! {
    PathSegment => PATH_SEGMENT
    child_token value -> IDENT;
}

ast_node! {
    IgnoreType => IGNORE_TYPE
}

ast_node! {
    FunctionType => FUNCTION_TYPE
    child parameter_types -> ParameterTypes;
    child ret_type -> Type;
}

ast_node! {
    ParameterTypes => PARAMETER_TYPES
    children types -> ParameterType;
}

pub enum ParameterType {
    Simple(SimpleParameterType),
    ArgsList(ArgsListParameterType),
    KwargsDict(KwargsDictParameterType),
}

impl ParameterType {
    pub fn type_(&self) -> Option<Type> {
        match self {
            ParameterType::Simple(type_) => type_.type_(),
            ParameterType::ArgsList(type_) => type_.type_(),
            ParameterType::KwargsDict(type_) => type_.type_(),
        }
    }
}

impl AstNode for ParameterType {
    type Language = StarlarkLanguage;

    fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SIMPLE_PARAMETER_TYPE | ARGS_LIST_PARAMETER_TYPE | KWARGS_DICT_PARAMETER_TYPE
        )
    }

    fn cast(syntax: rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            SIMPLE_PARAMETER_TYPE => Self::Simple(SimpleParameterType { syntax }),
            ARGS_LIST_PARAMETER_TYPE => Self::ArgsList(ArgsListParameterType { syntax }),
            KWARGS_DICT_PARAMETER_TYPE => Self::KwargsDict(KwargsDictParameterType { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            ParameterType::Simple(type_) => type_.syntax(),
            ParameterType::ArgsList(type_) => type_.syntax(),
            ParameterType::KwargsDict(type_) => type_.syntax(),
        }
    }
}

ast_node! {
    SimpleParameterType => SIMPLE_PARAMETER_TYPE
    child type_ -> Type;
}

ast_node! {
    ArgsListParameterType => ARGS_LIST_PARAMETER_TYPE
    child type_ -> Type;
}

ast_node! {
    KwargsDictParameterType => KWARGS_DICT_PARAMETER_TYPE
    child type_ -> Type;
}

struct Cursor<'a> {
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    fn new(text: &'a str) -> Self {
        Cursor {
            chars: text.chars(),
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn second(&self) -> Option<char> {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next()
    }

    fn text(&self) -> &str {
        self.chars.as_str()
    }
}
