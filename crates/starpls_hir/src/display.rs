use std::fmt::{self, Display, Write};

use crate::{
    def::Param as HirDefParam,
    module,
    typeck::{
        builtins::BuiltinFunctionParam, intrinsics::IntrinsicFunctionParam, with_tcx, Protocol,
        RuleKind, Tuple, TyKind, TypeRef,
    },
    Db, Name, Ty, Type,
};

pub trait DisplayWithDb {
    fn fmt(&self, db: &dyn Db, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn fmt_alt(&self, db: &dyn Db, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(db, f)
    }

    fn display<'a>(&'a self, db: &'a dyn Db) -> DisplayWithDbWrapper<'a, Self> {
        DisplayWithDbWrapper {
            db,
            item: self,
            alt: false,
        }
    }
}

pub fn delimited<D: DisplayWithDb>(
    db: &dyn Db,
    f: &mut fmt::Formatter,
    args: &[D],
    delimiter: &str,
) -> fmt::Result {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            f.write_str(delimiter)?;
        }
        arg.display(db).fmt(f)?;
    }

    Ok(())
}

pub struct DisplayWithDbWrapper<'a, T: DisplayWithDb + ?Sized> {
    db: &'a dyn Db,
    item: &'a T,
    alt: bool,
}

impl<'a, T: DisplayWithDb> DisplayWithDbWrapper<'a, T> {
    pub fn alt(self) -> Self {
        Self { alt: true, ..self }
    }
}

impl<'a, T: DisplayWithDb> fmt::Display for DisplayWithDbWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alt {
            self.item.fmt_alt(self.db, f)
        } else {
            self.item.fmt(self.db, f)
        }
    }
}

impl DisplayWithDb for Ty {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.kind().fmt(db, f)
    }
}

impl DisplayWithDb for Type {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ty.fmt(db, f)
    }

    fn fmt_alt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ty.fmt_alt(db, f)
    }
}

