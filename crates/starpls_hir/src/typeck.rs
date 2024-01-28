use crate::{
    def::{ExprId, Function, Param as HirDefParam, ParamId},
    display::DisplayWithDb,
    module,
    typeck::{
        builtins::{builtin_types, BuiltinFunction, BuiltinFunctionParam, BuiltinType},
        intrinsics::{
            intrinsic_field_types, intrinsic_types, IntrinsicClass, IntrinsicFunction,
            IntrinsicFunctionParam, Intrinsics,
        },
    },
    Db, Name, Type,
};
use crossbeam::atomic::AtomicCell;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use starpls_common::{Diagnostic, File};
use starpls_intern::{impl_internable, Interned};
use std::{
    fmt::{Display, Write},
    panic::{self, UnwindSafe},
    sync::Arc,
};

mod call;
mod infer;
mod lower;

pub(crate) mod builtins;
pub(crate) mod intrinsics;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct FileExprId {
    pub file: File,
    pub expr: ExprId,
}

impl FileExprId {
    fn new(file: File, expr: ExprId) -> Self {
        Self { file, expr }
    }
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
    Sequence(Box<TypeRef>),
    Union(Vec<TypeRef>),
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
            TypeRef::Union(names) => {
                for (i, type_ref) in names.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" | ")?;
                    }
                    type_ref.fmt(f)?;
                }
                return Ok(());
            }
            TypeRef::Sequence(type_ref) => return write!(f, "Sequence[{}]", type_ref),
            TypeRef::Unknown => "Unknown",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ty(Interned<TyKind>);

impl Ty {
    pub(crate) fn kind(&self) -> &TyKind {
        &self.0
    }

