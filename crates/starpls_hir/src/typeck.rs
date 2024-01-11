use crate::{
    def::{Expr, ExprId, Literal, ParamId},
    lower as lower_, Db, Declaration, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use starpls_common::{parse, File};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::ast::{self, AstNode, AstPtr, BinaryOp, UnaryOp};
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
            TyKind::Unknown => f.write_str("Unknown"),
            TyKind::None => f.write_str("NoneType"),
            TyKind::Bool => f.write_str("bool"),
            TyKind::Int => f.write_str("int"),
            TyKind::Float => f.write_str("float"),
            TyKind::Function(_) => f.write_str("function"),
            TyKind::Class(class) => f.write_str(&class.name.as_str()),
        }
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
    pub fn infer_expr(&self, db: &dyn Db, file: File, expr: ExprId) -> Ty {
        self.gcx.lock().infer_expr(db, file, expr)
    }
}

struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    type_of_expr: FxHashMap<FileExprId, Ty>,
}

impl GlobalCtxt {
    fn infer_expr(&mut self, db: &dyn Db, file: File, expr: ExprId) -> Ty {
        if let Some(ty) = self.type_of_expr.get(&FileExprId { file, expr }).cloned() {
            return ty;
        }

        if self.shared_state.cancelled.load() {
            Cancelled.throw();
        }

        let info = lower_(db, file);
        let ty = match &info.module(db).exprs[expr] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(db, file, expr);
                let decls = match resolver.resolve_name(name) {
                    Some(decls) => decls,
                    None => return self.set_expr_type(file, expr, self.builtins().unbound_ty()),
                };
                match decls.last() {
                    Some(Declaration::Variable { id, source }) => {
                        return source
                            .and_then(|source| {
                                self.infer_source_expr_assign(db, file, source);
                                self.type_of_expr
                                    .get(&FileExprId { file, expr: *id })
                                    .cloned()
                            })
                            .unwrap_or_else(|| self.builtins().unknown_ty())
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
            Expr::Unary { op, expr } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(db, file, *expr, op.clone()))
                .unwrap_or_else(|| self.builtins().unknown_ty()),
            Expr::Binary { lhs, rhs, op } => op
                .as_ref()
                .map(|op| self.infer_binary_expr(db, file, *lhs, *rhs, op.clone()))
                .unwrap_or_else(|| self.builtins().unknown_ty()),
            _ => self.builtins().any_ty(),
        };
        self.set_expr_type(file, expr, ty)
    }

    fn infer_unary_expr(&mut self, db: &dyn Db, file: File, expr: ExprId, op: UnaryOp) -> Ty {
        let ty = self.infer_expr(db, file, expr);
        let kind = ty.kind();
        if kind == &TyKind::Any {
            return self.builtins().any_ty();
        }

        match op {
            UnaryOp::Arith(_) => match kind {
                TyKind::Int => self.builtins().int_ty(),
                TyKind::Float => self.builtins().float_ty(),
                _ => self.builtins().unknown_ty(),
            },
            UnaryOp::Inv => match kind {
                TyKind::Int => self.builtins().int_ty(),
                _ => self.builtins().unknown_ty(),
            },
            UnaryOp::Not => self.builtins().bool_ty(),
        }
    }

    fn infer_binary_expr(
        &mut self,
        db: &dyn Db,
        file: File,
        lhs: ExprId,
        rhs: ExprId,
        op: BinaryOp,
    ) -> Ty {
        let lhs = self.infer_expr(db, file, lhs);
        let rhs = self.infer_expr(db, file, rhs);
        let lhs = lhs.kind();
        let rhs = rhs.kind();

        if lhs == &TyKind::Any || rhs == &TyKind::Any {
            return self.builtins().any_ty();
        }

        match op {
            // TODO(withered-magic): Handle string interoplation with "%".
            BinaryOp::Arith(_) => match (lhs, rhs) {
                (TyKind::Int, TyKind::Int) => self.builtins().int_ty(),
                (TyKind::Float, TyKind::Int)
                | (TyKind::Int, TyKind::Float)
                | (TyKind::Float, TyKind::Float) => self.builtins().float_ty(),
                _ => self.builtins().unknown_ty(),
            },
            BinaryOp::Bitwise(_) => match (lhs, rhs) {
                (TyKind::Int, TyKind::Int) => self.builtins().int_ty(),
                _ => self.builtins().unknown_ty(),
            },
            _ => self.builtins().bool_ty(),
        }
    }

    fn infer_source_expr_assign(&mut self, db: &dyn Db, file: File, source: ExprId) {
        // Find the parent assignment node. This can be either an assignment statement (`x = 0`), a `for` statement (`for x in 1, 2, 3`), or
        // a for comp clause in a list/dict comprehension (`[x + 1 for x in [1, 2, 3]]`).
        let source_ty = self.infer_expr(db, file, source);
        let info = lower_(db, file);
        let source_ptr = info.source_map(db).expr_map_back.get(&source).unwrap();
        let parent = source_ptr
            .to_node(&parse(db, file).syntax(db))
            .syntax()
            .parent()
            .unwrap();

        if let Some(stmt) = ast::AssignStmt::cast(parent.clone()) {
            if let Some(lhs) = stmt.lhs() {
                let lhs_ptr = AstPtr::new(&lhs);
                let expr = info.source_map(db).expr_map.get(&lhs_ptr).unwrap();
                self.assign_expr_source_ty(db, file, *expr, source_ty);
            }
        }
    }

    fn assign_expr_source_ty(&mut self, db: &dyn Db, file: File, expr: ExprId, source_ty: Ty) {
        let module = lower_(db, file);
        match module.module(db).exprs.get(expr).unwrap() {
            Expr::Name { .. } => {
                self.set_expr_type(file, expr, source_ty);
            }
            Expr::List { exprs } | Expr::Tuple { exprs } => {
                let source_ty = if source_ty == self.builtins().list_ty()
                    || source_ty == self.builtins().tuple_ty()
                    || source_ty == self.builtins().any_ty()
                {
                    self.builtins().any_ty()
                } else {
                    self.builtins().unknown_ty()
                };
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(db, file, expr, source_ty.clone());
                }
            }
            Expr::Paren { expr } => self.assign_expr_source_ty(db, file, *expr, source_ty),
            _ => {}
        }
    }

    fn set_expr_type(&mut self, file: File, expr: ExprId, ty: Ty) -> Ty {
        self.type_of_expr
            .insert(FileExprId { file, expr }, ty.clone());
        ty
    }

    fn builtins(&self) -> &Builtins {
        &self.shared_state.builtins
    }
}
