# Starpls
`starpls` is a language server for [Starlark](https://github.com/bazelbuild/starlark), the configuration language used by Bazel and Buck2.

## Installation
Make sure you have at least the [0.9.0](https://github.com/bazelbuild/vscode-bazel/releases/tag/0.9.0) version of the [vscode-bazel](https://github.com/bazelbuild/vscode-bazel) extension installed, as it adds support for launching a language server.

You can grab a release from the [releases page](https://github.com/withered-magic/starpls/releases). Make sure to download the appropriate version for your OS and architecture! After downloading the binary, make sure to adjust its permissions to make it executable, e.g.

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
    - [x] PEP-484 type comments
        - [x] Variables
        - [x] Parameters (only basic types currently supported)
        - [x] Other constructs where type comments are supported
- Third-party integrations
    - [x] Bazel builtins (partial, Bazel builtins are supported but still need to handle a number of edge cases)
    - Special handling for various Bazel constructs
        - [x] `struct`s (autocomplete fields)
        - [ ] providers (autocomplete and validate fields)
        - [ ] rules defined with `rule` and `repository_rule` (autocomplete and validate attributes)
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
- The `not in` operator is currently not supported and does not parse correctly.

## Disclaimer
This project is still heavily WIP, so expect a decent amount of bugs and crashes if you decide to try it out! Additionally, I've elected to focus on implementing the core Starlark specification first before introducing Bazel- or Buck2-specific features.

## Acknowledgements
- `starpls` is heavily based on the [rust-analyzer](https://github.com/rust-lang/rust-analyzer/tree/master) codebase; one might consider it a vastly simplified version of rust-analyzer that works on Starlark files! As such, major thanks to the rust-analyzer team, especially [Aleksey Kladov](https://matklad.github.io/), whose [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) series on YouTube proved invaluable as a learning resource!
- `starpls`'s mechanism for carrying out type inference is heavily derived from that of [Pyright](https://github.com/microsoft/pyright).
