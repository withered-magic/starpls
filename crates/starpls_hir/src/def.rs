use std::{collections::HashSet, ops::Index};

use either::Either;
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use smol_str::SmolStr;
use starpls_common::File;
use starpls_syntax::{
    ast::{self, AssignOp, AstPtr, BinaryOp, SyntaxNodePtr, UnaryOp},
    TextRange,
};

use crate::{typeck::TypeRef, Db};

pub(crate) mod codeflow;
mod lower;
pub(crate) mod resolver;
pub(crate) mod scope;

#[cfg(test)]
mod tests;

pub type ModulePtr = AstPtr<ast::Module>;

pub type ExprId = Id<Expr>;
pub type ExprPtr = AstPtr<ast::Expression>;

pub type StmtId = Id<Stmt>;
pub type StmtPtr = AstPtr<ast::Statement>;

pub type ParamId = Id<Param>;
pub type ParamPtr = AstPtr<ast::Parameter>;

pub type LoadItemId = Id<LoadItem>;
pub type LoadItemPtr = AstPtr<ast::LoadItem>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Module {
    pub(crate) exprs: Arena<Expr>,
    pub(crate) stmts: Arena<Stmt>,
    pub(crate) params: Arena<Param>,
    pub(crate) load_items: Arena<LoadItem>,
    pub(crate) top_level: Box<[StmtId]>,
    pub(crate) type_ignore_comment_lines: HashSet<u32>,
    pub(crate) call_expr_with_impl_fn: FxHashMap<Name, ExprId>,
    pub(crate) param_to_def_stmt: FxHashMap<ParamId, StmtId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ModuleSourceMap {
    pub root: ModulePtr,
    pub expr_map: FxHashMap<ExprPtr, ExprId>,
    pub expr_map_back: FxHashMap<ExprId, ExprPtr>,
    pub stmt_map: FxHashMap<StmtPtr, StmtId>,
    pub stmt_map_back: FxHashMap<StmtId, StmtPtr>,
    pub param_map: FxHashMap<ParamPtr, ParamId>,
    pub param_map_back: FxHashMap<ParamId, ParamPtr>,
    pub load_item_map: FxHashMap<LoadItemPtr, LoadItemId>,
    pub load_item_map_back: FxHashMap<LoadItemId, LoadItemPtr>,
}

impl Module {
    pub(crate) fn new_with_source_map(
        db: &dyn Db,
        file: File,
        syntax: ast::Module,
    ) -> (Module, ModuleSourceMap) {
        lower::lower_module(db, file, syntax)
    }
}

impl Index<ExprId> for Module {
    type Output = Expr;

    fn index(&self, index: ExprId) -> &Self::Output {
        &self.exprs[index]
    }
}

impl Index<StmtId> for Module {
    type Output = Stmt;

    fn index(&self, index: StmtId) -> &Self::Output {
        &self.stmts[index]
    }
}

impl Index<ParamId> for Module {
    type Output = Param;

    fn index(&self, index: ParamId) -> &Self::Output {
        &self.params[index]
    }
}

impl Index<LoadItemId> for Module {
    type Output = LoadItem;

    fn index(&self, index: LoadItemId) -> &Self::Output {
        &self.load_items[index]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Expr {
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
        lhs: ExprId,
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
            Expr::Slice {
                lhs,
                start,
                end,
                step,
            } => {
                f(*lhs);
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
pub(crate) enum Stmt {
    Def {
        func: Function,
        stmts: Box<[StmtId]>,
    },
    If {
        test: ExprId,
        if_stmts: Box<[StmtId]>,
        elif_or_else_stmts: Option<Either<StmtId, Box<[StmtId]>>>,
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
        type_ref: Option<(TypeRef, TextRange)>,
    },
    Load {
        load_stmt: LoadStmt,
        items: Box<[LoadItemId]>,
    },
    Expr {
        expr: ExprId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Argument {
    Simple { expr: ExprId },
    Keyword { name: Name, expr: ExprId },
    UnpackedList { expr: ExprId },
    UnpackedDict { expr: ExprId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Param {
    Simple {
        name: Name,
        default: Option<ExprId>,
        type_ref: Option<TypeRef>,
        doc: Option<Box<str>>,
    },
    ArgsList {
        name: Name,
        type_ref: Option<TypeRef>,
        doc: Option<Box<str>>,
    },
    KwargsDict {
        name: Name,
        type_ref: Option<TypeRef>,
        doc: Option<Box<str>>,
    },
}

impl Param {
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            &Self::Simple {
                default: Some(_),
                ..
            } | &Self::ArgsList { .. }
                | &Self::KwargsDict { .. }
        )
    }

    pub fn name(&self) -> &Name {
        match self {
            Param::Simple { name, .. }
            | Param::ArgsList { name, .. }
            | Param::KwargsDict { name, .. } => name,
        }
    }

    pub(crate) fn type_ref(&self) -> Option<TypeRef> {
        match self {
            Param::Simple { type_ref, .. }
            | Param::ArgsList { type_ref, .. }
            | Param::KwargsDict { type_ref, .. } => type_ref.clone(),
        }
    }

    pub fn doc(&self) -> Option<&str> {
        match self {
            Param::Simple { doc, .. }
            | Param::ArgsList { doc, .. }
            | Param::KwargsDict { doc, .. } => doc.as_deref(),
        }
    }
}

#[salsa::tracked]
pub struct LoadStmt {
    #[return_ref]
    pub(crate) module: Box<str>,
    pub(crate) ptr: SyntaxNodePtr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadItem {
    Direct {
        name: Box<str>,
        load_stmt: LoadStmt,
    },
    Aliased {
        alias: Name,
        name: Box<str>,
        load_stmt: LoadStmt,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CompClause {
    For {
        iterable: ExprId,
        targets: Box<[ExprId]>,
    },
    If {
        test: ExprId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DictEntry {
    pub(crate) key: ExprId,
    pub(crate) value: ExprId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Literal {
    Int(u64),
    Float,
    String(LiteralString),
    Bytes,
    Bool(bool),
    None,
}

impl Literal {
    fn from_ast_literal(db: &dyn Db, value: &ast::LiteralKind) -> Self {
        match value {
            ast::LiteralKind::Int(lit) => Literal::Int(lit.value().unwrap_or(0)),
            ast::LiteralKind::Float(_) => Literal::Float,
            ast::LiteralKind::String(lit) => {
                Literal::String(LiteralString::new(db, lit.value().unwrap_or_default()))
            }
            ast::LiteralKind::Bytes(_) => Literal::Bytes,
            ast::LiteralKind::Bool(lit) => Literal::Bool(*lit),
            ast::LiteralKind::None => Literal::None,
        }
    }
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

    pub fn is_missing(&self) -> bool {
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

#[salsa::interned]
pub(crate) struct LiteralString {
    pub(crate) value: Box<str>,
}

#[salsa::tracked]
pub(crate) struct Function {
    pub(crate) file: File,
    pub(crate) name: Name,
    pub(crate) ret_type_ref: Option<TypeRef>,
    pub(crate) doc: Option<Box<str>>,
    pub(crate) ptr: SyntaxNodePtr,
    #[return_ref]
    pub(crate) params: Box<[ParamId]>,
}
