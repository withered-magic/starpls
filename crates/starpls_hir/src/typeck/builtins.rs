use crate::{Db, Dialect, Name, Ty, TyKind, TypeRef};
use rustc_hash::FxHashMap;
use starpls_bazel::{
    builtin::Callable, Builtins, BUILTINS_TYPES_DENY_LIST, BUILTINS_VALUES_DENY_LIST,
};

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
    pub params: Vec<BuiltinFunctionParam>,
    pub ret_type_ref: TypeRef,
    #[return_ref]
    pub doc: String,
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
        type_ref: TypeRef,
        doc: String,
    },
    KwargsDict {
        doc: String,
    },
}

impl BuiltinFunctionParam {
    pub(crate) fn type_ref(&self) -> Option<TypeRef> {
        Some(match self {
            BuiltinFunctionParam::Simple { type_ref, .. }
            | BuiltinFunctionParam::ArgsList { type_ref, .. } => type_ref.clone(),
            BuiltinFunctionParam::KwargsDict { .. } => return None,
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
}

pub(crate) fn builtin_globals(db: &dyn Db) -> BuiltinGlobals {
    let defs = db.get_builtin_defs(&Dialect::Bazel);
    builtin_globals_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn builtin_globals_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinGlobals {
    let mut functions = FxHashMap::default();
    let mut variables = FxHashMap::default();
    let builtins = defs.builtins(db);

    for value in builtins.global.iter() {
        // Skip deny-listed globals, which are handled directly by the
        // language server.
        if value.name.is_empty() || BUILTINS_VALUES_DENY_LIST.contains(&value.name.as_str()) {
            continue;
        }

        if let Some(callable) = &value.callable {
            functions.insert(
                value.name.clone(),
                builtin_function(db, &value.name, callable, &value.doc),
            );
        } else {
            variables.insert(value.name.clone(), normalize_type_ref(&value.r#type));
        }
    }

    BuiltinGlobals::new(db, functions, variables)
}

pub(crate) fn builtin_types(db: &dyn Db) -> BuiltinTypes {
    let defs = db.get_builtin_defs(&Dialect::Bazel);
    builtin_types_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn builtin_types_query(db: &dyn Db, defs: BuiltinDefs) -> BuiltinTypes {
    let mut types = FxHashMap::default();
    let builtins = defs.builtins(db);

    for type_ in builtins.r#type.iter() {
        // Skip deny-listed types, which are handled directly by the
        // language server.
        if type_.name.is_empty() || BUILTINS_TYPES_DENY_LIST.contains(&type_.name.as_str()) {
            continue;
        }

        // Collect fields and methods.
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for field in type_.field.iter() {
            if let Some(callable) = &field.callable {
                methods.push(builtin_function(db, &field.name, callable, &field.doc));
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

fn builtin_function(db: &dyn Db, name: &str, callable: &Callable, doc: &str) -> BuiltinFunction {
    let mut params = Vec::new();

    for param in callable.param.iter() {
        // We need to apply a few normalization steps to parameter types.
        params.push(if param.is_star_arg {
            BuiltinFunctionParam::ArgsList {
                type_ref: normalize_type_ref(&param.r#type),
                doc: normalize_doc_text(&param.doc),
            }
        } else if param.is_star_star_arg {
            BuiltinFunctionParam::KwargsDict {
                doc: normalize_doc_text(&param.doc),
            }
        } else {
            BuiltinFunctionParam::Simple {
                name: Name::from_str(&param.name),
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
        params,
        normalize_type_ref(&callable.return_type),
        normalize_doc_text(&doc),
    )
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
