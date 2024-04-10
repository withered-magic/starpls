# Starpls
`starpls` is a language server for [Starlark](https://github.com/bazelbuild/starlark), the configuration language used by Bazel and Buck2.

## Installation

### VSCode

Make sure you have at least the [0.9.0](https://github.com/bazelbuild/vscode-bazel/releases/tag/0.9.0) version of the [vscode-bazel](https://github.com/bazelbuild/vscode-bazel) extension installed, as it adds support for launching a language server.

If you're on a Mac with Apple Silicon, then you can install `starpls` with Homebrew and skip ahead to the section about configuring VSCode:

```sh
brew install withered-magic/brew/starpls
```

Otherwise, you can grab a release from the [releases page](https://github.com/withered-magic/starpls/releases). Make sure to download the appropriate version for your OS and architecture! After downloading the binary, make sure to adjust its permissions to make it executable, e.g.

```sh
chmod +x starpls-darwin-arm64
```

Additionally, on Mac OS, you may see an error similar to

```
“starpls-darwin-arm64” can’t be opened because Apple cannot check it for malicious software.
```

To fix this, click `Show in Finder`, then right-click on the `starpls-darwin-arm64` executable, click `Open`, and select `Open` in the warning that comes up. This will cause the `com.apple.quarantine` xattr to be removed from the executable and will stop the warning from appearing further.

Either way, at this point you can put the executable somewhere on your `$PATH`.

Once done, add the following to your VSCode configuration and reload VSCode for it to take effect:

```json
{
  "bazel.lsp.enabled": true,
  "bazel.lsp.command": "starpls",
  "bazel.lsp.args": []
}
```

Note: If you don't put `starpls` directly on the `$PATH`, then for `bazel.lsp.command` you'll have to specify the absolute path to the `starpls` executable instead. Additionally, if your VSCode setup also has any tasks that run Bazel commands on open, those might temporarily block the server from starting up because of the Bazel lock; the server will still spin up once it is able to acquire the lock.

Alternatively, you can build `starpls` with Bazel:

```
bazel run -c opt //editors/code:copy_starpls
```

This builds the executable and copies it to `<repository_root>/editors/code/bin/starpls`. From there, you can add it to the `$PATH` or copy it to a different directory, remembering to update the extension settings as detailed above.

### Zed

Install the [zed-starlark](https://github.com/zaucy/zed-starlark) extension.

## Tips and Tricks

Make sure to use [PEP 484 type comments](https://peps.python.org/pep-0484/#type-comments) to document your function signatures. This helps a ton with autocomplete for situations like `rule` implementation functions. For example, if you add a type comment as in the following...

```python
def _impl(ctx):
    # type: (ctx) -> Unknown
    ctx.
    #  ^ and this period was just typed... 
```

then you'll get autocomplete suggestions for the attributes on `ctx`, like `ctx.actions`, `ctx.attr`, and so on!

## Roadmap

- Parsing
    - [x] Error resilient Starlark parser
    - [x] Syntax error reporting
- Semantic highlighting
    - [x] Unbound variables
    - [x] Type mismatches
    - [x] Function call argument validation
- Auto-completion
    - [x] Variables/function parameters
    - [x] Builtin type fields
    - [x] Rule attributes
    - [x] Custom provider fields
    - [x] Custom struct fields
- Hover
    - [x] Variable types
    - [x] Function signatures
    - [x] Function/method docs
- Go to definition
    - [x] Variables (including `load`ed symbols)
    - [x] Function definitions
    - [x] Struct fields
    - [x] Provider fields
    - [x] Labels in `load` statements
    - [ ] Rule attributes
- Type inference
    - [x] Basic type inference
    - [ ] Dataflow analysis
    - [x] PEP-484 type comments
        - [x] Variables
        - [x] Parameters (only basic types currently supported)
        - [x] Other constructs where type comments are supported
- Third-party integrations
    - [x] Bazel builtins (partial, Bazel builtins are supported but still need to handle a number of edge cases)
    - Special handling for various Bazel constructs
        - [x] `struct`s (autocomplete fields)
        - [x] providers (autocomplete and validate fields)
        - [x] rules defined with `rule` and `repository_rule` (autocomplete and validate attributes)
- Projects
    - [x] Type inference across multiple files
    - [x] `load` support
        - [x] Relative paths
        - [x] Bazel workspace
    - [x] Bazel external repositories

## Development

`starpls` currently requires a nightly build of Rust, due to usage of `trait_upcasting` as specified by [RFC3324](https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html).

### Prerequisites

- `pnpm`, for managing Node dependencies
- `protoc`, for compiling `builtin.proto`

Steps to get up and running:
1. Run `pnpm install` in `editors/code`.
2. Open VSCode, `Run and Debug > Run Extension (Debug Build)`.
3. In the extension development host, open a `.star` file and enjoy syntax highlighting and error messages!

## Known Issues

- Type guards are not supported.
- Type checker shows some false positives, especially when the definitions from the builtins proto are incorrect.
    - Because of these two issues, some type checking diagnostics are currently set to display as warnings.
- Type checking + goto definition for symbols loaded from external dependencies will only work if those dependencies have already been fetched. If you see `Could not resolve module` warnings in `load` statements, make sure to run `bazel fetch //...` to make sure the external output base is up-to-date.
- When `--enable-bzlmod` is set, type checking/goto definition may be slow for a given file the first time it is loaded. This is because resolution of repo mappings, done with `bazel mod dump_repo_mappings`, is done lazily.
    - Additionally, when new dependencies are added, the language server needs to be restarted to refresh the mappings. This is due to the fact that repo mappings are cached, which is necessary to avoid slow type checking.

## Acknowledgements

- `starpls` is heavily based on the [rust-analyzer](https://github.com/rust-lang/rust-analyzer/tree/master) codebase; one might consider it a vastly simplified version of rust-analyzer that works on Starlark files! As such, major thanks to the rust-analyzer team, especially [Aleksey Kladov](https://matklad.github.io/), whose [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) series on YouTube proved invaluable as a learning resource!
- `starpls`'s mechanism for carrying out type inference is heavily derived from that of [Pyright](https://github.com/microsoft/pyright).
