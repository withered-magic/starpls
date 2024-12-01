use std::collections::HashSet;
use std::sync::Arc;

use either::Either;
use rustc_hash::FxHashMap;
use smallvec::smallvec;
use starpls_bazel::attr;
use starpls_bazel::builtin::Callable;
use starpls_bazel::builtin::Param;
use starpls_bazel::builtin::Type;
use starpls_bazel::builtin::Value;
use starpls_bazel::env::make_workspace_builtins;
use starpls_bazel::env::{self};
use starpls_bazel::Builtins;
use starpls_bazel::BUILTINS_TYPES_DENY_LIST;
use starpls_bazel::BUILTINS_VALUES_DENY_LIST;
use starpls_bazel::KNOWN_PROVIDER_TYPES;
use starpls_common::parse;
use starpls_common::Dialect;
use starpls_common::File;
use starpls_common::InFile;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};

use crate::def::resolver::Export;
use crate::def::resolver::Resolver;
use crate::def::Argument;
use crate::source_map;
use crate::typeck::Attribute;
use crate::typeck::AttributeData;
use crate::typeck::AttributeKind;
use crate::typeck::CustomProvider;
use crate::typeck::CustomProviderFields;
use crate::typeck::ModuleExtension;
use crate::typeck::Provider;
use crate::typeck::ProviderField;
use crate::typeck::Rule as TyRule;
use crate::typeck::RuleAttributes;
use crate::typeck::RuleKind;
use crate::typeck::Struct;
use crate::typeck::TagClass;
use crate::typeck::TagClassData;
use crate::typeck::Tuple;
use crate::Db;
use crate::ExprId;
use crate::Name;
use crate::Ty;
use crate::TyContext;
use crate::TyKind;
use crate::TypeRef;

const DEFAULT_DOC: &str = "See the [Bazel Build Encyclopedia](https://bazel.build/reference/be/overview) for more details.";

#[salsa::tracked]
pub(crate) struct BuiltinTypes {
    #[return_ref]
    pub(crate) types: FxHashMap<String, Ty>,
}

