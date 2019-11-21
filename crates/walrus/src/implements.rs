use crate::{FuncId, WasmInterfaceTypes, WitIdsToIndices, WitIndicesToIds};
use anyhow::Result;
use id_arena::{Arena, Id};
use walrus::IndicesToIds;

#[derive(Debug, Default)]
pub struct Implements {
    arena: Arena<Implement>,
}

#[derive(Debug)]
pub struct Implement {
    id: ImplementId,
    pub adapter_func: FuncId,
    pub core_func: walrus::FunctionId,
}

pub type ImplementId = Id<Implement>;

impl WasmInterfaceTypes {
    pub(crate) fn parse_implements(
        &mut self,
        implements: wit_parser::Implements,
        ids: &IndicesToIds,
        wids: &mut WitIndicesToIds,
    ) -> Result<()> {
        for implement in implements {
            let implement = implement?;
            self.implements.add(
                wids.func(implement.adapter_func)?,
                ids.get_func(implement.core_func)?,
            );
        }

        Ok(())
    }

    pub(crate) fn encode_implements(
        &self,
        writer: &mut wit_writer::Writer,
        wids: &WitIdsToIndices,
        ids: &walrus::IdsToIndices,
    ) {
        let mut w = writer.implements(self.implements.arena.len() as u32);
        for implement in self.implements.iter() {
            w.add(
                ids.get_func_index(implement.core_func),
                wids.func(implement.adapter_func),
            );
        }
    }
}

impl Implements {
    /// Gets a reference to an implement given its id
    pub fn get(&self, id: ImplementId) -> &Implement {
        &self.arena[id]
    }

    /// Gets a reference to an implement given its id
    pub fn get_mut(&mut self, id: ImplementId) -> &mut Implement {
        &mut self.arena[id]
    }

    // /// Removes an implement from this module.
    // ///
    // /// It is up to you to ensure that any potential references to the deleted
    // /// implement are also removed, eg `get_global` expressions.
    // pub fn delete(&mut self, id: ImplementId) {
    //     self.arena.delete(id);
    // }

    /// Get a shared reference to this section's implements.
    pub fn iter(&self) -> impl Iterator<Item = &Implement> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get mutable references to this section's implements.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Implement> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Adds a new implement to this section
    pub fn add(&mut self, adapter_func: FuncId, core_func: walrus::FunctionId) -> ImplementId {
        self.arena.alloc_with_id(|id| Implement {
            id,
            core_func,
            adapter_func,
        })
    }
}

impl Implement {
    /// Returns the identifier for this `Implement`
    pub fn id(&self) -> ImplementId {
        self.id
    }
}
