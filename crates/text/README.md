<div align="center">
  <h1><code>wit-text</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust crate to parse the <a
    href="https://github.com/webassembly/interface-types">WebAssembly
    Interface Types</a> textual format.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wit-text"><img src="https://img.shields.io/crates/v/wit-text.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wit-text"><img src="https://img.shields.io/crates/d/wit-text.svg?style=flat-square" alt="Download" /></a>
    <a href="https://bytecodealliance.github.io/wit-text/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
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
wit-text = "0.1.0"
```

You can then use the free-functions of this crate to parse a text file or
in-memory string into the binary wasm representation. Note that compatibility is
provided with the [`wast`](https://crates.io/crates/wast) crate if you'd like to
reuse the parser for other file formats as well.

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
