use crate::{
    def::{scope::module_scopes, Expr, ExprId, ModuleSourceMap, ParamId},
    lower as lower_, Db, Module, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use starpls_common::File;
use starpls_intern::{impl_internable, Interned};
use std::sync::Arc;

use self::builtins::Builtins;

mod builtins;
mod lower;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileExprId(File, ExprId);

#[derive(Debug)]

pub struct Cancelled;

impl Cancelled {
    pub(crate) fn throw(self) -> ! {
        std::panic::resume_unwind(Box::new(self))
    }
}

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("type inference cancelled")
    }
}

impl std::error::Error for Cancelled {}

struct SharedState {
    cancelled: AtomicCell<bool>,
    builtins: Builtins,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TyKind {
    Unbound,
    Any,
    None,
    Function(FunctionKind),
    /// An instantiable type with methods.
    Class(Class),
}

impl_internable!(TyKind);

impl TyKind {
    fn intern(self) -> Ty {
        Ty(Interned::new(self))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FunctionKind {
    Builtin(BuiltinFunction),
    Source(Function),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BuiltinFunction {
    // Parameters for builtin functions are positional only and don't have names.
    params: Box<[TypeRef]>,
    ret_type_ref: Option<TypeRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    name: Option<Name>,
    params: Box<[ParamId]>,
    ret_type_ref: Option<TypeRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    name: Name,
    fields: Fields,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Fields {
    Builtin(Box<[BuiltinField]>),
    Source(Box<[Field]>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BuiltinField {
    name: Name,
    ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    name: Name,
    type_ref: TypeRef,
}

pub struct TyCtxt {
    shared_state: Arc<SharedState>,
    gcx: Arc<Mutex<GlobalCtxt>>,
}

pub struct TyCtxtSnapshot {
    shared_state: Arc<SharedState>,
    gcx: Arc<Mutex<GlobalCtxt>>,
}

impl TyCtxt {
    pub fn new_with_builtins(builtins: Builtins) -> Self {
        let shared_state = Arc::new(SharedState {
            builtins,
            cancelled: Default::default(),
        });
        let gcx = Arc::new(Mutex::new(GlobalCtxt {
            shared_state: Arc::clone(&shared_state),
            type_of_expr: Default::default(),
        }));
        Self { shared_state, gcx }
    }

    pub fn type_of_expr(&self, db: &dyn Db, expr: FileExprId) -> Ty {
        self.gcx.lock().type_of_expr(db, expr)
    }

    pub fn cancel(&self) {
        self.shared_state.cancelled.store(true);
        let mut gcx = self.gcx.lock();
        self.shared_state.cancelled.store(false);
        *gcx = GlobalCtxt {
            shared_state: Arc::clone(&self.shared_state),
            type_of_expr: FxHashMap::default(),
        }
    }

    pub fn snapshot(&self) -> TyCtxtSnapshot {
        TyCtxtSnapshot {
            shared_state: Arc::clone(&self.shared_state),
            gcx: Arc::clone(&self.gcx),
        }
    }
}

struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    type_of_expr: FxHashMap<FileExprId, Ty>,
}

impl GlobalCtxt {
    fn type_of_expr(&mut self, db: &dyn Db, expr: FileExprId) -> Ty {
        if let Some(ty) = self.type_of_expr.get(&expr).cloned() {
            return ty;
        }

        if self.shared_state.cancelled.load() {
            Cancelled.throw();
        }

        let info = lower_(db, expr.0);

        match &info.module(db).exprs[expr.1] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(db, expr.0, expr.1);
                let decls = match resolver.resolve_name(name) {
                    Some(decls) => decls,
                    None => return self.set_type_of_expr(expr, self.builtins().any_ty()),
                };
                self.set_type_of_expr(expr, self.builtins().any_ty())
            }
            _ => self.set_type_of_expr(expr, self.builtins().any_ty()),
        }
    }

    fn set_type_of_expr(&mut self, expr: FileExprId, ty: Ty) -> Ty {
        self.type_of_expr.insert(expr, ty.clone());
        ty
    }

    fn builtins(&self) -> &Builtins {
        &self.shared_state.builtins
    }
}
