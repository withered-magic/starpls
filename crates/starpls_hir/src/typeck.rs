use crate::{
    def::{Argument, Expr, ExprId, Function, Literal, ModuleSourceMap, Param, ParamId},
    display::DisplayWithDb,
    module, source_map,
    typeck::{
        builtins::{builtin_types, BuiltinFunction, BuiltinFunctionParam, BuiltinType},
        intrinsics::{
            intrinsic_field_types, intrinsic_types, IntrinsicClass, IntrinsicFunction,
            IntrinsicFunctionParam, IntrinsicTypes, Intrinsics,
        },
    },
    Db, Declaration, Name, Resolver,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};
use starpls_common::{parse, Diagnostic, File, FileRange, Severity};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::{
    ast::{self, ArithOp, AstNode, AstPtr, BinaryOp, UnaryOp},
    TextRange,
};
use std::{
    fmt::{Display, Write},
    panic::{self, UnwindSafe},
    sync::Arc,
};

mod lower;

pub(crate) mod builtins;
pub(crate) mod intrinsics;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileExprId {
    pub file: File,
    pub expr: ExprId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileParamId {
    pub file: File,
    pub param: ParamId,
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

/// A reference to a type in a source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeRef {
    Name(Name),
    Unknown,
}

impl TypeRef {
    pub(crate) fn from_str_opt(name: &str) -> Self {
        if name.is_empty() {
            Self::Unknown
        } else {
            Self::Name(Name::from_str(name))
        }
    }
}

impl std::fmt::Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeRef::Name(name) => name.as_str(),
            TypeRef::Unknown => "unknown",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ty(Interned<TyKind>);

impl Ty {
    pub(crate) fn kind(&self) -> &TyKind {
        &self.0
    }

    pub fn fields<'a>(&'a self, db: &'a dyn Db) -> Option<Vec<(&'a Name, Ty)>> {
        if let Some(class) = self.kind().builtin_class(db) {
            Some(self.builtin_class_fields(db, class))
        } else if let TyKind::BuiltinType(cty) = self.kind() {
            Some(
                cty.fields(db)
                    .iter()
                    .map(|field| {
                        (
                            &field.name,
                            resolve_type_ref(db, &field.type_ref)
                                .unwrap_or_else(|| TyKind::Unknown.intern()),
                        )
                    })
                    .chain(
                        cty.methods(db)
                            .iter()
                            .map(|func| (func.name(db), TyKind::BuiltinFunction(*func).intern())),
                    )
                    .collect(),
            )
        } else {
            None
        }
    }

