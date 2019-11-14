//! A crate to parse the textual format for WebAssembly interface types.
//!
//! This crate is a work-in-progress and should expect to have a good deal of
//! change as the official proposal evolves. The main purpose of this crate is
//! to parse a textual file into a binary representation, and the parsing
//! includes parsing of all of the WebAssembly core types/syntax as well.

#![deny(missing_docs)]

use anyhow::Context;
use std::path::Path;
use wast::parser::ParseBuffer;

mod ast;
mod binary;
mod resolve;
pub use ast::*;

/// Parses a `file` on the filesystem as a textual representation of WebAssembly
/// Interface Types, returning the binary representation of the module.
pub fn parse_file(file: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    _parse_file(file.as_ref())
}

fn _parse_file(file: &Path) -> anyhow::Result<Vec<u8>> {
    let contents = std::fs::read_to_string(file)
        .context(format!("failed to read `{}` to a string", file.display()))?;
    match _parse_str(&contents) {
        Ok(bytes) => Ok(bytes),
        Err(mut e) => {
            e.set_path(file);
            Err(e.into())
        }
    }
}

/// Parses an in-memory string as the textual representation of WebAssembly
/// interface types and returned the binary serialization of the module.
pub fn parse_str(wat: impl AsRef<str>) -> Result<Vec<u8>, wast::Error> {
    _parse_str(wat.as_ref())
}

fn _parse_str(wat: &str) -> Result<Vec<u8>, wast::Error> {
    let adjust = |mut err: wast::Error| {
        err.set_text(wat);
        err
    };
    let buf = ParseBuffer::new(&wat).map_err(adjust)?;
    let mut ast = wast::parser::parse::<Wit>(&buf).map_err(adjust)?;
    ast.module.encode().map_err(adjust)
}
