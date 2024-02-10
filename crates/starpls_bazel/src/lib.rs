use prost::Message;
use std::fs;
use std::path::Path;

pub use builtin::Builtins;

#[cfg(bazel)]
pub mod builtin {
    pub use builtin_proto::builtin::*;
}

#[cfg(not(bazel))]
pub mod builtin {
    include!(concat!(env!("OUT_DIR"), "/builtin.rs"));
}

pub const BUILTINS_TYPES_DENY_LIST: &[&str] = &[
    "bool",
    "bytes",
    "builtin_function_or_method",
    "dict",
    "float",
    "function",
    "int",
    "json",
    "list",
    "range",
    "string",
    "tuple",
    "None",
    "NoneType",
];

pub const BUILTINS_VALUES_DENY_LIST: &[&str] = &[
    "False",
    "True",
    "None",
    "abs",
    "all",
    "any",
    "bool",
    "dict",
    "dir",
    "enumerate",
    "fail",
    "float",
    "getattr",
    "hasattr",
    "hash",
    "int",
    "len",
    "list",
    "max",
    "min",
    "print",
    "range",
    "repr",
    "reversed",
    "sorted",
    "str",
    "tuple",
    "type",
    "zip",
];

pub fn load_builtins(path: impl AsRef<Path>) -> anyhow::Result<Builtins> {
    let data = fs::read(path)?;
    let builtins = Builtins::decode(&data[..])?;
    Ok(builtins)
}
