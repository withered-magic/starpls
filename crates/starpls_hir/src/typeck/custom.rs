use crate::{Db, Dialect, Name, Ty, TyKind, TypeRef};
use rustc_hash::FxHashMap;
use starpls_bazel::{
    builtin::Callable, Builtins, BUILTINS_TYPES_DENY_LIST, BUILTINS_VALUES_DENY_LIST,
};

#[salsa::tracked]
pub struct CustomTypes {
    #[return_ref]
    pub types: FxHashMap<String, Ty>,
}

#[salsa::tracked]
pub struct CustomType {
    #[return_ref]
    pub name: Name,
    #[return_ref]
    pub fields: Vec<CustomField>,
    #[return_ref]
    pub methods: Vec<CustomFunction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomField {
    pub name: Name,
    pub type_ref: TypeRef,
}

#[salsa::tracked]
pub struct CustomGlobals {
    #[return_ref]
    pub functions: FxHashMap<String, CustomFunction>,
    #[return_ref]
    pub variables: FxHashMap<String, TypeRef>,
}

#[salsa::tracked]
pub struct CustomFunction {
    #[return_ref]
    pub name: Name,
    #[return_ref]
    pub params: Vec<CustomFunctionParam>,
    pub ret_type_ref: TypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CustomFunctionParam {
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
pub struct CustomDefs {
    #[return_ref]
    pub builtins: Builtins,
}

pub(crate) fn custom_globals(db: &dyn Db) -> CustomGlobals {
    let defs = db.get_custom_defs(&Dialect::Bazel);
    custom_globals_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn custom_globals_query(db: &dyn Db, defs: CustomDefs) -> CustomGlobals {
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
                custom_function(db, &value.name, callable),
            );
        } else {
            variables.insert(value.name.clone(), TypeRef::from_str_opt(&value.r#type));
        }
    }

    CustomGlobals::new(db, functions, variables)
}

pub(crate) fn custom_types(db: &dyn Db) -> CustomTypes {
    let defs = db.get_custom_defs(&Dialect::Bazel);
    custom_types_query(db, defs)
}

#[salsa::tracked]
pub(crate) fn custom_types_query(db: &dyn Db, defs: CustomDefs) -> CustomTypes {
    let mut types = FxHashMap::default();
    let builtins = defs.builtins(db);

    for builtin_ty in builtins.r#type.iter() {
        // Skip deny-listed types, which are handled directly by the
        // language server.
        if builtin_ty.name.is_empty()
            || BUILTINS_TYPES_DENY_LIST.contains(&builtin_ty.name.as_str())
        {
            continue;
        }

        // Collect fields and methods.
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for field in builtin_ty.field.iter() {
            if let Some(callable) = &field.callable {
                methods.push(custom_function(db, &field.name, callable));
            } else {
                fields.push(CustomField {
                    name: Name::from_str(&field.name),
                    type_ref: TypeRef::from_str_opt(&field.r#type),
                });
            }
        }

        let type_ = CustomType::new(db, Name::from_str(&builtin_ty.name), fields, methods);
        types.insert(builtin_ty.name.clone(), TyKind::CustomType(type_).intern());
    }

    CustomTypes::new(db, types)
}

fn custom_function(db: &dyn Db, name: &str, callable: &Callable) -> CustomFunction {
    let mut params = Vec::new();
    for callable_param in callable.param.iter() {
        // We need to apply a few normalization steps to parameter types.
        params.push(if callable_param.is_star_arg {
            CustomFunctionParam::VarArgList {
                type_ref: TypeRef::Unknown,
            }
        } else if callable_param.is_star_star_arg {
            CustomFunctionParam::VarArgDict
        } else {
            CustomFunctionParam::Normal {
                name: Name::from_str(&callable_param.name),
                type_ref: TypeRef::Unknown,
                kw_only: false,
                optional: !callable_param.is_mandatory,
            }
        });
    }
    CustomFunction::new(
        db,
        Name::from_str(name),
        params,
        TypeRef::from_str_opt(&callable.return_type),
    )
}
