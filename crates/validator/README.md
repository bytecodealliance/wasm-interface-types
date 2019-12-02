<div align="center">
  <h1><code>wit-validator</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust crate to validate the <a
    href="https://github.com/webassembly/interface-types">WebAssembly
    Interface Types</a> binary format.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wit-validator"><img src="https://img.shields.io/crates/v/wit-validator.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wit-validator"><img src="https://img.shields.io/crates/d/wit-validator.svg?style=flat-square" alt="Download" /></a>
    <a href="https://bytecodealliance.github.io/wit-validator/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

> **Note**: [WebAssembly Interface
> Types](https://github.com/webassembly/interface-types) are experimental and
> subject to a good deal of change. It's not recommended to rely on this if
> you're not comfortable with some breakage.

## Usage

First you'll want to add this crate to your `Cargo.toml`:

```toml
[dependencies]
wit-validator = "0.1.0"
```

This crate currently only provides the functionality to validate a WebAssembly
Interface Types custom section, and it must also be given the full wasm module.
This is moreso meant to be a sort of reference validator rather than one ready
to integrate elsewhere, since it doesn't actually expose any results of
typechecking, it just validates that the interface types section, if present, is
valid.

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
