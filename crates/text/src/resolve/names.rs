use crate::ast::*;
use std::collections::HashMap;
use wast::{Error, Id, Index, Span};

#[derive(Copy, Clone)]
pub enum Ns {
    Func,
    Type,
}

impl Ns {
    fn desc(&self) -> &'static str {
        match self {
            Ns::Func => "adapter func",
            Ns::Type => "adapter type",
        }
    }
}

pub struct Resolver<'a, 'b> {
    ns: [Namespace<'a>; 2],
    tys: Vec<Type<'a>>,
    names: &'b wast::Names<'a>,
}

struct Type<'a> {
    params: Vec<(Option<Id<'a>>, ValType)>,
    results: Vec<ValType>,
}

#[derive(Default)]
struct Namespace<'a> {
    names: HashMap<Id<'a>, u32>,
    count: u32,
}

impl<'a, 'b> Resolver<'a, 'b> {
    pub fn new(names: &'b wast::Names<'a>) -> Resolver<'a, 'b> {
        Resolver {
            ns: Default::default(),
            tys: Default::default(),
            names,
        }
    }

    pub fn register(&mut self, item: &Adapter<'a>) {
        let mut register = |ns: Ns, name: Option<Id<'a>>| {
            self.ns_mut(ns).register(name);
        };
        match item {
            Adapter::Import(i) => register(Ns::Func, i.id),
            Adapter::Func(i) => register(Ns::Func, i.name),
            Adapter::Type(i) => {
                register(Ns::Type, i.name);
                self.tys.push(Type {
                    params: i.params.clone(),
                    results: i.results.clone(),
                });
            }
            Adapter::Implement(_) => {}
            Adapter::Export(_) => {}
        }
    }

    fn ns_mut(&mut self, ns: Ns) -> &mut Namespace<'a> {
        &mut self.ns[ns as usize]
    }

    fn ns(&self, ns: Ns) -> &Namespace<'a> {
        &self.ns[ns as usize]
    }

    pub fn resolve(&self, field: &mut Adapter<'a>) -> Result<(), Error> {
        match field {
            Adapter::Import(i) => self.resolve_type_use(i.span, &mut i.ty),

            Adapter::Func(f) => {
                self.resolve_type_use(f.span, &mut f.ty)?;
                if let FuncKind::Inline { instrs } = &mut f.kind {
                    let mut resolver = ExprResolver::new(self);

                    // Parameters come first in the local namespace
                    for (name, _) in f.ty.ty.params.iter() {
                        resolver.locals.register(*name);
                    }

                    // and then we can resolve the expression!
                    for instr in instrs.instrs.iter_mut() {
                        resolver.resolve_instr(instr)?;
                    }
                }
                Ok(())
            }

            Adapter::Export(e) => self.resolve_idx(&mut e.func, Ns::Func),

            Adapter::Implement(i) => {
                if let Implemented::ByIndex(i) = &mut i.implemented {
                    self.names.resolve_func(i)?;
                }
                if let Implementation::ByIndex(i) = &mut i.implementation {
                    self.resolve_idx(i, Ns::Func)?;
                }
                Ok(())
            }

            Adapter::Type(_) => Ok(()),
        }
    }

    fn resolve_type_use(&self, span: Span, ty: &mut TypeUse<'a>) -> Result<(), Error> {
        assert!(ty.index.is_some());
        let idx = self
            .ns(Ns::Type)
            .resolve(ty.index.as_mut().unwrap())
            .map_err(|id| self.resolve_error(id, "type"))?;

        // If the type was listed inline *and* it was specified via a type index
        // we need to assert they're the same.
        let expected = match self.tys.get(idx as usize) {
            Some(ty) => ty,
            None => return Ok(()),
        };
        if ty.ty.params.len() > 0 || ty.ty.results.len() > 0 {
            let params_not_equal = expected.params.iter().map(|t| &t.1).ne(ty
                .ty
                .params
                .iter()
                .map(|t| &t.1));
            if params_not_equal || expected.results != ty.ty.results {
                let span = ty.index_span.unwrap_or(span);
                return Err(Error::new(
                    span,
                    format!("inline function type type doesn't match type reference"),
                ));
            }
        } else {
            ty.ty.params = expected.params.clone();
            ty.ty.results = expected.results.clone();
        }

        Ok(())
    }

    pub fn resolve_idx(&self, idx: &mut Index<'a>, ns: Ns) -> Result<(), Error> {
        match self.ns(ns).resolve(idx) {
            Ok(_n) => Ok(()),
            Err(id) => Err(self.resolve_error(id, ns.desc())),
        }
    }

    fn resolve_error(&self, id: Id<'a>, ns: &str) -> Error {
        Error::new(
            id.span(),
            format!("failed to find {} named `${}`", ns, id.name()),
        )
    }
}

impl<'a> Namespace<'a> {
    fn register(&mut self, name: Option<Id<'a>>) {
        if let Some(name) = name {
            self.names.insert(name, self.count);
        }
        self.count += 1;
    }

    fn resolve(&self, idx: &mut Index<'a>) -> Result<u32, Id<'a>> {
        let id = match idx {
            Index::Num(n, _) => return Ok(*n),
            Index::Id(id) => id,
        };
        if let Some(&n) = self.names.get(id) {
            *idx = Index::Num(n, id.span());
            return Ok(n);
        }
        Err(*id)
    }
}

struct ExprResolver<'a, 'b, 'c> {
    resolver: &'c Resolver<'a, 'b>,
    locals: Namespace<'a>,
}

impl<'a, 'b, 'c> ExprResolver<'a, 'b, 'c> {
    fn new(resolver: &'c Resolver<'a, 'b>) -> ExprResolver<'a, 'b, 'c> {
        ExprResolver {
            resolver,
            locals: Default::default(),
        }
    }

    fn resolve_instr(&mut self, instr: &mut Instruction<'a>) -> Result<(), Error> {
        use crate::ast::Instruction::*;

        match instr {
            ArgGet(i) => self
                .locals
                .resolve(i)
                .map(|_| ())
                .map_err(|id| self.resolver.resolve_error(id, "local")),
            CallCore(i) => self.resolver.names.resolve_func(i),
            MemoryToString(m) => self.resolver.names.resolve_memory(&mut m.mem),
            StringToMemory(m) => {
                self.resolver.names.resolve_func(&mut m.malloc)?;
                self.resolver.names.resolve_memory(&mut m.mem)
            }
            CallAdapter(f) => self.resolver.resolve_idx(f, Ns::Func),
            DeferCallCore(f) => self.resolver.names.resolve_func(f),
            _ => Ok(()),
        }
    }
}
