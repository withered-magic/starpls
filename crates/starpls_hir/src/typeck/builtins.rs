use crate::{
    typeck::{Ty, TyKind},
    Db, Name,
};

// A reference to a builtin type. This is mainly used to avoid circular dependencies when
// constructing the types of a class's fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltinTypeRef {
    None,
    Bool,
    Int,
    Float,
    Name(Name),
}

#[salsa::tracked]
pub struct BuiltinTypes {
    pub(crate) any: Ty,
    pub(crate) unbound: Ty,
    pub(crate) unknown: Ty,
    pub(crate) none: Ty,
    pub(crate) bool: Ty,
    pub(crate) int: Ty,
    pub(crate) float: Ty,
    pub(crate) string: Ty,
    pub(crate) string_elems: Ty,
    pub(crate) bytes: Ty,
    pub(crate) bytes_elems: Ty,
    pub(crate) list: Ty,
    pub(crate) tuple: Ty,
    pub(crate) dict: Ty,
}

#[salsa::tracked]
pub struct BuiltinClass {
    name: Name,
    #[return_ref]
    fields: Vec<BuiltinField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuiltinField {
    name: Name,
    type_ref: BuiltinTypeRef,
}

impl BuiltinField {
    fn new_inline(name: &'static str, type_ref: BuiltinTypeRef) -> Self {
        Self {
            name: Name::new_inline(name),
            type_ref,
        }
    }
}

#[salsa::tracked]
pub struct BuiltinFieldTypes {
    #[return_ref]
    field_tys: Vec<Ty>,
}

#[salsa::tracked]
pub fn builtin_field_types(db: &dyn Db, class: BuiltinClass) -> BuiltinFieldTypes {
    let types = builtin_types(db);
    let field_tys = class
        .fields(db)
        .iter()
        .map(|field| match field.type_ref {
            BuiltinTypeRef::None => types.none(db),
            BuiltinTypeRef::Bool => types.bool(db),
            BuiltinTypeRef::Int => types.int(db),
            BuiltinTypeRef::Float => types.float(db),
            BuiltinTypeRef::Name(ref name) => match name.as_str() {
                "string" => types.string(db),
                "string.elems" => types.string_elems(db),
                "bytes" => types.bytes(db),
                "bytes.elems" => types.bytes_elems(db),
                "list" => types.list(db),
                "tuple" => types.tuple(db),
                "dict" => types.dict(db),
                _ => panic!("undefined builtin type: {}", name.as_str()),
            },
        })
        .collect();
    BuiltinFieldTypes::new(db, field_tys)
}

#[salsa::tracked]
pub struct BuiltinFunction {
    param_type_refs: Vec<BuiltinTypeRef>,
    ret_type_ref: BuiltinTypeRef,
}

#[salsa::tracked]
pub(crate) fn builtin_types(db: &dyn Db) -> BuiltinTypes {
    BuiltinTypes::new(
        db,
        TyKind::Any.intern(),
        TyKind::Unbound.intern(),
        TyKind::Unknown.intern(),
        TyKind::None.intern(),
        TyKind::Bool.intern(),
        TyKind::Int.intern(),
        TyKind::Float.intern(),
        intern_class(db, "string"),
        intern_class(db, "string.elems"),
        intern_class(db, "bytes"),
        intern_class(db, "bytes.elems"),
        intern_class(db, "list"),
        intern_class(db, "tuple"),
        intern_class(db, "dict"),
    )
}

fn intern_class(db: &dyn Db, name: &'static str) -> Ty {
    TyKind::BuiltinClass(BuiltinClass::new(
        db,
        Name::new_inline(name),
        vec![BuiltinField::new_inline("len", BuiltinTypeRef::Int)],
    ))
    .intern()
}
