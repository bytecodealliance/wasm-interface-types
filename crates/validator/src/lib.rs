use anyhow::{bail, Result};
use wasmparser::{ModuleReader, SectionCode};

pub fn validate(bytes: &[u8]) -> Result<()> {
    let mut printer = ModuleReader::new(bytes)?;
    let mut validator = Validator::default();
    while !printer.eof() {
        let section = printer.read()?;
        match section.code {
            SectionCode::Custom { name: "wasm-interface-types", .. } => {
                let range = section.range();
                validator.validate(&bytes[range.start..range.end])?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Default)]
struct Validator {
    visited: bool,
}

impl Validator {
    fn validate(&mut self, bytes: &[u8]) -> Result<()> {
        if self.visited {
            bail!("found two `wasm-interface-types` custom sections");
        }
        self.visited = true;
        Ok(())
    }
}
