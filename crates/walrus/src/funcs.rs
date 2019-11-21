use crate::WitIdsToIndices;
use crate::{ImportId, TypeId, ValType, WasmInterfaceTypes, WitIndicesToIds};
use anyhow::Result;
use id_arena::{Arena, Id};
use walrus::IndicesToIds;

#[derive(Debug, Default)]
pub struct Funcs {
    arena: Arena<Func>,
}

#[derive(Debug)]
pub struct Func {
    id: FuncId,
    pub ty: TypeId,
    pub kind: FuncKind,
}

#[derive(Debug)]
pub enum FuncKind {
    Import(ImportId),
    Local(Vec<Instruction>),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    CallCore(walrus::FunctionId),
    DeferCallCore(walrus::FunctionId),
    CallAdapter(FuncId),
    ArgGet(u32),
    MemoryToString(walrus::MemoryId),
    StringToMemory {
        mem: walrus::MemoryId,
        malloc: walrus::FunctionId,
    },
    IntToWasm {
        input: ValType,
        output: walrus::ValType,
        trap: bool,
    },
    WasmToInt {
        input: walrus::ValType,
        output: ValType,
        trap: bool,
    },
}

pub type FuncId = Id<Func>;

impl WasmInterfaceTypes {
    pub(crate) fn parse_funcs(
        &mut self,
        funcs: wit_parser::Funcs,
        ids: &IndicesToIds,
        wids: &mut WitIndicesToIds,
    ) -> Result<()> {
        // assign an id to everything first ...
        let mut instrs = Vec::new();
        for func in funcs {
            let func = func?;
            let ty = wids.ty(func.ty)?;
            let id = self.funcs.add_local(ty, Vec::new());
            wids.funcs.push(id);
            instrs.push((id, func.instrs()));
        }

        // ... and then parse all the instructions
        for (id, instrs) in instrs {
            let mut list = Vec::new();
            for instr in instrs {
                use wit_parser::Instruction as R; // "raw"
                use Instruction as W; // "walrus"
                use walrus::ValType as RVT; // "raw value type"
                use crate::ValType as VT;

                fn w2i(input: walrus::ValType, output: ValType, trap: bool) -> W {
                    W::WasmToInt { input, output, trap }
                }

                fn i2w(input: ValType, output: walrus::ValType, trap: bool) -> W {
                    W::IntToWasm { input, output, trap }
                }

                list.push(match instr? {
                    R::CallCore(id) => W::CallCore(ids.get_func(id)?),
                    R::CallAdapter(id) => W::CallAdapter(wids.func(id)?),
                    R::ArgGet(idx) => W::ArgGet(idx),
                    R::MemoryToString(mem) => W::MemoryToString(ids.get_memory(mem)?),
                    R::StringToMemory(args) => W::StringToMemory {
                        mem: ids.get_memory(args.mem)?,
                        malloc: ids.get_func(args.malloc)?,
                    },
                    R::DeferCallCore(id) => W::DeferCallCore(ids.get_func(id)?),

                    R::I32ToS8 => w2i(RVT::I32, VT::S8, false),
                    R::I32ToS8X => w2i(RVT::I32, VT::S8, true),
                    R::I32ToU8 => w2i(RVT::I32, VT::U8, false),
                    R::I32ToS16 => w2i(RVT::I32, VT::S16, false),
                    R::I32ToS16X => w2i(RVT::I32, VT::S16, true),
                    R::I32ToU16 => w2i(RVT::I32, VT::U16, false),
                    R::I32ToS32 => w2i(RVT::I32, VT::S32, false),
                    R::I32ToU32 => w2i(RVT::I32, VT::U32, false),
                    R::I32ToS64 => w2i(RVT::I32, VT::S64, false),
                    R::I32ToU64 => w2i(RVT::I32, VT::U64, false),

                    R::I64ToS8 => w2i(RVT::I64, VT::S8, false),
                    R::I64ToS8X => w2i(RVT::I64, VT::S8, true),
                    R::I64ToU8 => w2i(RVT::I64, VT::U8, false),
                    R::I64ToS16 => w2i(RVT::I64, VT::S16, false),
                    R::I64ToS16X => w2i(RVT::I64, VT::S16, true),
                    R::I64ToU16 => w2i(RVT::I64, VT::U16, false),
                    R::I64ToS32 => w2i(RVT::I64, VT::S32, false),
                    R::I64ToS32X => w2i(RVT::I64, VT::S32, true),
                    R::I64ToU32 => w2i(RVT::I64, VT::U32, false),
                    R::I64ToS64 => w2i(RVT::I64, VT::S64, false),
                    R::I64ToU64 => w2i(RVT::I64, VT::U64, false),

                    R::S8ToI32 => i2w(VT::S8, RVT::I32, false),
                    R::U8ToI32 => i2w(VT::U8, RVT::I32, false),
                    R::S16ToI32 => i2w(VT::S16, RVT::I32, false),
                    R::U16ToI32 => i2w(VT::U16, RVT::I32, false),
                    R::S32ToI32 => i2w(VT::S32, RVT::I32, false),
                    R::U32ToI32 => i2w(VT::U32, RVT::I32, false),
                    R::S64ToI32 => i2w(VT::S64, RVT::I32, false),
                    R::S64ToI32X => i2w(VT::S64, RVT::I32, true),
                    R::U64ToI32 => i2w(VT::U64, RVT::I32, false),
                    R::U64ToI32X => i2w(VT::U64, RVT::I32, true),

                    R::S8ToI64 => i2w(VT::S8, RVT::I64, false),
                    R::U8ToI64 => i2w(VT::U8, RVT::I64, false),
                    R::S16ToI64 => i2w(VT::S16, RVT::I64, false),
                    R::U16ToI64 => i2w(VT::U16, RVT::I64, false),
                    R::S32ToI64 => i2w(VT::S32, RVT::I64, false),
                    R::U32ToI64 => i2w(VT::U32, RVT::I64, false),
                    R::S64ToI64 => i2w(VT::S64, RVT::I64, false),
                    R::U64ToI64 => i2w(VT::U64, RVT::I64, false),

                    R::End => continue,
                });
            }

            match &mut self.funcs.arena.get_mut(id).unwrap().kind {
                FuncKind::Local(i) => *i = list,
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    pub(crate) fn encode_funcs(
        &self,
        writer: &mut wit_writer::Writer,
        wids: &mut WitIdsToIndices,
        ids: &walrus::IdsToIndices,
    ) {
        // Filter out imported functions since those went in the import section
        let funcs = self
            .funcs
            .iter()
            .filter_map(|f| match &f.kind {
                FuncKind::Local(instrs) => Some((f, instrs)),
                FuncKind::Import(_) => None,
            })
            .collect::<Vec<_>>();

        // Assign an index for all functions first so inter-function references
        // work.
        for (func, _) in funcs.iter() {
            wids.push_func(func.id());
        }

        let mut w = writer.funcs(funcs.len() as u32);
        for (func, instrs) in funcs {
            let mut w = w.add(wids.ty(func.ty));
            for instr in instrs {
                use Instruction::*;
                match *instr {
                    ArgGet(n) => w.arg_get(n),
                    CallCore(f) => w.call_core(ids.get_func_index(f)),
                    DeferCallCore(f) => w.defer_call_core(ids.get_func_index(f)),
                    CallAdapter(f) => w.call_adapter(wids.func(f)),
                    MemoryToString(m) => w.memory_to_string(ids.get_memory_index(m)),
                    StringToMemory { mem, malloc } => {
                        w.string_to_memory(ids.get_func_index(malloc), ids.get_memory_index(mem));
                    }
                    IntToWasm {
                        input,
                        output,
                        trap,
                    } => i2w(&mut w, input, output, trap),
                    WasmToInt {
                        input,
                        output,
                        trap,
                    } => w2i(&mut w, input, output, trap),
                }
            }
        }
        fn w2i(
            w: &mut wit_writer::Instructions<'_, '_>,
            input: walrus::ValType,
            output: ValType,
            trap: bool,
        ) {
            match (input, output, trap) {
                (walrus::ValType::I32, ValType::S8, false) => w.i32_to_s8(),
                (walrus::ValType::I32, ValType::S8, true) => w.i32_to_s8x(),
                (walrus::ValType::I32, ValType::U8, _) => w.i32_to_u8(),
                (walrus::ValType::I32, ValType::S16, false) => w.i32_to_s16(),
                (walrus::ValType::I32, ValType::S16, true) => w.i32_to_s16x(),
                (walrus::ValType::I32, ValType::U16, _) => w.i32_to_u16(),
                (walrus::ValType::I32, ValType::S32, _) => w.i32_to_s32(),
                (walrus::ValType::I32, ValType::U32, _) => w.i32_to_u32(),
                (walrus::ValType::I32, ValType::S64, _) => w.i32_to_s64(),
                (walrus::ValType::I32, ValType::U64, _) => w.i32_to_u64(),

                (walrus::ValType::I64, ValType::S8, false) => w.i64_to_s8(),
                (walrus::ValType::I64, ValType::S8, true) => w.i64_to_s8x(),
                (walrus::ValType::I64, ValType::U8, _) => w.i64_to_u8(),
                (walrus::ValType::I64, ValType::S16, false) => w.i64_to_s16(),
                (walrus::ValType::I64, ValType::S16, true) => w.i64_to_s16x(),
                (walrus::ValType::I64, ValType::U16, _) => w.i64_to_u16(),
                (walrus::ValType::I64, ValType::S32, false) => w.i64_to_s32(),
                (walrus::ValType::I64, ValType::S32, true) => w.i64_to_s32x(),
                (walrus::ValType::I64, ValType::U32, _) => w.i64_to_u32(),
                (walrus::ValType::I64, ValType::S64, _) => w.i64_to_s64(),
                (walrus::ValType::I64, ValType::U64, _) => w.i64_to_u64(),

                _ => unreachable!(),
            }
        }

        fn i2w(
            w: &mut wit_writer::Instructions<'_, '_>,
            input: ValType,
            output: walrus::ValType,
            trap: bool,
        ) {
            match (input, output, trap) {
                (ValType::S8, walrus::ValType::I32, _) => w.s8_to_i32(),
                (ValType::U8, walrus::ValType::I32, _) => w.u8_to_i32(),
                (ValType::S16, walrus::ValType::I32, _) => w.s16_to_i32(),
                (ValType::U16, walrus::ValType::I32, _) => w.u16_to_i32(),
                (ValType::S32, walrus::ValType::I32, _) => w.s32_to_i32(),
                (ValType::U32, walrus::ValType::I32, _) => w.u32_to_i32(),
                (ValType::S64, walrus::ValType::I32, false) => w.s64_to_i32(),
                (ValType::S64, walrus::ValType::I32, true) => w.s64_to_i32x(),
                (ValType::U64, walrus::ValType::I32, false) => w.u64_to_i32(),
                (ValType::U64, walrus::ValType::I32, true) => w.u64_to_i32x(),

                (ValType::S8, walrus::ValType::I64, _) => w.s8_to_i64(),
                (ValType::U8, walrus::ValType::I64, _) => w.u8_to_i64(),
                (ValType::S16, walrus::ValType::I64, _) => w.s16_to_i64(),
                (ValType::U16, walrus::ValType::I64, _) => w.u16_to_i64(),
                (ValType::S32, walrus::ValType::I64, _) => w.s32_to_i64(),
                (ValType::U32, walrus::ValType::I64, _) => w.u32_to_i64(),
                (ValType::S64, walrus::ValType::I64, _) => w.s64_to_i64(),
                (ValType::U64, walrus::ValType::I64, _) => w.u64_to_i64(),

                _ => unreachable!(),
            }
        }
    }
}

impl Funcs {
    /// Gets a reference to an func given its id
    pub fn get(&self, id: FuncId) -> &Func {
        &self.arena[id]
    }

    /// Gets a reference to an func given its id
    pub fn get_mut(&mut self, id: FuncId) -> &mut Func {
        &mut self.arena[id]
    }

    // /// Removes an func from this module.
    // ///
    // /// It is up to you to ensure that any potential references to the deleted
    // /// func are also removed, eg `get_global` expressions.
    // pub fn delete(&mut self, id: FuncId) {
    //     self.arena.delete(id);
    // }

    /// Get a shared reference to this section's funcs.
    pub fn iter(&self) -> impl Iterator<Item = &Func> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get mutable references to this section's funcs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Func> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Create a new externally defined, imported function.
    pub fn add_import(&mut self, ty: TypeId, import: ImportId) -> FuncId {
        self.arena.alloc_with_id(|id| Func {
            id,
            ty,
            kind: FuncKind::Import(import),
        })
    }

    /// Adds a new local func to this section
    pub fn add_local(&mut self, ty: TypeId, instrs: Vec<Instruction>) -> FuncId {
        self.arena.alloc_with_id(|id| Func {
            id,
            ty,
            kind: FuncKind::Local(instrs),
        })
    }
}

impl Func {
    /// Returns the identifier for this `Func`
    pub fn id(&self) -> FuncId {
        self.id
    }
}