    fn builtin_class_fields<'a>(
        &'a self,
        db: &'a dyn Db,
        class: IntrinsicClass,
    ) -> Vec<(&'a Name, Ty)> {
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

        let types = intrinsic_field_types(db, class)
            .field_tys(db)
            .iter()
            .map(|binders| binders.substitute(&subst));
        names.zip(types).collect()
    }

    pub fn param_names<'a>(&'a self, db: &'a dyn Db) -> Vec<Name> {
        match self.kind() {
            TyKind::Function(func) => func
                .params(db)
                .map(|param| match param {
                    Param::Simple { name, .. }
                    | Param::ArgsList { name, .. }
                    | Param::KwargsList { name, .. } => name.clone(),
                })
                .collect(),
            TyKind::IntrinsicFunction(func, _) => func
                .params(db)
                .iter()
                .filter_map(|param| param.name().cloned())
                .collect(),
            _ => vec![],
        }
    }

    pub fn is_fn(&self) -> bool {
        matches!(
            self.kind(),
            TyKind::Function(_) | TyKind::IntrinsicFunction(_, _) | TyKind::BuiltinFunction(_)
        )
    }

    pub fn is_user_defined_fn(&self) -> bool {
        matches!(self.kind(), TyKind::Function(_))
    }

    pub fn is_any(&self) -> bool {
        self.kind() == &TyKind::Any
    }

    pub fn is_unknown(&self) -> bool {
        self.kind() == &TyKind::Unknown || self.kind() == &TyKind::Unbound
    }

    pub fn is_iterable(&self) -> bool {
        matches!(
            self.kind(),
            TyKind::Dict(_, _)
                | TyKind::List(_)
                | TyKind::Tuple(_)
                | TyKind::StringElems
                | TyKind::BytesElems
        )
    }

    pub fn is_sequence(&self) -> bool {
        matches!(
            self.kind(),
            TyKind::Dict(_, _) | TyKind::List(_) | TyKind::Tuple(_)
        )
    }

    pub fn is_indexable(&self) -> bool {
        matches!(
            self.kind(),
            TyKind::String | TyKind::Bytes | TyKind::Tuple(_) | TyKind::List(_)
        )
    }

    pub fn is_set_indexable(&self) -> bool {
        matches!(self.kind(), TyKind::List(_))
    }

    pub fn is_mapping(&self) -> bool {
        matches!(self.kind(), TyKind::Dict(_, _))
    }

    pub fn is_var(&self) -> bool {
        matches!(self.kind(), TyKind::BoundVar(_))
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
            TyKind::IntrinsicFunction(data, subst) => {
                TyKind::IntrinsicFunction(*data, subst.substitute(args)).intern()
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
    /// An unbound variable, e.g. a variable without a corresponding
    /// declaration.
    Unbound,
    /// A value whose actual type is unknown. This is usually the
    /// result of failed type inference, e.g. calling an unbound
    /// function.
    Unknown,
    /// Similar to `Unknown`, but not necessarily the result of failed
    /// type inference.
    Any,
    /// The type of the predefined `None` variable.
    None,
    /// A boolean.
    Bool,
    /// A 64-bit integer.
    Int,
    /// A 64-bit floating point number.
    Float,
    /// A UTF-8 encoded string.
    String,
    /// The individual characters of a UTF-8 encoded string.
    StringElems,
    /// A series of bytes.
    Bytes,
    /// An iterable collection of bytes.
    BytesElems,
    /// A list type, e.g. `list[string]`
    List(Ty),
    /// A fixed-size collection of elements.
    Tuple(SmallVec<[Ty; 2]>),
    /// A mapping of keys to values.
    Dict(Ty, Ty),
    /// An iterable and indexable sequence of numbers. Obtained from
    /// the `range()` function.
    Range,
    /// A user-defined function.
    Function(Function),
    /// A function predefined by the Starlark specification.
    IntrinsicFunction(IntrinsicFunction, Substitution),
    /// A function defined outside of the Starlark specification.
    /// For example, common Bazel functions like `genrule()`.
    BuiltinFunction(BuiltinFunction),
    /// A type defined outside of the Starlark specification.
    /// For example, common Bazel types like `Label`.
    BuiltinType(BuiltinType),
    /// A bound type variable, e.g. the argument to the `append()` method
    /// of the `list[int]` class.
    BoundVar(usize),
    /// A marker type that indicates some specific behavior, e.g. Sequence[T].
    Protocol(Protocol),
}

impl DisplayWithDb for TyKind {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            TyKind::Range => "range",
            TyKind::Function(func) => {
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func.params(db).enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match param {
                        Param::Simple { name, type_ref, .. } => {
                            let type_ = match type_ref {
                                Some(TypeRef::Name(name)) => name.as_str(),
                                _ => "Unknown",
                            };
                            write!(f, "{}: {}", name.as_str(), type_)?;
                        }
                        Param::ArgsList { name, .. } => {
                            write!(f, "*{}: Unknown", name.as_str())?;
                        }
                        Param::KwargsList { name, .. } => {
                            write!(f, "**{}", name.as_str())?;
                        }
                    }
                }
                return f.write_str(") -> Unknown");
            }
            TyKind::IntrinsicFunction(func, subst) => {
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func.params(db).iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match param {
                        IntrinsicFunctionParam::Positional { ty, optional } => {
                            write!(f, "x{}: ", i)?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                            if *optional {
                                f.write_str(" = None")?;
                            }
                        }
                        IntrinsicFunctionParam::Keyword { name, ty } => {
                            f.write_str(name.as_str())?;
                            f.write_str(": ")?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                            f.write_str(" = None")?;
                        }
                        IntrinsicFunctionParam::VarArgList { ty } => {
                            f.write_str("*args: ")?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                        }
                        IntrinsicFunctionParam::VarArgDict => {
                            f.write_str("**kwargs")?;
                        }
                    }
                }
                f.write_str(") -> ")?;
                return func.ret_ty(db).substitute(&subst.args).fmt(db, f);
            }
            TyKind::BuiltinFunction(func) => {
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func.params(db).iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match param {
                        BuiltinFunctionParam::Normal { name, .. } => f.write_str(name.as_str())?,
                        BuiltinFunctionParam::VarArgList { .. } => f.write_str("*args")?,
                        BuiltinFunctionParam::VarArgDict => f.write_str("**kwargs")?,
                    }
                }
                f.write_str(") -> ")?;
                return func.ret_type_ref(db).fmt(f);
            }
            TyKind::BuiltinType(type_) => type_.name(db).as_str(),
            TyKind::BoundVar(index) => return write!(f, "'{}", index),
            TyKind::Protocol(_proto) => "protocol",
        };
        f.write_str(text)
    }

    fn fmt_alt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TyKind::Function(_) => f.write_str("function"),
            TyKind::IntrinsicFunction(_, _) | TyKind::BuiltinFunction(_) => {
                f.write_str("builtin_function_or_method")
            }
            _ => self.fmt(db, f),
        }
    }
}

