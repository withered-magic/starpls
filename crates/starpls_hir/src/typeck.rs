use crate::{
    def::{ExprId, ParamId},
    Name,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use starpls_intern::{impl_internable, Interned};
use std::sync::Arc;

mod builtins;
mod lower;

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

impl TyCtxt {
    pub fn type_of_expr(&self) {
        self.gcx.lock().type_of_expr()
    }

    pub fn cancel(&self) {}
}

struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    type_of_expr: FxHashMap<ExprId, ExprId>,
}

impl GlobalCtxt {
    fn type_of_expr(&mut self) {
        if self.shared_state.cancelled.load() {
            Cancelled.throw();
        }
    }
}
