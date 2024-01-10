use crate::{
    typeck::{
        BuiltinField, BuiltinFunction, BuiltinType, Class, Fields, FunctionKind, Ty, TyKind,
        TypeRef,
    },
    Name,
};

pub struct Builtins {
    any_ty: Ty,
    none_ty: Ty,
    bool_ty: Ty,
    int_ty: Ty,
    float_ty: Ty,
    string_ty: Ty,
    string_elems_ty: Ty,
    bytes_ty: Ty,
    bytes_elems_ty: Ty,
    list_ty: Ty,
    tuple_ty: Ty,
    dict_ty: Ty,
}

impl Builtins {
    pub fn any_ty(&self) -> Ty {
        self.any_ty.clone()
    }

    pub fn none_ty(&self) -> Ty {
        self.none_ty.clone()
    }

    pub fn bool_ty(&self) -> Ty {
        self.bool_ty.clone()
    }

    pub fn int_ty(&self) -> Ty {
        self.int_ty.clone()
    }

    pub fn float_ty(&self) -> Ty {
        self.float_ty.clone()
    }

    pub fn string_ty(&self) -> Ty {
        self.string_ty.clone()
    }

    pub fn string_elems_ty(&self) -> Ty {
        self.string_elems_ty.clone()
    }

    pub fn bytes_ty(&self) -> Ty {
        self.bytes_ty.clone()
    }

    pub fn bytes_elems_ty(&self) -> Ty {
        self.bytes_elems_ty.clone()
    }

    pub fn list_ty(&self) -> Ty {
        self.list_ty.clone()
    }

    pub fn tuple_ty(&self) -> Ty {
        self.tuple_ty.clone()
    }

    pub fn dict_ty(&self) -> Ty {
        self.dict_ty.clone()
    }
}

pub fn intern_builtins() -> Builtins {
    Builtins {
        any_ty: intern_any_ty(),
        none_ty: intern_none_ty(),
        bool_ty: intern_bool_ty(),
        int_ty: intern_int_ty(),
        float_ty: intern_float_ty(),
        string_ty: intern_string_ty(),
        string_elems_ty: intern_string_elems_ty(),
        bytes_ty: intern_bytes_ty(),
        bytes_elems_ty: intern_bytes_elems_ty(),
        list_ty: intern_list_ty(),
        tuple_ty: intern_tuple_ty(),
        dict_ty: intern_dict_ty(),
    }
}

fn intern_any_ty() -> Ty {
    TyKind::Any.intern()
}

fn intern_none_ty() -> Ty {
    TyKind::None.intern()
}

fn intern_bool_ty() -> Ty {
    intern_builtin_class("bool", Vec::new())
}

fn intern_int_ty() -> Ty {
    intern_builtin_class("int", Vec::new())
}

fn intern_float_ty() -> Ty {
    intern_builtin_class("float", Vec::new())
}

fn intern_string_ty() -> Ty {
    let mut fields = Vec::new();
    add_method(
        &mut fields,
        "capitalize",
        Vec::new(),
        Some(BuiltinType::String.into()),
    );
    add_method(
        &mut fields,
        "count",
        vec![BuiltinType::String.into()],
        Some(BuiltinType::Int.into()),
    );
    add_method(
        &mut fields,
        "elems",
        Vec::new(),
        Some(BuiltinType::StringElems.into()),
    );
    add_method(
        &mut fields,
        "endswith",
        vec![BuiltinType::String.into()],
        Some(BuiltinType::Bool.into()),
    );
    add_method(
        &mut fields,
        "find",
        vec![BuiltinType::String.into()],
        Some(BuiltinType::Int.into()),
    );
    // add_method(&mut fields, "format");
    // add_method(&mut fields, "index");
    // add_method(&mut fields, "isalnum");
    // add_method(&mut fields, "isdigit");
    // add_method(&mut fields, "islower");
    // add_method(&mut fields, "isspace");
    // add_method(&mut fields, "istitle");
    // add_method(&mut fields, "isupper");
    // add_method(&mut fields, "join");
    // add_method(&mut fields, "lower");
    // add_method(&mut fields, "lstrip");
    // add_method(&mut fields, "partition");
    // add_method(&mut fields, "removeprefix");
    // add_method(&mut fields, "replace");
    // add_method(&mut fields, "rfind");
    // add_method(&mut fields, "rindex");
    // add_method(&mut fields, "rpartition");
    // add_method(&mut fields, "rsplit");
    // add_method(&mut fields, "rstrip");
    // add_method(&mut fields, "split");
    // add_method(&mut fields, "splitlines");
    // add_method(&mut fields, "startswith");
    // add_method(&mut fields, "strip");
    // add_method(&mut fields, "title");
    // add_method(&mut fields, "upper");
    intern_builtin_class("string", fields)
}

fn intern_string_elems_ty() -> Ty {
    intern_builtin_class("string.elems", Vec::new())
}

fn intern_bytes_ty() -> Ty {
    let mut fields = Vec::new();
    // add_method(&mut fields, "elems");
    intern_builtin_class("bytes", fields)
}

fn intern_bytes_elems_ty() -> Ty {
    intern_builtin_class("bytes.elems", Vec::new())
}

fn intern_list_ty() -> Ty {
    let mut fields = Vec::new();
    // add_method(&mut fields, "append");
    // add_method(&mut fields, "clear");
    // add_method(&mut fields, "extend");
    // add_method(&mut fields, "index");
    // add_method(&mut fields, "insert");
    // add_method(&mut fields, "pop");
    // add_method(&mut fields, "remove");
    intern_builtin_class("list", fields)
}

fn intern_tuple_ty() -> Ty {
    let mut fields = Vec::new();
    intern_builtin_class("tuple", fields)
}

fn intern_dict_ty() -> Ty {
    let mut fields = Vec::new();
    add_method(
        &mut fields,
        "clear",
        Vec::new(),
        Some(BuiltinType::None.into()),
    );
    add_method(&mut fields, "get", vec![TypeRef::Any], Some(TypeRef::Any));
    add_method(
        &mut fields,
        "items",
        Vec::new(),
        Some(BuiltinType::List.into()),
    );
    add_method(
        &mut fields,
        "keys",
        Vec::new(),
        Some(BuiltinType::List.into()),
    );
    add_method(&mut fields, "pop", vec![TypeRef::Any], Some(TypeRef::Any));
    add_method(
        &mut fields,
        "popitem",
        Vec::new(),
        Some(BuiltinType::Tuple.into()),
    );
    add_method(
        &mut fields,
        "setdefault",
        vec![TypeRef::Any],
        Some(TypeRef::Any),
    );
    add_method(
        &mut fields,
        "values",
        Vec::new(),
        Some(BuiltinType::List.into()),
    );
    intern_builtin_class("dict", fields)
}

fn add_method(
    methods: &mut Vec<BuiltinField>,
    name: &'static str,
    params: Vec<TypeRef>,
    ret_type_ref: Option<TypeRef>,
) {
    methods.push(BuiltinField {
        name: Name::new_inline(name),
        ty: TyKind::Function(FunctionKind::Builtin(BuiltinFunction {
            params: params.into_boxed_slice(),
            ret_type_ref,
        }))
        .intern(),
    })
}

fn intern_builtin_class(name: &'static str, fields: Vec<BuiltinField>) -> Ty {
    TyKind::Class(Class {
        name: Name::new_inline(name),
        fields: Fields::Builtin(fields.into_boxed_slice()),
    })
    .intern()
}
