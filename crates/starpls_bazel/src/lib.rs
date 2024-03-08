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

pub fn resolve_workspace(from: impl AsRef<Path>) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let mut package: Option<PathBuf> = None;
    for ancestor in from
        .as_ref()
        .ancestors()
        .filter(|ancestor| ancestor.is_dir())
    {
        for entry in fs::read_dir(ancestor)? {
            match entry
                .ok()
                .map(|entry| entry.file_name())
                .as_ref()
                .and_then(|file_name| file_name.to_str())
            {
                Some("WORKSPACE" | "WORKSPACE.bazel" | "MODULE.bazel") => {
                    return Ok(Some((
                        ancestor.to_path_buf(),
                        package.unwrap_or_else(|| ancestor.to_path_buf()),
                    )));
                }
                Some("BUILD" | "BUILD.bazel") => {
                    package.get_or_insert(ancestor.to_path_buf());
                }
                _ => {}
            }
        }
    }

    Ok(None)
}
