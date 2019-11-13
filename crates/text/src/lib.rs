use std::path::Path;
use anyhow::Context;
use wast::parser::ParseBuffer;

mod binary;
mod resolve;
mod ast;
pub use ast::*;

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