impl_internable!(TyKind);

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty(Interned::new(self))
    }

    pub fn builtin_class(&self, db: &dyn Db) -> Option<IntrinsicClass> {
        let intrinsics = intrinsic_types(db);
        Some(match self {
            TyKind::String => intrinsics.string_base_class(db),
            TyKind::Bytes => intrinsics.bytes_base_class(db),
            TyKind::List(_) => intrinsics.list_base_class(db),
            TyKind::Dict(_, _) => intrinsics.dict_base_class(db),
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

/// A marker type indicating that a value fulfills some behavior.
/// For example, `list[int]` fulfills `Sequence[int]`. These types
/// are used mostly by builtins that might return values that fulfill
/// these behaviors but aren't known by the developer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Protocol {
    Iterable(Ty),
    Sequence(Ty),
    Indexable(Ty),
    SetIndexable(Ty),
    Mapping(Ty, Ty),
}

impl Protocol {
    // TODO(withered-magic): This doesn't yet take subtypes into account.
    pub(crate) fn assign_ty(&self, ty: &Ty) -> bool {
        let kind = ty.kind();
        match self {
            // Dicts, lists, and tuples are all iterable sequences.
            Protocol::Iterable(lhs_ty) | Protocol::Sequence(lhs_ty) => match kind {
                TyKind::List(rhs_ty) | TyKind::Dict(rhs_ty, _) => assign_tys(rhs_ty, lhs_ty),
                TyKind::Tuple(rhs_tys) => rhs_tys.iter().all(|rhs_ty| assign_tys(rhs_ty, lhs_ty)),
                _ => false,
            },
            // Strings, byte literals, tuples, and lists are indexable.
            Protocol::Indexable(target) => match kind {
                TyKind::String => assign_tys(&TyKind::String.intern(), target),
                TyKind::Bytes => assign_tys(&TyKind::Int.intern(), target),
                TyKind::Tuple(_) => true,
                TyKind::List(source) => assign_tys(source, target),
                _ => false,
            },
            // Only lists can have their elements set by an indexing expression.
            // Tuples are immutable and do not fall under this category.
            Protocol::SetIndexable(target) => match kind {
                TyKind::List(source) => assign_tys(source, target),
                _ => false,
            },
            Protocol::Mapping(target_key_ty, target_value_ty) => match kind {
                TyKind::Dict(source_key_ty, source_value_ty) => {
                    assign_tys(source_key_ty, target_key_ty)
                        && assign_tys(source_value_ty, target_value_ty)
                }
                _ => false,
            },
        }
    }
}

#[derive(Default)]
pub struct GlobalCtxt {
    shared_state: Arc<SharedState>,
    cx: Arc<Mutex<InferenceCtxt>>,
}

impl GlobalCtxt {
    pub fn cancel(&self) -> CancelGuard {
        CancelGuard::new(self)
    }

    pub fn with_tcx<F, T>(&self, db: &dyn Db, mut f: F) -> T
    where
        F: FnMut(&mut TyCtxt) -> T + std::panic::UnwindSafe,
    {
        let mut cx = self.cx.lock();
        let mut tcx = TyCtxt {
            db,
            cx: &mut cx,
            intrinsics: intrinsic_types(db),
            shared_state: Arc::clone(&self.shared_state),
        };
        f(&mut tcx)
    }
}

#[derive(Default)]
struct InferenceCtxt {
    diagnostics: Vec<Diagnostic>,
    param_tys: FxHashMap<FileParamId, Ty>,
    type_of_expr: FxHashMap<FileExprId, Ty>,
}

pub struct CancelGuard<'a> {
    gcx: &'a GlobalCtxt,
    cx: &'a Mutex<InferenceCtxt>,
}

impl<'a> CancelGuard<'a> {
    fn new(gcx: &'a GlobalCtxt) -> Self {
        gcx.shared_state.cancelled.store(true);
        Self { gcx, cx: &gcx.cx }
    }
}

impl Drop for CancelGuard<'_> {
    fn drop(&mut self) {
        let mut cx = self.cx.lock();
        self.gcx.shared_state.cancelled.store(false);
        *cx = Default::default();
    }
}

pub struct TyCtxt<'a> {
    db: &'a dyn Db,
    cx: &'a mut InferenceCtxt,
    intrinsics: Intrinsics,
    shared_state: Arc<SharedState>,
}

