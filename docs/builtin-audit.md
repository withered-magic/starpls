# Bazel `builtin.pb` audit

This document contains a list of known issues with the `builtin.pb` proto as currently exported by Bazel's [ApiExporter](https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/docgen/ApiExporter.java). The following issues apply to the proto as exported by the latest commit on [this PR](https://github.com/bazelbuild/bazel/pull/21135).

## Issues
- All `WORKSPACE` builtins missing:
    - `bind()`
    - `register_execution_platforms()`
    - `register_toolchains()`
    - `workspace()`
- All `MODULE.bazel` builtins missing:
    - `archive_override()`
    - `bazel_dep()`
    - `git_override()`
    - `local_path_override()`
    - `module()`
    - `multiple_version_override()`
    - `single_version_override()`
    - `use_extension()`
    - `use_repo()`
    - `use_repo_rule()`
- One `BUILD.bazel` builtin missing:
    - `package()`
- Some `.bzl` builtins missing:
    - `module_extension()`
    - `repository_rule()`
    - `tag_class`
- A number of providers are exported incorrectly. Particularly, some global provider constructors (e.g. the `PyInfo` constructor) are instead exported as _instances_ of those providers themselves. For example, instead of a callable that yields `PyInfo`s, the `PyInfo` global is instead an instance of `PyInfo` itself, causing the LSP to consider expressions like `PyInfo.imports` to be valid. Curiously, not all providers are exported incorrectly, just some; following is a list of providers exported incorrectly as described above:
    - `PyInfo`
    - `PyRuntimeInfo`
    - `InstrumentedFilesInfo`
    - `CcToolchainConfigInfo`
    - `PackageSpecificationInfo`
- The following providers are missing type info (i.e. these symbols in the global scope are missing corresponding type definitions):
    - `JavaInfo`
    - `ProtoInfo`
    - `PyWrapCcInfo`
    - `JavaPluginInfo`
    - `AnalysisFailureInfo`
    - `CcSharedLibraryInfo`
    - `CcSharedLibraryHintInfo`
- Most builtin rules (`genrule`, `py_library`, etc.) are missing. This is not as important though since the rules can be sourced from `bazel info build-language`.