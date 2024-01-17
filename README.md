# Starpls
`starpls` is a language server for [Starlark](https://github.com/bazelbuild/starlark), the configuration language used by Bazel and Buck2.

## Development
`starpls` currently requires a nightly build of Rust, due to usage of `trait_upcasting` as specified by [RFC3324](https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html).

## Disclaimer
This project is still heavily WIP, so expect a decent amount of bugs and crashes if you decide to try it out! Additionally, I've elected to focus on implementing the core Starlark specification first before introducing Bazel- or Buck2-specific features.

## Acknowledgements
- `starpls` is heavily based on the [rust-analyzer](https://github.com/rust-lang/rust-analyzer/tree/master) codebase; one might consider it a vastly simplified version of rust-analyzer that works on Starlark files! As such, major thanks to the rust-analyzer team, especially [Aleksey Kladov](https://matklad.github.io/), whose [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) series on YouTube proved invaluable as a learning resource!
- `starpls`'s mechanism for carrying out type inference is heavily derived from that of [Pyright](https://github.com/microsoft/pyright).