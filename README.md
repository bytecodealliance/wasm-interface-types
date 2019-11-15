# `wasm-interface-types`

**A [Bytecode Alliance](https://bytecodealliance.org/) project**

A Rust parser and decoder for the [WebAssembly Interface Types][wit] proposal.

[wit]: https://github.com/webassembly/interface-types

## About

This project is a work-in-progress. The official proposal itself has neither an
official text specification nor an official binary specification. Instability
and breaking changes should be expected from this project.

This is attempting to fill functionality gaps in the proposal to ensure that
toolchains can be developed for wasm interface types to help provide feedback on
the proposal and also provide a testing ground for ideas to be concretely gamed
out.

Functionality considered within the scope of this project includes:

* A parser for a textual format that includes wasm interface types.
* A transformation from the textual AST to a binary format.
* A parser for the binary format.
* A type checker and validator for the binary format.

This project is intended to be a pretty raw version of wasm interface types.
It's also hoped though that there can be convenience crates to do things like
gc, integrate with other tooling, etc. We'll sort of see where this goes!

# License

This project is license under the Apache 2.0 license with the LLVM exception.
See [LICENSE] for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
