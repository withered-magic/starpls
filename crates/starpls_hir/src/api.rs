use crate::{
    def::{Function as HirDefFunction, Stmt},
    module, source_map,
    typeck::Ty,
    Db, DisplayWithDb, Name, TyKind,
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

    pub fn type_of_expr(&self, file: File, expr: ast::Expression) -> Option<Type> {
        let ptr = AstPtr::new(&expr);
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        Some(self.db.infer_expr(file, *expr).into())
    }
}

pub struct Type {
    ty: Ty,
}

impl Type {
    pub fn is_function(&self) -> bool {
        matches!(
            self.ty.kind(),
            TyKind::Function(_) | TyKind::BuiltinFunction(_) | TyKind::IntrinsicFunction(_, _)
        )
    }

    pub fn is_user_defined_function(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Function(_))
    }

    pub fn params(&self, db: &dyn Db) -> Vec<Name> {
        match self.ty.params(db) {
            Some(params) => params.collect(),
            None => Vec::new(),
        }
    }
}

impl Type {
    pub fn fields(&self, db: &dyn Db) -> Vec<(Name, Type)> {
        let fields = match self.ty.fields(db) {
            Some(fields) => fields,
            None => return Vec::new(),
        };

        fields.map(|(name, ty)| (name, ty.into())).collect()
    }
}

impl From<Ty> for Type {
    fn from(ty: Ty) -> Self {
        Self { ty }
    }
}

impl DisplayWithDb for Type {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ty.fmt(db, f)
    }

    fn fmt_alt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ty.fmt_alt(db, f)
    }
}

pub struct Function {
    func: HirDefFunction,
}

impl Function {
    pub fn name(&self, db: &dyn Db) -> Name {
        self.func.name(db)
    }

    pub fn ty(&self) -> Type {
        TyKind::Function(self.func).intern().into()
    }
}

impl From<HirDefFunction> for Function {
    fn from(func: HirDefFunction) -> Self {
        Self { func }
    }
}
