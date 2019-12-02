use crate::TypeId;
use crate::{FuncId, WasmInterfaceTypes, WitIdsToIndices, WitIndicesToIds};
use anyhow::Result;
use id_arena::{Arena, Id};

#[derive(Debug, Default)]
pub struct Imports {
    arena: Arena<Import>,
}

#[derive(Debug)]
pub struct Import {
    id: ImportId,
    pub func: FuncId,
    pub module: String,
    pub name: String,
}

pub type ImportId = Id<Import>;

impl WasmInterfaceTypes {
    pub(crate) fn parse_imports(
        &mut self,
        imports: wit_parser::Imports,
        wids: &mut WitIndicesToIds,
    ) -> Result<()> {
        for import in imports {
            let import = import?;
            let ty = wids.ty(import.ty)?;
            let (func, _) = self.add_import_func(import.module, import.name, ty);
            wids.funcs.push(func);
        }
        Ok(())
    }

    pub(crate) fn encode_imports(
        &self,
        writer: &mut wit_writer::Writer,
        wids: &mut WitIdsToIndices,
    ) {
        let mut w = writer.imports(self.imports.arena.len() as u32);
        for import in self.imports.iter() {
            let ty = self.funcs.get(import.func).ty;
            w.add(&import.module, &import.name, wids.ty(ty));
            wids.push_func(import.func);
        }
    }

    pub fn add_import_func(&mut self, module: &str, name: &str, ty: TypeId) -> (FuncId, ImportId) {
        let func = self.funcs.add_import(ty, self.imports.arena.next_id());
        (func, self.imports.add(module, name, func))
    }
}

impl Imports {
    /// Gets a reference to an import given its id
    pub fn get(&self, id: ImportId) -> &Import {
        &self.arena[id]
    }

    /// Gets a reference to an import given its id
    pub fn get_mut(&mut self, id: ImportId) -> &mut Import {
        &mut self.arena[id]
    }

    // /// Removes an import from this module.
    // ///
    // /// It is up to you to ensure that any potential references to the deleted
    // /// import are also removed, eg `get_global` expressions.
    // pub fn delete(&mut self, id: ImportId) {
    //     self.arena.delete(id);
    // }

    /// Get a shared reference to this section's imports.
    pub fn iter(&self) -> impl Iterator<Item = &Import> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get mutable references to this section's imports.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Import> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Adds a new import to this section
    pub fn add(&mut self, module: &str, name: &str, func: FuncId) -> ImportId {
        self.arena.alloc_with_id(|id| Import {
            id,
            module: module.to_string(),
            name: name.to_string(),
            func,
        })
    }
}

impl Import {
    /// Returns the identifier for this `Import`
    pub fn id(&self) -> ImportId {
        self.id
    }
}
