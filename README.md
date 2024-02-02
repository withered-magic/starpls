# Starpls
`starpls` is a language server for [Starlark](https://github.com/bazelbuild/starlark), the configuration language used by Bazel and Buck2.

## Development
`starpls` currently requires a nightly build of Rust, due to usage of `trait_upcasting` as specified by [RFC3324](https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html).

### Prerequisites
Make sure you have `protoc` installed, as it's used to compile `builtin.proto`.

Steps to get up and running:
1. `rustup override set nightly`
2. `cargo build`
3. Open VSCode, `Run and Debug > Run Extension (Debug Build)`.
4. In the extension development host, open a `.star` file and enjoy syntax highlighting and error messages!

Additionally, you may want to enable auto-completion and other features for Bazel builtins. This can be done with the following steps.
1. Build the Bazel builtins proto. This can be done by cloning the [Bazel repository](https://github.com/bazelbuild/bazel) and then building the builtins proto with `bazel build //src/main/java/com/google/devtools/build/lib:gen_api_proto`.
2. Copy the Bazel builtins proto to your working copy of the `starpls` repo. Specifically, copy it to `<starpls repository root>/editors/code/builtin.pb`, as this is where the development setup expects the proto to be by default.

## Roadmap
- Parsing
    - [x] Error resilient Starlark parser
    - [x] Syntax error reporting
- Semantic highlighting
    - [x] Unbound variables
    - [x] Type mismatches
    - [x] Function call argument validation (partial, works for Starlark builtins)
- Auto-completion
    - [x] Variables/function parameters
    - [x] Fields 
- Hover
    - [x] Variable types
    - [x] Function signatures
    - [x] Function/method docs
- Type inference
    - [x] Basic type inference
    - [ ] Dataflow analysis
    - [ ] PEP-484 type comments
        - [ ] Variables
        - [x] Parameters (only basic types currently supported)
        - [ ] Other constructs where type comments are supported
- Third-party integrations
    - [ ] Bazel builtins (partial, Bazel builtins are supported but still need to handle a number of edge cases)
- Projects
    - [ ] Type inference across multiple files
    - [ ] `load` support
        - [ ] Relative paths
        - [ ] Bazel workspace
    - [ ] Third-party Starlark libraries

## Disclaimer
This project is still heavily WIP, so expect a decent amount of bugs and crashes if you decide to try it out! Additionally, I've elected to focus on implementing the core Starlark specification first before introducing Bazel- or Buck2-specific features.

## Acknowledgements
- `starpls` is heavily based on the [rust-analyzer](https://github.com/rust-lang/rust-analyzer/tree/master) codebase; one might consider it a vastly simplified version of rust-analyzer that works on Starlark files! As such, major thanks to the rust-analyzer team, especially [Aleksey Kladov](https://matklad.github.io/), whose [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) series on YouTube proved invaluable as a learning resource!
- `starpls`'s mechanism for carrying out type inference is heavily derived from that of [Pyright](https://github.com/microsoft/pyright).