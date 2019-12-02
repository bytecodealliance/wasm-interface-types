<div align="center">
  <h1><code>wit-walrus</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust crate integrate <a
    href="https://github.com/webassembly/interface-types">WebAssembly
    Interface Types</a> with the <a
    href="https://crates.io/crates/walrus"><code>walrus</code></a> crate.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wit-walrus"><img src="https://img.shields.io/crates/v/wit-walrus.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wit-walrus"><img src="https://img.shields.io/crates/d/wit-walrus.svg?style=flat-square" alt="Download" /></a>
    <a href="https://bytecodealliance.github.io/wit-walrus/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
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
wit-walrus = "0.1.0"
```

You'll then want to register the `on_parse` function in this crate when parsing
a wasm blob into a `walrus` module. Afterwards you can extract the
`WasmInterfaceTypes` custom section and you should be good to go!

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
