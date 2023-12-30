use id_arena::{Arena, Id};

pub struct Module {
    exprs: Arena<Expression>,
    stmts: Arena<Statement>,
}

pub type ExpressionId = Id<Expression>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Missing,
    Ident {
        if_expr: ExpressionId,
        test: ExpressionId,
        else_expr: ExpressionId,
    },
    Literal,
    If,
    Unary {
        expr: ExpressionId,
    },
    Binary {
        lhs: ExpressionId,
        rhs: ExpressionId,
    },
    Lambda {
        body: ExpressionId,
    },
    List,
    ListComp,
    Dict,
    DictComp,
    Tuple,
    Paren,
    Dot,
    Call {
        callee: ExpressionId,
    },
    Index {
        lhs: ExpressionId,
        index: ExpressionId,
    },
    Slice,
}

pub type StatementId = Id<Statement>;

pub enum Statement {
    Def,
    If,
    For,
    Return,
    Break,
    Continue,
    Pass,
    Assign,
    Load,
    Expr,
}

pub type ArgumentId = Id<Argument>;

pub enum Argument {}

pub type ParameterId = Id<Parameter>;

pub enum Parameter {
    Simple { default: ExpressionId },
    ArgsList { name},
    KwargsList { expr: ExpressionId },
}
