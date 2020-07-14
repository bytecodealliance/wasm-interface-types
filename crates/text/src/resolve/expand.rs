use crate::ast::*;
use std::mem;
use wast::{Error, Index};

#[derive(Default)]
pub struct Expander<'a> {
    to_append: Vec<Adapter<'a>>,
    funcs: u32,
}

impl<'a> Expander<'a> {
    pub fn process(
        &mut self,
        fields: &mut Vec<Adapter<'a>>,
        mut f: impl FnMut(&mut Self, &mut Adapter<'a>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut cur = 0;
        while cur < fields.len() {
            f(self, &mut fields[cur])?;
            for new in self.to_append.drain(..) {
                fields.insert(cur, new);
                cur += 1;
            }
            cur += 1;
        }
        Ok(())
    }

    pub fn deinline_import(&mut self, item: &mut Adapter<'a>) -> Result<(), Error> {
        match item {
            Adapter::Func(f) => {
                let (module, name) = match f.kind {
                    FuncKind::Import { module, name } => (module, name),
                    _ => return Ok(()),
                };
                if let Some(name) = f.export.take() {
                    self.to_append.push(Adapter::Export(Export {
                        name,
                        func: Index::Num(self.funcs, f.span),
                    }));
                }
                *item = Adapter::Import(Import {
                    span: f.span,
                    module,
                    name,
                    id: f.name,
                    ty: f.ty.clone(),
                });
                self.funcs += 1;
            }

            Adapter::Import(_) => self.funcs += 1,

            _ => {}
        }
        Ok(())
    }

    pub fn deinline_non_import(
        &mut self,
        item: &mut Adapter<'a>,
        core: &[wast::ModuleField<'a>],
    ) -> Result<(), Error> {
        match item {
            Adapter::Func(f) => {
                if let Some(name) = f.export.take() {
                    self.to_append.push(Adapter::Export(Export {
                        name,
                        func: Index::Num(self.funcs, f.span),
                    }));
                }
                self.funcs += 1;
            }

            Adapter::Implement(i) => {
                // If this `implement` directive is listed by a name then we
                // need to find the corresponding import in the core module and
                // switch it to `ByIndex`.
                if let Implemented::ByName { module, name } = i.implemented {
                    let idx = self.find_func_index(i.span, core, module, name)?;
                    i.implemented = Implemented::ByIndex(wast::Index::Num(idx, i.span));
                }

                // If we have an inline function declaration, then move this
                // function declaration into its own item.
                let tmp = Implementation::ByIndex(wast::Index::Num(self.funcs, i.span));
                match mem::replace(&mut i.implementation, tmp) {
                    Implementation::Inline { ty, instrs } => {
                        self.to_append.push(Adapter::Func(Func {
                            span: i.span,
                            name: None,
                            export: None,
                            ty,
                            kind: FuncKind::Inline { instrs },
                        }));

                        self.funcs += 1;
                    }
                    Implementation::ByIndex(idx) => {
                        i.implementation = Implementation::ByIndex(idx);
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn find_func_index(
        &self,
        span: wast::Span,
        core: &[wast::ModuleField<'_>],
        module: &str,
        field: &str,
    ) -> Result<u32, Error> {
        let mut idx = 0;
        let mut ret = None;
        for entry in core {
            let i = match entry {
                wast::ModuleField::Import(i) => i,
                _ => continue,
            };
            match i.item.kind {
                wast::ItemKind::Func(_) => {}
                _ => continue,
            }
            idx += 1;
            if i.module != module || i.field != Some(field) {
                continue;
            }
            if ret.is_some() {
                let msg = format!(
                    "import of `{}` from `{}` is ambiguous since \
                     it's listed twice in the core module",
                    module, field
                );
                return Err(Error::new(span, msg));
            }
            ret = Some(idx - 1);
        }

        match ret {
            Some(i) => Ok(i),
            None => {
                let msg = format!(
                    "import of `{}` from `{}` not found in core module",
                    module, field
                );
                Err(Error::new(span, msg))
            }
        }
    }
}
