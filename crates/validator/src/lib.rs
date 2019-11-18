use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashSet;
use wasmparser::{FuncType, ImportSectionEntryType, ModuleReader, SectionCode};
use wit_parser::*;

pub fn validate(bytes: &[u8]) -> Result<()> {
    let mut printer = ModuleReader::new(bytes)?;
    let mut validator = Validator::default();
    while !printer.eof() {
        let section = printer.read()?;
        match section.code {
            SectionCode::Type => {
                let s = section.get_type_section_reader()?;
                validator.validate_section(1, "type", s, |v, ty| {
                    v.core_types.push(ty);
                    Ok(())
                })?;
            }
            SectionCode::Import => {
                let s = section.get_import_section_reader()?;
                validator.validate_section(2, "import", s, |v, ty| {
                    if let ImportSectionEntryType::Function(ty) = ty.ty {
                        v.validate_core_type_idx(ty)?;
                        v.core_funcs.push((ty, CoreFunc::Import));
                    }
                    Ok(())
                })?;
            }
            SectionCode::Function => {
                let s = section.get_function_section_reader()?;
                validator.validate_section(3, "function", s, |v, ty| {
                    v.validate_core_type_idx(ty)?;
                    v.core_funcs.push((ty, CoreFunc::Local));
                    Ok(())
                })?;
            }
            SectionCode::Custom {
                name: "wasm-interface-types",
                ..
            } => {
                let range = section.range();
                validator
                    .validate(&bytes[range.start..range.end])
                    .context("failed to validate interface types section")?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Default)]
struct Validator<'a> {
    visited: bool,
    last_order: u8,
    types: Vec<Type>,
    func: Vec<u32>,
    exports: HashSet<&'a str>,
    core_types: Vec<FuncType>,
    core_funcs: Vec<(u32, CoreFunc)>,
}

enum CoreFunc {
    Import,
    Local,
}

impl<'a> Validator<'a> {
    fn validate(&mut self, bytes: &'a [u8]) -> Result<()> {
        if self.visited {
            bail!("found two `wasm-interface-types` custom sections");
        }
        self.visited = true;

        let mut parser = Parser::new(bytes).context("failed to parse interface types header")?;

        while !parser.is_empty() {
            match parser.section().context("failed to read section header")? {
                Section::Type(s) => {
                    self.validate_section(4, "adapter type", s, Self::validate_type)?
                }
                Section::Import(s) => {
                    self.validate_section(5, "adapter import", s, Self::validate_import)?
                }
                Section::Func(s) => {
                    self.validate_section(6, "adapter func", s, Self::validate_func)?
                }
                Section::Export(s) => {
                    self.validate_section(7, "adapter export", s, Self::validate_export)?
                }
                Section::Implement(s) => {
                    self.validate_section(8, "adapter implement", s, Self::validate_implement)?
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
        self.validate_adapter_type_idx(func.ty)?;
        self.func.push(func.ty);
        // TODO: validate instructions
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
                .any(|(a, b)| !tys_match(a, b))
            || adapter_ty.results.len() != core_ty.returns.len()
            || adapter_ty
                .results
                .iter()
                .zip(core_ty.returns.iter())
                .any(|(a, b)| !tys_match(a, b))
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

fn tys_match(a: &wit_parser::ValType, b: &wasmparser::Type) -> bool {
    match (a, b) {
        (wit_parser::ValType::S32, wasmparser::Type::I32) |
        (wit_parser::ValType::S64, wasmparser::Type::I64) |
        (wit_parser::ValType::F32, wasmparser::Type::F32) |
        (wit_parser::ValType::F64, wasmparser::Type::F64) => true,
        _ => false,
    }
}

