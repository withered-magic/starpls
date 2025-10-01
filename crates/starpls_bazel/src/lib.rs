use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use prost::Message;

pub use crate::builtin::Builtins;
// pub use crate::dialect::create_bazel_dialect; // Temporarily commented out due to circular dependency
pub use crate::dialect::BazelBuiltinProvider;
pub use crate::dialect::BazelDialectDetector;
pub use crate::label::Label;
pub use crate::label::ParseError;

pub mod attr;
pub mod build_language;
pub mod client;
pub mod dialect;
pub mod env;
pub mod label;

#[cfg(bazel)]
pub mod builtin {
    pub use builtin_proto::builtin::*;
}

#[cfg(not(bazel))]
pub mod builtin {
    include!(concat!(env!("OUT_DIR"), "/builtin.rs"));
}

#[cfg(bazel)]
pub mod build {
    pub use build_proto::blaze_query::*;
}

#[cfg(not(bazel))]
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/blaze_query.rs"));
}

pub const BUILTINS_TYPES_DENY_LIST: &[&str] = &[
    "Attribute",
    "bool",
    "bytes",
    "builtin_function_or_method",
    "dict",
    "float",
    "function",
    "int",
    "list",
    "range",
    "string",
    "struct",
    "Target",
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

pub const KNOWN_PROVIDER_TYPES: &[&str] = &[
    "AnalysisTestResultInfo",
    "AndroidNeverLinkLibrariesProvider",
    "ApkInfo",
    "BaselineProfileProvider",
    "CcInfo",
    "CcToolchainConfigInfo",
    "CcToolchainInfo",
    "ConstraintSettingInfo",
    "ConstraintValueInfo",
    "DebugPackageInfo",
    "DefaultInfo",
    "ExecutionInfo",
    "FeatureFlagInfo",
    "FilesToRunProvider",
    "GeneratedExtensionRegistryProvider",
    "IncompatiblePlatformProvider",
    "InstrumentedFilesInfo",
    "java_compilation_info",
    "java_output_jars",
    "JavaRuntimeInfo",
    "JavaToolchainInfo",
    "ObjcProvider",
    "OutputGroupInfo",
    "PackageSpecificationInfo",
    "PlatformInfo",
    "ProguardSpecProvider",
    "ProtoRegistryProvider",
    "PyInfo",
    "PyRuntimeInfo",
    "RunEnvironmentInfo",
    "TemplateVariableInfo",
    "ToolchainInfo",
    "ToolchainTypeInfo",
];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum APIContext {
    Bzl,
    Build,
    Module,
    Repo,
    Workspace,
    Prelude,
    Cquery,
    Vendor,
}

pub fn load_builtins(path: impl AsRef<Path>) -> anyhow::Result<Builtins> {
    let data = fs::read(path)?;
    decode_builtins(&data)
}

pub fn decode_builtins(data: &[u8]) -> anyhow::Result<Builtins> {
    let builtins = Builtins::decode(data)?;
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
                Some("WORKSPACE" | "WORKSPACE.bazel" | "MODULE.bazel" | "REPO.bazel") => {
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
