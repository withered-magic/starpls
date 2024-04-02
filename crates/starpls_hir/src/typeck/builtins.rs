use crate::{
    def::Argument,
    source_map,
    typeck::{Attribute, AttributeKind, Provider, ProviderField, Rule as TyRule, RuleKind},
    Db, ExprId, Name, Ty, TyKind, TypeRef,
};
use either::Either;
use rustc_hash::FxHashMap;
use starpls_bazel::{
    attr, builtin::Callable, env, Builtins, BUILTINS_TYPES_DENY_LIST, BUILTINS_VALUES_DENY_LIST,
};
use starpls_common::{parse, Dialect, File};
use starpls_syntax::ast::{self, AstNode};
use std::{collections::HashSet, sync::Arc};

const DEFAULT_DOC: &str = "See the [Bazel Build Encyclopedia](https://bazel.build/reference/be/overview) for more details.";

#[salsa::tracked]
pub(crate) struct BuiltinTypes {
    #[return_ref]
    pub(crate) types: FxHashMap<String, Ty>,
}

#[salsa::tracked]
pub struct BuiltinType {
    pub name: Name,
    #[return_ref]
    pub fields: Vec<BuiltinField>,
    #[return_ref]
    pub methods: Vec<BuiltinFunction>,
    #[return_ref]
    pub doc: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuiltinField {
    pub name: Name,
    pub type_ref: TypeRef,
    pub doc: String,
}

#[salsa::tracked]
pub struct BuiltinGlobals {
    #[return_ref]
    pub functions: FxHashMap<String, BuiltinFunction>,
    #[return_ref]
    pub variables: FxHashMap<String, TypeRef>,
}

#[salsa::tracked]
pub struct BuiltinFunction {
    pub name: Name,
    #[return_ref]
    pub parent_type: Option<String>,
    #[return_ref]
    pub params: Vec<BuiltinFunctionParam>,
    #[return_ref]
    pub ret_type_ref: TypeRef,
    #[return_ref]
    pub doc: String,
}

impl BuiltinFunction {
    pub(crate) fn maybe_unique_ret_type<'a, I>(
        &'a self,
        db: &'a dyn Db,
        file: File,
        call_expr: ExprId,
        args: I,
    ) -> Option<Ty>
    where
        I: Iterator<Item = (&'a Argument, &'a Ty)>,
    {
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
                TyKind::Struct(fields)
            }
            (None, "provider") => {
                let mut fields = None;
                let mut doc = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(s.clone());
                                }
                            }
                            "fields" => {
                                if let TyKind::Dict(_, _, Some(known_keys)) = ty.kind() {
                                    fields = Some(
                                        known_keys
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
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }

                let name = source_map(db, file)
                    .expr_map_back
                    .get(&call_expr)
                    .and_then(|ptr| ptr.try_to_node(&parse(db, file).syntax(db)))
                    .and_then(|expr| expr.syntax().parent())
                    .and_then(|parent| ast::AssignStmt::cast(parent))
                    .and_then(|assign_stmt| assign_stmt.lhs())
                    .and_then(|lhs| match lhs {
                        ast::Expression::Name(name_ref) => Some(name_ref),
                        _ => None,
                    })
                    .and_then(|name_ref| name_ref.name())
                    .as_ref()
                    .map(|name| Name::from_str(name.text()));

                TyKind::Provider(Arc::new(Provider { name, doc, fields }))
            }
            (None, name @ ("rule" | "repository_rule")) => {
                let mut attrs = None;
                let mut doc = None;
                for (arg, ty) in args {
                    if let Argument::Keyword { name, .. } = arg {
                        match name.as_str() {
                            "doc" => {
                                if let TyKind::String(Some(s)) = ty.kind() {
                                    doc = Some(s.clone());
                                }
                            }
                            "attrs" => {
                                if let TyKind::Dict(_, _, Some(known_keys)) = ty.kind() {
                                    attrs = Some(
                                        known_keys
                                            .iter()
                                            .filter(|(_, ty)| ty.is_attribute())
                                            .map(|(name, ty)| {
                                                (Name::from_str(&name.value(db)), ty.clone())
                                            })
                                            .collect::<Vec<_>>()
                                            .into_boxed_slice(),
                                    )
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
                    attrs: attrs.unwrap_or_else(|| Vec::new().into_boxed_slice()),
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

                TyKind::Attribute(Attribute::new(
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
                        "strict_list_dict" => AttributeKind::StringListDict,
                        _ => return None,
                    },
                    doc,
                    mandatory,
                    default_ptr.map(|text_range| Either::Left((file, text_range))),
                ))
            }
            _ => return None,
        };

        Some(ret_kind.intern())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinFunctionParam {
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
            BuiltinFunctionParam::Simple { name, .. } => name.clone(),
            _ => Name::missing(),
        }
    }
}

#[salsa::input]
pub struct BuiltinDefs {
    #[return_ref]
    pub builtins: Builtins,
    #[return_ref]
    pub rules: Builtins,
}

pub(crate) fn builtin_globals(db: &dyn Db, dialect: Dialect) -> BuiltinGlobals {
    let defs = db.get_builtin_defs(&dialect);
    builtin_globals_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn builtin_globals_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinGlobals {
    let mut functions = FxHashMap::default();
    let mut variables = FxHashMap::default();
    let builtins = defs.builtins(db);
    let rules = defs.rules(db);

    for value in env::make_bzl_builtins()
        .global
        .iter()
        .chain(env::make_build_builtins().global.iter())
        .chain(env::make_module_bazel_builtins().global.iter())
        .chain(env::make_workspace_builtins().global.iter())
        .chain(builtins.global.iter())
        .chain(rules.global.iter())
    {
        // Skip deny-listed globals, which are handled directly by the
        // language server.
        if value.name.is_empty() || BUILTINS_VALUES_DENY_LIST.contains(&value.name.as_str()) {
            continue;
        }

        if let Some(callable) = &value.callable {
            functions.insert(
                value.name.clone(),
                builtin_function(db, &value.name, callable, &value.doc, None),
            );
        } else {
            variables.insert(value.name.clone(), normalize_type_ref(&value.r#type));
        }
    }

    BuiltinGlobals::new(db, functions, variables)
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

    for type_ in builtins.r#type.iter() {
        // Skip deny-listed types, which are handled directly by the
        // language server.
        if type_.name.is_empty() || BUILTINS_TYPES_DENY_LIST.contains(&type_.name.as_str()) {
            continue;
        }

        // Collect fields and methods.
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut seen_methods = HashSet::new();

        // Special handling for the "native" type, which includes all native rules.
        if type_.name == "native" {
            for rule in rules.global.iter() {
                if let Some(callable) = &rule.callable {
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

        for field in type_.field.iter() {
            if let Some(callable) = &field.callable {
                // Filter out duplicates.
                if !seen_methods.contains(&field.name.as_str()) {
                    methods.push(builtin_function(
                        db,
                        &field.name,
                        callable,
                        &field.doc,
                        Some(&type_.name),
                    ));
                }
            } else {
                fields.push(BuiltinField {
                    name: Name::from_str(&field.name),
                    type_ref: normalize_type_ref(&field.r#type),
                    doc: normalize_doc_text(&field.doc),
                });
            }
        }

        types.insert(
            type_.name.clone(),
            TyKind::BuiltinType(BuiltinType::new(
                db,
                Name::from_str(&type_.name),
                fields,
                methods,
                normalize_doc_text(&type_.doc),
            ))
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
    let mut params = Vec::new();

    for param in callable.param.iter() {
        let name = Name::from_str(param.name.trim_start_matches('*'));

        // We need to apply a few normalization steps to parameter types.
        params.push(if param.is_star_arg {
            BuiltinFunctionParam::ArgsList {
                name: Name::from_str(&param.name),
                type_ref: maybe_strip_iterable_or_dict(normalize_type_ref(&param.r#type)),
                doc: normalize_doc_text(&param.doc),
            }
        } else if param.is_star_star_arg {
            BuiltinFunctionParam::KwargsDict {
                name,
                type_ref: maybe_strip_iterable_or_dict(normalize_type_ref(&param.r#type)),
                doc: normalize_doc_text(&param.doc),
            }
        } else {
            BuiltinFunctionParam::Simple {
                name,
                type_ref: normalize_type_ref(&param.r#type),
                doc: normalize_doc_text(&param.doc),
                default_value: if !param.default_value.is_empty() {
                    Some(param.default_value.clone())
                } else {
                    None
                },
                positional: true,
                is_mandatory: param.is_mandatory,
            }
        });
    }

    BuiltinFunction::new(
        db,
        Name::from_str(name),
        parent_name.map(|parent_name| parent_name.to_string()),
        params,
        normalize_type_ref(&callable.return_type),
        if doc.is_empty() {
            DEFAULT_DOC.to_string()
        } else {
            normalize_doc_text(&doc)
        },
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
                        doc: Some(attr.doc.into_boxed_str()),
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
    let mut chars = text.chars();
    let mut tag = String::new();

    while let Some(ch) = chars.next() {
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

fn normalize_type_ref(text: &str) -> TypeRef {
    let text = normalize_doc(text, true);
    let mut type_refs = text
        .split("; or ")
        .map(|part| {
            let mut parts = part.split(" of ");
            match (
                parts.next(),
                parts.next().map(|element| {
                    if element.ends_with('s') {
                        &element[..element.len() - 1]
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
                            element.map_or(TypeRef::Unknown, |element| normalize_type_ref(element)),
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
        Some(
            vec![element.map_or(TypeRef::Unknown, |element| normalize_type_ref(element))]
                .into_boxed_slice(),
        ),
    )
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
