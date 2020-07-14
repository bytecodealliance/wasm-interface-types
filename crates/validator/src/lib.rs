//! A validator for the wasm interface types binary format.
//!
//! This crate currently only provides the ability to validate a full in-memory
//! wasm module. It does not currently expose the results of typechecking, only
//! if a wasm binary typechecks or not.

#![deny(missing_docs)]

use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashSet;
use std::mem;
use wasmparser::{FuncType, ImportSectionEntryType, Payload, TypeDef};
use wit_parser::*;

/// Validates an entire WebAssembly module listed by `bytes`
///
/// The `bytes` given must be an entire WebAssembly module, not just the
/// interface types custom section. This will validate only the wasm interface
/// types custom section, it will only perform minimal validation of the rest of
/// the module as needed. For example core wasm functions aren't typechecked
/// here.
pub fn validate(bytes: &[u8]) -> Result<()> {
    let mut validator = Validator::default();
    for payload in wasmparser::Parser::new(0).parse_all(bytes) {
        match payload? {
            Payload::TypeSection(s) => {
                validator.validate_section(1, "type", s, |v, ty| {
                    if let TypeDef::Func(ty) = ty {
                        v.core_types.push(ty);
                    }
                    Ok(())
                })?;
            }
            Payload::ImportSection(s) => {
                validator.validate_section(2, "import", s, |v, ty| {
                    match ty.ty {
                        ImportSectionEntryType::Function(ty) => {
                            v.validate_core_type_idx(ty)?;
                            v.core_funcs.push((ty, CoreFunc::Import));
                        }
                        ImportSectionEntryType::Memory(_) => {
                            v.memories += 1;
                        }
                        _ => {}
                    }
                    Ok(())
                })?;
            }
            Payload::FunctionSection(s) => {
                validator.validate_section(3, "function", s, |v, ty| {
                    v.validate_core_type_idx(ty)?;
                    v.core_funcs.push((ty, CoreFunc::Local));
                    Ok(())
                })?;
            }
            Payload::MemorySection(s) => {
                validator.validate_section(4, "memory", s, |v, _| {
                    v.memories += 1;
                    Ok(())
                })?;
            }
            Payload::CustomSection {
                name: wit_schema_version::SECTION_NAME,
                data,
                data_offset,
            } => {
                validator
                    .validate_wit_custom_section(data_offset, data)
                    .context("failed to validate interface types section")?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// A validator for the wasm interface types section.
///
/// This structure is used to visit *just* the wasm interface types subsection,
/// if it's already been parsed out.
#[derive(Default)]
struct Validator<'a> {
    visited: bool,
    last_order: u8,
    memories: u32,
    types: Vec<Type>,
    func: Vec<u32>,
    exports: HashSet<&'a str>,
    core_types: Vec<FuncType>,
    core_funcs: Vec<(u32, CoreFunc)>,
    type_stack: Vec<ValType>,
}

enum CoreFunc {
    Import,
    Local,
}

impl<'a> Validator<'a> {
    /// Validates the wasm interface types custom section given.
    ///
    /// The `offset` given is the offset within the file that `bytes` was found
    /// at. This is purely used for error messages. The `bytes` given must be
    /// the entire contents of the wasm interface types section.
    fn validate_wit_custom_section(&mut self, offset: usize, bytes: &'a [u8]) -> Result<()> {
        if self.visited {
            bail!("found two `wasm-interface-types` custom sections");
        }
        self.visited = true;

        let mut parser =
            Parser::new(offset, bytes).context("failed to parse interface types header")?;

        while !parser.is_empty() {
            match parser.section().context("failed to read section header")? {
                Section::Type(s) => {
                    self.validate_section(100, "adapter type", s, Self::validate_type)?
                }
                Section::Import(s) => {
                    self.validate_section(101, "adapter import", s, Self::validate_import)?
                }
                Section::Func(s) => {
                    self.validate_section(102, "adapter func", s, Self::validate_func)?
                }
                Section::Export(s) => {
                    self.validate_section(103, "adapter export", s, Self::validate_export)?
                }
                Section::Implement(s) => {
                    self.validate_section(104, "adapter implement", s, Self::validate_implement)?
                }
            }
        }
        Ok(())
    }

    fn validate_section<S, T, E>(
        &mut self,
        id: u8,
        name: &str,
        iter: S,
        mut validate: impl FnMut(&mut Self, T) -> Result<()>,
    ) -> Result<()>
    where
        S: IntoIterator<Item = Result<T, E>>,
        E: Into<anyhow::Error>,
    {
        if id <= self.last_order {
            bail!("found `{}` section but was out of order", name);
        }
        self.last_order = id;
        for (i, item) in iter.into_iter().enumerate() {
            let item = item
                .map_err(|e| e.into())
                .with_context(|| format!("failed to parse {} {}", name, i))?;
            validate(self, item).with_context(|| format!("failed to validate {} {}", name, i))?;
        }
        Ok(())
    }

    fn validate_type(&mut self, ty: Type) -> Result<()> {
        self.types.push(ty);
        Ok(())
    }

    fn validate_import(&mut self, import: Import<'a>) -> Result<()> {
        self.validate_adapter_type_idx(import.ty)?;
        self.func.push(import.ty);
        Ok(())
    }

    fn validate_func(&mut self, func: Func<'a>) -> Result<()> {
        let mut type_stack = mem::replace(&mut self.type_stack, Vec::new());
        self.func.push(func.ty);
        let ty = self.validate_adapter_type_idx(func.ty)?;

        for instr in func.instrs() {
            self.validate_instr(instr?, &ty.params, &mut type_stack)?;
        }
        for result in ty.results.iter() {
            self.expect_interface(*result, &mut type_stack)?;
        }
        if !type_stack.is_empty() {
            bail!("value stack isn't empty on function exit");
        }
        self.type_stack = type_stack;
        return Ok(());
    }

    fn validate_instr(
        &self,
        instr: Instruction,
        params: &[ValType],
        stack: &mut Vec<ValType>,
    ) -> Result<()> {
        use Instruction::*;
        match instr {
            ArgGet(idx) => {
                let ty = params
                    .get(idx as usize)
                    .ok_or_else(|| anyhow!("parameter index out of bounds: {}", idx))?;
                stack.push(*ty);
            }
            CallCore(idx) => {
                let ty = self.validate_core_func_idx(idx)?.0;
                for param in ty.params.iter().rev() {
                    self.expect_wasm(*param, stack)?;
                }
                for result in ty.returns.iter() {
                    stack.push(wasm2adapter(*result)?);
                }
            }
            MemoryToString(mem) => {
                if mem >= self.memories {
                    bail!("memory index out of bounds: {}", mem);
                }
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::String);
            }
            StringToMemory(args) => {
                if args.mem >= self.memories {
                    bail!("memory index out of bounds: {}", args.mem);
                }
                let ty = self.validate_core_func_idx(args.malloc)?.0;
                if &*ty.params != [wasmparser::Type::I32] || &*ty.returns != [wasmparser::Type::I32]
                {
                    bail!(
                        "malloc function {} does not have correct signature",
                        args.malloc
                    );
                }
                self.expect_interface(ValType::String, stack)?;
                stack.push(ValType::I32);
                stack.push(ValType::I32);
            }
            CallAdapter(idx) => {
                let ty = self.validate_adapter_func_idx(idx)?;
                for param in ty.params.iter().rev() {
                    self.expect_interface(*param, stack)?;
                }
                for result in ty.results.iter() {
                    stack.push(*result);
                }
            }
            DeferCallCore(idx) => {
                let ty = self.validate_core_func_idx(idx)?.0;
                if ty.returns.len() > 0 {
                    bail!("cannot have returned values in deferred calls");
                }
                // Make sure everything on the stack is right...
                for param in ty.params.iter() {
                    self.expect_wasm(*param, stack)?;
                }
                // ... but don't actually consume it.
                for param in ty.params.iter().rev() {
                    stack.push(wasm2adapter(*param)?);
                }
            }
            End => bail!("extra `end` instruction found"),

            I32ToS8 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S8);
            }
            I32ToS8X => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S8);
            }
            I32ToU8 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::U8);
            }
            I32ToS16 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S16);
            }
            I32ToS16X => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S16);
            }
            I32ToU16 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::U16);
            }
            I32ToS32 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S32);
            }
            I32ToU32 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::U32);
            }
            I32ToS64 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::S64);
            }
            I32ToU64 => {
                self.expect_wasm(wasmparser::Type::I32, stack)?;
                stack.push(ValType::U64);
            }

            I64ToS8 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S8);
            }
            I64ToS8X => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S8);
            }
            I64ToU8 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::U8);
            }
            I64ToS16 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S16);
            }
            I64ToS16X => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S16);
            }
            I64ToU16 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::U16);
            }
            I64ToS32 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S32);
            }
            I64ToS32X => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S32);
            }
            I64ToU32 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::U32);
            }
            I64ToS64 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::S64);
            }
            I64ToU64 => {
                self.expect_wasm(wasmparser::Type::I64, stack)?;
                stack.push(ValType::U64);
            }

            S8ToI32 => {
                self.expect_interface(ValType::S8, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            U8ToI32 => {
                self.expect_interface(ValType::U8, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            S16ToI32 => {
                self.expect_interface(ValType::S16, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            U16ToI32 => {
                self.expect_interface(ValType::U16, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            S32ToI32 => {
                self.expect_interface(ValType::S32, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            U32ToI32 => {
                self.expect_interface(ValType::U32, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            S64ToI32 => {
                self.expect_interface(ValType::S64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            S64ToI32X => {
                self.expect_interface(ValType::S64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            U64ToI32 => {
                self.expect_interface(ValType::U64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }
            U64ToI32X => {
                self.expect_interface(ValType::U64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I32)?);
            }

            S8ToI64 => {
                self.expect_interface(ValType::S8, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            U8ToI64 => {
                self.expect_interface(ValType::U8, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            S16ToI64 => {
                self.expect_interface(ValType::S16, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            U16ToI64 => {
                self.expect_interface(ValType::U16, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            S32ToI64 => {
                self.expect_interface(ValType::S32, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            U32ToI64 => {
                self.expect_interface(ValType::U32, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            S64ToI64 => {
                self.expect_interface(ValType::S64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
            U64ToI64 => {
                self.expect_interface(ValType::U64, stack)?;
                stack.push(wasm2adapter(wasmparser::Type::I64)?);
            }
        }
        Ok(())
    }

    fn expect_wasm(&self, expected: wasmparser::Type, stack: &mut Vec<ValType>) -> Result<()> {
        let actual = match stack.pop() {
            Some(t) => t,
            None => bail!("expected {:?} on type stack, found nothing", expected),
        };
        if !tys_match(actual, expected) {
            bail!("expected {:?} on type stack, found {:?}", expected, actual);
        }
        Ok(())
    }

    fn expect_interface(&self, expected: ValType, stack: &mut Vec<ValType>) -> Result<()> {
        let actual = match stack.pop() {
            Some(t) => t,
            None => bail!("expected {:?} on type stack, found nothing", expected),
        };
        if expected != actual {
            bail!("expected {:?} on type stack, found {:?}", expected, actual);
        }
        Ok(())
    }

    fn validate_export(&mut self, export: Export<'a>) -> Result<()> {
        self.validate_adapter_func_idx(export.func)?;
        if !self.exports.insert(export.name) {
            bail!("found duplicate export `{}`", export.name);
        }
        Ok(())
    }

    fn validate_implement(&mut self, i: Implement) -> Result<()> {
        let adapter_ty = self.validate_adapter_func_idx(i.adapter_func)?;
        let (core_ty, kind) = self.validate_core_func_idx(i.core_func)?;
        match kind {
            CoreFunc::Import => {}
            CoreFunc::Local => {
                bail!(
                    "implement directive must be connected to imported \
                     function in the core module"
                );
            }
        }

        if adapter_ty.params.len() != core_ty.params.len()
            || adapter_ty
                .params
                .iter()
                .zip(core_ty.params.iter())
                .any(|(a, b)| !tys_match(*a, *b))
            || adapter_ty.results.len() != core_ty.returns.len()
            || adapter_ty
                .results
                .iter()
                .zip(core_ty.returns.iter())
                .any(|(a, b)| !tys_match(*a, *b))
        {
            bail!(
                "core function {} has a different type signature \
                 than adapter function {}",
                i.core_func,
                i.adapter_func
            );
        }
        Ok(())
    }

    fn validate_core_type_idx(&self, ty: u32) -> Result<&FuncType> {
        self.core_types
            .get(ty as usize)
            .ok_or_else(|| anyhow!("type index too large: {}", ty))
    }

    fn validate_adapter_type_idx(&self, ty: u32) -> Result<&Type> {
        self.types
            .get(ty as usize)
            .ok_or_else(|| anyhow!("adapter type index too large: {}", ty))
    }

    fn validate_adapter_func_idx(&self, ty: u32) -> Result<&Type> {
        let ty = self
            .func
            .get(ty as usize)
            .ok_or_else(|| anyhow!("adapter func index too large: {}", ty))?;
        self.validate_adapter_type_idx(*ty)
    }

    fn validate_core_func_idx(&self, ty: u32) -> Result<(&FuncType, &CoreFunc)> {
        let (ty, kind) = self
            .core_funcs
            .get(ty as usize)
            .ok_or_else(|| anyhow!("func index too large: {}", ty))?;
        Ok((self.validate_core_type_idx(*ty)?, kind))
    }
}

fn tys_match(a: ValType, b: wasmparser::Type) -> bool {
    match (a, b) {
        (ValType::I32, wasmparser::Type::I32)
        | (ValType::I64, wasmparser::Type::I64)
        | (ValType::F32, wasmparser::Type::F32)
        | (ValType::F64, wasmparser::Type::F64)
        | (ValType::Externref, wasmparser::Type::ExternRef) => true,
        _ => false,
    }
}

fn wasm2adapter(a: wasmparser::Type) -> Result<ValType> {
    Ok(match a {
        wasmparser::Type::I32 => ValType::I32,
        wasmparser::Type::I64 => ValType::I64,
        wasmparser::Type::F32 => ValType::F32,
        wasmparser::Type::F64 => ValType::F64,
        wasmparser::Type::ExternRef => ValType::Externref,
        _ => bail!("currently {:?} is not a valid wasm interface type", a),
    })
}
