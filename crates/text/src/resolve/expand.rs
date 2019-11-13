use crate::ast::*;
use wast::Index;

#[derive(Default)]
pub struct Expander<'a> {
    to_append: Vec<Adapter<'a>>,
    funcs: u32,
}

impl<'a> Expander<'a> {
    pub fn process(
        &mut self,
        fields: &mut Vec<Adapter<'a>>,
        mut f: impl FnMut(&mut Self, &mut Adapter<'a>),
    ) {
        let mut cur = 0;
        while cur < fields.len() {
            f(self, &mut fields[cur]);
            for new in self.to_append.drain(..) {
                fields.insert(cur, new);
                cur += 1;
            }
            cur += 1;
        }
    }

    pub fn deinline_import(&mut self, item: &mut Adapter<'a>) {
        match item {
            Adapter::Func(f) => {
                let (module, name) = match f.kind {
                    FuncKind::Import { module, name } => (module, name),
                    _ => return,
                };
                if let Some(name) = f.export.take() {
                    self.to_append.push(Adapter::Export(Export {
                        name,
                        func: Index::Num(self.funcs),
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
    }

    pub fn deinline_export(&mut self, item: &mut Adapter<'a>) {
        match item {
            Adapter::Func(f) => {
                if let Some(name) = f.export.take() {
                    self.to_append.push(Adapter::Export(Export {
                        name,
                        func: Index::Num(self.funcs),
                    }));
                }
                self.funcs += 1;
            }

            _ => {}
        }
    }
}
