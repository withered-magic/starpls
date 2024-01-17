use crate::{
    def::{Expr, ExprId, Literal},
    display::DisplayWithDb,
    lower as lower_,
    typeck::builtins::{
        builtin_field_types, builtin_types, BuiltinClass, BuiltinFunction, BuiltinTypes,
    },
    Db, Declaration, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use starpls_common::{parse, File};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::ast::{self, AstNode, AstPtr, BinaryOp, UnaryOp};
use std::{
    fmt::Write,
    panic::{self, UnwindSafe},
    sync::Arc,
};

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
    Typecheck(TypecheckCancelled),
}

impl Cancelled {
    pub fn catch<F, T>(f: F) -> Result<T, Cancelled>
    where
        F: FnOnce() -> T + UnwindSafe,
    {
        match panic::catch_unwind(f) {
            Ok(t) => Ok(t),
            Err(payload) => match payload.downcast::<salsa::Cancelled>() {
                Ok(cancelled) => Err(Cancelled::Salsa(*cancelled)),
                Err(payload) => match payload.downcast::<TypecheckCancelled>() {
                    Ok(cancelled) => Err(Cancelled::Typecheck(*cancelled)),
                    Err(payload) => panic::resume_unwind(payload),
                },
            },
        }
    }
}

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cancelled::Salsa(err) => err.fmt(f),
            Cancelled::Typecheck(err) => err.fmt(f),
        }
    }
}

#[derive(Debug)]

pub struct TypecheckCancelled;

impl TypecheckCancelled {
    pub(crate) fn throw(self) -> ! {
        std::panic::resume_unwind(Box::new(self))
    }
}

impl std::fmt::Display for TypecheckCancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("type inference cancelled")
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
        let class = self.kind().builtin_class(db)?;
        let names = class.fields(db).iter().map(|field| &field.name);
        let mut subst = Substitution::new();

        // Build the substitution for lists and dicts.
        match self.kind() {
            TyKind::List(ty) => {
                subst.args.push(ty.clone());
            }
            TyKind::Dict(key_ty, value_ty) => {
                subst.args.push(key_ty.clone());
                subst.args.push(value_ty.clone());
            }
            _ => {}
        }

        let types = builtin_field_types(db, class)
            .field_tys(db)
            .iter()
            .map(|binders| binders.substitute(&subst));
        Some(names.zip(types).collect())
    }

    pub fn is_fn(&self) -> bool {
        matches!(self.kind(), TyKind::BuiltinFunction(_, _))
    }

    pub fn is_any(&self) -> bool {
        self.kind() == &TyKind::Any
    }

    fn substitute(&self, args: &[Ty]) -> Ty {
        match self.kind() {
            TyKind::List(ty) => TyKind::List(ty.substitute(args)).intern(),
            TyKind::Tuple(tys) => {
                TyKind::Tuple(tys.iter().map(|ty| ty.substitute(args)).collect()).intern()
            }
            TyKind::Dict(key_ty, value_ty) => {
                TyKind::Dict(key_ty.substitute(args), value_ty.substitute(args)).intern()
            }
            TyKind::BuiltinFunction(data, subst) => {
                TyKind::BuiltinFunction(*data, subst.substitute(args)).intern()
            }
            TyKind::BoundVar(index) => args[*index].clone(),
            _ => self.clone(),
        }
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
    String,
    StringElems,
    Bytes,
    BytesElems,
    List(Ty),
    Tuple(SmallVec<[Ty; 2]>),
    Dict(Ty, Ty),
    BuiltinFunction(BuiltinFunction, Substitution),
    // BuiltinClass(BuiltinClass, Substitution),
    BoundVar(usize),
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
            TyKind::String => "string",
            TyKind::StringElems => "string.elems",
            TyKind::Bytes => "bytes",
            TyKind::BytesElems => "bytes.elems",
            TyKind::List(ty) => {
                f.write_str("list[")?;
                ty.fmt(db, f)?;
                return f.write_char(']');
            }
            TyKind::Tuple(tys) => {
                f.write_str("tuple[")?;
                for (i, ty) in tys.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    ty.fmt(db, f)?;
                }
                return f.write_char(']');
            }
            TyKind::Dict(key_ty, value_ty) => {
                f.write_str("dict[")?;
                key_ty.fmt(db, f)?;
                f.write_str(", ")?;
                value_ty.fmt(db, f)?;
                return f.write_char(']');
            }
            TyKind::BuiltinFunction(fun, subst) => {
                f.write_char('(')?;
                for (i, param_ty) in fun.param_tys(db).iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    param_ty.substitute(&subst.args).fmt(db, f)?;
                }
                f.write_str(") -> ")?;
                return fun.ret_ty(db).substitute(&subst.args).fmt(db, f);
            }
            TyKind::BoundVar(index) => return write!(f, "'{}", index),
        };
        f.write_str(text)
    }
}

