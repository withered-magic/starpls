use crate::{
    typeck::{Binders, Substitution, Ty, TyKind},
    Db, Name,
};
use rustc_hash::FxHashMap;
use smallvec::smallvec;

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
pub struct BuiltinFunctions {
    #[return_ref]
    pub functions: FxHashMap<Name, BuiltinFunction>,
}

#[salsa::tracked]
pub struct BuiltinFunction {
    pub name: Name,
    #[return_ref]
    pub params: Vec<BuiltinFunctionParam>,
    pub ret_ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinFunctionParam {
    Positional { ty: Ty, optional: bool },
    Keyword { name: Name, ty: Ty },
    VarArgList { ty: Ty },
    VarArgDict,
}

impl BuiltinFunctionParam {
    pub fn is_optional(&self) -> bool {
        match self {
            BuiltinFunctionParam::Positional { optional, .. } => *optional,
            _ => true,
        }
    }

    pub fn name(&self) -> Option<&Name> {
        match self {
            BuiltinFunctionParam::Keyword { name, .. } => Some(name),
            _ => None,
        }
    }
}

#[salsa::tracked]
pub(crate) fn builtin_functions(db: &dyn Db) -> BuiltinFunctions {
    // TODO(withered-magic): Many of these signatures are wrong
    // since the implementation of Starlark's type system is still
    // heavily WIP. For example, for the `list` builtin, we need to
    // support the `(List[T]) -> T` signature.
    // We also still need to support features like optional arguments,
    // keyword-only parameters, union types, "traits" like `Sequence[T]`,
    // function overloads, and so on.

    use BuiltinFunctionParam::*;
    use TyKind::*;
    let mut functions = FxHashMap::default();
    let mut add_function = |name, param_tys, ret_ty| {
        functions.insert(
            Name::new_inline(name),
            function(db, name, param_tys, ret_ty),
        );
    };

    // TODO(withered-magic): SupportsAbs[T] -> T
    add_function("abs", vec![positional(Any)], Any);
    add_function("any", vec![positional(Any)], Bool);
    add_function("all", vec![positional(Any)], Bool);
    add_function("bool", vec![positional_opt(Any)], Bool);
    // TODO(withered-magic): SupportsBytes[T] -> T
    add_function("bytes", vec![positional(Any)], Bytes);
    // TODO(withered-magic): Support dict()
    add_function(
        "dict",
        vec![
            positional_opt(List(Tuple(smallvec![Any.intern(), Any.intern()]).intern())),
            VarArgDict,
        ],
        Dict(Any.intern(), Any.intern()),
    );
    add_function("dir", vec![positional(Any)], List(String.intern()));
    add_function(
        "enumerate",
        vec![positional(Any)],
        List(Tuple(smallvec![Int.intern(), Any.intern()]).intern()),
    );
    add_function("float", vec![positional(Any)], Float);
    add_function("fail", vec![VarArgList { ty: Any.intern() }], None);
    add_function(
        "getattr",
        vec![positional(Any), positional(String), positional_opt(Any)],
        Any,
    );
    add_function("hasattr", vec![positional(Any), positional(String)], Bool);
    // // TODO(withered-magic): SupportsHash[T] -> T
    add_function("hash", vec![positional(Any)], Int);
    // // TODO(withered-magic): SupportInt[T] -> T
    add_function("int", vec![positional(Any), positional_opt(Int)], Int);
    add_function("len", vec![positional(Any)], Int);
    add_function("list", vec![positional(Any)], List(Any.intern()));
    add_function(
        "max",
        vec![
            VarArgList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        Any,
    );
    add_function(
        "min",
        vec![
            VarArgList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        Any,
    );
    add_function(
        "print",
        vec![
            VarArgList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("str"),
                ty: String.intern(),
            },
        ],
        None,
    );
    add_function(
        "range",
        vec![positional(Int), positional_opt(Int), positional_opt(Int)],
        Range,
    );
    add_function("repr", vec![positional(Any)], String);
    // TODO(withered-magic): Iterable[T] -> List[T]
    add_function("reversed", vec![positional(Any)], List(Any.intern()));
    // TODO(withered-magic): Iterable[T] -> List[T]
    add_function(
        "sorted",
        vec![
            positional(Any),
            Keyword {
                name: Name::new_inline("reverse"),
                ty: Bool.intern(),
            },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        List(Any.intern()),
    );
    add_function("str", vec![positional(Any)], String);
    // TODO(withered-magic): The tuple returned here can be of any size,
    // might have to introduce a separate type.
    add_function("tuple", vec![positional(Any)], Any);
    add_function(
        "zip",
        vec![VarArgList { ty: Any.intern() }],
        List(Any.intern()),
    );

    BuiltinFunctions::new(db, functions)
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
        make_string_base_class(db),
        make_bytes_base_class(db),
        make_list_base_class(db),
        make_dict_base_class(db),
    )
}

fn make_string_base_class(db: &dyn Db) -> BuiltinClass {
    use BuiltinFunctionParam::*;
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("string"),
        0,
        vec![
            function_field(db, "capitalize", vec![], String, 0),
            function_field(
                db,
                "count",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Int,
                0,
            ),
            function_field(db, "elems", vec![], StringElems, 0),
            function_field(
                db,
                "endswith",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Bool,
                0,
            ),
            function_field(
                db,
                "find",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Int,
                0,
            ),
            // TODO(withered-magic): Handle *args and **kwargs for format().
            function_field(
                db,
                "format",
                vec![VarArgList { ty: Any.intern() }, VarArgDict],
                String,
                0,
            ),
            function_field(
                db,
                "index",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Int,
                0,
            ),
            function_field(db, "isalnum", vec![], Bool, 0),
            function_field(db, "isalpha", vec![], Bool, 0),
            function_field(db, "isdigit", vec![], Bool, 0),
            function_field(db, "islower", vec![], Bool, 0),
            function_field(db, "isspace", vec![], Bool, 0),
            function_field(db, "istitle", vec![], Bool, 0),
            function_field(db, "isupper", vec![], Bool, 0),
            function_field(db, "join", vec![positional(Any)], String, 0),
            function_field(db, "lower", vec![], String, 0),
            function_field(db, "lstrip", vec![positional_opt(String)], String, 0),
            function_field(
                db,
                "partition",
                vec![positional(String)],
                Tuple(smallvec![String.intern(), String.intern(), String.intern()]),
                0,
            ),
            function_field(db, "removeprefix", vec![positional(String)], String, 0),
            function_field(db, "removesuffix", vec![positional(String)], String, 0),
            function_field(
                db,
                "replace",
                vec![positional(String), positional(String), positional_opt(Int)],
                String,
                0,
            ),
            function_field(
                db,
                "rfind",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Int,
                0,
            ),
            function_field(
                db,
                "rindex",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Int,
                0,
            ),
            function_field(
                db,
                "rpartition",
                vec![positional(String)],
                Tuple(smallvec![String.intern(), String.intern(), String.intern()]),
                0,
            ),
            function_field(
                db,
                "rsplit",
                vec![positional(String), positional_opt(Int)],
                List(String.intern()),
                0,
            ),
            function_field(db, "rstrip", vec![positional_opt(String)], String, 0),
            function_field(
                db,
                "split",
                vec![positional_opt(String), positional_opt(Int)],
                List(String.intern()),
                0,
            ),
            function_field(
                db,
                "splitlines",
                vec![positional_opt(Bool)],
                List(String.intern()),
                0,
            ),
            function_field(
                db,
                "startswith",
                vec![positional(String), positional_opt(Int), positional_opt(Int)],
                Bool,
                0,
            ),
            function_field(db, "strip", vec![positional_opt(Bool)], String, 0),
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
            function_field(db, "append", vec![positional(BoundVar(0))], None, 1),
            function_field(db, "clear", vec![], None, 1),
            function_field(db, "extend", vec![positional(Any)], None, 1),
            function_field(
                db,
                "index",
                vec![
                    positional(BoundVar(0)),
                    positional_opt(Int),
                    positional_opt(Int),
                ],
                Int,
                1,
            ),
            function_field(
                db,
                "insert",
                vec![positional(Int), positional(BoundVar(0))],
                None,
                1,
            ),
            function_field(db, "pop", vec![positional_opt(Int)], Any, 1),
            function_field(db, "remove", vec![positional(BoundVar(0))], None, 1),
        ],
    )
}

fn make_dict_base_class(db: &dyn Db) -> BuiltinClass {
    use BuiltinFunctionParam::*;
    use TyKind::*;
    BuiltinClass::new(
        db,
        crate::Name::new_inline("dict"),
        2,
        vec![
            function_field(db, "clear", vec![], None, 2),
            function_field(
                db,
                "get",
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                BoundVar(1),
                2,
            ),
            function_field(
                db,
                "items",
                vec![],
                List(Tuple(smallvec![BoundVar(0).intern(), BoundVar(1).intern()]).intern()),
                2,
            ),
            function_field(db, "keys", vec![], List(BoundVar(0).intern()), 2),
            function_field(
                db,
                "pop",
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                Any,
                2,
            ),
            function_field(
                db,
                "popitem",
                vec![],
                Tuple(smallvec![BoundVar(0).intern(), BoundVar(1).intern()]),
                2,
            ),
            function_field(
                db,
                "setdefault",
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                BoundVar(1),
                2,
            ),
            function_field(
                db,
                "update",
                vec![
                    positional(List(
                        Tuple(smallvec![BoundVar(0).intern(), BoundVar(1).intern()]).intern(),
                    )),
                    VarArgDict,
                ],
                None,
                2,
            ),
            function_field(db, "values", vec![], List(BoundVar(1).intern()), 2),
        ],
    )
}

fn function(
    db: &dyn Db,
    name: &'static str,
    params: Vec<BuiltinFunctionParam>,
    ret_ty: TyKind,
) -> BuiltinFunction {
    BuiltinFunction::new(db, Name::new_inline(name), params, ret_ty.intern())
}

fn function_field(
    db: &dyn Db,
    name: &'static str,
    params: Vec<BuiltinFunctionParam>,
    ret_ty: TyKind,
    num_vars: usize,
) -> BuiltinField {
    BuiltinField::new_inline(
        name,
        TyKind::BuiltinFunction(
            function(db, name, params, ret_ty),
            Substitution::new_identity(num_vars),
        )
        .intern(),
    )
}

fn positional(kind: TyKind) -> BuiltinFunctionParam {
    BuiltinFunctionParam::Positional {
        ty: kind.intern(),
        optional: false,
    }
}

fn positional_opt(kind: TyKind) -> BuiltinFunctionParam {
    BuiltinFunctionParam::Positional {
        ty: kind.intern(),
        optional: true,
    }
}
