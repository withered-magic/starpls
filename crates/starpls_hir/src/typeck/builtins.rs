use crate::{
    typeck::{Binders, Substitution, Ty, TyKind},
    Db, Name,
};

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

    // Base classes for types with fields/methods.
    pub(crate) string_base_class: BuiltinClass,
    pub(crate) bytes_base_class: BuiltinClass,
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
    ty: Ty,
}

impl BuiltinField {
    fn new_inline(name: &'static str, ty: Ty) -> Self {
        Self {
            name: Name::new_inline(name),
            ty,
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
        .map(|field| Binders::new(class.num_vars(db), field.ty.clone()))
        .collect();
    BuiltinFieldTypes::new(db, field_tys)
}

#[salsa::tracked]
pub struct BuiltinFunction {
    pub param_tys: Vec<Ty>,
    pub ret_ty: Ty,
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
        TyKind::String.intern(),
        TyKind::StringElems.intern(),
        TyKind::Bytes.intern(),
        TyKind::BytesElems.intern(),
        TyKind::Tuple.intern(),
        make_string_base_class(db),
        make_bytes_base_class(db),
        make_list_base_class(db),
        make_dict_base_class(db),
    )
}

fn make_string_base_class(db: &dyn Db) -> BuiltinClass {
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("string"),
        0,
        vec![
            function_field(db, "capitalize", vec![], String, 0),
            function_field(db, "count", vec![String, Int, Int], Int, 0),
            function_field(db, "elems", vec![], StringElems, 0),
            function_field(db, "endswith", vec![String, Int, Int], Bool, 0),
            function_field(db, "find", vec![String, Int, Int], Int, 0),
            // function_field(db, "format", param_type_refs, ret_type_ref),
            function_field(db, "index", vec![String, Int, Int], Int, 0),
            function_field(db, "isalnum", vec![], Bool, 0),
            function_field(db, "isalpha", vec![], Bool, 0),
            function_field(db, "isdigit", vec![], Bool, 0),
            function_field(db, "islower", vec![], Bool, 0),
            function_field(db, "isspace", vec![], Bool, 0),
            function_field(db, "istitle", vec![], Bool, 0),
            function_field(db, "isupper", vec![], Bool, 0),
            function_field(db, "join", vec![Any], Bool, 0),
            function_field(db, "lower", vec![], String, 0),
            function_field(db, "lstrip", vec![String], String, 0),
            function_field(db, "partition", vec![String], Tuple, 0),
            function_field(db, "removeprefix", vec![String], String, 0),
            function_field(db, "removesuffix", vec![String], String, 0),
            function_field(db, "replace", vec![String, String, Int], String, 0),
            function_field(db, "rfind", vec![String, Int, Int], Int, 0),
            function_field(db, "rindex", vec![String, Int, Int], Int, 0),
            function_field(db, "rpartition", vec![String], Tuple, 0),
            // function_field(db, "rsplit", vec![String, Int], List),
            function_field(db, "rstrip", vec![String], String, 0),
            // function_field(db, "split", vec![String, Int], List),
            // function_field(db, "splitlines", vec![Bool], List),
            function_field(db, "startswith", vec![String, Int, Int], Bool, 0),
            function_field(db, "strip", vec![String], String, 0),
            function_field(db, "title", vec![], String, 0),
            function_field(db, "upper", vec![], String, 0),
        ],
    )
}

fn make_bytes_base_class(db: &dyn Db) -> BuiltinClass {
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("bytes"),
        0,
        vec![function_field(db, "elems", vec![], BytesElems, 0)],
    )
}

fn make_list_base_class(db: &dyn Db) -> BuiltinClass {
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("list"),
        1,
        vec![
            function_field(db, "append", vec![BoundVar(0)], None, 1),
            function_field(db, "clear", vec![], None, 1),
            function_field(db, "extend", vec![Any], None, 1),
            function_field(db, "index", vec![BoundVar(0), Int, Int], Int, 1),
            function_field(db, "insert", vec![Int, BoundVar(0)], None, 1),
            function_field(db, "pop", vec![Int], Any, 1),
            function_field(db, "remove", vec![BoundVar(0)], None, 1),
        ],
    )
}

fn make_dict_base_class(db: &dyn Db) -> BuiltinClass {
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("dict"),
        2,
        vec![
            function_field(db, "clear", vec![], None, 2),
            function_field(db, "get", vec![BoundVar(0), BoundVar(1)], BoundVar(1), 2),
            // function_field(db, "items", vec![], List),
            // function_field(db, "keys", vec![], List),
            function_field(db, "pop", vec![BoundVar(0), BoundVar(1)], Any, 2),
            function_field(db, "popitem", vec![], Tuple, 2),
            function_field(
                db,
                "setdefault",
                vec![BoundVar(0), BoundVar(1)],
                BoundVar(1),
                2,
            ),
            // function_field(db, "update", vec![List], None),
            // function_field(db, "values", vec![], List),
        ],
    )
}

fn function_field(
    db: &dyn Db,
    name: &'static str,
    param_tys: Vec<TyKind>,
    ret_ty: TyKind,
    num_vars: usize,
) -> BuiltinField {
    let param_tys = param_tys.into_iter().map(TyKind::intern).collect();
    BuiltinField::new_inline(
        name,
        TyKind::BuiltinFunction(
            BuiltinFunction::new(db, param_tys, ret_ty.intern()),
            Substitution::new_identity(num_vars),
        )
        .intern(),
    )
}
