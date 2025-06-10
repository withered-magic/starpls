use std::fmt::Write;
use std::iter;
use std::panic::UnwindSafe;
use std::panic::{self};
use std::sync::Arc;

use crossbeam::atomic::AtomicCell;
use either::Either;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use smallvec::smallvec;
use smallvec::SmallVec;
use starpls_common::parse;
use starpls_common::Diagnostic;
use starpls_common::Dialect;
use starpls_common::File;
use starpls_common::InFile;
use starpls_intern::impl_internable;
use starpls_intern::Interned;
use starpls_syntax::ast::SyntaxNodePtr;

use crate::def::codeflow::FlowNodeId;
use crate::def::scope::ExecutionScopeId;
use crate::def::scope::FunctionDef;
use crate::def::ExprId;
use crate::def::InternedString;
use crate::def::LoadItemId;
use crate::def::LoadStmt;
use crate::def::Param as HirDefParam;
use crate::def::ParamId;
use crate::def::StmtId;
use crate::module;
use crate::source_map;
use crate::typeck::builtins::builtin_types;
use crate::typeck::builtins::common_attributes_query;
use crate::typeck::builtins::BuiltinFunction;
use crate::typeck::builtins::BuiltinFunctionParam;
use crate::typeck::builtins::BuiltinProvider;
use crate::typeck::builtins::BuiltinType;
use crate::typeck::intrinsics::intrinsic_field_types;
use crate::typeck::intrinsics::intrinsic_types;
use crate::typeck::intrinsics::IntrinsicClass;
use crate::typeck::intrinsics::IntrinsicFunction;
use crate::typeck::intrinsics::IntrinsicFunctionParam;
use crate::typeck::intrinsics::Intrinsics;
use crate::Db;
use crate::Name;
use crate::Param;
use crate::ParamInner;

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

#[derive(Clone, Debug, Default)]
pub struct InferenceOptions {
    pub infer_ctx_attributes: bool,
    pub use_code_flow_analysis: bool,
    pub allow_unused_definitions: bool,
}

#[derive(Default)]
struct SharedState {
    cancelled: AtomicCell<bool>,
    options: InferenceOptions,
}

