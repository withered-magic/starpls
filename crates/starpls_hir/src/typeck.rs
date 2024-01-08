use crate::{
    def::{ExprId, ParamId},
    Name,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use std::sync::Arc;

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

pub enum BuiltinType {
    None,
    Bool,
    Int,
    Float,
    String,
    Bytes,
    List,
    Tuple,
    Dict,
    Function,
}

/// A reference to a type in a source file.
pub enum TypeRef {
    Builtin(BuiltinType),
    Name(Name),
}

pub enum TyKind {
    Unbound,
    Any,
    None,
    Function(FunctionKind),
    /// An instantiable type with methods.
    Class(Class),
}

pub enum FunctionKind {
    Builtin(BuiltinFunction),
    Source(Function),
}

pub struct BuiltinFunction {
    name: Name,
    params: Box<[TypeRef]>,
}

pub struct Function {
    params: Box<[ParamId]>,
}

pub struct Class {
    name: Name,
    fields: Box<[Field]>,
}

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