impl_internable!(TyKind);

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty(Interned::new(self))
    }

    pub fn builtin_class(&self, db: &dyn Db) -> Option<BuiltinClass> {
        let types = builtin_types(db);
        Some(match self {
            TyKind::String => types.string_base_class(db),
            TyKind::Bytes => types.bytes_base_class(db),
            TyKind::List(_) => types.list_base_class(db),
            TyKind::Dict(_, _) => types.dict_base_class(db),
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binders {
    num_vars: usize,
    ty: Ty,
}

impl Binders {
    pub(crate) fn new(num_vars: usize, ty: Ty) -> Self {
        Self { num_vars, ty }
    }

    pub(crate) fn substitute(&self, subst: &Substitution) -> Ty {
        self.ty.substitute(&subst.args)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Substitution {
    args: SmallVec<[Ty; 2]>,
}

impl Substitution {
    pub(crate) fn new() -> Self {
        Self {
            args: Default::default(),
        }
    }

    pub(crate) fn new_identity(num_vars: usize) -> Self {
        let args = (0..num_vars)
            .map(|index| TyKind::BoundVar(index).intern())
            .collect();
        Self { args }
    }

    pub(crate) fn substitute(&self, args: &[Ty]) -> Self {
        let args = self.args.iter().map(|ty| ty.substitute(args)).collect();
        Self { args }
    }
}

#[derive(Default)]
pub struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    type_of_expr: Arc<Mutex<FxHashMap<FileExprId, Ty>>>,
}

impl GlobalCtxt {
    pub fn cancel(&self) -> CancelGuard {
        CancelGuard::new(self)
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

pub struct CancelGuard<'a> {
    gcx: &'a GlobalCtxt,
    type_of_expr: &'a Mutex<FxHashMap<FileExprId, Ty>>,
}

impl<'a> CancelGuard<'a> {
    fn new(gcx: &'a GlobalCtxt) -> Self {
        gcx.shared_state.cancelled.store(true);
        Self {
            gcx,
            type_of_expr: &gcx.type_of_expr,
        }
    }
}

impl Drop for CancelGuard<'_> {
    fn drop(&mut self) {
        let mut type_of_expr = self.type_of_expr.lock();
        self.gcx.shared_state.cancelled.store(false);
        *type_of_expr = FxHashMap::default();
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
            TypecheckCancelled.throw();
        }

        let db = self.db;
        let info = lower_(db, file);
        let ty = match &info.module(db).exprs[expr] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(db, file, expr);
                let decls = match resolver.resolve_name(name) {
                    Some(decls) => decls,
                    None => return self.set_expr_type(file, expr, self.types.unbound(db)),
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
                            .unwrap_or_else(|| self.types.unknown(db))
                    }
                    Some(
                        Declaration::Function { .. }
                        | Declaration::Parameter { .. }
                        | Declaration::LoadItem {},
                    ) => self.types.any(db),
                    _ => self.types.unbound(db),
                }
            }
            Expr::List { exprs } => {
                // Determine the full type of the list. If all of the specified elements are of the same type T, then
                // we assign the list the type `list[T]`. Otherwise, we assign it the type `list[Unknown]`.
                TyKind::List(self.get_common_type(
                    file,
                    exprs.iter().cloned(),
                    self.types.unknown(db),
                ))
                .intern()
            }
            Expr::ListComp { .. } => TyKind::List(self.types.any(db)).intern(),
            Expr::Dict { entries } => {
                // Determine the dict's key type. For now, if all specified entries have the key type `T`, then we also
                // use the type `T` as the dict's key tpe. Otherwise, we use `Any` as the key type.
                // TODO(withered-magic): Eventually, we should use a union type here.
                let key_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.key),
                    self.types.any(db),
                );

                // Similarly, determine the dict's value type.
                let value_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.value),
                    self.types.unknown(db),
                );
                TyKind::Dict(key_ty, value_ty).intern()
            }
            Expr::DictComp { .. } => TyKind::Dict(self.types.any(db), self.types.any(db)).intern(),
            Expr::Literal { literal } => match literal {
                Literal::Int => self.types.int(db),
                Literal::Float => self.types.float(db),
                Literal::String => self.types.string(db),
                Literal::Bytes => self.types.bytes(db),
                Literal::Bool => self.types.bool(db),
                Literal::None => self.types.none(db),
            },
            Expr::Unary { op, expr } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(file, *expr, op.clone()))
                .unwrap_or_else(|| self.types.unknown(db)),
            Expr::Binary { lhs, rhs, op } => op
                .as_ref()
                .map(|op| self.infer_binary_expr(file, *lhs, *rhs, op.clone()))
                .unwrap_or_else(|| self.types.unknown(db)),
            Expr::Dot { expr, field } => {
                let receiver_ty = self.infer_expr(file, *expr);
                receiver_ty
                    .fields(db)
                    .unwrap_or_else(|| Vec::new())
                    .iter()
                    .find_map(|(field2, ty)| {
                        if field == *field2 {
                            Some(ty.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.types.unknown(db))
            }
            Expr::Index { lhs, index } => {
                let lhs_ty = self.infer_expr(file, *lhs);
                let index_ty = self.infer_expr(file, *index);
                match (lhs_ty.kind(), index_ty.kind()) {
                    (TyKind::List(ty), TyKind::Int) => ty.clone(),
                    (TyKind::Dict(key_ty, value_ty), index_kind) => {
                        if key_ty.kind() == index_kind {
                            value_ty.clone()
                        } else {
                            self.types.unknown(db)
                        }
                    }
                    _ => self.types.unknown(db),
                }
            }
            Expr::Call { callee, .. } => match self.infer_expr(file, *callee).kind() {
                TyKind::BuiltinFunction(fun, subst) => fun.ret_ty(db).substitute(&subst.args),
                _ => self.types.unknown(db),
            },
            _ => self.types.any(db),
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
                let sub_ty = match source_ty.kind() {
                    TyKind::List(ty) => ty.clone(),
                    TyKind::Tuple(_) | TyKind::Any => self.types.any(self.db),
                    _ => self.types.unknown(self.db),
                };
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, expr, sub_ty.clone());
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

    fn get_common_type(
        &mut self,
        file: File,
        mut exprs: impl Iterator<Item = ExprId>,
        default: Ty,
    ) -> Ty {
        let first = exprs.next();
        first
            .map(|first| self.infer_expr(file, first))
            .and_then(|first_ty| {
                exprs
                    .map(|expr| self.infer_expr(file, expr))
                    .all(|ty| ty == first_ty)
                    .then_some(first_ty)
            })
            .unwrap_or(default)
    }

    fn type_is_assignable(&self, source: Ty, target: Ty) -> bool {
        if target.is_any() {
            return true;
        }
        true
    }
}
