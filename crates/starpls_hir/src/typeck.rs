use std::{
    fmt::Write,
    iter,
    panic::{self, UnwindSafe},
    sync::Arc,
};

use crossbeam::atomic::AtomicCell;
use either::Either;
use parking_lot::Mutex;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{smallvec, SmallVec};
use starpls_common::{parse, Diagnostic, Dialect, File, InFile};
use starpls_intern::{impl_internable, Interned};
use starpls_syntax::ast::SyntaxNodePtr;

use crate::{
    def::{
        codeflow::FlowNodeId,
        scope::{ExecutionScopeId, FunctionDef},
        ExprId, Function, LiteralString, LoadItemId, LoadStmt, Param as HirDefParam, ParamId,
    },
    module, source_map,
    typeck::{
        builtins::{
            builtin_types, common_attributes_query, BuiltinFunction, BuiltinFunctionParam,
            BuiltinProvider, BuiltinType,
        },
        intrinsics::{
            intrinsic_field_types, intrinsic_types, IntrinsicClass, IntrinsicFunction,
            IntrinsicFunctionParam, Intrinsics,
        },
    },
    Db, Name,
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

#[derive(Clone, Debug, Default)]
pub struct InferenceOptions {
    pub infer_ctx_attributes: bool,
    pub use_code_flow_analysis: bool,
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

        let fields =
            match kind {
                TyKind::BuiltinType(ty, data) => Fields::Builtin(
                    ty.fields(db)
                        .iter()
                        .enumerate()
                        .map(move |(index, field)| {
                            let resolved = resolve_builtin_type_ref(db, &field.type_ref).0;
                            let resolved = match (resolved.kind(), data) {
                                // If `TyData` is set, this means the current type is either `ctx` or `repository_ctx`.
                                // Override the `attr` field for both of these types.
                                (TyKind::Struct(_), Some(TyData::Attributes(attrs)))
                                    if field.name.as_str() == "attr" =>
                                {
                                    TyKind::Struct(Some(Struct::Attributes {
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
                    Provider::Custom(custom_provider) => {
                        ProviderFields::Custom(
                            custom_provider.fields.as_ref()?.1.iter().enumerate().map(
                                |(index, _)| {
                                    (
                                        Field(FieldInner::ProviderField {
                                            provider: provider.clone(),
                                            index,
                                        }),
                                        Ty::unknown(),
                                    )
                                },
                            ),
                        )
                    }
                }),
                TyKind::ModuleExtensionProxy(module_extension) => Fields::ModuleExtensionProxy(
                    module_extension
                        .tag_classes
                        .iter()
                        .flat_map(|tag_classes| tag_classes.iter())
                        .enumerate()
                        .map(|(index, (_, tag_class))| {
                            (
                                Field(FieldInner::ModuleExtensionProxyField {
                                    module_extension: module_extension.clone(),
                                    index,
                                }),
                                TyKind::Tag(tag_class.clone()).intern(),
                            )
                        }),
                ),
                TyKind::Target => {
                    let label_ty = builtin_types(db, Dialect::Bazel)
                        .types(db)
                        .get("Label")
                        .cloned()
                        .unwrap_or_else(|| Ty::unknown());
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
            TyKind::Function(def) => Params::Simple(def.func.params(db).iter().enumerate().map(
                |(index, param)| {
                    let file = def.func.file(db);
                    let ty = with_tcx(db, |tcx| tcx.infer_param(file, *param));
                    let param = Param(ParamInner::Param {
                        parent: Some(def.func),
                        index,
                    });
                    (param, ty)
                },
            )),
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
                        .chain(attrs.iter().map(|(name, attr)| {
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
                        }))
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
                            fields.1.iter().enumerate().map(|(index, _)| {
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
                    .map(|(name, attr)| {
                        (
                            Param(
                                TagParam::Keyword {
                                    name: name.clone(),
                                    attr: attr.clone(),
                                }
                                .into(),
                            ),
                            attr.expected_ty(),
                        )
                    })
                    .chain(iter::once((
                        Param(TagParam::Kwargs.into()),
                        TyKind::Dict(Ty::string(), Ty::any(), None).intern(),
                    ))),
            ),
            _ => return None,
        })
    }

    pub(crate) fn ret_ty(&self, db: &dyn Db) -> Option<Ty> {
        Some(match self.kind() {
            TyKind::Function(def) => resolve_builtin_type_ref_opt(db, def.func.ret_type_ref(db)),
            TyKind::IntrinsicFunction(func, subst) => func.ret_ty(db).substitute(&subst.args),
            TyKind::BuiltinFunction(func) => resolve_builtin_type_ref(db, &func.ret_type_ref(db)).0,
            TyKind::Rule(_) => Ty::none(),
            TyKind::Provider(provider) | TyKind::ProviderRawConstructor(_, provider) => {
                TyKind::ProviderInstance(provider.clone()).intern()
            }
            TyKind::Tag(_) => Ty::none(),
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

    pub(crate) fn known_keys(&self) -> Option<&[(LiteralString, Ty)]> {
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
            _ => self,
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
    ProviderParam {
        provider: Provider,
        index: usize,
    },
    TagParam(TagParam),
}

#[derive(Clone, Debug)]
pub(crate) enum RuleParam {
    Keyword { name: Name, attr: Arc<Attribute> },
    BuiltinKeyword(RuleKind, usize),
    Kwargs,
}

impl From<RuleParam> for ParamInner {
    fn from(value: RuleParam) -> Self {
        Self::RuleParam(value)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum TagParam {
    Keyword { name: Name, attr: Arc<Attribute> },
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
            ParamInner::ProviderParam {
                ref provider,
                index,
            } => Some(match provider {
                Provider::Builtin(provider) => provider.params(db)[index].name(),
                Provider::Custom(provider) => provider
                    .fields
                    .as_ref()
                    .expect("expected provider fields")
                    .1[index]
                    .name
                    .clone(),
            }),
            ParamInner::TagParam(TagParam::Keyword { ref name, .. }) => Some(name.clone()),
            ParamInner::TagParam(TagParam::Kwargs) => Some(Name::new_inline("kwargs")),
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
            ParamInner::BuiltinParam { parent, index } => {
                parent.params(db)[*index].doc().to_string()
            }
            ParamInner::IntrinsicParam { .. } => return None,
            ParamInner::RuleParam(RuleParam::Keyword { attr, .. }) => {
                return attr.doc.as_ref().map(Box::to_string)
            }
            ParamInner::RuleParam(RuleParam::BuiltinKeyword(kind, index)) => {
                return common_attributes_query(db)
                    .get(db, kind.clone(), *index)
                    .1
                    .doc
                    .as_ref()
                    .map(Box::to_string)
            }
            ParamInner::ProviderParam { provider, index } => match provider {
                Provider::Builtin(provider) => provider.params(db)[*index].doc().to_string(),
                Provider::Custom(provider) => {
                    return provider
                        .fields
                        .as_ref()
                        .expect("expected provider fields")
                        .1[*index]
                        .doc
                        .as_ref()
                        .map(Box::to_string)
                }
            },
            ParamInner::TagParam(TagParam::Keyword { attr, .. }) => {
                return attr.doc.as_ref().map(Box::to_string)
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

enum Params<I1, I2, I3, I4, I5, I6, I7> {
    Simple(I1),
    Intrinsic(I2),
    Builtin(I3),
    Rule(I4),
    Provider(ProviderParams<I5, I6>),
    Tag(I7),
}

impl<I1, I2, I3, I4, I5, I6, I7> Iterator for Params<I1, I2, I3, I4, I5, I6, I7>
where
    I1: Iterator<Item = (Param, Ty)>,
    I2: Iterator<Item = (Param, Ty)>,
    I3: Iterator<Item = (Param, Ty)>,
    I4: Iterator<Item = (Param, Ty)>,
    I5: Iterator<Item = (Param, Ty)>,
    I6: Iterator<Item = (Param, Ty)>,
    I7: Iterator<Item = (Param, Ty)>,
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
                    .1[index]
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
                .0
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
                    .1[index]
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
                .1
                .doc
                .as_ref()
                .map(|doc| doc.to_string())
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
    Attributes(Arc<Vec<(Name, Arc<Attribute>)>>),
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
    String(Option<LiteralString>),
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
    Attribute(Arc<Attribute>),
    /// A Bazel rule (https://bazel.build/rules/lib/builtins/rule).
    /// The `Ty`s contained in the boxed slice must have kind `TyKind::Attribute`.
    Rule(Rule),
    /// A Bazel provider (https://bazel.build/rules/lib/builtins/Provider.html).
    /// This is a callable the yields "provider instances".
    Provider(Provider),
    /// An instance of a Bazel provider. The contained `Ty` must have kind `TyKind::Provider`.
    ProviderInstance(Provider),
    /// The raw constructor for a Bazel provider.
    ProviderRawConstructor(Name, Provider),
    /// A Bazel tag class.
    TagClass(Arc<TagClass>),
    /// A Bazel module extension.
    ModuleExtension(Arc<ModuleExtension>),
    /// A Bazel module extension proxy.
    ModuleExtensionProxy(Arc<ModuleExtension>),
    /// A Bazel tag (e.g. `maven.artifact()`)
    Tag(Arc<TagClass>),
    /// A Bazel target (https://bazel.build/rules/lib/builtins/Target).
    Target,
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

    pub fn resolved_ty(&self) -> Ty {
        match self.kind {
            AttributeKind::Bool => Ty::bool(),
            AttributeKind::Int => Ty::int(),
            AttributeKind::IntList => Ty::list(Ty::int()),
            AttributeKind::String => Ty::string(),
            AttributeKind::Label => Ty::target(),
            AttributeKind::Output => Ty::unknown(),
            AttributeKind::StringDict => Ty::dict(Ty::string(), Ty::string(), None),
            AttributeKind::LabelKeyedStringDict => Ty::dict(Ty::target(), Ty::string(), None),
            AttributeKind::StringList => Ty::list(Ty::string()),
            AttributeKind::LabelList => Ty::list(TyKind::Target.intern()),
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
pub(crate) struct Rule {
    pub(crate) kind: RuleKind,
    pub(crate) doc: Option<Box<str>>,
    pub(crate) attrs: Arc<Vec<(Name, Arc<Attribute>)>>,
}

impl Rule {
    pub(crate) fn attrs<'a>(&'a self, db: &'a dyn Db) -> impl Iterator<Item = (&Name, &Attribute)> {
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
            .chain(self.attrs.iter().map(|(name, attr)| (name, &**attr)))
            .chain(common_attrs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CustomProvider {
    pub(crate) name: Option<Name>,
    pub(crate) doc: Option<LiteralString>,
    pub(crate) fields: Option<(Option<InFile<ExprId>>, Box<[ProviderField]>)>,
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
    Attributes {
        attrs: Arc<Vec<(Name, Arc<Attribute>)>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ModuleExtension {
    pub(crate) doc: Option<Box<str>>,
    pub(crate) tag_classes: Option<Box<[(Name, Arc<TagClass>)]>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TagClass {
    pub(crate) attrs: Option<Box<[(Name, Arc<Attribute>)]>>,
    pub(crate) doc: Option<Box<str>>,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ProviderField {
    pub(crate) name: Name,
    pub(crate) doc: Option<Box<str>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DictLiteral {
    pub(crate) expr: Option<InFile<ExprId>>,
    pub(crate) known_keys: Box<[(LiteralString, Ty)]>,
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

pub(crate) fn with_tcx<F, T>(db: &dyn Db, f: F) -> T
where
    F: FnMut(&mut TyCtxt) -> T + std::panic::UnwindSafe,
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
pub(crate) struct InferenceCtxt {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) resolved_load_stmts: FxHashMap<FileLoadStmt, Option<File>>,
    pub(crate) load_resolution_stack: Vec<(File, LoadStmt)>,
    pub(crate) type_of_expr: FxHashMap<FileExprId, Ty>,
    pub(crate) type_of_load_item: FxHashMap<FileLoadItemId, Ty>,
    pub(crate) type_of_param: FxHashMap<FileParamId, Ty>,
    pub(crate) source_assign_done: FxHashSet<FileExprId>,
    pub(crate) flow_node_type_cache: FxHashMap<CodeFlowCacheKey, Option<Ty>>,
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
    tcx: Option<&'a TyCtxt<'a>>,
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
                "struct" | "structure" => self.resolve_single_arg_type_constructor(args, |ty| {
                    TyKind::Struct(Some(Struct::FieldSignature { ty }))
                }),
                "Target" => TyKind::Target.intern(),
                "tuple" => match args.as_ref() {
                    Some(args) => {
                        // Handle variable tuples directly. The ellipsis type `...` is valid only when
                        // it is the second of exactly two type arguments.
                        if args.len() == 2 && &args[1] == &TypeRef::Ellipsis {
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
            },
            TypeRef::Union(args) => Ty::union(
                args.iter()
                    .map(|type_ref| self.resolve_type_ref_inner(&type_ref)),
            ),
            TypeRef::Provider(provider) => TyKind::Provider(Provider::Builtin(*provider)).intern(),
            TypeRef::Ellipsis => {
                // We handle ellipsis types only while processing tuples above, any other occurrences of
                // ellipsis types are invalid.
                self.errors
                    .push("\"...\" is not allowed in this context".to_string());
                types.unknown.clone()
            }
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

pub(crate) fn resolve_type_ref(tcx: &TyCtxt, type_ref: &TypeRef) -> (Ty, Vec<String>) {
    TypeRefResolver {
        db: tcx.db,
        tcx: Some(tcx),
        errors: vec![],
    }
    .resolve_type_ref(type_ref)
}

pub(crate) fn resolve_type_ref_opt(tcx: &TyCtxt, type_ref: Option<TypeRef>) -> Ty {
    type_ref
        .map(|type_ref| resolve_type_ref(tcx, &type_ref).0)
        .unwrap_or_else(|| Ty::unknown())
}

pub(crate) fn resolve_builtin_type_ref(db: &dyn Db, type_ref: &TypeRef) -> (Ty, Vec<String>) {
    TypeRefResolver {
        db,
        tcx: None,
        errors: vec![],
    }
    .resolve_type_ref(type_ref)
}

pub(crate) fn resolve_builtin_type_ref_opt(db: &dyn Db, type_ref: Option<TypeRef>) -> Ty {
    type_ref
        .map(|type_ref| resolve_builtin_type_ref(db, &type_ref).0)
        .unwrap_or_else(|| Ty::unknown())
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
