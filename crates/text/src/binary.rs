use crate::ast::*;

pub fn append(adapters: &[Adapter<'_>], wasm: &mut Vec<u8>) {
    if adapters.len() == 0 {
        return;
    }
    let mut types = Vec::new();
    let mut imports = Vec::new();
    let mut funcs = Vec::new();
    let mut exports = Vec::new();
    let mut implements = Vec::new();
    for adapter in adapters {
        match adapter {
            Adapter::Type(i) => types.push(i),
            Adapter::Import(i) => imports.push(i),
            Adapter::Func(i) => funcs.push(i),
            Adapter::Export(i) => exports.push(i),
            Adapter::Implement(i) => implements.push(i),
        }
    }

    let mut writer = wit_writer::Writer::new();

    // First up is the type section ...
    let mut w = writer.types(types.len() as u32);
    for ty in types {
        w.add(
            ty.params.len() as u32,
            |w| {
                for (_, param) in ty.params.iter() {
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
    }

    drop(w);

    // ... then the import section ...
    let mut w = writer.imports(imports.len() as u32);
    for import in imports {
        w.add(
            import.module,
            import.name,
            get_num(import.ty.index.as_ref().expect("unresolved type use")),
        );
    }
    drop(w);

    // ... then the function section ...
    let mut w = writer.funcs(funcs.len() as u32);
    for func in funcs {
        let mut w = w.add(get_num(
            &func.ty.index.as_ref().expect("unresolved type use"),
        ));
        assert!(func.export.is_none());
        let instrs = match &func.kind {
            FuncKind::Inline { instrs } => instrs,
            FuncKind::Import { .. } => panic!("imports should be de-inlined"),
        };
        for instr in instrs.instrs.iter() {
            use Instruction::*;

            match instr {
                ArgGet(a) => w.arg_get(get_num(a)),
                CallCore(a) => w.call_core(get_num(a)),
                DeferCallCore(a) => w.defer_call_core(get_num(a)),
                CallAdapter(a) => w.call_adapter(get_num(a)),
                MemoryToString(a) => w.memory_to_string(get_num(&a.mem)),
                StringToMemory(a) => w.string_to_memory(get_num(&a.malloc), get_num(&a.mem)),

                I32ToS8 => w.i32_to_s8(),
                I32ToS8X => w.i32_to_s8x(),
                I32ToU8 => w.i32_to_u8(),
                I32ToS16 => w.i32_to_s16(),
                I32ToS16X => w.i32_to_s16x(),
                I32ToU16 => w.i32_to_u16(),
                I32ToS32 => w.i32_to_s32(),
                I32ToU32 => w.i32_to_u32(),
                I32ToS64 => w.i32_to_s64(),
                I32ToU64 => w.i32_to_u64(),

                I64ToS8 => w.i64_to_s8(),
                I64ToS8X => w.i64_to_s8x(),
                I64ToU8 => w.i64_to_u8(),
                I64ToS16 => w.i64_to_s16(),
                I64ToS16X => w.i64_to_s16x(),
                I64ToU16 => w.i64_to_u16(),
                I64ToS32 => w.i64_to_s32(),
                I64ToS32X => w.i64_to_s32x(),
                I64ToU32 => w.i64_to_u32(),
                I64ToS64 => w.i64_to_s64(),
                I64ToU64 => w.i64_to_u64(),

                S8ToI32 => w.s8_to_i32(),
                U8ToI32 => w.u8_to_i32(),
                S16ToI32 => w.s16_to_i32(),
                U16ToI32 => w.u16_to_i32(),
                S32ToI32 => w.s32_to_i32(),
                U32ToI32 => w.u32_to_i32(),
                S64ToI32 => w.s64_to_i32(),
                S64ToI32X => w.s64_to_i32x(),
                U64ToI32 => w.u64_to_i32(),
                U64ToI32X => w.u64_to_i32x(),

                S8ToI64 => w.s8_to_i64(),
                U8ToI64 => w.u8_to_i64(),
                S16ToI64 => w.s16_to_i64(),
                U16ToI64 => w.u16_to_i64(),
                S32ToI64 => w.s32_to_i64(),
                U32ToI64 => w.u32_to_i64(),
                S64ToI64 => w.s64_to_i64(),
                U64ToI64 => w.u64_to_i64(),
            }
        }
    }
    drop(w);

    // ... then the export section ...
    let mut w = writer.exports(exports.len() as u32);
    for export in exports {
        w.add(export.name, get_num(&export.func));
    }
    drop(w);

    // ... and finally the implements section
    let mut w = writer.implements(implements.len() as u32);
    for implement in implements {
        let implemented = match &implement.implemented {
            Implemented::ByIndex(i) => i,
            Implemented::ByName { .. } => panic!("should be `ByIndex`"),
        };
        let implementation = match &implement.implementation {
            Implementation::ByIndex(i) => i,
            Implementation::Inline { .. } => panic!("should be `ByIndex`"),
        };
        w.add(get_num(implemented), get_num(implementation));
    }
    drop(w);

    wasm.extend_from_slice(&writer.into_custom_section());
}

fn get_num(idx: &wast::Index<'_>) -> u32 {
    match idx {
        wast::Index::Num(n, _) => *n,
        wast::Index::Id(s) => panic!("unresolved name: {}", s.name()),
    }
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
