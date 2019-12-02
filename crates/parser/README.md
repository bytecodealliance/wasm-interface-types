<div align="center">
  <h1><code>wit-parser</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust crate to parse the <a
    href="https://github.com/webassembly/interface-types">WebAssembly
    Interface Types</a> binary format.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wit-parser"><img src="https://img.shields.io/crates/v/wit-parser.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wit-parser"><img src="https://img.shields.io/crates/d/wit-parser.svg?style=flat-square" alt="Download" /></a>
    <a href="https://bytecodealliance.github.io/wit-parser/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
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
wit-parser = "0.1.0"
```

You'll likely want to pair this with the
[`wasmparser`](https://crates.io/crates/wasmparser) crate to parse a full
WebAssembly file. This crate only contains the ability to parse the contents of
the binary WebAssembly Interface Types section.

You should be able to get started with a
[`Parser`](https://docs.rs/wit-parser/*/wit_parser/struct.Parser.html) and
parsing sections of the binary format. You can find more about the binary
format [in some documentation](../../BINARY.md).

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
