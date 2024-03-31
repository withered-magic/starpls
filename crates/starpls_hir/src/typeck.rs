use crate::{
    def::{ExprId, Function, LoadItemId, LoadStmt, Param as HirDefParam, ParamId},
    module, source_map,
    typeck::{
        builtins::{
            builtin_types, common_attributes_query, BuiltinFunction, BuiltinFunctionParam,
            BuiltinType,
        },
        intrinsics::{
            intrinsic_field_types, intrinsic_types, IntrinsicClass, IntrinsicFunction,
            IntrinsicFunctionParam, Intrinsics,
        },
    },
    Db, Name,
};
use crossbeam::atomic::AtomicCell;
use either::Either;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};
use starpls_common::{parse, Diagnostic, Dialect, File};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::ast::SyntaxNodePtr;
use std::{
    fmt::Write,
    iter,
    panic::{self, UnwindSafe},
    sync::Arc,
};

mod call;
mod infer;

#[cfg(test)]
mod tests;

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
pub(crate) struct FileParamId {
    pub(crate) file: File,
    pub(crate) param: ParamId,
}

impl FileParamId {
    fn new(file: File, param: ParamId) -> Self {
        Self { file, param }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct FileLoadStmt {
    pub(crate) file: File,
    pub(crate) load_stmt: LoadStmt,
}

impl FileLoadStmt {
    fn new(file: File, load_stmt: LoadStmt) -> Self {
        Self { file, load_stmt }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct FileLoadItemId {
    pub(crate) file: File,
    pub(crate) load_item: LoadItemId,
}

impl FileLoadItemId {
    fn new(file: File, load_item: LoadItemId) -> Self {
        Self { file, load_item }
    }
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
    Name(Name, Option<Box<[TypeRef]>>),
    Union(Vec<TypeRef>),
    Unknown,
}

impl TypeRef {
    pub(crate) fn from_str_opt(name: &str) -> Self {
        if name.is_empty() {
            Self::Unknown
        } else {
            Self::Name(Name::from_str(name), None)
        }
    }

    pub(crate) fn is_unknown(&self) -> bool {
        self == &Self::Unknown
    }
}

impl std::fmt::Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeRef::Name(name, args) => {
                f.write_str(name.as_str())?;
                if let Some(args) = args {
                    f.write_char('[')?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }
                        arg.fmt(f)?;
                    }
                    f.write_char(']')
                } else {
                    Ok(())
                }
            }
            TypeRef::Union(names) => {
                for (i, type_ref) in names.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" | ")?;
                    }
                    type_ref.fmt(f)?;
                }
                return Ok(());
            }
            TypeRef::Unknown => f.write_str("Unknown"),
        }
    }
}

