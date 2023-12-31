use crate::{
    StarlarkLanguage,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};
use std::marker::PhantomData;

pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<StarlarkLanguage>;

pub struct AstPtr<N: AstNode> {
    inner: SyntaxNodePtr,
    phantom: PhantomData<fn() -> N>,
}

/// A trait that allows converting between untyped `SyntaxNode`s and typed AST nodes.
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

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

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}

fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

fn children<N: AstNode>(parent: &SyntaxNode) -> impl Iterator<Item = N> {
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

/// A statement.
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
}

ast_node! {
    LoadStmt => LOAD_STMT
    children items -> LoadItem;
}

pub enum Expression {
    Name(Name),
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
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            NAME | LITERAL_EXPR
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
            NAME => Self::Name(Name { syntax }),
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
            Expression::Name(Name { syntax }) => syntax,
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

ast_node! {
    Name => NAME
    child_token name -> IDENT;
}

ast_node! {
    LiteralExpr => LITERAL_EXPR
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

ast_node! {
    BinaryExpr => BINARY_EXPR
    child lhs -> Expression;
}

impl BinaryExpr {
    pub fn rhs(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
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
    child field -> Name;
}

ast_node! {
    CallExpr => CALL_EXPR
    child callee -> Expression;
    children arguments -> Arguments;
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
    child start -> Expression;
}

impl SliceExpr {
    pub fn end(&self) -> Option<Expression> {
        children(self.syntax()).nth(1)
    }

    pub fn step(&self) -> Option<Expression> {
        children(self.syntax()).nth(2)
    }
}

ast_node! {
    Suite => SUITE
    children statements -> Statement;
}

pub enum Argument {
    Simple(SimpleArgument),
    Keyword(KeywordArgument),
    UnpackedList(UnpackedListArgument),
    UnpackedDict(UnpackedDictArgument),
}

impl AstNode for Argument {
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

pub enum Parameter {
    Simple(SimpleParameter),
    ArgsList(ArgsListParameter),
    KwargsList(KwargsListParameter),
}

impl AstNode for Parameter {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SIMPLE_PARAMETER | ARGS_LIST_PARAMETER | KWARGS_LIST_PARAMETER
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            SIMPLE_PARAMETER => Self::Simple(SimpleParameter { syntax }),
            ARGS_LIST_PARAMETER => Self::ArgsList(ArgsListParameter { syntax }),
            KWARGS_LIST_PARAMETER => Self::KwargsList(KwargsListParameter { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Simple(SimpleParameter { syntax }) => syntax,
            Self::ArgsList(ArgsListParameter { syntax }) => syntax,
            Self::KwargsList(KwargsListParameter { syntax }) => syntax,
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
    KwargsListParameter => KWARGS_LIST_PARAMETER
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
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
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