#[salsa::tracked]
pub(crate) struct BuiltinType {
    pub(crate) name: Name,
    #[return_ref]
    pub(crate) fields: Vec<BuiltinField>,
    #[return_ref]
    pub(crate) methods: Vec<BuiltinFunction>,
    #[return_ref]
    pub(crate) doc: String,
    pub(crate) indexable_by: Option<(TypeRef, TypeRef)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BuiltinField {
    pub(crate) name: Name,
    pub(crate) type_ref: TypeRef,
    pub(crate) doc: String,
}

#[salsa::tracked]
pub(crate) struct BuiltinGlobals {
    #[return_ref]
    pub(crate) bzl_globals: APIGlobals,
    #[return_ref]
    pub(crate) bzlmod_globals: APIGlobals,
    #[return_ref]
    pub(crate) repo_globals: APIGlobals,
    #[return_ref]
    pub(crate) workspace_globals: APIGlobals,
    #[return_ref]
    pub(crate) cquery_globals: APIGlobals,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct APIGlobals {
    pub(crate) functions: FxHashMap<String, BuiltinFunction>,
    pub(crate) variables: FxHashMap<String, TypeRef>,
}

impl APIGlobals {
    fn from_values<'a, I>(db: &dyn Db, providers: BuiltinProviders, values: I) -> Self
    where
        I: Iterator<Item = &'a Value>,
    {
        let mut functions = FxHashMap::default();
        let mut variables = FxHashMap::default();
        let providers = providers.providers(db);

        for value in values {
            // Skip deny-listed globals, which are handled directly by the
            // language server.
            if value.name.is_empty() || BUILTINS_VALUES_DENY_LIST.contains(&value.name.as_str()) {
                continue;
            }

            match (providers.get(value.name.as_str()), &value.callable) {
                (Some(provider), _) => {
                    variables.insert(value.name.clone(), TypeRef::Provider(*provider));
                }
                (None, Some(callable)) => {
                    functions.insert(
                        value.name.clone(),
                        builtin_function(db, &value.name, callable, &value.doc, None),
                    );
                }
                (None, None) => {
                    variables.insert(value.name.clone(), parse_type_ref(&value.r#type));
                }
            }
        }

        Self {
            functions,
            variables,
        }
    }
}

#[salsa::tracked]
pub(crate) struct BuiltinFunction {
    pub(crate) name: Name,
    #[return_ref]
    pub(crate) parent_type: Option<String>,
    #[return_ref]
    pub(crate) params: Vec<BuiltinFunctionParam>,
    #[return_ref]
    pub(crate) ret_type_ref: TypeRef,
    #[return_ref]
    pub(crate) doc: String,
}

impl BuiltinFunction {
    pub(crate) fn maybe_unique_ret_type<'a, I>(
        &'a self,
        tcx: &'a mut TyContext,
        file: File,
        call_expr: ExprId,
        mut args: I,
    ) -> Option<Ty>
    where
        I: Iterator<Item = (&'a Argument, &'a Ty)>,
    {
        let resolve_load_like = |db: &dyn Db, args: &mut I| {
            let mut next_string_arg = || {
                args.next().and_then(|(arg, ty)| match (arg, ty.kind()) {
                    (Argument::Simple { .. }, TyKind::String(Some(s))) => Some(s.value(db)),
                    _ => None,
                })
            };

            let path = next_string_arg()?;
            let name = next_string_arg()?;
            let loaded_file = db.load_file(&path, file.dialect(db), file.id(db)).ok()??;

            Some(
                match Resolver::resolve_export_in_file(db, loaded_file, &Name::from_str(&name))? {
                    Export::Variable(expr) => InFile {
                        file: loaded_file,
                        value: expr.expr,
                    },
                    _ => return None,
                },
            )
        };

        let db = tcx.db;
        let ret_kind = match (
            self.parent_type(db)
                .as_ref()
                .map(|parent_type| parent_type.as_str()),
            self.name(db).as_str(),
        ) {
            (None, "struct") => {
                let fields = args
                    .filter_map(|(arg, ty)| match arg {
                        Argument::Keyword { name, .. } => Some((name.clone(), ty.clone())),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                TyKind::Struct(Some(Struct::Inline {
                    call_expr: InFile {
                        file,
                        value: call_expr,
                    },
                    fields,
                }))
            }
            (None, "provider") => {
                let mut fields = None;
                let mut doc = None;
                let mut has_init = false;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(*s);
                                }
                            }
                            "fields" => {
                                if let TyKind::Dict(_, _, Some(lit)) = ty.kind() {
                                    fields = Some(CustomProviderFields {
                                        expr: lit.expr,
                                        fields: lit
                                            .known_keys
                                            .iter()
                                            .flat_map(|(key, value)| {
                                                let name = &key.value(db);
                                                if !name.is_empty() {
                                                    Some(ProviderField {
                                                        name: Name::from_str(&key.value(db)),
                                                        doc: match value.kind() {
                                                            TyKind::String(Some(s)) => Some(
                                                                s.value(db)
                                                                    .to_string()
                                                                    .into_boxed_str(),
                                                            ),
                                                            _ => None,
                                                        },
                                                    })
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect(),
                                    });
                                }
                            }
                            "init" => {
                                has_init = true;
                            }
                            _ => {}
                        }
                    }
                }

                let lhs = source_map(db, file)
                    .expr_map_back
                    .get(&call_expr)
                    .and_then(|ptr| ptr.try_to_node(&parse(db, file).syntax(db)))
                    .and_then(|expr| expr.syntax().parent())
                    .and_then(ast::AssignStmt::cast)
                    .and_then(|assign_stmt| assign_stmt.lhs());

                let extract_name = |expr: ast::Expression| {
                    match expr {
                        ast::Expression::Name(name_ref) => Some(name_ref),
                        _ => None,
                    }
                    .and_then(|name_ref| name_ref.name())
                    .as_ref()
                    .and_then(|name| {
                        let text = name.text();
                        if !text.is_empty() {
                            Some(Name::from_str(text))
                        } else {
                            None
                        }
                    })
                };

                if has_init {
                    let (provider_name, ctor_name) = lhs
                        .and_then(|lhs| match lhs {
                            ast::Expression::Tuple(tuple_expr) => {
                                let mut elements = tuple_expr.elements();
                                let provider_name = elements.next().and_then(extract_name);
                                let ctor_name = elements.next().and_then(extract_name);
                                Some((provider_name, ctor_name))
                            }
                            _ => None,
                        })
                        .unwrap_or_default();

                    let provider = Provider::Custom(Arc::new(CustomProvider {
                        name: provider_name,
                        doc,
                        fields,
                    }));

                    TyKind::Tuple(Tuple::Simple(smallvec![
                        TyKind::Provider(provider.clone()).intern(),
                        TyKind::ProviderRawConstructor(
                            ctor_name.unwrap_or_else(|| Name::new_inline("ctor")),
                            provider
                        )
                        .intern(),
                    ]))
                } else {
                    let name = lhs
                        .and_then(|lhs| match lhs {
                            ast::Expression::Name(name_ref) => Some(name_ref),
                            _ => None,
                        })
                        .and_then(|name_ref| name_ref.name())
                        .as_ref()
                        .map(|name| Name::from_str(name.text()));
                    TyKind::Provider(Provider::Custom(Arc::new(CustomProvider {
                        name,
                        doc,
                        fields,
                    })))
                }
            }

            (None, name @ ("rule" | "repository_rule")) => {
                let mut attrs = None;
                let mut doc = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(*s);
                                }
                            }
                            "attrs" => {
                                if let TyKind::Dict(_, _, Some(lit)) = ty.kind() {
                                    attrs = Some(RuleAttributes {
                                        attrs: lit
                                            .known_keys
                                            .iter()
                                            .filter_map(|(name, ty)| match ty.kind() {
                                                TyKind::Attribute(Some(attr)) => Some((
                                                    Name::from_str(&name.value(db)),
                                                    attr.clone(),
                                                )),
                                                _ => None,
                                            })
                                            .collect::<Vec<_>>(),
                                        expr: lit.expr,
                                    })
                                }
                            }
                            _ => {}
                        }
                    }
                }

