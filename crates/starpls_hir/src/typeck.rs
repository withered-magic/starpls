use crate::{
    def::{Expr, ExprId, Literal},
    display::DisplayWithDb,
    lower as lower_,
    typeck::builtins::{builtin_types, BuiltinClass, BuiltinTypes},
    Db, Declaration, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use starpls_common::{parse, File};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::ast::{self, AstNode, AstPtr, BinaryOp, UnaryOp};
use std::{
    fmt::Write,
    panic::{self, UnwindSafe},
    sync::Arc,
};

use self::builtins::builtin_field_types;

mod lower;

pub(crate) mod builtins;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileExprId {
    pub file: File,
    pub expr: ExprId,
}

#[derive(Debug)]

pub enum Cancelled {
    Salsa(salsa::Cancelled),
    Typecheck,
}

impl Cancelled {
    pub(crate) fn throw(self) -> ! {
        std::panic::resume_unwind(Box::new(self))
    }

    pub fn catch<F, T>(f: F) -> Result<T, Cancelled>
    where
        F: FnOnce() -> T + UnwindSafe,
    {
        match panic::catch_unwind(f) {
            Ok(t) => Ok(t),
            Err(payload) => match payload.downcast::<salsa::Cancelled>() {
                Ok(cancelled) => Err(Cancelled::Salsa(*cancelled)),
                Err(payload) => match payload.downcast::<Cancelled>() {
                    Ok(cancelled) => Err(*cancelled),
                    Err(payload) => panic::resume_unwind(payload),
                },
            },
        }
    }
}

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            err @ Cancelled::Salsa(_) => err.fmt(f),
            Cancelled::Typecheck => f.write_str("type inference cancelled"),
        }
    }
}

impl std::error::Error for Cancelled {}

#[derive(Default)]
struct SharedState {
    cancelled: AtomicCell<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    None,
    Bool,
    Int,
    Float,
    String,
    StringElems,
    Bytes,
    BytesElems,
    List,
    Tuple,
    Dict,
}

/// A reference to a type in a source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeRef {
    Any,
    Builtin(BuiltinType),
    Name(Name),
}

impl From<BuiltinType> for TypeRef {
    fn from(value: BuiltinType) -> Self {
        Self::Builtin(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ty(Interned<TyKind>);

impl Ty {
    pub fn kind(&self) -> &TyKind {
        &self.0
    }

    pub fn fields<'a>(&'a self, db: &'a dyn Db) -> Option<Vec<(&'a Name, Ty)>> {
        Some(match self.kind() {
            TyKind::List { base, .. }
            | TyKind::Tuple { base }
            | TyKind::Dict { base }
            | TyKind::BuiltinClass(base) => base
                .fields(db)
                .iter()
                .map(|field| &field.name)
                .zip(builtin_field_types(db, *base).field_tys(db).iter().cloned())
                .collect(),

            _ => return None,
        })
    }

    pub fn is_fn(&self) -> bool {
        matches!(self.kind(), TyKind::BuiltinFunction)
    }
}

impl DisplayWithDb for Ty {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.kind().fmt(db, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TyKind {
    Unbound,
    Unknown,
    Any,
    None,
    Bool,
    Int,
    Float,
    StringElems,
    BytesElems,
    List { ty: Ty, base: BuiltinClass },
    Tuple { base: BuiltinClass },
    Dict { base: BuiltinClass },
    // Tuple(SmallVec<[Ty; 2]>),
    BuiltinFunction,
    BuiltinClass(BuiltinClass),
}

impl DisplayWithDb for TyKind {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = match self {
            TyKind::Unbound => "Unbound",
            TyKind::Unknown => "Unknown",
            TyKind::Any => "Any",
            TyKind::None => "None",
            TyKind::Bool => "bool",
            TyKind::Int => "int",
            TyKind::Float => "float",
            TyKind::StringElems => "string.elems",
            TyKind::BytesElems => "bytes.elems",
            TyKind::List { ty, .. } => {
                f.write_str("list[")?;
                ty.fmt(db, f)?;
                return f.write_char(']');
            }
            TyKind::Tuple { .. } => "tuple",
            TyKind::Dict { .. } => "dict",
            TyKind::BuiltinFunction => "function",
            TyKind::BuiltinClass(class) => return f.write_str(class.name(db).as_str()),
        };
        f.write_str(text)
    }
}

impl_internable!(TyKind);

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty(Interned::new(self))
    }
}

#[derive(Default)]
pub struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    type_of_expr: Arc<Mutex<FxHashMap<FileExprId, Ty>>>,
}