impl DisplayWithDb for TyKind {
    fn fmt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TyKind::Unbound => "Unbound",
            TyKind::Unknown => "Unknown",
            TyKind::Any => "Any",
            TyKind::Never => "Never",
            TyKind::None => "None",
            TyKind::Bool(Some(b)) => {
                f.write_str("Literal[")?;
                f.write_str(if *b { "True" } else { "False" })?;
                return f.write_char(']');
            }
            TyKind::Bool(None) => "bool",
            TyKind::Int(Some(x)) => return write!(f, "Literal[{}]", x),
            TyKind::Int(None) => "int",
            TyKind::Float => "float",
            TyKind::String(Some(s)) => return write!(f, "Literal[{:?}]", s.value(db)),
            TyKind::String(None) => "string",
            TyKind::StringElems => "string.elems",
            TyKind::Bytes => "bytes",
            TyKind::BytesElems => "bytes.elems",
            TyKind::List(ty) => {
                f.write_str("list[")?;
                ty.fmt(db, f)?;
                return f.write_char(']');
            }
            TyKind::Tuple(tuple) => {
                f.write_str("tuple[")?;
                match tuple {
                    Tuple::Simple(tys) => {
                        delimited(db, f, &tys, ", ")?;
                    }
                    Tuple::Variable(ty) => {
                        ty.fmt(db, f)?;
                        f.write_str(", ...")?;
                    }
                }
                return f.write_char(']');
            }
            TyKind::Dict(key_ty, value_ty, _) => {
                f.write_str("dict[")?;
                key_ty.fmt(db, f)?;
                f.write_str(", ")?;
                value_ty.fmt(db, f)?;
                return f.write_char(']');
            }
            TyKind::Range => "range",
            TyKind::Function(def) => {
                let module = module(db, def.func.file(db));
                write!(f, "def {}(", def.func.name(db).as_str())?;
                for (i, param) in def
                    .func
                    .params(db)
                    .iter()
                    .map(|param| &module[*param])
                    .enumerate()
                {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    let format_type_ref =
                        |f, type_ref| with_tcx(db, |tcx| tcx.resolve_type_ref(type_ref)).fmt(db, f);

                    match param {
                        HirDefParam::Simple { name, type_ref, .. } => {
                            f.write_str(name.as_str())?;
                            if let Some(type_ref) = type_ref.as_ref() {
                                f.write_str(": ")?;
                                format_type_ref(f, type_ref)?;
                            }
                        }
                        HirDefParam::ArgsList { name, type_ref, .. } => {
                            f.write_char('*')?;
                            if !name.is_missing() {
                                f.write_str(name.as_str())?;
                                f.write_str(": ")?;
                                match type_ref.as_ref() {
                                    Some(type_ref) => format_type_ref(f, type_ref),
                                    None => f.write_str("Unknown"),
                                }?;
                            }
                        }
                        HirDefParam::KwargsDict { name, type_ref, .. } => {
                            f.write_str("**")?;
                            if !name.is_missing() {
                                f.write_str(name.as_str())?;
                                f.write_str(": ")?;
                                match type_ref.as_ref() {
                                    Some(type_ref) => format_type_ref(f, type_ref),
                                    None => f.write_str("Unknown"),
                                }?;
                            }
                        }
                    }
                }
                return write!(
                    f,
                    ") -> {}",
                    def.func.ret_type_ref(db).unwrap_or(TypeRef::Unknown)
                );
            }
            TyKind::IntrinsicFunction(func, subst) => {
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func.params(db).iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match param {
                        IntrinsicFunctionParam::Positional { ty, optional } => {
                            write!(f, "x{}: ", i)?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                            if *optional {
                                f.write_str(" = None")?;
                            }
                        }
                        IntrinsicFunctionParam::Keyword { name, ty } => {
                            f.write_str(name.as_str())?;
                            f.write_str(": ")?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                            f.write_str(" = None")?;
                        }
                        IntrinsicFunctionParam::ArgsList { ty } => {
                            f.write_str("*args: ")?;
                            ty.substitute(&subst.args).fmt(db, f)?;
                        }
                        IntrinsicFunctionParam::KwargsDict => {
                            f.write_str("**kwargs")?;
                        }
                    }
                }
                f.write_str(") -> ")?;
                return func.ret_ty(db).substitute(&subst.args).fmt(db, f);
            }
            TyKind::BuiltinFunction(func) => {
                write!(f, "def {}(", func.name(db).as_str())?;
                for (i, param) in func.params(db).iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match param {
                        BuiltinFunctionParam::Simple {
                            name,
                            type_ref,
                            default_value,
                            ..
                        } => {
                            f.write_str(name.as_str())?;
                            if !type_ref.is_unknown() {
                                f.write_str(": ")?;
                                type_ref.fmt(f)?;
                            }
                            if let Some(default_value) = default_value {
                                f.write_str(" = ")?;
                                f.write_str(&default_value)?;
                            }
                        }
                        BuiltinFunctionParam::ArgsList { name, type_ref, .. } => {
                            f.write_char('*')?;
                            f.write_str(name.as_str())?;
                            if !type_ref.is_unknown() {
                                f.write_str(": ")?;
                                type_ref.fmt(f)?;
                            }
                        }
                        BuiltinFunctionParam::KwargsDict { name, type_ref, .. } => {
                            f.write_str("**")?;
                            f.write_str(name.as_str())?;
                            if !type_ref.is_unknown() {
                                f.write_str(": ")?;
                                type_ref.fmt(f)?;
                            }
                        }
                    }
                }
                f.write_str(") -> ")?;
                return func.ret_type_ref(db).fmt(f);
            }
            TyKind::BuiltinType(ty, _) => return f.write_str(ty.name(db).as_str()),
            TyKind::BoundVar(index) => return write!(f, "'{}", index),
            TyKind::Protocol(proto) => {
                let (name, ty) = match proto {
                    Protocol::Iterable(ty) => ("Iterable", ty),
                    Protocol::Sequence(ty) => ("Sequence", ty),
                };
                return write!(f, "{}[{}]", name, ty.display(db).alt());
            }
            TyKind::Union(tys) => {
                return delimited(db, f, tys, " | ");
            }
            TyKind::Struct(_) => "struct",
            TyKind::Attribute(_) => "Attribute",
            TyKind::Rule(rule) => match rule.kind {
                RuleKind::Build => "rule",
                RuleKind::Repository => "repository_rule",
            },
            TyKind::Provider(provider) => {
                return write!(
                    f,
                    "Provider[{}]",
                    provider.name(db).map_or("_", Name::as_str)
                );
            }
            TyKind::ProviderInstance(provider) => provider.name(db).map_or("_", Name::as_str),
            TyKind::ProviderRawConstructor(_, _) => "ProviderRawConstructor",
            TyKind::TagClass(_) => "tag_class",
            TyKind::ModuleExtension(_) => "module_extension",
            TyKind::ModuleExtensionProxy(_) => "module_extension_proxy",
            TyKind::Tag(_) => "tag",
            TyKind::Target => "Target",
        };

        f.write_str(text)
    }

    fn fmt_alt(&self, db: &dyn Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TyKind::Function(_) => f.write_str("function"),
            TyKind::IntrinsicFunction(_, _) | TyKind::BuiltinFunction(_) => {
                f.write_str("builtin_function_or_method")
            }
            _ => self.fmt(db, f),
        }
    }
}