                TyKind::Rule(TyRule {
                    kind: if name == "rule" {
                        RuleKind::Build
                    } else {
                        RuleKind::Repository
                    },
                    doc: doc.map(|doc| doc.value(db).clone()),
                    attrs: attrs.map(Arc::new),
                })
            }

            (Some("attr"), attr) => {
                let mut doc: Option<Box<str>> = None;
                let mut mandatory = false;
                let mut default_ptr = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, expr } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(s.value(db).clone());
                                }
                            }
                            "mandatory" => {
                                if let TyKind::Bool(Some(b)) = ty.kind() {
                                    mandatory = *b;
                                }
                            }
                            "default" => {
                                if let Some(ptr) = source_map(db, file).expr_map_back.get(expr) {
                                    default_ptr = Some(ptr.syntax_node_ptr());
                                }
                            }
                            _ => {}
                        }
                    }
                }

                TyKind::Attribute(Some(Arc::new(Attribute::new(
                    match attr {
                        "bool" => AttributeKind::Bool,
                        "int" => AttributeKind::Int,
                        "int_list" => AttributeKind::IntList,
                        "label" => AttributeKind::Label,
                        "label_keyed_string_dict" => AttributeKind::LabelKeyedStringDict,
                        "label_list" => AttributeKind::LabelList,
                        "output" => AttributeKind::Output,
                        "output_list" => AttributeKind::OutputList,
                        "string" => AttributeKind::String,
                        "string_dict" => AttributeKind::StringDict,
                        "string_list" => AttributeKind::StringList,
                        "string_list_dict" => AttributeKind::StringListDict,
                        _ => return None,
                    },
                    doc,
                    mandatory,
                    default_ptr.map(|text_range| Either::Left((file, text_range))),
                ))))
            }

            (None, "tag_class") => {
                let mut attrs = None;
                let mut doc = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "attrs" => {
                                if let TyKind::Dict(_, _, Some(lit)) = ty.kind() {
                                    attrs = Some(
                                        lit.known_keys
                                            .iter()
                                            .filter_map(|(name, ty)| match ty.kind() {
                                                TyKind::Attribute(Some(attr)) => {
                                                    Some(AttributeData {
                                                        name: Name::from_str(&name.value(db)),
                                                        attr: attr.clone(),
                                                    })
                                                }
                                                _ => None,
                                            })
                                            .collect::<Vec<_>>()
                                            .into_boxed_slice(),
                                    )
                                }
                            }
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(s.value(db).clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }

                TyKind::TagClass(Arc::new(TagClass { attrs, doc }))
            }

            (None, "module_extension") => {
                let mut doc = None;
                let mut tag_classes = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(s.value(db).clone());
                                }
                            }
                            "tag_classes" => {
                                let lit = match ty.kind() {
                                    TyKind::Dict(_, _, Some(lit)) => lit,
                                    _ => continue,
                                };

                                tag_classes = Some(
                                    lit.known_keys
                                        .iter()
                                        .filter_map(|(name, ty)| match ty.kind() {
                                            TyKind::TagClass(tag_class) => Some(TagClassData {
                                                name: Name::from_str(&name.value(db)),
                                                tag_class: tag_class.clone(),
                                            }),
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                        .into_boxed_slice(),
                                );
                            }
                            _ => {}
                        }
                    }
                }

                TyKind::ModuleExtension(Arc::new(ModuleExtension { doc, tag_classes }))
            }

            (None, "use_extension") => {
                let expr = resolve_load_like(db, &mut args)?;
                let module_extension = match tcx.infer_expr(expr.file, expr.value).kind() {
                    TyKind::ModuleExtension(module_extension) => module_extension.clone(),
                    _ => return None,
                };
                TyKind::ModuleExtensionProxy(module_extension)
            }

            (None, "use_repo_rule") => {
                let expr = resolve_load_like(db, &mut args)?;
                let ty = tcx.infer_expr(expr.file, expr.value);
                return match ty.kind() {
                    TyKind::Rule(TyRule {
                        kind: RuleKind::Repository,
                        ..
                    }) => Some(ty),
                    _ => None,
                };
            }

            _ => return None,
        };

        Some(ret_kind.intern())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum BuiltinFunctionParam {
    Simple {
        name: Name,
        type_ref: TypeRef,
        doc: String,
        default_value: Option<String>,
        positional: bool,
        is_mandatory: bool,
    },
    ArgsList {
        name: Name,
        type_ref: TypeRef,
        doc: String,
    },
    KwargsDict {
        name: Name,
        type_ref: TypeRef,
        doc: String,
    },
}