/// A reference to a type in a source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TypeRef {
    /// A named type, e.g. `string` or `dict[int, bool]`
    Name(Name, Option<Box<[TypeRef]>>),
    /// A potentially-qualified type as specified by a type comment.
    Path(SmallVec<[Name; 1]>, Option<Box<[TypeRef]>>),
    /// A union of one or more types, e.g. `int | None`
    Union(Vec<TypeRef>),
    /// A provider type created with `provider()`, Bazel-only
    Provider(BuiltinProvider),
    /// Used to indicate a variable-length tuple, e.g. `tuple[int, ...]`
    Ellipsis,
    /// An unknown type
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
        fn write_type_args_opt(
            f: &mut std::fmt::Formatter<'_>,
            args: Option<&[TypeRef]>,
        ) -> std::fmt::Result {
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

        match self {
            TypeRef::Name(name, args) => {
                f.write_str(name.as_str())?;
                write_type_args_opt(f, args.as_deref())
            }
            TypeRef::Path(segments, args) if !segments.is_empty() => {
                f.write_str(segments.last().unwrap().as_str())?;
                write_type_args_opt(f, args.as_deref())
            }
            TypeRef::Union(names) => {
                for (i, type_ref) in names.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" | ")?;
                    }
                    type_ref.fmt(f)?;
                }
                Ok(())
            }
            _ => f.write_str("Unknown"),
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
            return Some(Fields::Intrinsic(self.intrinsic_class_fields(db, class)));
        }

        let fields = match kind {
            TyKind::BuiltinType(ty, data) => Fields::Builtin(
                ty.fields(db)
                    .iter()
                    .enumerate()
                    .map(move |(index, field)| {
                        let resolved = resolve_builtin_type_ref(db, &field.type_ref).0;
                        let resolved = match (resolved.kind(), data) {
                            // If `TyData` is set, this means the current type is either `ctx` or `repository_ctx`.
                            // Override the `attr` field for both of these types.
                            (TyKind::Struct(_), Some(TyData::Attributes(kind, attrs)))
                                if field.name.as_str() == "attr" =>
                            {
                                TyKind::Struct(Some(Struct::RuleAttributes {
                                    rule_kind: kind.clone(),
                                    attrs: attrs.clone(),
                                }))
                                .intern()
                            }
                            _ => resolved,
                        };
                        let field = Field(FieldInner::BuiltinField { parent: *ty, index });
                        (field, resolved)
                    })
                    .chain(ty.methods(db).iter().map(|func| {
                        (
                            Field(FieldInner::BuiltinMethod { func: *func }),
                            TyKind::BuiltinFunction(*func).intern(),
                        )
                    })),
            ),
            TyKind::Union(tys) => {
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
                Fields::Union(acc.into_iter())
            }
            TyKind::Struct(strukt) => Fields::Struct(
                strukt
                    .as_ref()
                    .and_then(|strukt| match strukt {
                        Struct::Inline { fields, .. } => Some(fields),
                        _ => None,
                    })
                    .into_iter()
                    .flat_map(|fields| fields.iter())
                    .map(|(name, ty)| {
                        (
                            Field(FieldInner::StructField {
                                name: name.clone(),
                                doc: None,
                            }),
                            ty.clone(),
                        )
                    }),
            ),
            TyKind::ProviderInstance(provider) => Fields::Provider(match provider {
                Provider::Builtin(builtin_provier) => {
                    ProviderFields::Builtin(builtin_provier.fields(db).iter().enumerate().map(
                        |(index, field)| {
                            (
                                Field(FieldInner::ProviderField {
                                    provider: provider.clone(),
                                    index,
                                }),
                                resolve_builtin_type_ref(db, &field.type_ref).0,
                            )
                        },
                    ))
                }
                Provider::Custom(custom_provider) => ProviderFields::Custom(
                    custom_provider
                        .fields
                        .as_ref()?
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(index, _)| {
                            (
                                Field(FieldInner::ProviderField {
                                    provider: provider.clone(),
                                    index,
                                }),
                                Ty::unknown(),
                            )
                        }),
                ),
            }),
            TyKind::ModuleExtensionProxy(module_extension) => Fields::ModuleExtensionProxy(
                module_extension
                    .tag_classes
                    .iter()
                    .flat_map(|tag_classes| tag_classes.iter())
                    .enumerate()
                    .map(|(index, data)| {
                        (
                            Field(FieldInner::ModuleExtensionProxyField {
                                module_extension: module_extension.clone(),
                                index,
                            }),
                            TyKind::Tag(data.tag_class.clone()).intern(),
                        )
                    }),
            ),
            TyKind::Target => {
                let label_ty = builtin_types(db, Dialect::Bazel)
                    .types(db)
                    .get("Label")
                    .cloned()
                    .unwrap_or_else(Ty::unknown);
                Fields::Static(iter::once((
                    Field(FieldInner::StaticField {
                        name: "label",
                        doc: Some("The identifier of the target."),
                    }),
                    label_ty,
                )))
            }
            _ => return None,
        };

        Some(fields)
    }

    fn intrinsic_class_fields<'a>(
        &'a self,
        db: &'a dyn Db,
        class: IntrinsicClass,
    ) -> impl Iterator<Item = (Field, Ty)> + 'a {
        let fields = (0..class.fields(db).len()).map(move |index| {
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
            TyKind::Function(def) => Params::Simple(def.func().params(db).iter().enumerate().map(
                |(index, param)| {
                    let file = def.func().file(db);
                    let ty = with_tcx(db, |tcx| tcx.infer_param(file, *param));
                    let param = Param(ParamInner::Param {
                        func: def.func(),
                        index,
                    });
                    (param, ty)
                },
            )),
            TyKind::IntrinsicFunction(func, subst) => {
                Params::Intrinsic(func.params(db).iter().enumerate().map(|(index, param)| {
                    let ty = param
                        .ty()
                        .unwrap_or_else(Ty::unknown)
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
                    let ty = resolve_builtin_type_ref_opt(db, param.type_ref());
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
                        .chain(
                            attrs
                                .as_ref()
                                .map(|attrs| {
                                    attrs.attrs.iter().filter_map(|(name, attr)| {
                                        attr.as_ref().map(|attr| {
                                            (
                                                Param(
                                                    RuleParam::Keyword {
                                                        name: name.clone(),
                                                        attr: attr.clone(),
                                                    }
                                                    .into(),
                                                ),
                                                attr.expected_ty(),
                                            )
                                        })
                                    })
                                })
                                .into_iter()
                                .flatten(),
                        )
                        .chain(common_attrs)
                        .chain(iter::once((
                            Param(RuleParam::Kwargs.into()),
                            TyKind::Dict(Ty::string(), Ty::any(), None).intern(),
                        ))),
                )
            }
            TyKind::Provider(provider) | TyKind::ProviderRawConstructor(_, provider) => {
                Params::Provider(match provider {
                    Provider::Builtin(builtin_provider) => {
                        ProviderParams::Builtin(builtin_provider.params(db).iter().enumerate().map(
                            |(index, param)| {
                                (
                                    Param(ParamInner::ProviderParam {
                                        provider: provider.clone(),
                                        index,
                                    }),
                                    resolve_builtin_type_ref_opt(db, param.type_ref()),
                                )
                            },
                        ))
                    }
                    Provider::Custom(custom_provider) => {
                        ProviderParams::Custom(custom_provider.fields.iter().flat_map(|fields| {
                            fields.fields.iter().enumerate().map(|(index, _)| {
                                (
                                    Param(ParamInner::ProviderParam {
                                        provider: provider.clone(),
                                        index,
                                    }),
                                    Ty::unknown(),
                                )
                            })
                        }))
                    }
                })
            }
            TyKind::Tag(tag_class) => Params::Tag(
                tag_class
                    .attrs
                    .iter()
                    .flat_map(|attrs| attrs.iter())
                    .map(|data| {
                        (
                            Param(
                                TagParam::Keyword {
                                    name: data.name.clone(),
                                    attr: data.attr.clone(),
                                }
                                .into(),
                            ),
                            data.attr.expected_ty(),
                        )
                    })
                    .chain(iter::once((
                        Param(TagParam::Kwargs.into()),
                        TyKind::Dict(Ty::string(), Ty::any(), None).intern(),
                    ))),
            ),
            TyKind::Macro(makro) => Params::Macro(
                makro
                    .attrs
                    .as_ref()
                    .map(|attrs| {
                        attrs.attrs.iter().filter_map(|(name, attr)| {
                            attr.as_ref().map(|attr| {
                                (
                                    Param(
                                        RuleParam::Keyword {
                                            name: name.clone(),
                                            attr: attr.clone(),
                                        }
                                        .into(),
                                    ),
                                    attr.expected_ty(),
                                )
                            })
                        })
                    })
                    .into_iter()
                    .flatten(),
            ),
            _ => return None,
        })
    }

    pub(crate) fn ret_ty(&self, db: &dyn Db) -> Option<Ty> {
        Some(match self.kind() {
            TyKind::Function(def) => resolve_builtin_type_ref_opt(db, def.func().ret_type_ref(db)),
            TyKind::IntrinsicFunction(func, subst) => func.ret_ty(db).substitute(&subst.args),
            TyKind::BuiltinFunction(func) => resolve_builtin_type_ref(db, func.ret_type_ref(db)).0,
            TyKind::Provider(provider) | TyKind::ProviderRawConstructor(_, provider) => {
                TyKind::ProviderInstance(provider.clone()).intern()
            }
            TyKind::Tag(_) | TyKind::Macro(_) | TyKind::Rule(_) => Ty::none(),
            _ => return None,
        })
    }

    pub(crate) fn none() -> Ty {
        TyKind::None.intern()
    }

    pub(crate) fn unknown() -> Ty {
        TyKind::Unknown.intern()
    }

    pub(crate) fn any() -> Ty {
        TyKind::Any.intern()
    }

    pub(crate) fn never() -> Ty {
        TyKind::Never.intern()
    }

    pub(crate) fn bool() -> Ty {
        TyKind::Bool(None).intern()
    }

    pub(crate) fn int() -> Ty {
        TyKind::Int(None).intern()
    }

    pub(crate) fn string() -> Ty {
        TyKind::String(None).intern()
    }

    pub(crate) fn list(ty: Ty) -> Ty {
        TyKind::List(ty).intern()
    }

    pub(crate) fn dict(key_ty: Ty, value_ty: Ty, known_keys: Option<Arc<DictLiteral>>) -> Ty {
        TyKind::Dict(key_ty, value_ty, known_keys).intern()
    }

    pub(crate) fn union(tys: impl Iterator<Item = Ty>) -> Ty {
        let mut unique_tys = smallvec![];

        // Deduplicate types. Dicts and structs are handled separately because the metadata
        // that they store // (for their declarations, etc.) is not relevant to determining
        // whether or not types are duplicates.
        for ty in tys {
            let mut check_unique = |ty: Ty| {
                let ty = ty.normalize();
                if unique_tys
                    .iter()
                    .any(|unique_ty: &Ty| Ty::eq(&ty, &unique_ty.clone().normalize()))
                {
                    return;
                }
                unique_tys.push(ty);
            };
            match ty.kind() {
                TyKind::Union(tys) => {
                    tys.iter().cloned().for_each(check_unique);
                }
                TyKind::Never => {}
                _ => check_unique(ty),
            }
        }

        match unique_tys.len() {
            0 => Ty::never(),
            1 => unique_tys.into_iter().next().unwrap(),
            _ => TyKind::Union(unique_tys).intern(),
        }
    }

    pub(crate) fn target() -> Ty {
        TyKind::Target.intern()
    }

    #[allow(unused)]
    fn is_any(&self) -> bool {
        self.kind() == &TyKind::Any
    }

    #[allow(unused)]
    fn is_unknown(&self) -> bool {
        self.kind() == &TyKind::Unknown || self.kind() == &TyKind::Unbound
    }

    pub(crate) fn is_unbound(&self) -> bool {
        self.kind() == &TyKind::Unbound
    }

    pub(crate) fn is_possibly_unbound(&self) -> bool {
        match self.kind() {
            TyKind::Union(tys) => tys.iter().any(|ty| ty.is_possibly_unbound()),
            TyKind::Unbound => true,
            _ => false,
        }
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
            TyKind::Dict(key_ty, value_ty, lit) => Ty::dict(
                key_ty.substitute(args),
                value_ty.substitute(args),
                lit.as_ref().cloned(),
            ),
            TyKind::IntrinsicFunction(data, subst) => {
                TyKind::IntrinsicFunction(*data, subst.substitute(args)).intern()
            }
            TyKind::BoundVar(index) => args[*index].clone(),
            _ => self.clone(),
        }
    }

    pub(crate) fn known_keys(&self) -> Option<&[(InternedString, Ty)]> {
        match self.kind() {
            TyKind::Dict(_, _, known_keys) => known_keys.as_ref().map(|lit| &*lit.known_keys),
            _ => None,
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
            (TyKind::Attribute(_), TyKind::Attribute(_)) => true,
            (TyKind::BuiltinType(ty1, _), TyKind::BuiltinType(ty2, _)) => ty1 == ty2,
            _ => ty1 == ty2,
        }
    }

    fn normalize(self) -> Ty {
        match self.kind() {
            TyKind::Bool(_) => Ty::bool(),
            TyKind::Int(_) => Ty::int(),
            TyKind::String(_) => Ty::string(),
            TyKind::Attribute(_) => TyKind::Attribute(None).intern(),
            _ => self,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RuleParam {
    Keyword { name: Name, attr: Attribute },
    BuiltinKeyword(RuleKind, usize),
    Kwargs,
}

impl From<RuleParam> for ParamInner {
    fn from(value: RuleParam) -> Self {
        Self::RuleParam(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TagParam {
    Keyword { name: Name, attr: Attribute },
    Kwargs,
}

impl From<TagParam> for ParamInner {
    fn from(value: TagParam) -> Self {
        Self::TagParam(value)
    }
}

impl Param {
    pub fn name(&self, db: &dyn Db) -> Option<Name> {
        match self.0 {
            ParamInner::Param { func, index } => {
                let module = module(db, func.file(db));
                Some(module[func.params(db)[index]].name().clone())
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
            ParamInner::ProviderParam {
                ref provider,
                index,
            } => Some(match provider {
                Provider::Builtin(provider) => provider.params(db)[index].name(),
                Provider::Custom(provider) => provider
                    .fields
                    .as_ref()
                    .expect("expected provider fields")
                    .fields[index]
                    .name
                    .clone(),
            }),
            ParamInner::TagParam(TagParam::Keyword { ref name, .. }) => Some(name.clone()),
            ParamInner::TagParam(TagParam::Kwargs) => Some(Name::new_inline("kwargs")),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> Option<String> {
        Some(match &self.0 {
            ParamInner::Param { func, index } => {
                let module = module(db, func.file(db));
                return module[func.params(db)[*index]]
                    .doc()
                    .map(|doc| doc.to_string());
            }
            ParamInner::BuiltinParam { parent, index } => {
                parent.params(db)[*index].doc().to_string()
            }
            ParamInner::IntrinsicParam { .. } => return None,
            ParamInner::RuleParam(RuleParam::Keyword { attr, .. }) => {
                return attr.doc.map(|doc| doc.value(db).to_string())
            }
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(kind, index)) => {
                return common_attributes_query(db)
                    .get(db, kind.clone(), *index)
                    .1
                    .doc
                    .as_ref()
                    .map(|doc| doc.value(db).to_string())
            }
            ParamInner::ProviderParam { provider, index } => match provider {
                Provider::Builtin(provider) => provider.params(db)[*index].doc().to_string(),
                Provider::Custom(provider) => {
                    return provider
                        .fields
                        .as_ref()
                        .expect("expected provider fields")
                        .fields[*index]
                        .doc
                        .as_ref()
                        .map(Box::to_string)
                }
            },
            ParamInner::TagParam(TagParam::Keyword { attr, .. }) => {
                return attr.doc.map(|doc| doc.value(db).to_string())
            }
            _ => return None,
        })
    }

    pub fn is_args_list(&self, db: &dyn Db) -> bool {
        match self.0 {
            // TODO(withered-magic): Handle lambda parameters.
            ParamInner::Param { func, index } => {
                let module = module(db, func.file(db));
                matches!(module[func.params(db)[index]], HirDefParam::ArgsList { .. })
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
            ParamInner::Param { func, index } => {
                let module = module(db, func.file(db));
                matches!(
                    module[func.params(db)[index]],
                    HirDefParam::KwargsDict { .. }
                )
            }
            ParamInner::IntrinsicParam { parent, index } => {
                matches!(parent.params(db)[index], IntrinsicFunctionParam::KwargsDict)
            }
            ParamInner::BuiltinParam { parent, index } => matches!(
                parent.params(db)[index],
                BuiltinFunctionParam::KwargsDict { .. }
            ),
            ParamInner::RuleParam(RuleParam::Kwargs) => true,
            ParamInner::TagParam(TagParam::Kwargs) => true,
            ParamInner::ProviderParam {
                provider: Provider::Builtin(ref provider),
                index,
            } => {
                matches!(
                    provider.params(db)[index],
                    BuiltinFunctionParam::KwargsDict { .. }
                )
            }
            _ => false,
        }
    }

    pub fn syntax_node_ptr(&self, db: &dyn Db) -> Option<InFile<SyntaxNodePtr>> {
        match self.0 {
            ParamInner::Param { func, index } => {
                let file = func.file(db);
                source_map(db, file)
                    .param_map_back
                    .get(&func.params(db)[index])
                    .map(|ptr| InFile {
                        file,
                        value: ptr.syntax_node_ptr(),
                    })
            }
            _ => None,
        }
    }

    pub fn is_positional_only(&self, db: &dyn Db) -> bool {
        match self.0 {
            ParamInner::IntrinsicParam { parent, index } => matches!(
                parent.params(db)[index],
                IntrinsicFunctionParam::Positional { .. }
            ),
            _ => false,
        }
    }

    pub fn default_value(&self, db: &dyn Db) -> Option<String> {
        let common = common_attributes_query(db);
        let attr = match &self.0 {
            ParamInner::BuiltinParam { parent, index } => match &parent.params(db)[*index] {
                BuiltinFunctionParam::Simple { default_value, .. } => return default_value.clone(),
                _ => return None,
            },
            ParamInner::RuleParam(RuleParam::Keyword { attr, .. }) => attr,
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(kind, index)) => {
                common.get(db, kind.clone(), *index).1
            }
            _ => return None,
        };

        attr.default_value.as_ref().and_then(|e| {
            Some(match e {
                Either::Left(ptr) => ptr
                    .value
                    .to_owned()
                    .try_to_node(&parse(db, ptr.file).syntax(db))?
                    .text()
                    .to_string(),
                Either::Right(s) => s.value(db).to_string(),
            })
        })
    }
}

enum ProviderParams<I1, I2> {
    Builtin(I1),
    Custom(I2),
}

impl<I1, I2> Iterator for ProviderParams<I1, I2>
where
    I1: Iterator<Item = (Param, Ty)>,
    I2: Iterator<Item = (Param, Ty)>,
{
    type Item = (Param, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ProviderParams::Builtin(it) => it.next(),
            ProviderParams::Custom(it) => it.next(),
        }
    }
}

enum Params<I1, I2, I3, I4, I5, I6, I7, I8> {
    Simple(I1),
    Intrinsic(I2),
    Builtin(I3),
    Rule(I4),
    Provider(ProviderParams<I5, I6>),
    Tag(I7),
    Macro(I8),
}

impl<I1, I2, I3, I4, I5, I6, I7, I8> Iterator for Params<I1, I2, I3, I4, I5, I6, I7, I8>
where
    I1: Iterator<Item = (Param, Ty)>,
    I2: Iterator<Item = (Param, Ty)>,
    I3: Iterator<Item = (Param, Ty)>,
    I4: Iterator<Item = (Param, Ty)>,
    I5: Iterator<Item = (Param, Ty)>,
    I6: Iterator<Item = (Param, Ty)>,
    I7: Iterator<Item = (Param, Ty)>,
    I8: Iterator<Item = (Param, Ty)>,
{
    type Item = (Param, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Params::Simple(it) => it.next(),
            Params::Intrinsic(it) => it.next(),
            Params::Builtin(it) => it.next(),
            Params::Rule(it) => it.next(),
            Params::Provider(it) => it.next(),
            Params::Tag(it) => it.next(),
            Params::Macro(it) => it.next(),
        }
    }
}

pub struct Field(pub(crate) FieldInner);

impl Field {
    pub fn name(&self, db: &dyn Db) -> Name {
        match self.0 {
            FieldInner::BuiltinField { parent, index } => parent.fields(db)[index].name.clone(),
            FieldInner::BuiltinMethod { func } => func.name(db),
            FieldInner::IntrinsicField { parent, index } => parent.fields(db)[index].name.clone(),
            FieldInner::StructField { ref name, .. } => name.clone(),
            FieldInner::ProviderField {
                ref provider,
                index,
            } => match provider {
                Provider::Builtin(provider) => provider.fields(db)[index].name.clone(),
                Provider::Custom(provider) => provider
                    .fields
                    .as_ref()
                    .expect("expected provider fields")
                    .fields[index]
                    .name
                    .clone(),
            },
            FieldInner::ModuleExtensionProxyField {
                ref module_extension,
                index,
            } => module_extension
                .tag_classes
                .as_ref()
                .expect("expected module_extension tag classes")[index]
                .name
                .clone(),
            FieldInner::StaticField { name, .. } => Name::from_str(name),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> String {
        match self.0 {
            FieldInner::BuiltinField { parent, index } => parent.fields(db)[index].doc.clone(),
            FieldInner::BuiltinMethod { func } => func.doc(db).clone(),
            FieldInner::IntrinsicField { parent, index } => parent.fields(db)[index].doc.clone(),
            FieldInner::StructField { ref doc, .. } => doc.as_ref().cloned().unwrap_or_default(),
            FieldInner::ProviderField {
                ref provider,
                index,
            } => match provider {
                Provider::Builtin(provider) => provider.fields(db)[index].doc.clone(),
                Provider::Custom(provider) => provider
                    .fields
                    .as_ref()
                    .expect("expected provider fields")
                    .fields[index]
                    .doc
                    .as_ref()
                    .map(Box::to_string)
                    .unwrap_or_default(),
            },
            FieldInner::ModuleExtensionProxyField {
                ref module_extension,
                index,
            } => module_extension
                .tag_classes
                .as_ref()
                .expect("expected module_extension tag classes")[index]
                .tag_class
                .doc
                .as_ref()
                .map(|doc| doc.value(db).to_string())
                .unwrap_or_default(),
            FieldInner::StaticField { doc, .. } => doc.unwrap_or_default().to_string(),
        }
    }
}

pub(crate) enum FieldInner {
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
        doc: Option<String>,
    },
    ProviderField {
        provider: Provider,
        index: usize,
    },
    ModuleExtensionProxyField {
        module_extension: Arc<ModuleExtension>,
        index: usize,
    },
    StaticField {
        name: &'static str,
        doc: Option<&'static str>,
    },
}

enum ProviderFields<I1, I2> {
    Builtin(I1),
    Custom(I2),
}

impl<I1, I2> Iterator for ProviderFields<I1, I2>
where
    I1: Iterator<Item = (Field, Ty)>,
    I2: Iterator<Item = (Field, Ty)>,
{
    type Item = (Field, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ProviderFields::Builtin(it) => it.next(),
            ProviderFields::Custom(it) => it.next(),
        }
    }
}

enum Fields<I1, I2, I3, I4, I5, I6, I7, I8> {
    Intrinsic(I1),
    Builtin(I2),
    Union(I3),
    Struct(I4),
    Provider(ProviderFields<I5, I6>),
    ModuleExtensionProxy(I7),
    Static(I8),
}

impl<I1, I2, I3, I4, I5, I6, I7, I8> Iterator for Fields<I1, I2, I3, I4, I5, I6, I7, I8>
where
    I1: Iterator<Item = (Field, Ty)>,
    I2: Iterator<Item = (Field, Ty)>,
    I3: Iterator<Item = (Field, Ty)>,
    I4: Iterator<Item = (Field, Ty)>,
    I5: Iterator<Item = (Field, Ty)>,
    I6: Iterator<Item = (Field, Ty)>,
    I7: Iterator<Item = (Field, Ty)>,
    I8: Iterator<Item = (Field, Ty)>,
{
    type Item = (Field, Ty);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Intrinsic(it) => it.next(),
            Self::Builtin(it) => it.next(),
            Self::Union(it) => it.next(),
            Self::Struct(it) => it.next(),
            Self::Provider(it) => it.next(),
            Self::ModuleExtensionProxy(it) => it.next(),
            Self::Static(it) => it.next(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TyData {
    Attributes(RuleKind, Arc<RuleAttributes>),
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

    /// Indicates that the corresponding expression will never be evaluated.
    Never,

    /// The type of the predefined `None` variable.
    None,

    /// A boolean.
    Bool(Option<bool>),

    /// A 64-bit integer.
    Int(Option<i64>),

    /// A 64-bit floating point number.
    Float,

    /// A UTF-8 encoded string.
    String(Option<InternedString>),

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
    Dict(Ty, Ty, Option<Arc<DictLiteral>>),

    /// An iterable and indexable sequence of numbers. Obtained from
    /// the `range()` function.
    Range,

    /// A user-defined function.
    Function(FunctionDef),

    /// A function predefined by the Starlark specification.
    IntrinsicFunction(IntrinsicFunction, Substitution),

    /// A function defined outside of the Starlark specification.
    /// For example, common Bazel functions like `genrule()`.
    BuiltinFunction(BuiltinFunction),

    /// A type defined outside of the Starlark specification.
    /// For example, common Bazel types like `Label`.
    BuiltinType(BuiltinType, Option<TyData>),

    /// A bound type variable, e.g. the argument to the `append()` method
    /// of the `list[int]` class.
    BoundVar(usize),

    /// A marker type that indicates some specific behavior, e.g. Sequence[T].
    Protocol(Protocol),

    /// A union of two or more types.
    Union(SmallVec<[Ty; 2]>),

    /// A Bazel struct (https://bazel.build/rules/lib/builtins/struct).
    /// Use this instead of the `struct` type defined in `builtin.pb`.
    Struct(Option<Struct>),

    /// A Bazel attribute (https://bazel.build/rules/lib/builtins/Attribute.html).
    /// Use this instead of the `Attribute` type defined in `builtin.pb`.
    Attribute(Option<Attribute>),

    /// A Bazel rule (https://bazel.build/rules/lib/builtins/rule).
    Rule(Rule),

    /// A Bazel provider (https://bazel.build/rules/lib/builtins/Provider.html).
    /// This is a callable that yields "provider instances".
    Provider(Provider),

    /// An instance of a Bazel provider.
    ProviderInstance(Provider),

    /// The raw constructor for a Bazel provider.
    ProviderRawConstructor(Name, Provider),

    /// A Bazel tag class.
    TagClass(Arc<TagClass>),

    /// A Bazel module extension.
    ModuleExtension(Arc<ModuleExtension>),

    /// A Bazel module extension proxy.
    ModuleExtensionProxy(Arc<ModuleExtension>),

    /// A Bazel tag (e.g. `maven.artifact()`).
    Tag(Arc<TagClass>),

    /// A Bazel target (https://bazel.build/rules/lib/builtins/Target).
    Target,

    /// A Bazel symbolic macro.
    Macro(Macro),
}

impl_internable!(TyKind);

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
    StringKeyedLabelDict,
    StringList,
    StringListDict,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub kind: AttributeKind,
    pub doc: Option<InternedString>,
    pub mandatory: bool,
    pub default_value: Option<Either<InFile<SyntaxNodePtr>, InternedString>>,
}

impl Attribute {
    pub fn new(
        kind: AttributeKind,
        doc: Option<InternedString>,
        mandatory: bool,
        default_value: Option<Either<InFile<SyntaxNodePtr>, InternedString>>,
    ) -> Self {
        Self {
            kind,
            doc,
            mandatory,
            default_value,
        }
    }

    pub fn expected_ty(&self) -> Ty {
        match self.kind {
            AttributeKind::Bool => Ty::bool(),
            AttributeKind::Int => Ty::int(),
            AttributeKind::IntList => Ty::list(Ty::int()),
            AttributeKind::String | AttributeKind::Label | AttributeKind::Output => Ty::string(),
            AttributeKind::StringDict
            | AttributeKind::LabelKeyedStringDict
            | AttributeKind::StringKeyedLabelDict => Ty::dict(Ty::string(), Ty::string(), None),
            AttributeKind::StringList | AttributeKind::LabelList | AttributeKind::OutputList => {
                Ty::list(Ty::string())
            }
            AttributeKind::StringListDict => Ty::dict(Ty::string(), Ty::list(Ty::string()), None),
        }
    }

    pub fn resolved_ty(&self, rule_kind: &RuleKind) -> Ty {
        let resolved_label_ty = || match rule_kind {
            RuleKind::Build => Ty::target(),
            // TODO(withered-magic): This should be the `Label` type, maybe we should retrieve it
            // from the builtins?
            RuleKind::Repository => Ty::unknown(),
        };

        match self.kind {
            AttributeKind::Bool => Ty::bool(),
            AttributeKind::Int => Ty::int(),
            AttributeKind::IntList => Ty::list(Ty::int()),
            AttributeKind::String => Ty::string(),
            AttributeKind::Label => resolved_label_ty(),
            AttributeKind::Output => Ty::unknown(),
            AttributeKind::StringDict => Ty::dict(Ty::string(), Ty::string(), None),
            AttributeKind::StringKeyedLabelDict => {
                Ty::dict(Ty::string(), resolved_label_ty(), None)
            }
            AttributeKind::LabelKeyedStringDict => {
                Ty::dict(resolved_label_ty(), Ty::string(), None)
            }
            AttributeKind::StringList => Ty::list(Ty::string()),
            AttributeKind::LabelList => Ty::list(resolved_label_ty()),
            AttributeKind::OutputList => Ty::list(Ty::unknown()),
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
pub(crate) struct RuleAttributes {
    /// The actual attributes.
    /// `None` in the second element of the tuple means that the attribute
    /// *cannot* be specfied during an invocation of the corresponding macro.
    pub(crate) attrs: Vec<(Name, Option<Attribute>)>,
    /// The dict expression where these attributes were defined.
    pub(crate) expr: Option<InFile<ExprId>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Rule {
    pub(crate) kind: RuleKind,
    pub(crate) doc: Option<Box<str>>,
    pub(crate) attrs: Option<Arc<RuleAttributes>>,
}

impl Rule {
    pub(crate) fn attrs<'a>(
        &'a self,
        db: &'a dyn Db,
    ) -> impl Iterator<Item = (&'a Name, &'a Attribute)> {
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
                    .as_ref()
                    .map(|attrs| {
                        attrs
                            .attrs
                            .iter()
                            .filter_map(|(name, attr)| attr.as_ref().map(|attr| (name, attr)))
                    })
                    .into_iter()
                    .flatten(),
            )
            .chain(common_attrs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CustomProviderFields {
    pub(crate) fields: Box<[ProviderField]>,
    pub(crate) expr: Option<InFile<ExprId>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CustomProvider {
    pub(crate) name: Option<Name>,
    pub(crate) doc: Option<InternedString>,
    pub(crate) fields: Option<CustomProviderFields>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Provider {
    Builtin(BuiltinProvider),
    Custom(Arc<CustomProvider>),
}

impl Provider {
    pub(crate) fn name<'a>(&'a self, db: &'a dyn Db) -> Option<&'a Name> {
        match self {
            Provider::Builtin(provider) => Some(provider.name(db)),
            Provider::Custom(provider) => provider.name.as_ref(),
        }
    }

    pub(crate) fn doc(&self, db: &dyn Db) -> Option<String> {
        match self {
            Provider::Builtin(provider) => Some(provider.doc(db).clone()),
            Provider::Custom(provider) => provider.doc.map(|doc| doc.value(db).to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Struct {
    Inline {
        call_expr: InFile<ExprId>,
        fields: Box<[(Name, Ty)]>,
    },
    FieldSignature {
        ty: Ty,
    },
    RuleAttributes {
        rule_kind: RuleKind,
        attrs: Arc<RuleAttributes>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TagClassData {
    pub(crate) name: Name,
    pub(crate) tag_class: Arc<TagClass>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ModuleExtension {
    pub(crate) doc: Option<Box<str>>,
    pub(crate) tag_classes: Option<Box<[TagClassData]>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AttributeData {
    pub(crate) name: Name,
    pub(crate) attr: Attribute,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TagClass {
    pub(crate) attrs: Option<Box<[AttributeData]>>,
    pub(crate) doc: Option<InternedString>,
}

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty(Interned::new(self))
    }

    pub fn builtin_class(&self, db: &dyn Db) -> Option<IntrinsicClass> {
        let intrinsics = intrinsic_types(db);
        Some(match self {
            TyKind::String(_) => intrinsics.string_base_class(db),
            TyKind::Bytes => intrinsics.bytes_base_class(db),
            TyKind::List(_) => intrinsics.list_base_class(db),
            TyKind::Dict(_, _, _) => intrinsics.dict_base_class(db),
            _ => return None,
        })
    }
}

/// A Bazel symbolic macro created by the `macro()` function.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Macro {
    /// Attributes defined in the `attrs` argument to the `macro()` function.
    pub(crate) attrs: Option<Arc<RuleAttributes>>,
    pub(crate) doc: Option<InternedString>,
}

impl Macro {
    pub(crate) fn attrs(&self) -> impl Iterator<Item = (&Name, &Attribute)> {
        self.attrs
            .as_ref()
            .map(|attrs| {
                attrs
                    .attrs
                    .iter()
                    .filter_map(|(name, attr)| attr.as_ref().map(|attr| (name, attr)))
            })
            .into_iter()
            .flatten()
    }

    pub(crate) fn disallowed_attrs(&self) -> impl Iterator<Item = &Name> {
        self.attrs
            .as_ref()
            .map(|attrs| {
                attrs
                    .attrs
                    .iter()
                    .filter_map(|(name, attr)| if attr.is_none() { Some(name) } else { None })
            })
            .into_iter()
            .flatten()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ProviderField {
    pub(crate) name: Name,
    pub(crate) doc: Option<Box<str>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DictLiteral {
    pub(crate) expr: Option<InFile<ExprId>>,
    pub(crate) known_keys: Box<[(InternedString, Ty)]>,
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
pub struct GlobalContext {
    shared_state: Arc<SharedState>,
    cx: Arc<Mutex<InferenceContext>>,
}

impl GlobalContext {
    pub fn new(options: InferenceOptions) -> Self {
        Self {
            shared_state: Arc::new(SharedState {
                options,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn cancel(&self) -> CancelGuard {
        CancelGuard::new(self)
    }

    pub fn with_tcx<F, T>(&self, db: &dyn Db, mut f: F) -> T
    where
        F: FnMut(&mut TyContext) -> T + std::panic::UnwindSafe,
    {
        let mut cx = self.cx.lock();
        let mut tcx = TyContext {
            db,
            cx: &mut cx,
            intrinsics: intrinsic_types(db),
            shared_state: Arc::clone(&self.shared_state),
        };
        f(&mut tcx)
    }
}

pub(crate) fn with_tcx<F, T>(db: &dyn Db, f: F) -> T
where
    F: FnMut(&mut TyContext) -> T + std::panic::UnwindSafe,
{
    db.gcx().with_tcx(db, f)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CodeFlowCacheKey {
    file: File,
    execution_scope: ExecutionScopeId,
    name: Name,
    flow_node: FlowNodeId,
}

#[allow(unused)]
#[derive(Default)]
pub(crate) struct InferenceContext {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) resolved_load_stmts: FxHashMap<FileLoadStmt, Option<File>>,
    pub(crate) load_resolution_stack: Vec<(File, LoadStmt)>,
    pub(crate) type_of_expr: FxHashMap<FileExprId, Ty>,
    pub(crate) type_of_load_item: FxHashMap<FileLoadItemId, Ty>,
    pub(crate) type_of_param: FxHashMap<FileParamId, Ty>,
    pub(crate) source_assign_done: FxHashSet<FileExprId>,
    pub(crate) flow_node_type_cache: FxHashMap<CodeFlowCacheKey, Option<Ty>>,
    pub(crate) definition_is_used: FxHashMap<InFile<Either<ExprId, StmtId>>, bool>,
}

pub struct CancelGuard<'a> {
    gcx: &'a GlobalContext,
    cx: &'a Mutex<InferenceContext>,
}

impl<'a> CancelGuard<'a> {
    fn new(gcx: &'a GlobalContext) -> Self {
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

pub struct TyContext<'a> {
    db: &'a dyn Db,
    cx: &'a mut InferenceContext,
    intrinsics: Intrinsics,
    shared_state: Arc<SharedState>,
}

struct TypeRefResolver<'a, 'b> {
    db: &'a dyn Db,
    context: Option<(&'a mut TyContext<'b>, InFile<StmtId>)>,
    errors: Vec<String>,
}

impl<'a, 'b> TypeRefResolver<'a, 'b> {
    fn resolve_type_ref(mut self, type_ref: &TypeRef) -> (Ty, Vec<String>) {
        let ty = self.resolve_type_ref_inner(type_ref);
        (ty, self.errors)
    }

    fn resolve_type_ref_inner(&mut self, type_ref: &TypeRef) -> Ty {
        let types = intrinsic_types(self.db).types(self.db);
        // TODO(withered-magic): Need to resolve based on the dialect, but unclear how
        // to get that information from things like the `DisplayWithDb` impl for `TyKind`.
        match type_ref {
            TypeRef::Name(name, args) => self.resolve_path([name].into_iter(), args),
            TypeRef::Path(segments, args) => self.resolve_path(segments.iter(), args),
            TypeRef::Union(args) => Ty::union(
                args.iter()
                    .map(|type_ref| self.resolve_type_ref_inner(type_ref)),
            ),
            TypeRef::Provider(provider) => TyKind::Provider(Provider::Builtin(*provider)).intern(),
            TypeRef::Ellipsis => {
                // We handle ellipsis types only while processing tuples above, any other occurrences of
                // ellipsis types are invalid.
                self.errors
                    .push("\"...\" is not allowed in this context".to_string());
                types.unknown.clone()
            }
            _ => types.unknown.clone(),
        }
    }

    fn resolve_segments<'c>(
        &mut self,
        db: &dyn Db,
        name: &Name,
        mut next: &'c Name,
        mut segments: impl Iterator<Item = &'c Name>,
    ) -> Option<Ty> {
        let (tcx, usage) = match &mut self.context {
            Some(context) => context,
            None => return None,
        };
        let mut ty = tcx.infer_name(usage.file, name, usage.value)?;
        loop {
            ty = ty
                .fields(db)
                .and_then(|mut fields| fields.find(|(field, _ty)| field.name(db).eq(next)))
                .map(|(_field, ty)| ty.clone())?;
            next = match segments.next() {
                Some(next) => next,
                None => break,
            }
        }
        match ty.kind() {
            TyKind::Provider(provider) => Some(TyKind::ProviderInstance(provider.clone()).intern()),
            _ => None,
        }
    }

    fn resolve_path<'c>(
        &mut self,
        mut segments: impl Iterator<Item = &'c Name>,
        args: &Option<Box<[TypeRef]>>,
    ) -> Ty {
        let types = intrinsic_types(self.db).types(self.db);
        let builtin_types = builtin_types(self.db, Dialect::Bazel);
        let name = match segments.next() {
            Some(name) => name,
            None => return types.unknown.clone(),
        };

        if let Some(next) = segments.next() {
            return self
                .resolve_segments(self.db, name, next, segments)
                .unwrap_or_else(|| types.unknown.clone());
        }

        // If `usage` was passed, try to resolve as a custom provider defined in the corresponding scope.
        if let Some((tcx, usage)) = &mut self.context {
            if let Some(ty) = tcx.infer_name(usage.file, name, usage.value) {
                if let TyKind::Provider(provider) = ty.kind() {
                    return TyKind::ProviderInstance(provider.clone()).intern();
                }
            }
        }

        match name.as_str() {
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
            "Iterable" | "iterable" => self.resolve_single_arg_protocol(args, Protocol::Iterable),
            "Sequence" | "sequence" => self.resolve_single_arg_protocol(args, Protocol::Sequence),
            "Union" | "union" => Ty::union(
                args.iter()
                    .flat_map(|args| args.iter())
                    .map(|type_ref| self.resolve_type_ref_inner(type_ref)),
            ),
            "struct" | "structure" => self.resolve_single_arg_type_constructor(args, |ty| {
                TyKind::Struct(Some(Struct::FieldSignature { ty }))
            }),
            "Target" => TyKind::Target.intern(),
            "tuple" => match args.as_ref() {
                Some(args) => {
                    // Handle variable tuples directly. The ellipsis type `...` is valid only when
                    // it is the second of exactly two type arguments.
                    if args.len() == 2 && args[1] == TypeRef::Ellipsis {
                        TyKind::Tuple(Tuple::Variable(self.resolve_type_ref_inner(&args[0])))
                            .intern()
                    } else {
                        TyKind::Tuple(Tuple::Simple(
                            args.iter()
                                .map(|type_ref| self.resolve_type_ref_inner(type_ref))
                                .collect(),
                        ))
                        .intern()
                    }
                }
                None => TyKind::Tuple(Tuple::Variable(Ty::unknown())).intern(),
            },
            name => match builtin_types.types(self.db).get(name).cloned() {
                Some(ty) => ty,
                None => {
                    self.errors.push(format!("Unknown type \"{}\"", name));
                    types.unknown.clone()
                }
            },
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

pub(crate) fn resolve_type_ref(
    tcx: &mut TyContext,
    type_ref: &TypeRef,
    usage: Option<InFile<StmtId>>,
) -> (Ty, Vec<String>) {
    TypeRefResolver {
        db: tcx.db,
        context: usage.map(|usage| (tcx, usage)),
        errors: vec![],
    }
    .resolve_type_ref(type_ref)
}

pub(crate) fn resolve_type_ref_opt(
    tcx: &mut TyContext,
    type_ref: Option<TypeRef>,
    usage: Option<InFile<StmtId>>,
) -> Ty {
    type_ref
        .map(|type_ref| resolve_type_ref(tcx, &type_ref, usage).0)
        .unwrap_or_else(Ty::unknown)
}

pub(crate) fn resolve_builtin_type_ref(db: &dyn Db, type_ref: &TypeRef) -> (Ty, Vec<String>) {
    TypeRefResolver {
        db,
        context: None,
        errors: vec![],
    }
    .resolve_type_ref(type_ref)
}

pub(crate) fn resolve_builtin_type_ref_opt(db: &dyn Db, type_ref: Option<TypeRef>) -> Ty {
    type_ref
        .map(|type_ref| resolve_builtin_type_ref(db, &type_ref).0)
        .unwrap_or_else(Ty::unknown)
}

// TODO(withered-magic): This function currently assumes that all types are covariant in their arguments.
pub(crate) fn assign_tys(db: &dyn Db, source: &Ty, target: &Ty) -> bool {
    use Protocol::*;

    // Assignments involving "Any", "Unknown", or "Unbound" at the top-level
    // are always valid to avoid confusion.
    match (source.kind(), target.kind()) {
        (TyKind::Any | TyKind::Unknown, _) | (_, TyKind::Any | TyKind::Unknown) => true,
        (
            TyKind::List(source),
            TyKind::List(target) | TyKind::Protocol(Iterable(target) | Sequence(target)),
        )
        | (TyKind::Protocol(Sequence(source)), TyKind::List(target)) => {
            assign_tys(db, source, target)
        }
        (
            TyKind::Tuple(tuple),
            TyKind::Protocol(Iterable(target) | Sequence(target))
            | TyKind::Tuple(Tuple::Variable(target)),
        ) => match tuple {
            Tuple::Simple(sources) => sources.iter().all(|source| assign_tys(db, source, target)),
            Tuple::Variable(source) => assign_tys(db, source, target),
        },
        (TyKind::Tuple(Tuple::Simple(sources)), TyKind::Tuple(Tuple::Simple(targets))) => {
            sources.len() == targets.len()
                && sources
                    .iter()
                    .zip(targets.iter())
                    .all(|(source, target)| assign_tys(db, source, target))
        }
        (TyKind::Protocol(source), TyKind::Protocol(target)) => match &(source, target) {
            (Iterable(source), Iterable(target))
            | (Sequence(source), Sequence(target))
            | (Sequence(source), Iterable(target)) => assign_tys(db, source, target),
            _ => false,
        },
        (TyKind::Dict(key_source, value_source, _), TyKind::Dict(key_target, value_target, _)) => {
            assign_tys(db, key_source, key_target) && assign_tys(db, value_source, value_target)
        }
        (TyKind::String(_), TyKind::BuiltinType(ty, _))
        | (TyKind::BuiltinType(ty, _), TyKind::String(_))
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
        (TyKind::BuiltinType(source, _), TyKind::BuiltinType(target, _)) => source == target,
        (TyKind::String(_), TyKind::String(_))
        | (TyKind::Attribute(_), TyKind::Attribute(_))
        | (TyKind::Struct(_), TyKind::Struct(_))
        | (TyKind::Bool(_), TyKind::Bool(_))
        | (TyKind::Int(_), TyKind::Int(_)) => true,
        (source, target) => source == target,
    }
}