impl GlobalCtxt {
    pub fn cancel(&self) {
        self.shared_state.cancelled.store(true);
        let mut type_of_exr = self.type_of_expr.lock();
        self.shared_state.cancelled.store(false);
        *type_of_exr = FxHashMap::default();
    }

    pub fn with_tcx<F, T>(&self, db: &dyn Db, mut f: F) -> T
    where
        F: FnMut(&mut TyCtxt) -> T + std::panic::UnwindSafe,
    {
        let mut type_of_expr = self.type_of_expr.lock();
        let mut tcx = TyCtxt {
            db,
            types: builtin_types(db),
            shared_state: Arc::clone(&self.shared_state),
            type_of_expr: &mut type_of_expr,
        };
        f(&mut tcx)
    }
}

pub struct TyCtxt<'a> {
    db: &'a dyn Db,
    types: BuiltinTypes,
    shared_state: Arc<SharedState>,
    type_of_expr: &'a mut FxHashMap<FileExprId, Ty>,
}

impl TyCtxt<'_> {
    pub fn infer_expr(&mut self, file: File, expr: ExprId) -> Ty {
        if let Some(ty) = self.type_of_expr.get(&FileExprId { file, expr }).cloned() {
            return ty;
        }

        if self.shared_state.cancelled.load() {
            Cancelled::Typecheck.throw()
        }

        let info = lower_(self.db, file);
        let ty = match &info.module(self.db).exprs[expr] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(self.db, file, expr);
                let decls = match resolver.resolve_name(name) {
                    Some(decls) => decls,
                    None => return self.set_expr_type(file, expr, self.types.unbound(self.db)),
                };
                match decls.last() {
                    Some(Declaration::Variable { id, source }) => {
                        return source
                            .and_then(|source| {
                                self.infer_source_expr_assign(file, source);
                                self.type_of_expr
                                    .get(&FileExprId { file, expr: *id })
                                    .cloned()
                            })
                            .unwrap_or_else(|| self.types.unknown(self.db))
                    }
                    Some(
                        Declaration::Function { .. }
                        | Declaration::Parameter { .. }
                        | Declaration::LoadItem {},
                    ) => self.types.any(self.db),
                    _ => self.types.unbound(self.db),
                }
            }
            Expr::List { exprs } => {
                let mut exprs = exprs.iter();
                let first = exprs.next();
                let ty = first
                    .map(|first| self.infer_expr(file, *first))
                    .and_then(|first_ty| {
                        exprs
                            .map(|expr| self.infer_expr(file, *expr))
                            .all(|ty| ty == first_ty)
                            .then_some(first_ty)
                    })
                    .unwrap_or_else(|| self.types.unknown(self.db));
                self.types.make_list_ty(self.db, ty)
            }
            Expr::ListComp { .. } => self.types.make_list_ty(self.db, self.types.any(self.db)),
            Expr::Dict { .. } | Expr::DictComp { .. } => self.types.dict(self.db),
            Expr::Literal { literal } => match literal {
                Literal::Int => self.types.int(self.db),
                Literal::Float => self.types.float(self.db),
                Literal::String => self.types.string(self.db),
                Literal::Bytes => self.types.bytes(self.db),
                Literal::Bool => self.types.bool(self.db),
                Literal::None => self.types.none(self.db),
            },
            Expr::Unary { op, expr } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(file, *expr, op.clone()))
                .unwrap_or_else(|| self.types.unknown(self.db)),
            Expr::Binary { lhs, rhs, op } => op
                .as_ref()
                .map(|op| self.infer_binary_expr(file, *lhs, *rhs, op.clone()))
                .unwrap_or_else(|| self.types.unknown(self.db)),
            Expr::Index { lhs, index } => {
                let lhs_ty = self.infer_expr(file, *lhs);
                let index_ty = self.infer_expr(file, *index);
                match (lhs_ty.kind(), index_ty.kind()) {
                    (TyKind::List { ty, .. }, TyKind::Int) => ty.clone(),
                    _ => self.types.unknown(self.db),
                }
            }
            _ => self.types.any(self.db),
        };
        self.set_expr_type(file, expr, ty)
    }

    fn infer_unary_expr(&mut self, file: File, expr: ExprId, op: UnaryOp) -> Ty {
        let ty = self.infer_expr(file, expr);
        let kind = ty.kind();
        if kind == &TyKind::Any {
            return self.types.any(self.db);
        }

        match op {
            UnaryOp::Arith(_) => match kind {
                TyKind::Int => self.types.int(self.db),
                TyKind::Float => self.types.float(self.db),
                _ => self.types.unknown(self.db),
            },
            UnaryOp::Inv => match kind {
                TyKind::Int => self.types.int(self.db),
                _ => self.types.unknown(self.db),
            },
            UnaryOp::Not => self.types.bool(self.db),
        }
    }

    fn infer_binary_expr(&mut self, file: File, lhs: ExprId, rhs: ExprId, op: BinaryOp) -> Ty {
        let lhs = self.infer_expr(file, lhs);
        let rhs = self.infer_expr(file, rhs);
        let lhs = lhs.kind();
        let rhs = rhs.kind();

        if lhs == &TyKind::Any || rhs == &TyKind::Any {
            return self.types.any(self.db);
        }

        match op {
            // TODO(withered-magic): Handle string interoplation with "%".
            BinaryOp::Arith(_) => match (lhs, rhs) {
                (TyKind::Int, TyKind::Int) => self.types.int(self.db),
                (TyKind::Float, TyKind::Int)
                | (TyKind::Int, TyKind::Float)
                | (TyKind::Float, TyKind::Float) => self.types.float(self.db),
                _ => self.types.unknown(self.db),
            },
            BinaryOp::Bitwise(_) => match (lhs, rhs) {
                (TyKind::Int, TyKind::Int) => self.types.int(self.db),
                _ => self.types.unknown(self.db),
            },
            _ => self.types.bool(self.db),
        }
    }

    fn infer_source_expr_assign(&mut self, file: File, source: ExprId) {
        // Find the parent assignment node. This can be either an assignment statement (`x = 0`), a `for` statement (`for x in 1, 2, 3`), or
        // a for comp clause in a list/dict comprehension (`[x + 1 for x in [1, 2, 3]]`).
        let source_ty = self.infer_expr(file, source);
        let info = lower_(self.db, file);
        let source_ptr = info.source_map(self.db).expr_map_back.get(&source).unwrap();
        let parent = source_ptr
            .to_node(&parse(self.db, file).syntax(self.db))
            .syntax()
            .parent()
            .unwrap();

        if let Some(stmt) = ast::AssignStmt::cast(parent.clone()) {
            if let Some(lhs) = stmt.lhs() {
                let lhs_ptr = AstPtr::new(&lhs);
                let expr = info.source_map(self.db).expr_map.get(&lhs_ptr).unwrap();
                self.assign_expr_source_ty(file, *expr, source_ty);
            }
        }
    }

    fn assign_expr_source_ty(&mut self, file: File, expr: ExprId, source_ty: Ty) {
        let module = lower_(self.db, file);
        match module.module(self.db).exprs.get(expr).unwrap() {
            Expr::Name { .. } => {
                self.set_expr_type(file, expr, source_ty);
            }
            Expr::List { exprs } | Expr::Tuple { exprs } => {
                let source_ty = if matches!(source_ty.kind(), TyKind::List { .. })
                    || source_ty == self.types.tuple(self.db)
                    || source_ty == self.types.any(self.db)
                {
                    self.types.any(self.db)
                } else {
                    self.types.unknown(self.db)
                };
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, expr, source_ty.clone());
                }
            }
            Expr::Paren { expr } => self.assign_expr_source_ty(file, *expr, source_ty),
            _ => {}
        }
    }

    fn set_expr_type(&mut self, file: File, expr: ExprId, ty: Ty) -> Ty {
        self.type_of_expr
            .insert(FileExprId { file, expr }, ty.clone());
        ty
    }
}