impl BuiltinFunctionParam {
    pub(crate) fn type_ref(&self) -> Option<TypeRef> {
        Some(match self {
            BuiltinFunctionParam::Simple { type_ref, .. }
            | BuiltinFunctionParam::ArgsList { type_ref, .. }
            | BuiltinFunctionParam::KwargsDict { type_ref, .. } => type_ref.clone(),
        })
    }

    pub(crate) fn is_mandatory(&self) -> bool {
        match self {
            BuiltinFunctionParam::Simple { is_mandatory, .. } => *is_mandatory,
            _ => false,
        }
    }

    pub(crate) fn name(&self) -> Name {
        match self {
            BuiltinFunctionParam::Simple { name, .. }
            | BuiltinFunctionParam::ArgsList { name, .. }
            | BuiltinFunctionParam::KwargsDict { name, .. } => name.clone(),
        }
    }

    pub(crate) fn doc(&self) -> &str {
        match self {
            BuiltinFunctionParam::Simple { doc, .. }
            | BuiltinFunctionParam::ArgsList { doc, .. }
            | BuiltinFunctionParam::KwargsDict { doc, .. } => doc,
        }
    }
}

#[salsa::tracked]
pub(crate) struct BuiltinProvider {
    #[return_ref]
    pub(crate) name: Name,
    #[return_ref]
    pub(crate) params: Vec<BuiltinFunctionParam>,
    #[return_ref]
    pub(crate) fields: Vec<BuiltinField>,
    #[return_ref]
    pub(crate) doc: String,
}

