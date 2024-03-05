use prost::Message;
use std::io;
use std::path::Path;
use std::{fs, path::PathBuf};

pub use crate::{
    builtin::Builtins,
    label::{Label, ParseError},
};

pub mod label;

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
    decode_builtins(&data)
}

pub fn decode_builtins(data: &[u8]) -> anyhow::Result<Builtins> {
    let builtins = Builtins::decode(&data[..])?;
    Ok(builtins)
}

pub fn resolve_workspace_root(from: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    for ancestor in from
        .as_ref()
        .ancestors()
        .filter(|ancestor| ancestor.is_dir())
    {
        for entry in fs::read_dir(ancestor)? {
            if let Some("WORKSPACE" | "WORKSPACE.bazel" | "MODULE.bazel") = entry
                .ok()
                .map(|entry| entry.file_name())
                .as_ref()
                .and_then(|file_name| file_name.to_str())
            {
                return Ok(Some(ancestor.to_path_buf()));
            }
        }
    }

    Ok(None)
}