    pub(crate) fn fields<'a>(
        &'a self,
        db: &'a dyn Db,
    ) -> Option<impl Iterator<Item = (Field, Ty)> + 'a> {
        if let Some(class) = self.kind().builtin_class(db) {
            Some(Fields::Intrinsic(self.intrinsic_class_fields(db, class)))
        } else if let TyKind::BuiltinType(ty) = self.kind() {
            Some(Fields::Builtin(
                ty.fields(db)
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        (
                            Field(FieldInner::BuiltinField { parent: *ty, index }),
                            resolve_type_ref(db, &field.type_ref)
                                .unwrap_or_else(|| TyKind::Unknown.intern()),
                        )
                    })
                    .chain(ty.methods(db).iter().map(|func| {
                        (
                            Field(FieldInner::BuiltinMethod { func: *func }),
                            TyKind::BuiltinFunction(*func).intern(),
                        )
                    })),
            ))
        } else {
            None
        }
    }

    fn intrinsic_class_fields<'a>(
        &'a self,
        db: &'a dyn Db,
        class: IntrinsicClass,
    ) -> impl Iterator<Item = (Field, Ty)> + 'a {
        let fields = (0..class.fields(db).len()).into_iter().map(move |index| {
            Field(FieldInner::IntrinsicField {
                parent: class,
                index,
            })
        });

        // Build the substitution for lists and dicts.
        let mut subst = Substitution::new();
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
            .map(move |binders| binders.substitute(&subst));
        fields.zip(types)
    }

    pub(crate) fn params<'a>(&'a self, db: &'a dyn Db) -> Option<impl Iterator<Item = Param> + 'a> {
        Some(match self.kind() {
            TyKind::Function(func) => Params::Simple((0..func.params(db).len()).map(|index| {
                Param(ParamInner::Param {
                    parent: *func,
                    index,
                })
            })),
            TyKind::IntrinsicFunction(func, _) => {
                Params::Intrinsic((0..func.params(db).len()).map(|index| {
                    Param(ParamInner::IntrinsicParam {
                        parent: *func,
                        index,
                    })
                }))
            }
            TyKind::BuiltinFunction(func) => {
                Params::Builtin((0..func.params(db).len()).map(|index| {
                    Param(ParamInner::BuiltinParam {
                        parent: *func,
                        index,
                    })
                }))
            }
            _ => return None,
        })
    }

    fn is_any(&self) -> bool {
        self.kind() == &TyKind::Any
    }

    fn is_unknown(&self) -> bool {
        self.kind() == &TyKind::Unknown || self.kind() == &TyKind::Unbound
    }

    fn substitute(&self, args: &[Ty]) -> Ty {
        match self.kind() {
            TyKind::List(ty) => TyKind::List(ty.substitute(args)).intern(),
            // TyKind::Tuple(tup) => {
            //     TyKind::Tuple(tys.iter().map(|ty| ty.substitute(args)).collect()).intern()
            // }
            TyKind::Tuple(tup) => match tup {
                Tuple::Simple(tys) => TyKind::Tuple(Tuple::Simple(
                    tys.iter().map(|ty| ty.substitute(args)).collect(),
                )),
                Tuple::Variable(ty) => TyKind::Tuple(Tuple::Variable(ty.substitute(args))),
            }
            .intern(),
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

pub struct Param(ParamInner);

enum ParamInner {
    Param {
        parent: Function,
        index: usize,
    },
    IntrinsicParam {
        parent: IntrinsicFunction,
        index: usize,
    },
    BuiltinParam {
        parent: BuiltinFunction,
        index: usize,
    },
}

impl Param {
    pub fn name(&self, db: &dyn Db) -> Option<Name> {
        match self.0 {
            ParamInner::Param { parent, index } => {
                let module = module(db, parent.file(db));
                Some(module[parent.params(db)[index]].name().clone())
            }
            ParamInner::IntrinsicParam { parent, index } => {
                parent.params(db)[index].name().cloned()
            }
            ParamInner::BuiltinParam { parent, index } => match &parent.params(db)[index] {
                BuiltinFunctionParam::Simple { name, .. } => Some(name.clone()),
                _ => None,
            },
        }
    }

    pub fn doc(&self, db: &dyn Db) -> Option<String> {
        Some(match self.0 {
            ParamInner::BuiltinParam { parent, index } => match &parent.params(db)[index] {
                BuiltinFunctionParam::Simple { doc, .. }
                | BuiltinFunctionParam::ArgsList { doc, .. }
                | BuiltinFunctionParam::KwargsDict { doc } => doc.clone(),
            },
            _ => return None,
        })
    }

    pub fn ty(&self, db: &dyn Db) -> Type {
        match self.0 {
            ParamInner::Param { parent, index } => {
                let module = module(db, parent.file(db));
                module[parent.params(db)[index]]
                    .type_ref()
                    .and_then(|type_ref| resolve_type_ref(db, &type_ref))
            }
            ParamInner::IntrinsicParam { parent, index } => parent.params(db)[index].ty(),
            ParamInner::BuiltinParam { parent, index } => parent.params(db)[index]
                .type_ref()
                .and_then(|type_ref| resolve_type_ref(db, &type_ref)),
        }
        .unwrap_or_else(|| TyKind::Unknown.intern())
        .into()
    }
}

enum Params<I1, I2, I3> {
    Simple(I1),
    Intrinsic(I2),
    Builtin(I3),
}

impl<I1, I2, I3> Iterator for Params<I1, I2, I3>
where
    I1: Iterator<Item = Param>,
    I2: Iterator<Item = Param>,
    I3: Iterator<Item = Param>,
{
    type Item = Param;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Params::Simple(iter) => iter.next(),
            Params::Intrinsic(iter) => iter.next(),
            Params::Builtin(iter) => iter.next(),
        }
    }
}

pub struct Field(FieldInner);

