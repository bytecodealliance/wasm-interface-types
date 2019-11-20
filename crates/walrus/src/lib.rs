use anyhow::{anyhow, Context, Result};
use std::borrow::Cow;
use std::collections::HashMap;
use walrus::{passes::Roots, CustomSection, IdsToIndices, IndicesToIds, Module};
use wit_schema_version::SECTION_NAME;

#[derive(Debug, Default)]
pub struct WasmInterfaceTypes {
    pub types: Types,
    pub imports: Imports,
    pub implements: Implements,
    pub exports: Exports,
    pub funcs: Funcs,
}

mod exports;
mod funcs;
mod implements;
mod imports;
mod types;
pub use self::exports::*;
pub use self::funcs::*;
pub use self::implements::*;
pub use self::imports::*;
pub use self::types::*;

impl CustomSection for WasmInterfaceTypes {
    fn name(&self) -> &str {
        SECTION_NAME
    }

    fn data(&self, indices: &IdsToIndices) -> Cow<'_, [u8]> {
        let mut writer = wit_writer::Writer::new();
        let mut wids = WitIdsToIndices::default();
        self.encode_types(&mut writer, &mut wids);
        self.encode_imports(&mut writer, &mut wids);
        self.encode_funcs(&mut writer, &mut wids, indices);
        self.encode_exports(&mut writer, &wids);
        self.encode_implements(&mut writer, &wids, indices);
        writer.into_payload().into()
    }

    fn add_gc_roots(&self, roots: &mut Roots) {
        for i in self.implements.iter() {
            roots.push_func(i.core_func);
        }
        for f in self.funcs.iter() {
            let instrs = match &f.kind {
                FuncKind::Local(instrs) => instrs,
                _ => continue,
            };
            for instr in instrs {
                match instr {
                    Instruction::CallCore(f) | Instruction::DeferCallCore(f) => {
                        roots.push_func(*f);
                    }
                    Instruction::MemoryToString(mem) => {
                        roots.push_memory(*mem);
                    }
                    Instruction::StringToMemory { mem, malloc } => {
                        roots.push_memory(*mem).push_func(*malloc);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Default)]
struct WitIndicesToIds {
    types: Vec<TypeId>,
    funcs: Vec<FuncId>,
}

impl WitIndicesToIds {
    fn ty(&self, ty: u32) -> Result<TypeId> {
        self.types
            .get(ty as usize)
            .cloned()
            .ok_or_else(|| anyhow!("adapter type index out of bounds: {}", ty))
    }

    fn func(&self, ty: u32) -> Result<FuncId> {
        self.funcs
            .get(ty as usize)
            .cloned()
            .ok_or_else(|| anyhow!("adapter func index out of bounds: {}", ty))
    }
}

#[derive(Default)]
struct WitIdsToIndices {
    types: HashMap<TypeId, u32>,
    funcs: HashMap<FuncId, u32>,
}

impl WitIdsToIndices {
    fn push_ty(&mut self, ty: TypeId) {
        self.types.insert(ty, self.types.len() as u32);
    }

    fn ty(&self, ty: TypeId) -> u32 {
        self.types
            .get(&ty)
            .cloned()
            .unwrap_or_else(|| panic!("reference to dead type found {:?}", ty))
    }

    fn push_func(&mut self, func: FuncId) {
        self.funcs.insert(func, self.funcs.len() as u32);
    }

    fn func(&self, f: FuncId) -> u32 {
        self.funcs
            .get(&f)
            .cloned()
            .unwrap_or_else(|| panic!("reference to dead function found {:?}", f))
    }
}

/// Callback for the `ModuleConfig::on_parse` function in `walrus` to act as a
/// convenience to parse the wasm interface types custom section, if present.
pub fn on_parse(module: &mut Module, ids: &IndicesToIds) -> Result<()> {
    let section = match module.customs.remove_raw(SECTION_NAME) {
        Some(s) => s,
        None => return Ok(()),
    };
    let mut parser = wit_parser::Parser::new(0, &section.data)
        .context("failed parsing wasm interface types header")?;
    let mut section = WasmInterfaceTypes::default();
    let mut wids = WitIndicesToIds::default();
    while !parser.is_empty() {
        let s = parser
            .section()
            .context("failed parsing wasm interface types section header")?;
        match s {
            wit_parser::Section::Type(t) => section.parse_types(t, &mut wids)?,
            wit_parser::Section::Import(t) => section.parse_imports(t, &mut wids)?,
            wit_parser::Section::Func(t) => section.parse_funcs(t, ids, &mut wids)?,
            wit_parser::Section::Implement(t) => section.parse_implements(t, ids, &mut wids)?,
            wit_parser::Section::Export(t) => section.parse_exports(t, &mut wids)?,
        }
    }

    module.customs.add(section);
    Ok(())
}