/// A reference to a function type, i.e. in a function type comment.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionTypeRef(pub Vec<TypeRef>, pub TypeRef);

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
        let kind = self.kind();
        if let Some(class) = kind.builtin_class(db) {
            Some(Fields::Intrinsic(self.intrinsic_class_fields(db, class)))
        } else if let TyKind::BuiltinType(ty) = kind {
            Some(Fields::Builtin(
                ty.fields(db)
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        (
                            Field(FieldInner::BuiltinField { parent: *ty, index }),
                            resolve_type_ref(db, &field.type_ref).0,
                        )
                    })
                    .chain(ty.methods(db).iter().map(|func| {
                        (
                            Field(FieldInner::BuiltinMethod { func: *func }),
                            TyKind::BuiltinFunction(*func).intern(),
                        )
                    })),
            ))
        } else if let TyKind::Union(tys) = kind {
            // TODO(withered-magic): Can probably do better than a Vec here?
            let mut acc = Vec::new();
            tys.iter().for_each(|ty| {
                let fields = match ty.fields(db) {
                    Some(fields) => fields,
                    None => return,
                };

                for (field, ty) in fields {
                    acc.push((field, ty));
                }
            });
            Some(Fields::Union(acc.into_iter()))
        } else if let TyKind::Struct(fields) = kind {
            Some(Fields::Struct(fields.iter().map(|(name, ty)| {
                (
                    Field(FieldInner::StructField { name: name.clone() }),
                    ty.clone(),
                )
            })))
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
            TyKind::Dict(key_ty, value_ty, _) => {
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

    pub(crate) fn params<'a>(
        &'a self,
        db: &'a dyn Db,
    ) -> Option<impl Iterator<Item = (Param, Ty)> + 'a> {
        Some(match self.kind() {
            TyKind::Function(func) => {
                Params::Simple(func.params(db).iter().enumerate().map(|(index, param)| {
                    let file = func.file(db);
                    let ty = db.infer_param(file, *param);
                    let param = Param(ParamInner::Param {
                        parent: Some(*func),
                        index,
                    });
                    (param, ty)
                }))
            }
            TyKind::IntrinsicFunction(func, subst) => {
                Params::Intrinsic(func.params(db).iter().enumerate().map(|(index, param)| {
                    let ty = param
                        .ty()
                        .unwrap_or_else(|| Ty::unknown())
                        .substitute(&subst.args);
                    let param = Param(ParamInner::IntrinsicParam {
                        parent: *func,
                        index,
                    });
                    (param, ty)
                }))
            }
            TyKind::BuiltinFunction(func) => {
                Params::Builtin(func.params(db).iter().enumerate().map(|(index, param)| {
                    let ty = resolve_type_ref_opt(db, param.type_ref());
                    let ty = match param {
                        BuiltinFunctionParam::Simple { .. } => ty,
                        BuiltinFunctionParam::ArgsList { .. } => {
                            TyKind::Tuple(Tuple::Variable(ty)).intern()
                        }
                        BuiltinFunctionParam::KwargsDict { .. } => Ty::dict(Ty::string(), ty, None),
                    };
                    let param = Param(ParamInner::BuiltinParam {
                        parent: *func,
                        index,
                    });
                    (param, ty)
                }))
            }
            TyKind::Rule(Rule { attrs, kind, .. }) => {
                let common = common_attributes_query(db);
                let mut common_attrs = match kind {
                    RuleKind::Build => common.build(db),
                    RuleKind::Repository => common.repository(db),
                }
                .iter()
                .enumerate()
                .map(|(index, (_, attr))| {
                    (
                        Param(RuleParam::BuiltinKeyword(kind.clone(), index).into()),
                        attr.expected_ty(),
                    )
                });

                // This chaining is done to put the `name` attribute first.
                Params::Rule(
                    common_attrs
                        .next()
                        .into_iter()
                        .chain(attrs.iter().map(|(name, ty)| {
                            (
                                Param(
                                    RuleParam::Keyword {
                                        name: name.clone(),
                                        attr: ty.clone(),
                                    }
                                    .into(),
                                ),
                                ty.as_attribute().expected_ty(),
                            )
                        }))
                        .chain(common_attrs)
                        .chain(iter::once((
                            Param(RuleParam::Kwargs.into()),
                            TyKind::Dict(Ty::string(), Ty::any(), None).intern(),
                        ))),
                )
            }
            _ => return None,
        })
    }

    pub(crate) fn ret_ty(&self, db: &dyn Db) -> Option<Ty> {
        Some(match self.kind() {
            TyKind::Function(func) => resolve_type_ref_opt(db, func.ret_type_ref(db)),
            TyKind::IntrinsicFunction(func, subst) => func.ret_ty(db).substitute(&subst.args),
            TyKind::BuiltinFunction(func) => resolve_type_ref(db, &func.ret_type_ref(db)).0,
            TyKind::Rule(_) => TyKind::None.intern(),
            _ => return None,
        })
    }

    pub(crate) fn unknown() -> Ty {
        TyKind::Unknown.intern()
    }

    pub(crate) fn any() -> Ty {
        TyKind::Any.intern()
    }

    pub(crate) fn bool() -> Ty {
        TyKind::Bool.intern()
    }

    pub(crate) fn int() -> Ty {
        TyKind::Int.intern()
    }

    pub(crate) fn string() -> Ty {
        TyKind::String.intern()
    }

    pub(crate) fn list(ty: Ty) -> Ty {
        TyKind::List(ty).intern()
    }

    pub(crate) fn dict(key_ty: Ty, value_ty: Ty, known_keys: Option<Arc<[(Box<str>, Ty)]>>) -> Ty {
        TyKind::Dict(key_ty, value_ty, known_keys).intern()
    }

    pub(crate) fn union(tys: impl Iterator<Item = Ty>) -> Ty {
        let mut unique_tys = smallvec![];

        // Deduplicate types. Dicts and structs are handled separately because the metadata
        // that they store // (for their declarations, etc.) is not relevant to determining
        // whether or not types are duplicates.
        for ty in tys {
            if unique_tys
                .iter()
                .any(|unique_ty: &Ty| Ty::eq(&ty, unique_ty))
            {
                continue;
            }
            unique_tys.push(ty);
        }

        match unique_tys.len() {
            0 => TyKind::Unknown.intern(),
            1 => unique_tys.into_iter().next().unwrap(),
            _ => TyKind::Union(unique_tys).intern(),
        }
    }

    fn is_any(&self) -> bool {
        self.kind() == &TyKind::Any
    }

    fn is_unknown(&self) -> bool {
        self.kind() == &TyKind::Unknown || self.kind() == &TyKind::Unbound
    }

    pub(crate) fn substitute(&self, args: &[Ty]) -> Ty {
        match self.kind() {
            TyKind::List(ty) => Ty::list(ty.substitute(args)),
            TyKind::Tuple(tup) => match tup {
                Tuple::Simple(tys) => TyKind::Tuple(Tuple::Simple(
                    tys.iter().map(|ty| ty.substitute(args)).collect(),
                )),
                Tuple::Variable(ty) => TyKind::Tuple(Tuple::Variable(ty.substitute(args))),
            }
            .intern(),
            TyKind::Dict(key_ty, value_ty, known_keys) => Ty::dict(
                key_ty.substitute(args),
                value_ty.substitute(args),
                known_keys.clone(),
            ),
            TyKind::IntrinsicFunction(data, subst) => {
                TyKind::IntrinsicFunction(*data, subst.substitute(args)).intern()
            }
            TyKind::BoundVar(index) => args[*index].clone(),
            _ => self.clone(),
        }
    }

    pub(crate) fn known_keys(&self) -> Option<&[(Box<str>, Ty)]> {
        match self.kind() {
            TyKind::Dict(_, _, known_keys) => known_keys.as_ref().map(|known_keys| &**known_keys),
            _ => None,
        }
    }

    fn is_attribute(&self) -> bool {
        matches!(self.kind(), TyKind::Attribute(_))
    }

    fn as_attribute(&self) -> &Attribute {
        match self.kind() {
            TyKind::Attribute(attr) => attr,
            _ => panic!("attribute() called on invalid TyKind"),
        }
    }

    fn eq(ty1: &Ty, ty2: &Ty) -> bool {
        match (ty1.kind(), ty2.kind()) {
            (TyKind::Dict(key_ty1, value_ty1, _), TyKind::Dict(key_ty2, value_ty2, _)) => {
                key_ty1 == key_ty2 && value_ty1 == value_ty2
            }
            (TyKind::Struct(_), TyKind::Struct(_)) => true,
            (TyKind::Union(tys1), TyKind::Union(tys2)) => {
                // Check that the union types have the same cardinality.
                if tys1.len() != tys2.len() {
                    return false;
                }

                // Check that for each type in the first union, there is an equal type in the second union.
                // This only works assuming the `TyKind::Union` was created with `Ty::union()`.
                tys1.iter()
                    .all(|ty1| tys2.iter().any(|ty2| Ty::eq(ty1, ty2)))
            }
            _ => ty1 == ty2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Param(pub(crate) ParamInner);

#[derive(Clone, Debug)]
pub(crate) enum ParamInner {
    Param {
        parent: Option<Function>,
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
    RuleParam(RuleParam),
}

#[derive(Clone, Debug)]
pub(crate) enum RuleParam {
    Keyword { name: Name, attr: Ty },
    BuiltinKeyword(RuleKind, usize),
    Kwargs,
}

impl From<RuleParam> for ParamInner {
    fn from(value: RuleParam) -> Self {
        Self::RuleParam(value)
    }
}

impl Param {
    pub fn name(&self, db: &dyn Db) -> Option<Name> {
        match self.0 {
            ParamInner::Param { parent, index } => {
                let parent = parent?;
                let module = module(db, parent.file(db));
                Some(module[parent.params(db)[index]].name().clone())
            }
            ParamInner::IntrinsicParam { parent, index } => {
                let param = &parent.params(db)[index];

                param.name().cloned().or_else(|| {
                    Some(match param {
                        IntrinsicFunctionParam::Positional { .. } => {
                            Name::from_str(&format!("x{index}"))
                        }
                        IntrinsicFunctionParam::ArgsList { .. } => Name::new_inline("args"),
                        IntrinsicFunctionParam::KwargsDict => Name::new_inline("kwargs"),
                        _ => unreachable!(),
                    })
                })
            }
            ParamInner::BuiltinParam { parent, index } => match &parent.params(db)[index] {
                BuiltinFunctionParam::Simple { name, .. }
                | BuiltinFunctionParam::ArgsList { name, .. }
                | BuiltinFunctionParam::KwargsDict { name, .. } => Some(name.clone()),
            },
            ParamInner::RuleParam(RuleParam::Keyword { ref name, .. }) => Some(name.clone()),
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(ref kind, index)) => Some(
                common_attributes_query(db)
                    .get(db, kind.clone(), index)
                    .0
                    .clone(),
            ),
            ParamInner::RuleParam(RuleParam::Kwargs) => Some(Name::new_inline("kwargs")),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> Option<String> {
        Some(match &self.0 {
            ParamInner::Param { parent, index } => {
                let parent = parent.as_ref()?;
                let module = module(db, parent.file(db));
                return module[parent.params(db)[*index]]
                    .doc()
                    .map(|doc| doc.to_string());
            }
            ParamInner::BuiltinParam { parent, index } => match &parent.params(db)[*index] {
                BuiltinFunctionParam::Simple { doc, .. }
                | BuiltinFunctionParam::ArgsList { doc, .. }
                | BuiltinFunctionParam::KwargsDict { doc, .. } => doc.clone(),
            },
            ParamInner::IntrinsicParam { .. } => return None,
            ParamInner::RuleParam(RuleParam::Keyword { attr, .. }) => {
                return attr.as_attribute().doc.as_ref().map(Box::to_string)
            }
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(kind, index)) => {
                return common_attributes_query(db)
                    .get(db, kind.clone(), *index)
                    .1
                    .doc
                    .as_ref()
                    .map(Box::to_string)
            }

            _ => return None,
        })
    }

    pub fn is_args_list(&self, db: &dyn Db) -> bool {
        match self.0 {
            // TODO(withered-magic): Handle lambda parameters.
            ParamInner::Param { parent, index } => {
                let parent = match parent {
                    Some(parent) => parent,
                    None => return false,
                };
                let module = module(db, parent.file(db));
                matches!(
                    module[parent.params(db)[index]],
                    HirDefParam::ArgsList { .. }
                )
            }
            ParamInner::IntrinsicParam { parent, index } => matches!(
                parent.params(db)[index],
                IntrinsicFunctionParam::ArgsList { .. }
            ),
            ParamInner::BuiltinParam { parent, index } => matches!(
                parent.params(db)[index],
                BuiltinFunctionParam::ArgsList { .. }
            ),
            _ => false,
        }
    }

    pub fn is_kwargs_dict(&self, db: &dyn Db) -> bool {
        match self.0 {
            // TODO(withered-magic): Handle lambda parameters.
            ParamInner::Param { parent, index } => {
                let parent = match parent {
                    Some(parent) => parent,
                    None => return false,
                };
                let module = module(db, parent.file(db));
                matches!(
                    module[parent.params(db)[index]],
                    HirDefParam::KwargsDict { .. }
                )
            }
            ParamInner::IntrinsicParam { parent, index } => matches!(
                parent.params(db)[index],
                IntrinsicFunctionParam::KwargsDict { .. }
            ),
            ParamInner::BuiltinParam { parent, index } => matches!(
                parent.params(db)[index],
                BuiltinFunctionParam::KwargsDict { .. }
            ),
            ParamInner::RuleParam(RuleParam::Kwargs) => true,
            _ => false,
        }
    }

    pub fn syntax_node_ptr(&self, db: &dyn Db) -> Option<SyntaxNodePtr> {
        match self.0 {
            ParamInner::Param { parent, index } => parent.and_then(|parent| {
                source_map(db, parent.file(db))
                    .param_map_back
                    .get(&parent.params(db)[index])
                    .map(|ptr| ptr.syntax_node_ptr())
            }),

            _ => None,
        }
    }

    pub fn default_value(&self, db: &dyn Db) -> Option<String> {
        let common = common_attributes_query(db);
        let attr = match &self.0 {
            ParamInner::RuleParam(RuleParam::Keyword { attr, .. }) => attr.as_attribute(),
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(kind, index)) => {
                common.get(db, kind.clone(), *index).1
            }
            _ => return None,
        };

        attr.default_text_range.as_ref().and_then(|e| {
            Some(match e {
                Either::Left((file, ptr)) => ptr
                    .try_to_node(&parse(db, *file).syntax(db))?
                    .text()
                    .to_string(),
                Either::Right(s) => s.clone(),
            })
        })
    }
}

enum Params<I1, I2, I3, I4> {
    Simple(I1),
    Intrinsic(I2),
    Builtin(I3),
    Rule(I4),
}

impl<I1, I2, I3, I4> Iterator for Params<I1, I2, I3, I4>
where
    I1: Iterator<Item = (Param, Ty)>,
    I2: Iterator<Item = (Param, Ty)>,
    I3: Iterator<Item = (Param, Ty)>,
    I4: Iterator<Item = (Param, Ty)>,
{
    type Item = (Param, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Params::Simple(iter) => iter.next(),
            Params::Intrinsic(iter) => iter.next(),
            Params::Builtin(iter) => iter.next(),
            Params::Rule(iter) => iter.next(),
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
            FieldInner::StructField { ref name, .. } => name.clone(),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> String {
        match self.0 {
            FieldInner::BuiltinField { parent, index } => parent.fields(db)[index].doc.clone(),
            FieldInner::BuiltinMethod { func } => func.doc(db).clone(),
            FieldInner::IntrinsicField { parent, index } => parent.fields(db)[index].doc.clone(),
            FieldInner::StructField { .. } => String::new(),
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
    StructField {
        name: Name,
    },
}

enum Fields<I1, I2, I3, I4> {
    Intrinsic(I1),
    Builtin(I2),
    Union(I3),
    Struct(I4),
}

impl<I1, I2, I3, I4> Iterator for Fields<I1, I2, I3, I4>
where
    I1: Iterator<Item = (Field, Ty)>,
    I2: Iterator<Item = (Field, Ty)>,
    I3: Iterator<Item = (Field, Ty)>,
    I4: Iterator<Item = (Field, Ty)>,
{
    type Item = (Field, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Intrinsic(iter) => iter.next(),
            Self::Builtin(iter) => iter.next(),
            Self::Union(iter) => iter.next(),
            Self::Struct(iter) => iter.next(),
        }
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
    Dict(Ty, Ty, Option<Arc<[(Box<str>, Ty)]>>),
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
    /// A union of two or more types.
    Union(SmallVec<[Ty; 2]>),
    /// A Bazel struct (https://bazel.build/rules/lib/builtins/struct).
    /// Use this instead of the `struct` type defined in `builtin.pb`.
    Struct(Box<[(Name, Ty)]>),
    /// A Bazel attribute (https://bazel.build/rules/lib/builtins/Attribute.html).
    /// Use this instead of the `Attribute` type defined in `builtin.pb`.
    Attribute(Attribute),
    /// A Bazel rule (https://bazel.build/rules/lib/builtins/rule).
    /// The `Ty`s contained in the boxed slice must be `TyKind::Attribute`s.
    Rule(Rule),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tuple {
    Simple(SmallVec<[Ty; 2]>),
    Variable(Ty),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AttributeKind {
    Bool,
    Int,
    IntList,
    Label,
    LabelKeyedStringDict,
    LabelList,
    Output,
    OutputList,
    String,
    StringDict,
    StringList,
    StringListDict,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub kind: AttributeKind,
    pub doc: Option<Box<str>>,
    pub mandatory: bool,
    pub default_text_range: Option<Either<(File, SyntaxNodePtr), String>>,
}

impl Attribute {
    pub fn new(
        kind: AttributeKind,
        doc: Option<Box<str>>,
        mandatory: bool,
        default_text_range: Option<Either<(File, SyntaxNodePtr), String>>,
    ) -> Self {
        Self {
            kind,
            doc,
            mandatory,
            default_text_range,
        }
    }

    pub fn expected_ty(&self) -> Ty {
        match self.kind {
            AttributeKind::Bool => Ty::bool(),
            AttributeKind::Int => Ty::int(),
            AttributeKind::IntList => Ty::list(Ty::int()),
            AttributeKind::String | AttributeKind::Label | AttributeKind::Output => Ty::string(),
            AttributeKind::StringDict | AttributeKind::LabelKeyedStringDict => {
                Ty::dict(Ty::string(), Ty::string(), None)
            }
            AttributeKind::StringList | AttributeKind::LabelList | AttributeKind::OutputList => {
                Ty::list(Ty::string())
            }
            AttributeKind::StringListDict => Ty::dict(Ty::string(), Ty::list(Ty::string()), None),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RuleKind {
    Build,
    Repository,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rule {
    pub kind: RuleKind,
    pub doc: Option<Box<str>>,
    attrs: Box<[(Name, Ty)]>,
}

impl Rule {
    pub fn attrs<'a>(&'a self, db: &'a dyn Db) -> impl Iterator<Item = (&Name, &Attribute)> {
        // This chaining is done to put the `name` attribute first.
        let common = common_attributes_query(db);
        let mut common_attrs = match self.kind {
            RuleKind::Build => common.build(db),
            RuleKind::Repository => common.repository(db),
        }
        .iter()
        .map(|(ref name, ref attr)| (name, attr));

        common_attrs
            .next()
            .into_iter()
            .chain(
                self.attrs
                    .iter()
                    .map(|(name, ty)| (name, ty.as_attribute())),
            )
            .chain(common_attrs)
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
            TyKind::Dict(_, _, _) => intrinsics.dict_base_class(db),
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
    pub(crate) args: SmallVec<[Ty; 2]>,
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
pub(crate) struct InferenceCtxt {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) resolved_load_stmts: FxHashMap<FileLoadStmt, Option<File>>,
    pub(crate) load_resolution_stack: Vec<(File, LoadStmt)>,
    pub(crate) type_of_expr: FxHashMap<FileExprId, Ty>,
    pub(crate) type_of_load_item: FxHashMap<FileLoadItemId, Ty>,
    pub(crate) type_of_param: FxHashMap<FileParamId, Ty>,
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

struct TypeRefResolver<'a> {
    db: &'a dyn Db,
    errors: Vec<String>,
}

impl<'a> TypeRefResolver<'a> {
    fn resolve_type_ref(mut self, type_ref: &TypeRef) -> (Ty, Vec<String>) {
        let ty = self.resolve_type_ref_inner(type_ref);
        (ty, self.errors)
    }

    fn resolve_type_ref_inner(&mut self, type_ref: &TypeRef) -> Ty {
        let types = intrinsic_types(self.db).types(self.db);
        // TODO(withered-magic): Need to resolve based on the dialect, but unclear how
        // to get that information from things like the `DisplayWithDb` impl for `TyKind`.
        let builtin_types = builtin_types(self.db, Dialect::Bazel);
        match type_ref {
            TypeRef::Name(name, args) => match name.as_str() {
                "Any" => types.any.clone(),
                "Unknown" | "unknown" => types.unknown.clone(),
                "None" | "NoneType" => types.none.clone(),
                "bool" => types.bool.clone(),
                "int" => types.int.clone(),
                "float" => types.float.clone(),
                "string" => types.string.clone(),
                "bytes" => types.bytes.clone(),
                "list" => self.resolve_single_arg_type_constructor(args, TyKind::List),
                "dict" => {
                    args.as_ref()
                        .and_then(|args| {
                            let mut args = args.iter();
                            match (args.next(), args.next()) {
                                (Some(key_ty), Some(value_ty)) => Some(TyKind::Dict(
                                    self.resolve_type_ref_inner(key_ty),
                                    self.resolve_type_ref_inner(value_ty),
                                    None,
                                )),
                                _ => None,
                            }
                        })
                        .unwrap_or_else(|| TyKind::Dict(types.any.clone(), types.any.clone(), None))
                }
                .intern(),
                "range" => types.range.clone(),
                "Iterable" | "iterable" => {
                    self.resolve_single_arg_protocol(args, Protocol::Iterable)
                }
                "Sequence" | "sequence" => {
                    self.resolve_single_arg_protocol(args, Protocol::Sequence)
                }
                "Union" | "union" => Ty::union(
                    args.iter()
                        .map(|args| args.iter())
                        .flatten()
                        .map(|type_ref| self.resolve_type_ref_inner(&type_ref)),
                ),
                "struct" | "structure" => TyKind::Struct(Vec::new().into_boxed_slice()).intern(),
                name => match builtin_types.types(self.db).get(name).cloned() {
                    Some(ty) => ty,
                    None => {
                        self.errors.push(format!("Unknown type \"{}\"", name));
                        types.unknown.clone()
                    }
                },
            },
            TypeRef::Union(args) => Ty::union(
                args.iter()
                    .map(|type_ref| self.resolve_type_ref_inner(&type_ref)),
            ),
            TypeRef::Unknown => types.unknown.clone(),
        }
    }

    fn resolve_single_arg_protocol(
        &mut self,
        args: &Option<Box<[TypeRef]>>,
        f: fn(Ty) -> Protocol,
    ) -> Ty {
        self.resolve_single_arg_type_constructor(args, |ty| TyKind::Protocol(f(ty)))
    }

    fn resolve_single_arg_type_constructor(
        &mut self,
        args: &Option<Box<[TypeRef]>>,
        f: impl Fn(Ty) -> TyKind,
    ) -> Ty {
        let arg = if let Some(args) = args {
            let mut args = args.iter();
            match (args.next(), args.next()) {
                (Some(first), second) => {
                    if second.is_some() {
                        self.errors
                            .push("Expected exactly one type argument".to_string())
                    }
                    self.resolve_type_ref_inner(first)
                }
                _ => Ty::unknown(),
            }
        } else {
            Ty::unknown()
        };

        f(arg).intern()
    }
}

pub(crate) fn resolve_type_ref(db: &dyn Db, type_ref: &TypeRef) -> (Ty, Vec<String>) {
    TypeRefResolver { db, errors: vec![] }.resolve_type_ref(type_ref)
}

pub(crate) fn resolve_type_ref_opt(db: &dyn Db, type_ref: Option<TypeRef>) -> Ty {
    type_ref
        .map(|type_ref| resolve_type_ref(db, &type_ref).0)
        .unwrap_or_else(|| Ty::unknown())
}

// TODO(withered-magic): This function currently assumes that all types are covariant in their arguments.
pub(crate) fn assign_tys(db: &dyn Db, source: &Ty, target: &Ty) -> bool {
    use Protocol::*;

    // // Assignments involving "Any", "Unknown", or "Unbound" at the top-level
    // // are always valid to avoid confusion.
    match (source.kind(), target.kind()) {
        (TyKind::Any | TyKind::Unknown, _) | (_, TyKind::Any | TyKind::Unknown) => true,
        (
            TyKind::List(source),
            TyKind::List(target) | TyKind::Protocol(Iterable(target) | Sequence(target)),
        ) => assign_tys(db, source, target),
        (TyKind::Tuple(tuple), TyKind::Protocol(Iterable(target) | Sequence(target))) => {
            match tuple {
                Tuple::Simple(sources) => {
                    sources.iter().all(|source| assign_tys(db, source, target))
                }
                Tuple::Variable(source) => assign_tys(db, source, target),
            }
        }
        (TyKind::Protocol(source), TyKind::Protocol(target)) => match &(source, target) {
            (Iterable(source), Iterable(target)) | (Sequence(source), Sequence(target)) => {
                assign_tys(db, source, target)
            }
            _ => false,
        },
        (TyKind::Dict(key_source, value_source, _), TyKind::Dict(key_target, value_target, _)) => {
            assign_tys(db, key_source, key_target) && assign_tys(db, value_source, value_target)
        }
        (TyKind::String, TyKind::BuiltinType(ty)) | (TyKind::BuiltinType(ty), TyKind::String)
            if ty.name(db).as_str() == "Label" =>
        {
            true
        }
        (TyKind::Union(source_tys), TyKind::Union(target_tys)) => {
            source_tys.iter().all(|source_ty| {
                target_tys
                    .iter()
                    .any(|target_ty| assign_tys(db, source_ty, target_ty))
            })
        }
        // TODO(withered-magic): The logic below also temporarily allows assignments like `int | None` to `int`. Fix
        // this once we support type guards.
        (_, TyKind::Union(tys)) => tys.iter().any(|target| assign_tys(db, source, target)),
        (TyKind::Union(tys), _) => tys.iter().any(|source| assign_tys(db, source, target)),
        (TyKind::Struct(_), TyKind::Struct(_)) => true,
        (TyKind::Attribute(_), TyKind::Attribute(_)) => true,
        (source, target) => source == target,
    }
}