impl TyCtxt<'_> {
    pub fn infer_all_exprs(&mut self, file: File) {
        for (expr, _) in module(self.db, file).exprs.iter() {
            self.infer_expr(file, expr);
        }
    }

    pub fn diagnostics_for_file(&self, file: File) -> Vec<Diagnostic> {
        self.cx
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.range.file_id == file.id(self.db))
            .cloned()
            .collect()
    }

    fn unwind_if_cancelled(&self) {
        if self.shared_state.cancelled.load() {
            TypecheckCancelled.throw();
        }
    }

    pub fn infer_expr(&mut self, file: File, expr: ExprId) -> Ty {
        if let Some(ty) = self
            .cx
            .type_of_expr
            .get(&FileExprId { file, expr })
            .cloned()
        {
            return ty;
        }

        self.unwind_if_cancelled();

        let db = self.db;
        let ty = match &module(db, file)[expr] {
            Expr::Name { name } => {
                let resolver = Resolver::new_for_expr(db, file, expr);
                resolver
                    .resolve_name(name)
                    .and_then(|decls| decls.into_iter().last())
                    .map(|decl| match decl {
                        Declaration::Variable { id, source } => self
                            .cx
                            .type_of_expr
                            .get(&FileExprId { file, expr: id })
                            .cloned()
                            .unwrap_or_else(|| {
                                source
                                    .and_then(|source| {
                                        self.infer_source_expr_assign(file, source);
                                        self.cx
                                            .type_of_expr
                                            .get(&FileExprId { file, expr: id })
                                            .cloned()
                                    })
                                    .unwrap_or_else(|| self.unknown_ty())
                            }),
                        Declaration::Function { func, .. } => func.ty(),
                        Declaration::IntrinsicFunction { func } => {
                            TyKind::IntrinsicFunction(func, Substitution::new_identity(0)).intern()
                        }
                        Declaration::BuiltinFunction { func } => {
                            TyKind::BuiltinFunction(func).intern()
                        }
                        Declaration::CustomVariable { type_ref } => {
                            resolve_type_ref(db, &type_ref).unwrap_or_else(|| self.unknown_ty())
                        }
                        Declaration::Parameter { id, .. } => self.param_ty(file, id),
                        _ => self.any_ty(),
                    })
                    .unwrap_or_else(|| {
                        self.add_expr_diagnostic(
                            file,
                            expr,
                            format!("\"{}\" is not defined", name.as_str()),
                        );
                        self.unbound_ty()
                    })
            }
            Expr::List { exprs } => {
                // Determine the full type of the list. If all of the specified elements are of the same type T, then
                // we assign the list the type `list[T]`. Otherwise, we assign it the type `list[Unknown]`.
                TyKind::List(self.get_common_type(file, exprs.iter().cloned(), self.unknown_ty()))
                    .intern()
            }
            Expr::ListComp { expr, .. } => TyKind::List(self.infer_expr(file, *expr)).intern(),
            Expr::Dict { entries } => {
                // Determine the dict's key type. For now, if all specified entries have the key type `T`, then we also
                // use the type `T` as the dict's key tpe. Otherwise, we use `Any` as the key type.
                // TODO(withered-magic): Eventually, we should use a union type here.
                let key_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.key),
                    self.any_ty(),
                );

                // Similarly, determine the dict's value type.
                let value_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.value),
                    self.unknown_ty(),
                );
                TyKind::Dict(key_ty, value_ty).intern()
            }
            Expr::DictComp { entry, .. } => {
                let key_ty = self.infer_expr(file, entry.key);
                let value_ty = self.infer_expr(file, entry.value);
                TyKind::Dict(key_ty, value_ty).intern()
            }
            Expr::Literal { literal } => match literal {
                Literal::Int(_) => self.int_ty(),
                Literal::Float => self.float_ty(),
                Literal::String(_) => self.string_ty(),
                Literal::Bytes => self.bytes_ty(),
                Literal::Bool(_) => self.bool_ty(),
                Literal::None => self.none_ty(),
            },
            Expr::Unary {
                op,
                expr: unary_expr,
            } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(file, expr, *unary_expr, op.clone()))
                .unwrap_or_else(|| self.unknown_ty()),
            Expr::Binary { lhs, rhs, op } => op
                .as_ref()
                .map(|op| self.infer_binary_expr(file, expr, *lhs, *rhs, op.clone()))
                .unwrap_or_else(|| self.unknown_ty()),
            Expr::Dot {
                expr: dot_expr,
                field,
            } => {
                let receiver_ty = self.infer_expr(file, *dot_expr);

                // Special-casing for "Any", "Unknown", "Unbound", invalid field
                // names, and Bazel `struct`s.
                // TODO(withered-magic): Is there a better way to handle `struct`s here?
                if receiver_ty.is_any() {
                    return self.any_ty();
                }

                if receiver_ty.is_unknown() || field.is_missing() {
                    return self.unknown_ty();
                }

                if let TyKind::BuiltinType(type_) = receiver_ty.kind() {
                    if type_.name(db).as_str() == "struct" {
                        return self.unknown_ty();
                    }
                }

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
                    .unwrap_or_else(|| {
                        self.add_expr_diagnostic(
                            file,
                            expr,
                            format!(
                                "Cannot access field \"{}\" for type \"{}\"",
                                field.as_str(),
                                receiver_ty.display(db)
                            ),
                        )
                    })
            }
            Expr::Index { lhs, index } => {
                let lhs_ty = self.infer_expr(file, *lhs);
                let index_ty = self.infer_expr(file, *index);
                let int_ty = self.int_ty();
                let string_ty = self.string_ty();

                // Lists, dictionaries, strings, byte literals, and range values, as
                // well as the `Indexable` and `SetIndexable` protocols, support indexing.
                let (target, value, name) = match lhs_ty.kind() {
                    TyKind::List(ty) => (&int_ty, ty, "list"),
                    TyKind::Dict(key_ty, value_ty) => (key_ty, value_ty, "dict"),
                    TyKind::String => (&int_ty, &string_ty, "string"),
                    TyKind::Bytes => (&int_ty, &int_ty, "bytes"),
                    TyKind::Range => (&int_ty, &int_ty, "range"),
                    TyKind::Protocol(Protocol::Indexable(ty) | Protocol::SetIndexable(ty)) => {
                        (&int_ty, ty, "Indexable")
                    }
                    _ => {
                        let return_ty = self.add_expr_diagnostic(
                            file,
                            expr,
                            format!("Type \"{}\" is not indexable", lhs_ty.display(db).alt()),
                        );
                        return self.set_expr_type(file, expr, return_ty);
                    }
                };

                if assign_tys(&index_ty, target) {
                    value.clone()
                } else {
                    self.add_expr_diagnostic(
                        file,
                        *lhs,
                        format!(
                            "Cannot index {} with type \"{}\"",
                            name,
                            index_ty.display(db).alt()
                        ),
                    )
                }
            }
            Expr::Call { callee, args } => {
                let callee_ty = self.infer_expr(file, *callee);
                let args_with_ty: SmallVec<[(Argument, Ty); 5]> = args
                    .iter()
                    .cloned()
                    .map(|arg| {
                        let arg_ty = match &arg {
                            Argument::Simple { expr }
                            | Argument::Keyword { expr, .. }
                            | Argument::UnpackedList { expr }
                            | Argument::UnpackedDict { expr } => self.infer_expr(file, *expr),
                        };
                        (arg, arg_ty)
                    })
                    .collect();

                match callee_ty.kind() {
                    TyKind::Function(_) => {
                        // TODO: Handle slot assignments.
                        self.any_ty()
                    }
                    TyKind::IntrinsicFunction(func, subst) => {
                        // Match arguments with their corresponding parameters.
                        // The following routine is based on PEP 3102 (https://peps.python.org/pep-3102),
                        // but with a couple of modifications for handling "*args" and "**kwargs" arguments.
                        #[derive(Clone, Debug, PartialEq, Eq)]
                        enum SlotProvider {
                            Missing,
                            Single(ExprId, Ty),
                            VarArgList(ExprId, Ty),
                            VarArgDict(ExprId, Ty),
                        }

                        enum Slot {
                            Positional {
                                provider: SlotProvider,
                            },
                            Keyword {
                                name: Name,
                                provider: SlotProvider,
                            },
                            VarArgList {
                                providers: SmallVec<[SlotProvider; 1]>,
                            },
                            VarArgDict {
                                providers: SmallVec<[SlotProvider; 1]>,
                            },
                        }

                        let mut slots: SmallVec<[Slot; 5]> = smallvec![];

                        // Only match valid parameters. For example, don't match a second `*args` or
                        // `**kwargs` parameter.
                        let mut saw_vararg = false;
                        let mut saw_kwargs = false;
                        let params = func.params(db);
                        for param in params {
                            let slot = match param {
                                IntrinsicFunctionParam::Positional { .. } => {
                                    if saw_vararg {
                                        // TODO: Emit diagnostics for invalid parameters.
                                        break;
                                    }
                                    Slot::Positional {
                                        provider: SlotProvider::Missing,
                                    }
                                }
                                IntrinsicFunctionParam::Keyword { name, .. } => Slot::Keyword {
                                    name: name.clone(),
                                    provider: SlotProvider::Missing,
                                },
                                IntrinsicFunctionParam::VarArgList { .. } => {
                                    saw_vararg = true;
                                    Slot::VarArgList {
                                        providers: smallvec![],
                                    }
                                }
                                IntrinsicFunctionParam::VarArgDict => {
                                    saw_kwargs = true;
                                    Slot::VarArgDict {
                                        providers: smallvec![],
                                    }
                                }
                            };

                            slots.push(slot);

                            // Nothing can follow a "**kwargs" parameter.
                            if saw_kwargs {
                                break;
                            }
                        }

                        'outer: for (arg, arg_ty) in args_with_ty {
                            match arg {
                                Argument::Simple { expr } => {
                                    // Look for a positional parameter with no provider, or for a "*args"
                                    // parameter.
                                    let provider = SlotProvider::Single(expr, arg_ty);
                                    for slot in slots.iter_mut() {
                                        match slot {
                                            Slot::Positional {
                                                provider: provider2 @ SlotProvider::Missing,
                                            } => {
                                                *provider2 = provider;
                                                continue 'outer;
                                            }
                                            Slot::VarArgList { providers } => {
                                                providers.push(provider);
                                                continue 'outer;
                                            }
                                            _ => {}
                                        }
                                    }
                                    self.add_expr_diagnostic(
                                        file,
                                        expr,
                                        "Unexpected positional argument",
                                    );
                                }
                                Argument::Keyword {
                                    name: ref arg_name,
                                    expr,
                                } => {
                                    // Look for either a keyword parameter matching this argument's
                                    // name, or for the "**kwargs" parameter.
                                    let provider = SlotProvider::Single(expr, arg_ty);
                                    for slot in slots.iter_mut() {
                                        match slot {
                                            Slot::Keyword {
                                                name,
                                                provider:
                                                    provider2 @ (SlotProvider::Missing
                                                    | SlotProvider::VarArgDict(_, _)),
                                            } if arg_name == name => {
                                                *provider2 = provider;
                                                continue 'outer;
                                            }
                                            Slot::VarArgList { providers } => {
                                                providers.push(provider);
                                                continue 'outer;
                                            }
                                            _ => {}
                                        }
                                    }
                                    self.add_expr_diagnostic(
                                        file,
                                        expr,
                                        format!(
                                            "Unexpected keyword argument \"{}\"",
                                            arg_name.as_str(),
                                        ),
                                    );
                                }
                                Argument::UnpackedList { expr } => {
                                    // Mark all unfilled positional slots as well as the "*args" slot as being
                                    // provided by this argument.
                                    for slot in slots.iter_mut() {
                                        match slot {
                                            Slot::Positional {
                                                provider: provider @ SlotProvider::Missing,
                                            } => {
                                                *provider =
                                                    SlotProvider::VarArgList(expr, arg_ty.clone())
                                            }
                                            Slot::VarArgList { providers } => {
                                                providers.push(SlotProvider::VarArgList(
                                                    expr,
                                                    arg_ty.clone(),
                                                ));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                Argument::UnpackedDict { expr } => {
                                    // Mark all keyword slots as well as the "**kwargs" slot as being provided by
                                    // this argument.
                                    for slot in slots.iter_mut() {
                                        match slot {
                                            Slot::Keyword { provider, .. } => {
                                                *provider =
                                                    SlotProvider::VarArgDict(expr, arg_ty.clone())
                                            }
                                            Slot::VarArgDict { providers } => providers.push(
                                                SlotProvider::VarArgDict(expr, arg_ty.clone()),
                                            ),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }

                        // Validate argument types.
                        for (param, slot) in params.iter().zip(slots) {
                            let param_ty = match param {
                                IntrinsicFunctionParam::Positional { ty, .. }
                                | IntrinsicFunctionParam::Keyword { ty, .. }
                                | IntrinsicFunctionParam::VarArgList { ty } => ty.clone(),
                                IntrinsicFunctionParam::VarArgDict => {
                                    TyKind::Dict(self.any_ty(), self.any_ty()).intern()
                                }
                            };
                            let param_ty = param_ty.substitute(&subst.args);

                            let mut validate_provider = |provider| match provider {
                                SlotProvider::Missing => {
                                    if !param.is_optional() {
                                        self.add_expr_diagnostic(
                                            file,
                                            expr,
                                            format!(
                                                "Missing expected argument of type \"{}\"",
                                                param_ty.display(db)
                                            ),
                                        );
                                    }
                                }
                                SlotProvider::Single(expr, ty) => {
                                    if !assign_tys(&ty, &param_ty) {
                                        self.add_expr_diagnostic(file, expr, format!("Argument of type \"{}\" cannot be assigned to paramter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
                                    }
                                }
                                _ => {}
                            };

                            match slot {
                                Slot::Positional { provider } | Slot::Keyword { provider, .. } => {
                                    validate_provider(provider)
                                }
                                Slot::VarArgList { providers } | Slot::VarArgDict { providers } => {
                                    providers.into_iter().for_each(validate_provider);
                                }
                            }
                        }

                        func.ret_ty(db).substitute(&subst.args)
                    }
                    TyKind::BuiltinFunction(func) => resolve_type_ref(db, &func.ret_type_ref(db))
                        .unwrap_or_else(|| self.unknown_ty()),
                    TyKind::Unknown | TyKind::Any | TyKind::Unbound => self.unknown_ty(),
                    _ => self.add_expr_diagnostic(
                        file,
                        expr,
                        format!("Type \"{}\" is not callable", callee_ty.display(db).alt()),
                    ),
                }
            }
            Expr::Tuple { exprs } => TyKind::Tuple(
                exprs
                    .iter()
                    .map(|expr| self.infer_expr(file, *expr))
                    .collect(),
            )
            .intern(),
            _ => self.any_ty(),
        };
        self.set_expr_type(file, expr, ty)
    }

    fn infer_unary_expr(&mut self, file: File, parent: ExprId, expr: ExprId, op: UnaryOp) -> Ty {
        let db = self.db;
        let ty = self.infer_expr(file, expr);
        let mut unknown = || {
            self.add_expr_diagnostic(
                file,
                parent,
                format!(
                    "Operator \"{}\" is not supported for type \"{}\"",
                    op,
                    ty.display(db)
                ),
            )
        };

        // Special handling for "Any".
        if ty.is_any() {
            return self.any_ty();
        }

        // Special handling for "Unknown" and "Unbound".
        if ty.is_unknown() {
            return self.unknown_ty();
        }

        let kind = ty.kind();
        match op {
            UnaryOp::Arith(_) => match kind {
                TyKind::Int => self.int_ty(),
                TyKind::Float => self.float_ty(),
                _ => unknown(),
            },
            UnaryOp::Inv => match kind {
                TyKind::Int => self.int_ty(),
                _ => unknown(),
            },
            UnaryOp::Not => self.bool_ty(),
        }
    }

    fn infer_binary_expr(
        &mut self,
        file: File,
        parent: ExprId,
        lhs: ExprId,
        rhs: ExprId,
        op: BinaryOp,
    ) -> Ty {
        let db = self.db;
        let lhs = self.infer_expr(file, lhs);
        let rhs = self.infer_expr(file, rhs);
        let lhs = lhs.kind();
        let rhs = rhs.kind();
        let mut unknown = || {
            self.add_expr_diagnostic(
                file,
                parent,
                format!(
                    "Operator \"{}\" not supported for types \"{}\" and \"{}\"",
                    op,
                    lhs.display(db),
                    rhs.display(db)
                ),
            )
        };

        if lhs == &TyKind::Any || rhs == &TyKind::Any {
            return self.any_ty();
        }

        match op {
            // TODO(withered-magic): Handle string interoplation with "%".
            BinaryOp::Arith(op) => match (lhs, rhs) {
                (TyKind::String, TyKind::String) if op == ArithOp::Add => self.string_ty(),
                (TyKind::Int, TyKind::Int) => self.int_ty(),
                (TyKind::Float, TyKind::Int)
                | (TyKind::Int, TyKind::Float)
                | (TyKind::Float, TyKind::Float) => self.float_ty(),
                _ => unknown(),
            },
            BinaryOp::Bitwise(_) => match (lhs, rhs) {
                (TyKind::Int, TyKind::Int) => self.int_ty(),
                _ => unknown(),
            },
            _ => self.bool_ty(),
        }
    }

    fn infer_source_expr_assign(&mut self, file: File, source: ExprId) {
        // Find the parent assignment node. This can be either an assignment statement (`x = 0`), a `for` statement (`for x in 1, 2, 3`), or
        // a for comp clause in a list/dict comprehension (`[x + 1 for x in [1, 2, 3]]`).
        let db = self.db;
        let source_map = source_map(db, file);
        let source_ptr = match source_map.expr_map_back.get(&source) {
            Some(ptr) => ptr,
            _ => return,
        };
        let parent = source_ptr
            .to_node(&parse(db, file).syntax(db))
            .syntax()
            .parent()
            .unwrap();

        // Convert "Unbound" to "Unknown" in assignments to avoid confusion.
        let mut source_ty = self.infer_expr(file, source);
        if matches!(source_ty.kind(), TyKind::Unbound) {
            source_ty = self.unknown_ty();
        }

        // Handle standard assigments, e.g. `x, y = 1, 2`.
        if let Some(lhs) = ast::AssignStmt::cast(parent.clone()).and_then(|stmt| stmt.lhs()) {
            let lhs_ptr = AstPtr::new(&lhs);
            let expr = source_map.expr_map.get(&lhs_ptr).unwrap();
            self.assign_expr_source_ty(file, *expr, *expr, source_ty);
            return;
        }

        // Handle assignments in "for" statements and comphrehensions.
        // e.g. `for x in 1, 2, 3` or `[x*y for x in range(5) for y in range(5)]`
        let targets = ast::ForStmt::cast(parent.clone())
            .and_then(|stmt| stmt.targets())
            .or_else(|| {
                ast::CompClauseFor::cast(parent).and_then(|comp_clause| comp_clause.targets())
            });

        let targets = match targets {
            Some(targets) => targets
                .exprs()
                .map(|expr| source_map.expr_map.get(&AstPtr::new(&expr)).unwrap())
                .copied()
                .collect::<Vec<_>>(),
            None => return,
        };

        let sub_ty = match source_ty.kind() {
            TyKind::List(ty) => ty.clone(),
            TyKind::Dict(key_ty, _) => key_ty.clone(),
            TyKind::Tuple(_) | TyKind::Any => self.any_ty(),
            TyKind::Range => self.int_ty(),
            TyKind::StringElems => self.string_ty(),
            TyKind::BytesElems => self.int_ty(),
            _ => {
                self.add_expr_diagnostic(
                    file,
                    source,
                    format!("Type \"{}\" is not iterable", source_ty.display(db)),
                );
                for expr in targets.iter() {
                    self.assign_expr_unknown_rec(file, *expr);
                }
                return;
            }
        };
        if targets.len() == 1 {
            self.assign_expr_source_ty(file, targets[0], targets[0], sub_ty);
        } else {
            self.assign_exprs_source_ty(file, source, &targets, sub_ty);
        }
    }

    fn assign_expr_source_ty(&mut self, file: File, root: ExprId, expr: ExprId, source_ty: Ty) {
        match module(self.db, file).exprs.get(expr).unwrap() {
            Expr::Name { .. } => {
                self.set_expr_type(file, expr, source_ty);
            }
            Expr::List { exprs } | Expr::Tuple { exprs } => {
                self.assign_exprs_source_ty(file, root, exprs, source_ty);
            }
            Expr::Paren { expr } => self.assign_expr_source_ty(file, root, *expr, source_ty),
            _ => {}
        }
    }

    fn assign_exprs_source_ty(
        &mut self,
        file: File,
        root: ExprId,
        exprs: &[ExprId],
        source_ty: Ty,
    ) {
        match source_ty.kind() {
            TyKind::List(ty) => {
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, root, expr, ty.clone());
                }
            }
            TyKind::Tuple(tys) => {
                let mut pairs = exprs.iter().copied().zip(tys.iter());
                while let Some((expr, ty)) = pairs.next() {
                    self.assign_expr_source_ty(file, root, expr, ty.clone());
                }
                if exprs.len() != tys.len() {
                    if exprs.len() > tys.len() {
                        for expr in &exprs[tys.len()..] {
                            self.assign_expr_unknown_rec(file, *expr);
                        }
                    }
                    self.add_expr_diagnostic(
                        file,
                        root,
                        format!(
                            "Tuple size mismatch, {} on left-hand side and {} on right-hand side",
                            exprs.len(),
                            tys.len(),
                        ),
                    );
                }
            }
            TyKind::Any => {
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, root, expr, self.any_ty());
                }
            }
            _ => {
                self.add_expr_diagnostic(
                    file,
                    root,
                    format!("Type \"{}\" is not iterable", source_ty.display(self.db)),
                );
                for expr in exprs.iter() {
                    self.assign_expr_unknown_rec(file, *expr);
                }
                return;
            }
        };
    }

    fn assign_expr_unknown_rec(&mut self, file: File, expr: ExprId) {
        self.set_expr_type(file, expr, self.unknown_ty());
        module(self.db, file)[expr].walk_child_exprs(|expr| {
            self.assign_expr_unknown_rec(file, expr);
        })
    }

    fn set_expr_type(&mut self, file: File, expr: ExprId, ty: Ty) -> Ty {
        self.cx
            .type_of_expr
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

    fn add_expr_diagnostic<T: Into<String>>(&mut self, file: File, expr: ExprId, message: T) -> Ty {
        let range = match source_map(self.db, file).expr_map_back.get(&expr) {
            Some(ptr) => ptr.syntax_node_ptr().text_range(),
            None => return self.unknown_ty(),
        };
        self.add_diagnostic_for_range(file, range, message);
        self.unknown_ty()
    }

    fn add_diagnostic_for_range<T: Into<String>>(
        &mut self,
        file: File,
        range: TextRange,
        message: T,
    ) {
        self.cx.diagnostics.push(Diagnostic {
            message: message.into(),
            severity: Severity::Error,
            range: FileRange {
                file_id: file.id(self.db),
                range,
            },
        });
    }

    fn param_ty(&mut self, file: File, param: ParamId) -> Ty {
        if let Some(ty) = self.cx.param_tys.get(&FileParamId { file, param }) {
            return ty.clone();
        }

        let module = module(self.db, file);
        let source_map = source_map(self.db, file);
        let ty = match &module.params[param] {
            Param::Simple { type_ref, .. } => type_ref
                .as_ref()
                .and_then(|type_ref| self.lower_param_type_ref(file, source_map, param, &type_ref))
                .unwrap_or_else(|| self.unknown_ty()),
            Param::ArgsList { type_ref, .. } => TyKind::List(
                type_ref
                    .as_ref()
                    .and_then(|type_ref| {
                        self.lower_param_type_ref(file, source_map, param, type_ref)
                    })
                    .unwrap_or_else(|| self.unknown_ty()),
            )
            .intern(),
            Param::KwargsList { .. } => TyKind::Dict(self.any_ty(), self.any_ty()).intern(),
        };

        self.cx
            .param_tys
            .insert(FileParamId { file, param }, ty.clone());
        ty
    }

    fn lower_param_type_ref(
        &mut self,
        file: File,
        source_map: &ModuleSourceMap,
        param: ParamId,
        type_ref: &TypeRef,
    ) -> Option<Ty> {
        let opt = resolve_type_ref(self.db, type_ref);

        // TODO(withered-magic): This will eventually need to handle diagnostics
        // for other places that type comments can appear.
        if opt.is_none() {
            let name = match type_ref {
                TypeRef::Name(name) => name,
                TypeRef::Unknown => return None,
            };
            let ptr = source_map.param_map_back.get(&param)?;
            self.add_diagnostic_for_range(
                file,
                ptr.syntax_node_ptr().text_range(),
                format!("Unknown type \"{}\" in type comment", name.as_str()),
            );
        }
        opt
    }

    fn types(&self) -> &IntrinsicTypes {
        self.intrinsics.types(self.db)
    }

    fn any_ty(&self) -> Ty {
        self.types().any.clone()
    }

    fn unbound_ty(&self) -> Ty {
        self.types().unbound.clone()
    }

    fn unknown_ty(&self) -> Ty {
        self.types().unknown.clone()
    }

    fn none_ty(&self) -> Ty {
        self.types().none.clone()
    }

    fn bool_ty(&self) -> Ty {
        self.types().bool.clone()
    }

    fn int_ty(&self) -> Ty {
        self.types().int.clone()
    }

    fn float_ty(&self) -> Ty {
        self.types().float.clone()
    }

    fn string_ty(&self) -> Ty {
        self.types().string.clone()
    }

    fn string_elems_ty(&self) -> Ty {
        self.types().string_elems.clone()
    }

    fn bytes_ty(&self) -> Ty {
        self.types().bytes.clone()
    }

    fn bytes_elems_ty(&self) -> Ty {
        self.types().bytes_elems.clone()
    }

    fn range_ty(&self) -> Ty {
        self.types().range.clone()
    }
}

fn resolve_type_ref(db: &dyn Db, type_ref: &TypeRef) -> Option<Ty> {
    let types = intrinsic_types(db).types(db);
    let builtin_types = builtin_types(db);
    Some(match type_ref {
        TypeRef::Name(name) => match name.as_str() {
            "None" | "NoneType" => types.any.clone(),
            "bool" => types.bool.clone(),
            "int" => types.int.clone(),
            "float" => types.float.clone(),
            "string" => types.string.clone(),
            "bytes" => types.bytes.clone(),
            "list" => TyKind::List(types.any.clone()).intern(),
            "dict" => TyKind::Dict(types.any.clone(), types.any.clone()).intern(),
            "range" => types.range.clone(),
            name => return builtin_types.types(db).get(name).cloned(),
        },
        TypeRef::Unknown => types.unknown.clone(),
    })
}

fn assign_tys(source: &Ty, target: &Ty) -> bool {
    // Assignments involving "Any", "Unknown", or "Unbound" at the top-level
    // are always valid to avoid confusion.
    if source.is_any() || source.is_unknown() || target.is_any() || target.is_unknown() {
        return true;
    }

    // With the exception of protocols, all other types are compared for equality.
    match target.kind() {
        TyKind::Protocol(protocol) => protocol.assign_ty(source),
        _ => source == target,
    }
}
