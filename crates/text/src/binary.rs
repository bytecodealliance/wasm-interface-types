use crate::ast::*;

pub fn encode(adapters: &[Adapter<'_>]) -> Vec<u8> {
    if adapters.len() == 0 {
        return Vec::new();
    }
    let mut types = Vec::new();
    let mut imports = Vec::new();
    let mut funcs = Vec::new();
    let mut exports = Vec::new();
    for adapter in adapters {
        match adapter {
            Adapter::Type(i) => types.push(i),
            Adapter::Import(i) => imports.push(i),
            Adapter::Func(i) => funcs.push(i),
            Adapter::Export(i) => exports.push(i),
        }
    }

    let mut wasm = Vec::new();
    let mut tmp = Vec::new();
    env!("CARGO_PKG_VERSION").encode(&mut wasm);
    section_list(0, &types, &mut tmp, &mut wasm);
    section_list(1, &imports, &mut tmp, &mut wasm);
    section_list(2, &exports, &mut tmp, &mut wasm);
    section_list(3, &funcs, &mut tmp, &mut wasm);

    fn section_list<T: Encode>(id: u8, list: &[T], tmp: &mut Vec<u8>, dst: &mut Vec<u8>) {
        if !list.is_empty() {
            section(id, list, tmp, dst)
        }
    }

    fn section<T: Encode>(id: u8, list: T, tmp: &mut Vec<u8>, dst: &mut Vec<u8>) {
        tmp.truncate(0);
        list.encode(tmp);
        dst.push(id);
        tmp.encode(dst);
    }
    tmp.truncate(0);
    tmp.push(0xfe);
    wasm.encode(&mut tmp);

    return tmp;
}

pub(crate) trait Encode {
    fn encode(&self, e: &mut Vec<u8>);
}

impl<T: Encode + ?Sized> Encode for &'_ T {
    fn encode(&self, e: &mut Vec<u8>) {
        T::encode(self, e)
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, e: &mut Vec<u8>) {
        self.len().encode(e);
        for item in self {
            item.encode(e);
        }
    }
}

impl Encode for [u8] {
    fn encode(&self, e: &mut Vec<u8>) {
        self.len().encode(e);
        e.extend_from_slice(self);
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, e: &mut Vec<u8>) {
        <[T]>::encode(self, e)
    }
}

impl Encode for str {
    fn encode(&self, e: &mut Vec<u8>) {
        self.as_bytes().encode(e);
    }
}

impl Encode for usize {
    fn encode(&self, e: &mut Vec<u8>) {
        assert!(*self <= u32::max_value() as usize);
        (*self as u32).encode(e)
    }
}

impl Encode for u32 {
    fn encode(&self, e: &mut Vec<u8>) {
        leb128::write::unsigned(e, (*self).into()).unwrap();
    }
}

impl Encode for wast::Index<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        match self {
            wast::Index::Num(n) => n.encode(e),
            wast::Index::Id(_) => panic!("unresolved name"),
        }
    }
}

impl Encode for Type<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        self.params.len().encode(e);
        for (_, param) in self.params.iter() {
            param.encode(e);
        }
        self.results.encode(e);
    }
}

impl Encode for ValType {
    fn encode(&self, e: &mut Vec<u8>) {
        e.push(self.clone() as u8);
    }
}

impl Encode for TypeUse<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        self.index
            .as_ref()
            .expect("TypeUse should be filled in")
            .encode(e)
    }
}

impl Encode for Import<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        self.module.encode(e);
        self.name.encode(e);
        self.ty.encode(e);
    }
}

impl Encode for Export<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        self.func.encode(e);
        self.name.encode(e);
    }
}

impl Encode for Func<'_> {
    fn encode(&self, e: &mut Vec<u8>) {
        assert!(self.export.is_none());
        self.ty.encode(e);
        let instrs = match &self.kind {
            FuncKind::Inline { instrs } => instrs,
            FuncKind::Import { .. } => panic!("imports should be de-inlined"),
        };
        instrs.encode(e);
    }
}