#[salsa::tracked]
pub(crate) struct BuiltinProviders {
    #[return_ref]
    pub(crate) providers: FxHashMap<String, BuiltinProvider>,
}

#[salsa::input]
pub struct BuiltinDefs {
    #[return_ref]
    pub builtins: Builtins,
    #[return_ref]
    pub rules: Builtins,
}

#[salsa::tracked]
pub(crate) fn builtin_providers_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinProviders {
    // Collect all known provider types.
    let builtins = defs.builtins(db);
    let mut providers = FxHashMap::default();
    let known_provider_tys: FxHashMap<String, &Type> = builtins
        .r#type
        .iter()
        .filter(|ty| KNOWN_PROVIDER_TYPES.contains(&ty.name.as_str()))
        .map(|ty| (ty.name.clone(), ty))
        .collect();

    for value in builtins
        .global
        .iter()
        .chain(builtins.r#type.iter().flat_map(|ty| ty.field.iter()))
    {
        if let Some(ty) = known_provider_tys.get(value.name.as_str()) {
            providers.insert(
                ty.name.clone(),
                builtin_provider(db, ty, value.callable.as_ref()),
            );
        }
    }

    for (name, ty) in &known_provider_tys {
        if !providers.contains_key(name.as_str()) {
            providers.insert(ty.name.clone(), builtin_provider(db, ty, None));
        }
    }

    BuiltinProviders::new(db, providers)
}

pub(crate) fn builtin_globals(db: &dyn Db, dialect: Dialect) -> BuiltinGlobals {
    let defs = db.get_builtin_defs(&dialect);
    builtin_globals_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn builtin_globals_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinGlobals {
    let builtins = defs.builtins(db);
    let rules = defs.rules(db);
    let providers = builtin_providers_query(db, defs);

    let bzl_globals = APIGlobals::from_values(
        db,
        providers,
        env::make_bzl_builtins()
            .global
            .iter()
            .chain(env::make_build_builtins().global.iter())
            .chain(builtins.global.iter())
            .chain(rules.global.iter()),
    );
    let bzlmod_globals = APIGlobals::from_values(
        db,
        providers,
        env::make_module_bazel_builtins().global.iter(),
    );
    let repo_globals =
        APIGlobals::from_values(db, providers, env::make_repo_builtins().global.iter());
    let workspace_globals =
        APIGlobals::from_values(db, providers, env::make_workspace_builtins().global.iter());
    let cquery_globals =
        APIGlobals::from_values(db, providers, env::make_cquery_builtins().global.iter());

    BuiltinGlobals::new(
        db,
        bzl_globals,
        bzlmod_globals,
        repo_globals,
        workspace_globals,
        cquery_globals,
    )
}

pub(crate) fn builtin_types(db: &dyn Db, dialect: Dialect) -> BuiltinTypes {
    let defs = db.get_builtin_defs(&dialect);
    builtin_types_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn builtin_types_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinTypes {
    let mut types = FxHashMap::default();
    let builtins = defs.builtins(db);
    let rules = defs.rules(db);
    let mut missing_module_members = env::make_missing_module_members();
    let providers = builtin_providers_query(db, defs).providers(db);

    // Add all builtin providers.
    types.extend(providers.iter().map(|(name, provider)| {
        (
            name.clone(),
            TyKind::ProviderInstance(Provider::Builtin(*provider)).intern(),
        )
    }));

    for type_ in builtins.r#type.iter() {
        // Skip deny-listed types, which are handled directly by `intrinsics.rs`, and provider types,
        // which are handled above.
        if type_.name.is_empty()
            || BUILTINS_TYPES_DENY_LIST.contains(&type_.name.as_str())
            || KNOWN_PROVIDER_TYPES.contains(&type_.name.as_str())
        {
            continue;
        }

        // Collect fields and methods.
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut seen_methods = HashSet::new();
        let workspace_builtins = make_workspace_builtins();

        // Special handling for the "native" type, which includes all native rules.
        if type_.name == "native" {
            // We also add symbols that are normally only available from `WORKSPACE` files, like
            // `register_execution_platforms` and `register_toolchains`. This is technically
            // incorrect if bzlmod is enabled, so we should revisit this approach in the future.
            for rule in rules.global.iter().chain(
                workspace_builtins
                    .global
                    .iter()
                    .filter(|global| !["workspace"].contains(&global.name.as_str())),
            ) {
                if let Some(callable) = &rule.callable {
                    if seen_methods.contains(&rule.name.as_str()) {
                        continue;
                    }

                    seen_methods.insert(rule.name.as_str());
                    methods.push(builtin_function(
                        db,
                        &rule.name,
                        callable,
                        &rule.doc,
                        Some(&type_.name),
                    ));
                }
            }
        }

        for field in type_.field.iter().chain(
            missing_module_members
                .remove(&type_.name)
                .unwrap_or_default()
                .iter(),
        ) {
            if let Some(callable) = &field.callable {
                // Filter out duplicates.
                if !seen_methods.contains(&field.name.as_str()) {
                    match providers.get(field.name.as_str()) {
                        Some(provider) => {
                            fields.push(BuiltinField {
                                name: provider.name(db).clone(),
                                type_ref: TypeRef::Provider(*provider),
                                doc: normalize_doc_text(&field.doc),
                            });
                        }
                        None => {
                            methods.push(builtin_function(
                                db,
                                &field.name,
                                callable,
                                &field.doc,
                                Some(&type_.name),
                            ));
                        }
                    }
                }
            } else {
                let type_ref = match providers.get(field.name.as_str()) {
                    Some(provider) => TypeRef::Provider(*provider),
                    None => maybe_field_type_ref_override(&type_.name, &field.name)
                        .unwrap_or_else(|| parse_type_ref(&field.r#type)),
                };

                fields.push(BuiltinField {
                    name: Name::from_str(&field.name),
                    type_ref,
                    doc: normalize_doc_text(&field.doc),
                });
            }
        }

        let indexable_by = match type_.name.as_str() {
            "ToolchainContext" => Some(("string", "ToolchainInfo")),
            // TODO(withered-magic): Audit Bazel docs for other indexable builtin types.
            _ => None,
        }
        .map(|(expected_index_ty, return_ty)| {
            (
                TypeRef::Name(Name::new_inline(expected_index_ty), None),
                TypeRef::Name(Name::new_inline(return_ty), None),
            )
        });

        types.insert(
            type_.name.clone(),
            TyKind::BuiltinType(
                BuiltinType::new(
                    db,
                    Name::from_str(&type_.name),
                    fields,
                    methods,
                    normalize_doc_text(&type_.doc),
                    indexable_by,
                ),
                None,
            )
            .intern(),
        );
    }

    BuiltinTypes::new(db, types)
}

fn builtin_function(
    db: &dyn Db,
    name: &str,
    callable: &Callable,
    doc: &str,
    parent_name: Option<&str>,
) -> BuiltinFunction {
    // Apply overrides for function return types known to be incorrect. For now, this
    // consists only of the `Label()` constructor.
    let ret_type_ref = match name {
        "Label" => "Label",
        _ => callable.return_type.as_str(),
    };

    BuiltinFunction::new(
        db,
        Name::from_str(name),
        parent_name.map(|parent_name| parent_name.to_string()),
        callable.param.iter().map(builtin_param).collect(),
        parse_type_ref(ret_type_ref),
        if doc.is_empty() {
            DEFAULT_DOC.to_string()
        } else {
            normalize_doc_text(doc)
        },
    )
}

fn builtin_param(param: &Param) -> BuiltinFunctionParam {
    let name = Name::from_str(param.name.trim_start_matches('*'));
    if param.is_star_arg {
        BuiltinFunctionParam::ArgsList {
            name,
            type_ref: maybe_strip_iterable_or_dict(parse_type_ref(&param.r#type)),
            doc: normalize_doc_text(&param.doc),
        }
    } else if param.is_star_star_arg {
        BuiltinFunctionParam::KwargsDict {
            name,
            type_ref: maybe_strip_iterable_or_dict(parse_type_ref(&param.r#type)),
            doc: normalize_doc_text(&param.doc),
        }
    } else {
        BuiltinFunctionParam::Simple {
            name,
            type_ref: parse_type_ref(&param.r#type),
            doc: normalize_doc_text(&param.doc),
            default_value: if !param.default_value.is_empty() {
                Some(param.default_value.clone())
            } else {
                None
            },
            positional: true,
            is_mandatory: param.is_mandatory,
        }
    }
}

fn builtin_provider(db: &dyn Db, ty: &Type, callable: Option<&Callable>) -> BuiltinProvider {
    let params = match callable {
        Some(callable) => callable.param.iter().map(builtin_param).collect(),
        None => ty
            .field
            .iter()
            .filter(|field| field.callable.is_none())
            .map(|field| BuiltinFunctionParam::Simple {
                name: Name::from_str(&field.name),
                type_ref: parse_type_ref(&field.r#type),
                doc: normalize_doc_text(&field.doc),
                default_value: None,
                positional: false,
                is_mandatory: false,
            })
            .collect(),
    };

    let provider_fields = ty
        .field
        .iter()
        .map(|field| BuiltinField {
            name: Name::from_str(&field.name),
            type_ref: maybe_field_type_ref_override(&ty.name, &field.name)
                .unwrap_or_else(|| parse_type_ref(&field.r#type)),
            doc: normalize_doc_text(&field.doc),
        })
        .collect();
    BuiltinProvider::new(
        db,
        Name::from_str(&ty.name),
        params,
        provider_fields,
        normalize_doc_text(&ty.doc),
    )
}

#[salsa::tracked]
pub(crate) struct CommonAttributes {
    #[return_ref]
    pub(crate) build: Vec<(Name, Attribute)>,
    #[return_ref]
    pub(crate) repository: Vec<(Name, Attribute)>,
}

impl CommonAttributes {
    pub(crate) fn get<'a>(
        &'a self,
        db: &'a dyn Db,
        kind: RuleKind,
        index: usize,
    ) -> (&'a Name, &'a Attribute) {
        let (ref name, ref attr) = match kind {
            RuleKind::Build => self.build(db),
            RuleKind::Repository => self.repository(db),
        }[index];
        (name, attr)
    }
}

#[salsa::tracked]
pub(crate) fn common_attributes_query(db: &dyn Db) -> CommonAttributes {
    let map_attrs = |attrs: Vec<attr::Attribute>| {
        attrs
            .into_iter()
            .map(|attr| {
                use AttributeKind::*;

                (
                    Name::from_str(&attr.name),
                    Attribute {
                        kind: match attr.r#type {
                            attr::AttributeKind::Bool => Bool,
                            attr::AttributeKind::Int => Int,
                            attr::AttributeKind::IntList => IntList,
                            attr::AttributeKind::Label => Label,
                            attr::AttributeKind::LabelKeyedStringDict => LabelKeyedStringDict,
                            attr::AttributeKind::LabelList => LabelList,
                            attr::AttributeKind::Output => Output,
                            attr::AttributeKind::OutputList => OutputList,
                            attr::AttributeKind::String => String,
                            attr::AttributeKind::StringDict => StringDict,
                            attr::AttributeKind::StringList => StringList,
                            attr::AttributeKind::StringListDict => StringListDict,
                        },
                        doc: Some(normalize_doc_text(&attr.doc).into_boxed_str()),
                        mandatory: attr.is_mandatory,
                        default_text_range: Some(Either::Right(attr.default_value)),
                    },
                )
            })
            .collect()
    };

    let common = attr::make_common_attributes();
    CommonAttributes::new(db, map_attrs(common.build), map_attrs(common.repository))
}

/// Normalizes text from the generated Bazel documentation.
fn normalize_doc_text(text: &str) -> String {
    normalize_doc(text, false)
}

fn normalize_doc(text: &str, is_type: bool) -> String {
    // The main thing we need to normalize is that many Bazel types in
    // builtins file are wrapped with HTML tags, e.g. `<a>None</a>`.
    // We fix this by removing any text between angle brackets.
    let mut s = String::new();
    let mut in_tag = false;
    let chars = text.chars();
    let mut tag = String::new();

    for ch in chars {
        match (ch, in_tag) {
            ('<', _) => in_tag = true,
            ('>', _) => {
                match tag.as_str() {
                    "p" => s.push_str("\n\n"),
                    "code" | "/code" if !is_type => s.push('`'),
                    _ => {}
                }
                in_tag = false;
                tag.truncate(0);
            }
            (_, true) => tag.push(ch),
            (_, false) => s.push(ch),
        }
    }

    s.to_string()
}

fn maybe_strip_iterable_or_dict(type_ref: TypeRef) -> TypeRef {
    match type_ref {
        TypeRef::Name(name, Some(args)) => match (args.len(), name.as_str()) {
            (1, "Iterable" | "Sequence" | "list") => args[0].clone(),
            (2, "dict") => args[1].clone(),
            _ => TypeRef::Name(name, Some(args)),
        },
        _ => type_ref,
    }
}

fn parse_type_ref(text: &str) -> TypeRef {
    let text = normalize_doc(text, true);
    let mut type_refs = text
        .split("; or ")
        .map(|part| {
            let mut parts = part.split(" of ");
            match (
                parts.next(),
                parts.next().map(|element| {
                    if let Some(stripped) = element.strip_suffix('s') {
                        stripped
                    } else {
                        element
                    }
                }),
            ) {
                (Some("Iterable" | "iterable"), element) => {
                    type_ref_with_single_arg("Iterable", element)
                }
                (Some("Sequence" | "sequence"), element) => {
                    type_ref_with_single_arg("Sequence", element)
                }
                (Some("List" | "list"), element) => type_ref_with_single_arg("list", element),
                (Some("Dict" | "dict" | "Dictionary"), element) => TypeRef::Name(
                    Name::new_inline("dict"),
                    Some(
                        vec![
                            TypeRef::from_str_opt("string"),
                            element.map_or(TypeRef::Unknown, parse_type_ref),
                        ]
                        .into_boxed_slice(),
                    ),
                ),
                (Some("String"), _) => TypeRef::from_str_opt("string"),
                (Some("Boolean" | "boolean"), _) => TypeRef::from_str_opt("bool"),
                (Some("label"), _) => TypeRef::from_str_opt("Label"),
                // Quick hack to normalize `NoneType`.
                (Some("NoneType"), _) => TypeRef::from_str_opt("None"),
                (Some(name), _) => TypeRef::from_str_opt(name),
                _ => TypeRef::Unknown,
            }
        })
        .collect::<Vec<_>>();

    if type_refs.is_empty() {
        TypeRef::Unknown
    } else if type_refs.len() == 1 {
        type_refs.pop().unwrap()
    } else {
        TypeRef::Union(type_refs)
    }
}

fn type_ref_with_single_arg(name: &str, element: Option<&str>) -> TypeRef {
    TypeRef::Name(
        Name::from_str(name),
        Some(vec![element.map_or(TypeRef::Unknown, parse_type_ref)].into_boxed_slice()),
    )
}

fn maybe_field_type_ref_override(typ: &str, field: &str) -> Option<TypeRef> {
    let type_ref = match (typ, field) {
        ("ctx", "executable" | "file" | "outputs") => TypeRef::Name(
            Name::new_inline("struct"),
            Some(vec![TypeRef::Name(Name::new_inline("File"), None)].into_boxed_slice()),
        ),

        ("ctx", "files") => TypeRef::Name(
            Name::new_inline("struct"),
            Some(
                vec![TypeRef::Name(
                    Name::new_inline("list"),
                    Some(vec![TypeRef::Name(Name::new_inline("File"), None)].into_boxed_slice()),
                )]
                .into_boxed_slice(),
            ),
        ),
        _ => return None,
    };

    Some(type_ref)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_doc_text() {
        assert_eq!(normalize_doc_text("int").as_str(), "int");
        assert_eq!(normalize_doc_text("<a>int</a>").as_str(), "int");
        assert_eq!(
            normalize_doc_text("<a>int</a>; or <a>string</a>").as_str(),
            "int; or string"
        )
    }
}
