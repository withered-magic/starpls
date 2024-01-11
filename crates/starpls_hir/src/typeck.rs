use crate::{
    def::{Expr, ExprId, Literal, ParamId},
    lower as lower_, Db, Declaration, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use starpls_common::File;
use starpls_intern::{impl_internable, Interned};
use std::sync::Arc;

pub use crate::typeck::builtins::{intern_builtins, Builtins};

mod builtins;
mod lower;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileExprId {
    pub file: File,
    pub expr: ExprId,
}

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

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            TyKind::Unbound => f.write_str("Unbound"),
            TyKind::Any => f.write_str("Any"),
            TyKind::None => f.write_str("NoneType"),
            TyKind::Function(_) => f.write_str("function"),
            TyKind::Class(class) => f.write_str(&class.name.as_str()),
        }
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
            gcx: Arc::clone(&self.gcx),
        }
    }
}

impl TyCtxtSnapshot {
    pub fn type_of_expr(&self, db: &dyn Db, expr: FileExprId) -> Ty {
        self.gcx.lock().type_of_expr(db, expr)
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

        let info = lower_(db, expr.file);

        let ty = match &info.module(db).exprs[expr.expr] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(db, expr.file, expr.expr);
                let decls = match resolver.resolve_name(name) {
                    Some(decls) => decls,
                    None => return self.set_expr_type(expr, self.builtins().unbound_ty()),
                };
                match decls.last() {
                    Some(Declaration::Variable { id, source }) => {
                        let source_ty = match source {
                            Some(source) => self.type_of_expr(
                                db,
                                FileExprId {
                                    file: expr.file,
                                    expr: *source,
                                },
                            ),
                            None => self.builtins().any_ty(),
                        };
                        // TODO(withered-magic): Find the parent assignment node and call assign_expr_type_rec on the real lhs.
                        self.assign_expr_type_rec(
                            FileExprId {
                                file: expr.file,
                                expr: *id,
                            },
                            source_ty,
                        );
                        self.type_of_expr
                            .get(&FileExprId {
                                file: expr.file,
                                expr: *id,
                            })
                            .cloned()
                            .unwrap()
                    }
                    Some(
                        Declaration::Function { .. }
                        | Declaration::Parameter { .. }
                        | Declaration::LoadItem {},
                    ) => self.builtins().any_ty(),
                    _ => self.builtins().unbound_ty(),
                }
            }
            Expr::List { .. } | Expr::ListComp { .. } => self.builtins().list_ty(),
            Expr::Dict { .. } | Expr::DictComp { .. } => self.builtins().dict_ty(),
            Expr::Literal { literal } => match literal {
                Literal::Int => self.builtins().int_ty(),
                Literal::Float => self.builtins().float_ty(),
                Literal::String => self.builtins().string_ty(),
                Literal::Bytes => self.builtins().bytes_ty(),
                Literal::Bool => self.builtins().bool_ty(),
                Literal::None => self.builtins().none_ty(),
            },
            _ => self.builtins().any_ty(),
        };

        self.set_expr_type(expr, ty)
    }

    fn assign_expr_type_rec(&mut self, lhs: FileExprId, rhs_ty: Ty) {
        self.type_of_expr.insert(lhs, rhs_ty);
    }

    fn set_expr_type(&mut self, expr: FileExprId, ty: Ty) -> Ty {
        self.type_of_expr.insert(expr, ty.clone());
        ty
    }

    fn builtins(&self) -> &Builtins {
        &self.shared_state.builtins
    }
}
