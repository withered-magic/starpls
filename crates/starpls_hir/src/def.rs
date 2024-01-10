use crate::{typeck::TypeRef, Db};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use smol_str::SmolStr;
use starpls_syntax::ast::{self, AssignOp, AstPtr, BinaryOp, UnaryOp};

pub mod lower;
pub mod resolver;
pub mod scope;

#[cfg(test)]
mod tests;

pub type ModulePtr = AstPtr<ast::Module>;

pub type ExprId = Id<Expr>;
pub type ExprPtr = AstPtr<ast::Expression>;

pub type StmtId = Id<Stmt>;
pub type StmtPtr = AstPtr<ast::Statement>;

pub type ParamId = Id<Param>;
pub type ParamPtr = AstPtr<ast::Parameter>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub(crate) exprs: Arena<Expr>,
    pub(crate) stmts: Arena<Stmt>,
    pub(crate) params: Arena<Param>,
    pub(crate) top_level: Box<[StmtId]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSourceMap {
    pub root: ModulePtr,
    pub expr_map: FxHashMap<ExprPtr, ExprId>,
    pub expr_map_back: FxHashMap<ExprId, ExprPtr>,
    pub stmt_map: FxHashMap<StmtPtr, StmtId>,
    pub stmt_map_back: FxHashMap<StmtId, StmtPtr>,
    pub param_map: FxHashMap<ParamPtr, ParamId>,
    pub param_map_back: FxHashMap<ParamId, ParamPtr>,
}

impl Module {
    pub(crate) fn new_with_source_map(
        db: &dyn Db,
        syntax: ast::Module,
    ) -> (Module, ModuleSourceMap) {
        lower::lower_module(db, syntax)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Missing,
    Name {
        name: Name,
    },
    Literal {
        literal: Literal,
    },
    If {
        if_expr: ExprId,
        test: ExprId,
        else_expr: ExprId,
    },
    Unary {
        op: Option<UnaryOp>,
        expr: ExprId,
    },
    Binary {
        lhs: ExprId,
        rhs: ExprId,
        op: Option<BinaryOp>,
    },
    Lambda {
        params: Box<[ParamId]>,
        body: ExprId,
    },
    List {
        exprs: Box<[ExprId]>,
    },
    ListComp {
        expr: ExprId,
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
        exprs: Box<[ExprId]>,
    },
    Paren {
        expr: ExprId,
    },
    Dot {
        expr: ExprId,
        field: Name,
    },
    Call {
        callee: ExprId,
        args: Box<[Argument]>,
    },
    Index {
        lhs: ExprId,
        index: ExprId,
    },
    Slice {
        start: Option<ExprId>,
        end: Option<ExprId>,
        step: Option<ExprId>,
    },
}

impl Expr {
    pub(crate) fn walk_child_exprs(&self, mut f: impl FnMut(ExprId)) {
        match self {
            Expr::If {
                if_expr,
                test,
                else_expr,
            } => {
                f(*if_expr);
                f(*test);
                f(*else_expr);
            }
            Expr::Unary { expr, .. } => f(*expr),
            Expr::Binary { lhs, rhs, .. } => {
                f(*lhs);
                f(*rhs);
            }
            Expr::Lambda { body, .. } => f(*body),
            Expr::List { exprs } => exprs.iter().copied().for_each(f),
            Expr::ListComp { expr, comp_clauses } => {
                self.walk_comp_clause_expressions(comp_clauses, &mut f);
                f(*expr);
            }
            Expr::Dict { entries } => {
                for entry in entries.iter() {
                    f(entry.key);
                    f(entry.value);
                }
            }
            Expr::DictComp {
                entry,
                comp_clauses,
            } => {
                self.walk_comp_clause_expressions(comp_clauses, &mut f);
                f(entry.key);
                f(entry.value);
            }
            Expr::Tuple { exprs } => exprs.iter().copied().for_each(f),
            Expr::Paren { expr } => f(*expr),
            Expr::Dot { expr, .. } => f(*expr),
            Expr::Call { callee, args } => {
                f(*callee);
                for arg in args.iter() {
                    match arg {
                        Argument::Simple { expr }
                        | Argument::Keyword { expr, .. }
                        | Argument::UnpackedList { expr }
                        | Argument::UnpackedDict { expr } => f(*expr),
                    }
                }
            }
            Expr::Index { lhs, index } => {
                f(*lhs);
                f(*index);
            }
            Expr::Slice { start, end, step } => {
                start.map(&mut f);
                end.map(&mut f);
                step.map(&mut f);
            }
            _ => {}
        }
    }

    fn walk_comp_clause_expressions(
        &self,
        comp_clauses: &Box<[CompClause]>,
        mut f: impl FnMut(ExprId),
    ) {
        for comp_clause in comp_clauses.iter() {
            match comp_clause {
                CompClause::For { iterable, targets } => {
                    f(*iterable);
                    targets.iter().copied().for_each(&mut f);
                }
                CompClause::If { test } => f(*test),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Def {
        name: Name,
        params: Box<[ParamId]>,
        stmts: Box<[StmtId]>,
    },
    If {
        test: ExprId,
        if_stmts: Box<[StmtId]>,
        elif_stmt: Option<StmtId>,
        else_stmts: Box<[StmtId]>,
    },
    For {
        iterable: ExprId,
        targets: Box<[ExprId]>,
        stmts: Box<[StmtId]>,
    },
    Return {
        expr: Option<ExprId>,
    },
    Break,
    Continue,
    Pass,
    Assign {
        lhs: ExprId,
        rhs: ExprId,
        op: Option<AssignOp>,
    },
    Load {
        items: Box<[LoadItem]>,
    },
    Expr {
        expr: ExprId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Argument {
    Simple { expr: ExprId },
    Keyword { name: Name, expr: ExprId },
    UnpackedList { expr: ExprId },
    UnpackedDict { expr: ExprId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Param {
    Simple {
        name: Name,
        default: Option<ExprId>,
        type_ref: Option<TypeRef>,
    },
    ArgsList {
        name: Name,
        type_ref: Option<TypeRef>,
    },

    KwargsList {
        name: Name,
        type_ref: Option<TypeRef>,
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
        iterable: ExprId,
        targets: Box<[ExprId]>,
    },
    If {
        test: ExprId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DictEntry {
    key: ExprId,
    value: ExprId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Int,
    Float,
    String,
    Bytes,
    Bool,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Declaration {
    Function { id: StmtId },
    Variable { id: ExprId, source: Option<ExprId> },
    Parameter { id: ParamId },
    LoadItem {},
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name(SmolStr);

impl Name {
    pub(crate) fn from_str(name: &str) -> Self {
        Self::new(SmolStr::new(name))
    }

    pub(crate) fn missing() -> Self {
        Self::new(SmolStr::new_inline("[missing name]"))
    }

    pub(crate) fn is_missing(&self) -> bool {
        &self.0 == "[missing name]"
    }

    pub fn from_ast_node(name: ast::NameRef) -> Self {
        Self::from_str(name.name().as_ref().map_or_else(|| "", |name| name.text()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub(crate) fn new_inline(name: &'static str) -> Self {
        Self::new(SmolStr::new_inline(name))
    }

    fn new(repr: SmolStr) -> Self {
        Self(repr)
    }
}
