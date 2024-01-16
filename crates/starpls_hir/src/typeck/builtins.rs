use crate::{
    typeck::{Binders, Ty, TyKind},
    Db, Name,
};

// A reference to a builtin type. This is mainly used to avoid circular dependencies when
// constructing the types of a class's fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltinTypeRef {
    Any,
    None,
    Bool,
    Int,
    Float,
    String,
    StringElems,
    Bytes,
    BytesElems,
    List(Box<BuiltinTypeRef>),
    Tuple,
    Dict(Box<BuiltinTypeRef>, Box<BuiltinTypeRef>),
    Function(BuiltinFunction),
    Name(Name),
    BoundVar(usize),
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
    pub(crate) tuple: Ty,
    pub(crate) list_base_class: BuiltinClass,
    pub(crate) dict_base_class: BuiltinClass,
}

#[salsa::tracked]
pub struct BuiltinClass {
    pub name: Name,
    pub num_vars: usize,
    #[return_ref]
    pub fields: Vec<BuiltinField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuiltinField {
    pub name: Name,
    pub type_ref: BuiltinTypeRef,
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
    pub(crate) field_tys: Vec<Binders>,
}

#[salsa::tracked]
pub fn builtin_field_types(db: &dyn Db, class: BuiltinClass) -> BuiltinFieldTypes {
    let field_tys = class
        .fields(db)
        .iter()
        .map(|field| lower_builtin_type_ref(db, &field.type_ref))
        .map(|ty| Binders::new(class.num_vars(db), ty))
        .collect();
    BuiltinFieldTypes::new(db, field_tys)
}

fn lower_builtin_type_ref(db: &dyn Db, type_ref: &BuiltinTypeRef) -> Ty {
    let types = builtin_types(db);
    match type_ref {
        BuiltinTypeRef::Any => types.any(db),
        BuiltinTypeRef::None => types.none(db),
        BuiltinTypeRef::Bool => types.bool(db),
        BuiltinTypeRef::Int => types.int(db),
        BuiltinTypeRef::Float => types.float(db),
        BuiltinTypeRef::String => types.string(db),
        BuiltinTypeRef::StringElems => types.string_elems(db),
        BuiltinTypeRef::Bytes => types.bytes(db),
        BuiltinTypeRef::BytesElems => types.bytes_elems(db),
        BuiltinTypeRef::List(type_ref) => {
            TyKind::List(lower_builtin_type_ref(db, type_ref)).intern()
        }
        BuiltinTypeRef::Tuple => types.tuple(db),
        BuiltinTypeRef::Dict(key, value) => TyKind::Dict(
            lower_builtin_type_ref(db, key),
            lower_builtin_type_ref(db, value),
        )
        .intern(),
        BuiltinTypeRef::Function(_) => TyKind::BuiltinFunction.intern(),
        BuiltinTypeRef::Name(_) => todo!(),
        BuiltinTypeRef::BoundVar(index) => TyKind::BoundVar(*index).intern(),
    }
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
        intern_string(db),
        TyKind::StringElems.intern(),
        intern_bytes(db),
        TyKind::BytesElems.intern(),
        intern_class(db, "tuple"),
        make_list_base_class(db),
        make_dict_base_class(db),
    )
}

fn intern_class(db: &dyn Db, name: &'static str) -> Ty {
    TyKind::BuiltinClass(BuiltinClass::new(db, Name::new_inline(name), 0, vec![])).intern()
}

fn intern_string(db: &dyn Db) -> Ty {
    use BuiltinTypeRef::*;
    TyKind::BuiltinClass(BuiltinClass::new(
        db,
        crate::Name::new_inline("string"),
        0,
        vec![
            function_field(db, "capitalize", vec![], String),
            function_field(db, "count", vec![String, Int, Int], Int),
            function_field(db, "elems", vec![], StringElems),
            function_field(db, "endswith", vec![String, Int, Int], Bool),
            function_field(db, "find", vec![String, Int, Int], Int),
            // function_field(db, "format", param_type_refs, ret_type_ref),
            function_field(db, "index", vec![String, Int, Int], Int),
            function_field(db, "isalnum", vec![], Bool),
            function_field(db, "isalpha", vec![], Bool),
            function_field(db, "isdigit", vec![], Bool),
            function_field(db, "islower", vec![], Bool),
            function_field(db, "isspace", vec![], Bool),
            function_field(db, "istitle", vec![], Bool),
            function_field(db, "isupper", vec![], Bool),
            function_field(db, "join", vec![Any], Bool),
            function_field(db, "lower", vec![], String),
            function_field(db, "lstrip", vec![String], String),
            function_field(db, "partition", vec![String], Tuple),
            function_field(db, "removeprefix", vec![String], String),
            function_field(db, "removesuffix", vec![String], String),
            function_field(db, "replace", vec![String, String, Int], String),
            function_field(db, "rfind", vec![String, Int, Int], Int),
            function_field(db, "rindex", vec![String, Int, Int], Int),
            function_field(db, "rpartition", vec![String], Tuple),
            // function_field(db, "rsplit", vec![String, Int], List),
            function_field(db, "rstrip", vec![String], String),
            // function_field(db, "split", vec![String, Int], List),
            // function_field(db, "splitlines", vec![Bool], List),
            function_field(db, "startswith", vec![String, Int, Int], Bool),
            function_field(db, "strip", vec![String], String),
            function_field(db, "title", vec![], String),
            function_field(db, "upper", vec![], String),
        ],
    ))
    .intern()
}

fn intern_bytes(db: &dyn Db) -> Ty {
    use BuiltinTypeRef::*;
    TyKind::BuiltinClass(BuiltinClass::new(
        db,
        crate::Name::new_inline("bytes"),
        0,
        vec![function_field(db, "elems", vec![], BytesElems)],
    ))
    .intern()
}

fn make_list_base_class(db: &dyn Db) -> BuiltinClass {
    use BuiltinTypeRef::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("list"),
        1,
        vec![
            function_field(db, "append", vec![BoundVar(0)], None),
            function_field(db, "clear", vec![], None),
            function_field(db, "extend", vec![Any], None),
            function_field(db, "index", vec![BoundVar(0), Int, Int], Int),
            function_field(db, "insert", vec![Int, BoundVar(0)], None),
            function_field(db, "pop", vec![Int], Any),
            function_field(db, "remove", vec![BoundVar(0)], None),
        ],
    )
}

fn make_dict_base_class(db: &dyn Db) -> BuiltinClass {
    use BuiltinTypeRef::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("dict"),
        2,
        vec![
            function_field(db, "clear", vec![], None),
            function_field(db, "get", vec![BoundVar(0), Any], BoundVar(1)),
            // function_field(db, "items", vec![], List),
            // function_field(db, "keys", vec![], List),
            function_field(db, "pop", vec![BoundVar(0), BoundVar(1)], Any),
            function_field(db, "popitem", vec![], Tuple),
            function_field(
                db,
                "setdefault",
                vec![BoundVar(0), BoundVar(1)],
                BoundVar(1),
            ),
            // function_field(db, "update", vec![List], None),
            // function_field(db, "values", vec![], List),
        ],
    )
}

fn function_field(
    db: &dyn Db,
    name: &'static str,
    param_type_refs: Vec<BuiltinTypeRef>,
    ret_type_ref: BuiltinTypeRef,
) -> BuiltinField {
    BuiltinField::new_inline(
        name,
        BuiltinTypeRef::Function(BuiltinFunction::new(db, param_type_refs, ret_type_ref)),
    )
}
