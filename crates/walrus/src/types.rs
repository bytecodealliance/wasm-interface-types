use crate::{WasmInterfaceTypes, WitIdsToIndices, WitIndicesToIds};
use anyhow::Result;
use id_arena::{Arena, Id};

#[derive(Debug, Default)]
pub struct Types {
    arena: Arena<Type>,
}

#[derive(Debug)]
pub struct Type {
    id: TypeId,
    params: Box<[ValType]>,
    results: Box<[ValType]>,
}

pub type TypeId = Id<Type>;

#[derive(Debug, Copy, Clone)]
pub enum ValType {
    S8,
    S16,
    S32,
    S64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Externref,
    I32,
    I64,
}

impl WasmInterfaceTypes {
    pub(crate) fn parse_types(
        &mut self,
        types: wit_parser::Types,
        wids: &mut WitIndicesToIds,
    ) -> Result<()> {
        for ty in types {
            let ty = ty?;
            let id = self.types.add(
                ty.params.iter().cloned().map(parse2walrus).collect(),
                ty.results.iter().cloned().map(parse2walrus).collect(),
            );
            wids.types.push(id);
        }
        Ok(())
    }

    pub(crate) fn encode_types(&self, writer: &mut wit_writer::Writer, wids: &mut WitIdsToIndices) {
        let mut w = writer.types(self.types.arena.len() as u32);
        for (id, ty) in self.types.arena.iter() {
            w.add(
                ty.params.len() as u32,
                |w| {
                    for param in ty.params.iter() {
                        write_ty(w, param);
                    }
                },
                ty.results.len() as u32,
                |w| {
                    for result in ty.results.iter() {
                        write_ty(w, result);
                    }
                },
            );
            wids.push_ty(id);
        }

        fn write_ty(w: &mut wit_writer::Type<'_>, ty: &ValType) {
            match ty {
                ValType::S8 => w.s8(),
                ValType::S16 => w.s16(),
                ValType::S32 => w.s32(),
                ValType::S64 => w.s64(),
                ValType::U8 => w.u8(),
                ValType::U16 => w.u16(),
                ValType::U32 => w.u32(),
                ValType::U64 => w.u64(),
                ValType::F32 => w.f32(),
                ValType::F64 => w.f64(),
                ValType::String => w.string(),
                ValType::Externref => w.externref(),
                ValType::I32 => w.i32(),
                ValType::I64 => w.i64(),
            }
        }
    }
}

fn parse2walrus(parse: wit_parser::ValType) -> ValType {
    match parse {
        wit_parser::ValType::S8 => ValType::S8,
        wit_parser::ValType::S16 => ValType::S16,
        wit_parser::ValType::S32 => ValType::S32,
        wit_parser::ValType::S64 => ValType::S64,
        wit_parser::ValType::U8 => ValType::U8,
        wit_parser::ValType::U16 => ValType::U16,
        wit_parser::ValType::U32 => ValType::U32,
        wit_parser::ValType::U64 => ValType::U64,
        wit_parser::ValType::F32 => ValType::F32,
        wit_parser::ValType::F64 => ValType::F64,
        wit_parser::ValType::String => ValType::String,
        wit_parser::ValType::Externref => ValType::Externref,
        wit_parser::ValType::I32 => ValType::I32,
        wit_parser::ValType::I64 => ValType::I64,
    }
}

impl Types {
    /// Gets a reference to an type given its id
    pub fn get(&self, id: TypeId) -> &Type {
        &self.arena[id]
    }

    /// Gets a reference to an type given its id
    pub fn get_mut(&mut self, id: TypeId) -> &mut Type {
        &mut self.arena[id]
    }

    // /// Removes an type from this module.
    // ///
    // /// It is up to you to ensure that any potential references to the deleted
    // /// type are also removed, eg `get_global` expressions.
    // pub fn delete(&mut self, id: TypeId) {
    //     self.arena.delete(id);
    // }

    /// Get a shared reference to this section's types.
    pub fn iter(&self) -> impl Iterator<Item = &Type> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get mutable references to this section's types.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Type> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Adds a new type to this section
    pub fn add(&mut self, params: Vec<ValType>, results: Vec<ValType>) -> TypeId {
        self.arena.alloc_with_id(|id| Type {
            id,
            params: params.into_boxed_slice(),
            results: results.into_boxed_slice(),
        })
    }
}

impl Type {
    /// Returns the identifier for this `Type`
    pub fn id(&self) -> TypeId {
        self.id
    }

    /// Returns parameters of this function type
    pub fn params(&self) -> &[ValType] {
        &self.params
    }

    /// Returns results of this function type
    pub fn results(&self) -> &[ValType] {
        &self.results
    }
}
