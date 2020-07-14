use crate::ast::*;
use std::collections::HashMap;
use wast::{Index, Span};

#[derive(Default)]
pub struct Expander<'a> {
    pub to_prepend: Vec<Adapter<'a>>,
    types: HashMap<(Vec<ValType>, Vec<ValType>), u32>,
    ntypes: u32,
}

impl<'a> Expander<'a> {
    pub fn expand(&mut self, item: &mut Adapter<'a>) {
        match item {
            Adapter::Type(t) => self.register_type(t),
            Adapter::Import(i) => self.expand_import(i),
            Adapter::Func(f) => self.expand_func(f),
            Adapter::Implement(i) => self.expand_implement(i),
            Adapter::Export(_) => {}
        }
    }

    fn register_type(&mut self, ty: &Type<'a>) {
        let key = self.key(ty);
        if !self.types.contains_key(&key) {
            self.types.insert(key, self.ntypes);
        }
        self.ntypes += 1;
    }

    fn expand_import(&mut self, import: &mut Import<'a>) {
        self.expand_type_use(&mut import.ty);
    }

    fn expand_func(&mut self, func: &mut Func<'a>) {
        self.expand_type_use(&mut func.ty);
    }

    fn expand_implement(&mut self, implement: &mut Implement<'a>) {
        if let Implementation::Inline { ty, .. } = &mut implement.implementation {
            self.expand_type_use(ty);
        }
    }

    fn expand_type_use(&mut self, item: &mut TypeUse<'a>) {
        if item.index.is_some() {
            return;
        }
        let key = self.key(&item.ty);
        item.index = Some(Index::Num(
            match self.types.get(&key) {
                Some(i) => *i,
                None => self.prepend(key),
            },
            Span::from_offset(0),
        ));
    }

    fn key(&self, ty: &Type<'_>) -> (Vec<ValType>, Vec<ValType>) {
        let params = ty.params.iter().map(|p| p.1.clone()).collect::<Vec<_>>();
        let results = ty.results.clone();
        (params, results)
    }

    fn prepend(&mut self, key: (Vec<ValType>, Vec<ValType>)) -> u32 {
        self.to_prepend.push(Adapter::Type(Type {
            name: None,
            params: key.0.iter().map(|t| (None, t.clone())).collect(),
            results: key.1.clone(),
        }));
        self.types.insert(key, self.ntypes);
        self.ntypes += 1;
        return self.ntypes - 1;
    }
}