impl Field {
    pub fn name(&self, db: &dyn Db) -> Name {
        match self.0 {
            FieldInner::BuiltinField { parent, index } => parent.fields(db)[index].name.clone(),
            FieldInner::BuiltinMethod { func } => func.name(db),
            FieldInner::IntrinsicField { parent, index } => parent.fields(db)[index].name.clone(),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> String {
        match self.0 {
            FieldInner::BuiltinField { parent, index } => parent.fields(db)[index].doc.clone(),
            FieldInner::BuiltinMethod { func } => func.doc(db).clone(),
            FieldInner::IntrinsicField { .. } => String::new(),
        }
    }
}

enum FieldInner {
    BuiltinField {
        parent: BuiltinType,
        index: usize,
    },
    BuiltinMethod {
        func: BuiltinFunction,
    },
    IntrinsicField {
        parent: IntrinsicClass,
        index: usize,
    },
}

enum Fields<I1, I2> {
    Intrinsic(I1),
    Builtin(I2),
}

impl<I1, I2> Iterator for Fields<I1, I2>
where
    I1: Iterator<Item = (Field, Ty)>,
    I2: Iterator<Item = (Field, Ty)>,
{
    type Item = (Field, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Intrinsic(iter) => iter.next(),
            Self::Builtin(iter) => iter.next(),
        }
    }
}

impl DisplayWithDb for Ty {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.kind().fmt(db, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TyKind {
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
    Tuple(Tuple),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tuple {
    Simple(SmallVec<[Ty; 2]>),
    Variable(Ty),
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
            TyKind::Tuple(tuple) => {
                f.write_str("tuple[")?;
                match tuple {
                    Tuple::Simple(tys) => {
                        for (i, ty) in tys.iter().enumerate() {
                            if i > 0 {
                                f.write_str(", ")?;
                            }
                            ty.fmt(db, f)?;
                        }
                    }
                    Tuple::Variable(ty) => {
                        ty.fmt(db, f)?;
                        f.write_str(", ...")?;
                    }
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
                let module = module(db, func.file(db));
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func
                    .params(db)
                    .iter()
                    .map(|param| &module[*param])
                    .enumerate()
                {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    let format_type_ref_opt = |f, type_ref: &Option<TypeRef>| match type_ref
                        .as_ref()
                        .and_then(|type_ref| resolve_type_ref(db, &type_ref))
                    {
                        Some(ty) => ty.fmt(db, f),
                        None => f.write_str("Unknown"),
                    };

                    match param {
                        HirDefParam::Simple { name, type_ref, .. } => {
                            f.write_str(name.as_str())?;
                            f.write_str(": ")?;
                            format_type_ref_opt(f, type_ref)?;
                        }
                        HirDefParam::ArgsList { name, type_ref, .. } => {
                            f.write_char('*')?;
                            if !name.is_missing() {
                                f.write_str(name.as_str())?;
                                f.write_str(": tuple[")?;
                                format_type_ref_opt(f, type_ref)?;
                                f.write_str(", ...]")?;
                            }
                        }
                        HirDefParam::KwargsDict { name, type_ref, .. } => {
                            f.write_str("**")?;
                            if !name.is_missing() {
                                f.write_str(name.as_str())?;
                                f.write_str(": dict[string, ")?;
                                format_type_ref_opt(f, type_ref)?;
                                f.write_char(']')?;
                            }
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
                        IntrinsicFunctionParam::ArgsList { ty } => {
                            f.write_str("*args: ")?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                        }
                        IntrinsicFunctionParam::KwargsDict => {
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
                        BuiltinFunctionParam::Simple {
                            name,
                            type_ref,
                            default_value,
                            ..
                        } => {
                            f.write_str(name.as_str())?;
                            write!(f, ": {}", type_ref)?;
                            if let Some(default_value) = default_value {
                                f.write_str(" = ")?;
                                f.write_str(&default_value)?;
                            }
                        }
                        BuiltinFunctionParam::ArgsList { .. } => f.write_str("*args")?,
                        BuiltinFunctionParam::KwargsDict { .. } => f.write_str("**kwargs")?,
                    }
                }
                f.write_str(") -> ")?;
                return func.ret_type_ref(db).fmt(f);
            }
            TyKind::BuiltinType(type_) => return f.write_str(type_.name(db).as_str()),
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
pub(crate) struct Binders {
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
pub(crate) struct Substitution {
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
                // TyKind::Tuple(rhs_tys) => rhs_tys.iter().all(|rhs_ty| assign_tys(rhs_ty, lhs_ty)),
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

pub(crate) fn resolve_type_ref(db: &dyn Db, type_ref: &TypeRef) -> Option<Ty> {
    let types = intrinsic_types(db).types(db);
    let builtin_types = builtin_types(db);
    Some(match type_ref {
        TypeRef::Name(name) => match name.as_str() {
            "None" | "NoneType" => types.none.clone(),
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
        _ => types.unknown.clone(),
    })
}

pub(crate) fn assign_tys(source: &Ty, target: &Ty) -> bool {
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
