//! A crate to parse the textual format for WebAssembly interface types.
//!
//! This crate is a work-in-progress and should expect to have a good deal of
//! change as the official proposal evolves. The main purpose of this crate is
//! to parse a textual file into a binary representation, and the parsing
//! includes parsing of all of the WebAssembly core types/syntax as well.

#![deny(missing_docs)]

use anyhow::{bail, Context};
use std::borrow::Cow;
use std::path::Path;
use std::str;
use wast::parser::ParseBuffer;

mod ast;
mod binary;
mod resolve;
pub use ast::*;

/// Parses a `file` on the filesystem as a textual representation of WebAssembly
/// Interface Types, returning the binary representation of the module.
///
/// Note that the `file` could either be a valid `*.wat` or `*.wasm`
/// file. In the `*.wasm` case the bytes are passed through unmodified.
///
/// # Errors
///
/// For information about errors, see the [`parse_bytes`] documentation.
pub fn parse_file(file: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    _parse_file(file.as_ref())
}

fn _parse_file(file: &Path) -> anyhow::Result<Vec<u8>> {
    let contents = std::fs::read(file).context(format!("failed to read `{}`", file.display()))?;
    match parse_bytes(&contents) {
        Ok(bytes) => Ok(bytes.into_owned()),
        Err(mut e) => {
            if let Some(e) = e.downcast_mut::<wast::Error>() {
                e.set_path(file);
            }
            Err(e)
        }
    }
}

/// Parses in-memory bytes as either the text format or a binary WebAssembly
/// module.
///
/// This function will attempt to interpret the given bytes as one of two
/// options:
///
/// * A utf-8 string which is a `*.wat` file to be parsed.
/// * A binary WebAssembly file starting with `b"\0asm"`
///
/// If the input is a string then it will be parsed as `*.wat`, and then after
/// parsing it will be encoded back into a WebAssembly binary module. If the
/// input is a binary that starts with `b"\0asm"` it will be returned verbatim.
/// Everything that doesn't start with `b"\0asm"` will be parsed as a utf-8
/// `*.wat` file, returning errors as appropriate.
///
/// # Errors
///
/// In addition to all of the errors that can be returned from [`parse_str`],
/// this function will also return an error if the input does not start with
/// `b"\0asm"` and is invalid utf-8. (failed to even try to call [`parse_str`]).
pub fn parse_bytes(bytes: &[u8]) -> anyhow::Result<Cow<'_, [u8]>> {
    if bytes.starts_with(b"\0asm") {
        return Ok(bytes.into());
    }
    let result = match str::from_utf8(bytes) {
        Ok(s) => _parse_str(s)?,
        Err(_) => bail!("input bytes aren't valid utf-8"),
    };
    Ok(result.into())
}

/// Parses an in-memory string as the text format, returning the file as a
/// binary WebAssembly file.
///
/// This function is intended to be a stable convenience function for parsing a
/// `*.wat` file into a WebAssembly binary. This is a high-level operation which
/// does not expose any parsing internals, for that you'll want to use the
/// [`Module`] type and the `wast` crate.
///
/// # Errors
///
/// This function can fail for a number of reasons, including (but not limited
/// to):
///
/// * The `wat` input may fail to lex, such as having invalid tokens or syntax
/// * The `wat` input may fail to parse, such as having incorrect syntactical
///   structure
/// * The `wat` input may contain names that could not be resolved
///
pub fn parse_str(wat: impl AsRef<str>) -> Result<Vec<u8>, wast::Error> {
    _parse_str(wat.as_ref())
}

fn _parse_str(wat: &str) -> Result<Vec<u8>, wast::Error> {
    let adjust = |mut err: wast::Error| {
        err.set_text(wat);
        err
    };
    let buf = ParseBuffer::new(&wat).map_err(adjust)?;
    let mut ast = wast::parser::parse::<Wat>(&buf).map_err(adjust)?;
    ast.module.encode().map_err(adjust)
}
