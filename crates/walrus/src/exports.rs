use crate::{FuncId, WasmInterfaceTypes, WitIdsToIndices, WitIndicesToIds};
use anyhow::Result;
use id_arena::{Arena, Id};

#[derive(Debug, Default)]
pub struct Exports {
    arena: Arena<Export>,
}

#[derive(Debug)]
pub struct Export {
    id: ExportId,
    pub name: String,
    pub func: FuncId,
}

pub type ExportId = Id<Export>;

impl WasmInterfaceTypes {
    pub(crate) fn parse_exports(
        &mut self,
        exports: wit_parser::Exports,
        wids: &mut WitIndicesToIds,
    ) -> Result<()> {
        for export in exports {
            let export = export?;
            let func = wids.func(export.func)?;
            self.exports.add(export.name, func);
        }
        Ok(())
    }

    pub(crate) fn encode_exports(&self, writer: &mut wit_writer::Writer, wids: &WitIdsToIndices) {
        let mut w = writer.exports(self.exports.arena.len() as u32);
        for export in self.exports.iter() {
            w.add(&export.name, wids.func(export.func));
        }
    }
}

impl Exports {
    /// Gets a reference to an export given its id
    pub fn get(&self, id: ExportId) -> &Export {
        &self.arena[id]
    }

    /// Gets a reference to an export given its id
    pub fn get_mut(&mut self, id: ExportId) -> &mut Export {
        &mut self.arena[id]
    }

    // /// Removes an export from this module.
    // ///
    // /// It is up to you to ensure that any potential references to the deleted
    // /// export are also removed, eg `get_global` expressions.
    // pub fn delete(&mut self, id: ExportId) {
    //     self.arena.delete(id);
    // }

    /// Get a shared reference to this section's exports.
    pub fn iter(&self) -> impl Iterator<Item = &Export> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get mutable references to this section's exports.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Export> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Adds a new export to this section
    pub fn add(&mut self, name: &str, func: FuncId) -> ExportId {
        self.arena.alloc_with_id(|id| Export {
            id,
            name: name.to_string(),
            func,
        })
    }
}

impl Export {
    /// Returns the identifier for this `Export`
    pub fn id(&self) -> ExportId {
        self.id
    }
}
