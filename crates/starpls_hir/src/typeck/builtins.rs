use crate::{Db, Dialect, Name, Ty, TyKind, TypeRef};
use rustc_hash::FxHashMap;
use starpls_bazel::{
    builtin::Callable, Builtins, BUILTINS_TYPES_DENY_LIST, BUILTINS_VALUES_DENY_LIST,
};

#[salsa::tracked]
pub struct BuiltinTypes {
    #[return_ref]
    pub types: FxHashMap<String, Ty>,
}

#[salsa::tracked]
pub struct BuiltinType {
    #[return_ref]
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
    #[return_ref]
    pub name: Name,
    #[return_ref]
    pub params: Vec<BuiltinFunctionParam>,
    pub ret_type_ref: TypeRef,
    pub doc: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinFunctionParam {
    Normal {
        name: Name,
        type_ref: TypeRef,
        kw_only: bool,
        optional: bool,
    },
    VarArgList {
        type_ref: TypeRef,
    },
    VarArgDict,
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
                builtin_function(db, &value.name, callable, value.doc.clone()),
            );
        } else {
            variables.insert(
                value.name.clone(),
                TypeRef::from_str_opt(&normalize_typestr(&value.r#type)),
            );
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
                methods.push(builtin_function(
                    db,
                    &field.name,
                    callable,
                    field.doc.clone(),
                ));
            } else {
                fields.push(BuiltinField {
                    name: Name::from_str(&field.name),
                    type_ref: TypeRef::from_str_opt(&field.r#type),
                    doc: field.doc.clone(),
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
                type_.doc.clone(),
            ))
            .intern(),
        );
    }

    BuiltinTypes::new(db, types)
}

fn builtin_function(db: &dyn Db, name: &str, callable: &Callable, doc: String) -> BuiltinFunction {
    let mut params = Vec::new();

    for callable_param in callable.param.iter() {
        // We need to apply a few normalization steps to parameter types.
        params.push(if callable_param.is_star_arg {
            BuiltinFunctionParam::VarArgList {
                type_ref: TypeRef::Unknown,
            }
        } else if callable_param.is_star_star_arg {
            BuiltinFunctionParam::VarArgDict
        } else {
            BuiltinFunctionParam::Normal {
                name: Name::from_str(&callable_param.name),
                type_ref: TypeRef::Unknown,
                kw_only: false,
                optional: !callable_param.is_mandatory,
            }
        });
    }

    BuiltinFunction::new(
        db,
        Name::from_str(name),
        params,
        TypeRef::from_str_opt(&normalize_typestr(&callable.return_type)),
        doc,
    )
}

/// Normalizes types from the generated Bazel documentation.
fn normalize_typestr(text: &str) -> String {
    // The main thing we need to normalize is that many Bazel types in
    // builtins file are wrapped with HTML tags, e.g. `<a>None</a>`.
    // We fix this by removing any text between angle brackets.
    let mut s = String::new();
    let mut in_tag = false;
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        match (ch, in_tag) {
            ('<', _) => in_tag = true,
            ('>', _) => in_tag = false,
            (_, false) => s.push(ch),
            _ => {}
        }
    }

    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_typestr() {
        assert_eq!(normalize_typestr("int").as_str(), "int");
        assert_eq!(normalize_typestr("<a>int</a>").as_str(), "int");
        assert_eq!(
            normalize_typestr("<a>int</a>; or <a>string</a>").as_str(),
            "int; or string"
        )
    }
}
