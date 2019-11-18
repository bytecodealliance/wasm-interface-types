<div align="center">
  <h1><code>wasm-interface-types</code></h1>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust toolkit for working with <a href="https://github.com/webassembly/interface-types">WebAssembly Interface Tyeps</a>.</strong>
  </p>
</div>


## About

This project is a work-in-progress. The official proposal itself has neither an
official text specification nor an official binary specification. Instability
and breaking changes should be expected from this project.

This is attempting to fill functionality gaps in the proposal to ensure that
toolchains can be developed for wasm interface types to help provide feedback on
the proposal and also provide a testing ground for ideas to be concretely gamed
out.

Functionality provided by this project currently is:

* `wit2wasm` - a converter from a textual format of wasm interface types to a
  binary `*.wasm` module

* `wasm2wit` - same as above, but the other way around. Takes a `*.wasm` module
  and prints out the `*.wat` file, including interface types annotations if
  there are any.

* `crates/text` - a Rust library which parses the text format for wasm interface
  types

* `crates/parser` - a Rust library which parses the binary format for wasm
  interface types

* `crates/validator` - a Rust library which performs validation over a wasm
  module which contains wasm interface types, specifically focusing on the wasm
  interface types section.

* `crates/printer` - a Rust library which will print the binary representation
  of a wasm interface types `*.wasm` blob into its textual format.

The current state of the binary encoding as well as some semantic nodes are
located in [`BINARY.md`](BINARY.md) as well as [`SEMANTICS.md`](SEMANTICS.md).
To reiterate though, this is not an official specification and the official
specification is in flux, use these tools appropriately!

## Tests

The top-level `tests` directory contains a number of `*.wit` files which are
intended to be various forms of tests for the wasm interface types proposal.
They're annotated at the top with things like `;; parse-fail` or `;;
validate-fail` if they're expected to be invalid, otherwise the comment at the
bottom is the round-trip representation through the tooling here (text -> binary
-> printing).

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
