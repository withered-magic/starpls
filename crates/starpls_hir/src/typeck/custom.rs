use crate::{Db, Dialect, Name, Ty, TyKind, TypeRef};
use rustc_hash::FxHashMap;
use starpls_bazel::Builtins;

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
    pub functions: FxHashMap<Name, CustomFunction>,
    #[return_ref]
    pub variables: FxHashMap<Name, CustomVariable>,
}

#[salsa::tracked]
pub struct CustomFunction {
    #[return_ref]
    pub name: Name,
    #[return_ref]
    pub params: Vec<CustomFunctionParam>,
    pub ret_type_ref: TypeRef,
}

#[salsa::tracked]
pub struct CustomVariable {
    #[return_ref]
    pub name: Name,
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CustomFunctionParam {
    Positional {
        name: Name,
        type_ref: TypeRef,
        kw_only: bool,
        optional: bool,
    },
    VarArgList {
        ret_type_ref: TypeRef,
    },
    VarArgDict,
}

#[salsa::input]
pub struct CustomDefs {
    #[return_ref]
    pub builtins: Builtins,
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
        // Collect fields and methods.
        let mut fields = Vec::new();

        for field in builtin_ty.field.iter() {
            if let Some(_callable) = &field.callable {
            } else {
                fields.push(CustomField {
                    name: Name::from_str(&field.name),
                    type_ref: TypeRef::Unknown,
                });
            }
        }

        let type_ = CustomType::new(db, Name::from_str(&builtin_ty.name), fields, vec![]);
        types.insert(builtin_ty.name.clone(), TyKind::CustomType(type_).intern());
    }

    CustomTypes::new(db, types)
}
