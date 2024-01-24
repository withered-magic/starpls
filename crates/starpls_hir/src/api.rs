use crate::{
    def::{Function as Function_, Stmt},
    module, source_map,
    typeck::{builtins::BuiltinFunction, intrinsics::IntrinsicFunction, Ty},
    Db, Name,
};
use starpls_common::File;
use starpls_syntax::ast::{self, AstNode, AstPtr};

pub struct Semantics<'a> {
    db: &'a dyn Db,
}

impl<'a> Semantics<'a> {
    pub fn new(db: &'a dyn Db) -> Self {
        Self { db }
    }

    pub fn function_for_def(&self, file: File, stmt: ast::DefStmt) -> Option<Function> {
        let ptr = AstPtr::new(&ast::Statement::cast(stmt.syntax().clone())?);
        let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
        match &module(self.db, file)[*stmt] {
            Stmt::Def { func, .. } => Some((*func).into()),
            _ => None,
        }
    }
}

pub struct Type {
    ty: Ty,
}

pub struct Function {
    inner: FunctionInner,
}

impl Function {
    pub fn name(&self, db: &dyn Db) -> Name {
        self.inner.name(db)
    }
}

impl From<Function_> for Function {
    fn from(value: Function_) -> Self {
        Self {
            inner: FunctionInner::Function(value),
        }
    }
}

enum FunctionInner {
    Function(Function_),
    Builtin(BuiltinFunction),
    Intrinsic(IntrinsicFunction),
}

impl FunctionInner {
    fn name(&self, db: &dyn Db) -> Name {
        match self {
            FunctionInner::Function(func) => func.name(db),
            FunctionInner::Builtin(func) => func.name(db),
            FunctionInner::Intrinsic(func) => todo!(),
        }
    }
}
