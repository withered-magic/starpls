use crate::Db;
use id_arena::{Arena, Id};
use smol_str::SmolStr;
use starpls_syntax::ast::{self, AstPtr};

pub mod lower;

pub type ExpressionId = Id<Expression>;
pub type ExpressionPtr = AstPtr<Expression>;

pub type StatementId = Id<Statement>;
pub type StatementPtr = AstPtr<Statement>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    expressions: Arena<Expression>,
    statements: Arena<Statement>,
}

impl Module {
    pub(crate) fn new(db: &dyn Db, syntax: ast::Module) -> Self {
        lower::lower_module(db, syntax)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Missing,
    Name {
        name: Name,
    },
    Literal,
    If {
        if_expression: ExpressionId,
        test: ExpressionId,
        else_expression: ExpressionId,
    },
    Unary {
        expression: ExpressionId,
    },
    Binary {
        lhs: ExpressionId,
        rhs: ExpressionId,
    },
    Lambda {
        parameters: Box<[Parameter]>,
        body: ExpressionId,
    },
    List {
        elements: Box<[ExpressionId]>,
    },
    ListComp {
        expression: ExpressionId,
        comp_clauses: Box<[CompClause]>,
    },
    Dict {
        entries: Box<[DictEntry]>,
    },
    DictComp {
        entry: DictEntry,
        comp_clauses: Box<[CompClause]>,
    },
    Tuple {
        expressions: Box<[ExpressionId]>,
    },
    Paren {
        expression: ExpressionId,
    },
    Dot {
        expression: ExpressionId,
        field: Name,
    },
    Call {
        callee: ExpressionId,
        arguments: Box<[Argument]>,
    },
    Index {
        lhs: ExpressionId,
        index: ExpressionId,
    },
    Slice {
        start: Option<ExpressionId>,
        end: Option<ExpressionId>,
        step: Option<ExpressionId>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Def {
        name: Name,
        parameters: Box<[Parameter]>,
        statements: Box<[StatementId]>,
    },
    If {
        test: ExpressionId,
        if_statements: Box<[StatementId]>,
        elif_statement: Option<StatementId>,
        else_statements: Box<[StatementId]>,
    },
    For {
        iterable: ExpressionId,
        targets: Box<[ExpressionId]>,
        statements: Box<[StatementId]>,
    },
    Return {
        expr: Option<ExpressionId>,
    },
    Break,
    Continue,
    Pass,
    Assign {
        lhs: ExpressionId,
        rhs: ExpressionId,
    },
    Load {
        items: Box<[LoadItem]>,
    },
    Expr {
        expr: ExpressionId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Argument {
    Simple { expr: ExpressionId },
    Keyword { name: Name, expr: ExpressionId },
    UnpackedList { expr: ExpressionId },
    UnpackedDict { expr: ExpressionId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parameter {
    Simple {
        name: Name,
        default: Option<ExpressionId>,
    },
    ArgsList {
        name: Name,
    },
    KwargsList {
        name: Name,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadItem {
    Direct { name: Box<str> },
    Aliased { alias: Name, name: Box<str> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompClause {
    For {
        iterable: ExpressionId,
        targets: Box<[ExpressionId]>,
    },
    If {
        test: ExpressionId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DictEntry {
    key: ExpressionId,
    value: ExpressionId,
}

#[salsa::interned]
pub struct Name {
    inner: SmolStr,
}

impl Name {
    fn from_str(db: &dyn Db, name: &str) -> Self {
        Self::new(db, SmolStr::new(name))
    }

    fn missing(db: &dyn Db) -> Self {
        Name::new(db, SmolStr::new_inline("[missing name]"))
    }
}
