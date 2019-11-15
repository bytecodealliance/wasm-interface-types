use anyhow::Context;
use std::path::Path;
use wasmprinter::Printer;
use wit_parser::*;

pub fn print_file(file: impl AsRef<Path>) -> anyhow::Result<String> {
    _print_file(file.as_ref())
}

fn _print_file(file: &Path) -> anyhow::Result<String> {
    let contents = std::fs::read(file).context(format!("failed to read `{}`", file.display()))?;
    _print_bytes(&contents)
}

pub fn print_bytes(wasm: impl AsRef<[u8]>) -> anyhow::Result<String> {
    _print_bytes(wasm.as_ref())
}

fn _print_bytes(wasm: &[u8]) -> anyhow::Result<String> {
    let mut printer = Printer::new();
    printer.add_custom_section_printer("wasm-interface-types", print_wit);
    printer.print(wasm)
}

fn print_wit(printer: &mut Printer, bytes: &[u8]) -> anyhow::Result<()> {
    let mut parser = Parser::new(bytes).context("failed to parse header")?;
    while !parser.is_empty() {
        match parser.section().context("failed to parse section")? {
            Section::Type(types) => {
                let ret = printer.result_mut();
                for ty in types {
                    let ty = ty.context("failed to parse type")?;
                    ret.push_str("\n  (@interface type (func");
                    for param in ty.params.iter() {
                        ret.push_str(" (param ");
                        push_ty(ret, param);
                        ret.push_str(")");
                    }
                    for result in ty.results.iter() {
                        ret.push_str(" (result ");
                        push_ty(ret, result);
                        ret.push_str(")");
                    }
                    ret.push_str("))");
                }
            }
            Section::Import(imports) => {
                let ret = printer.result_mut();
                for i in imports {
                    let i = i.context("failed to parse import")?;
                    ret.push_str("\n  (@interface import ");
                    ret.push_str("\"");
                    ret.push_str(i.module);
                    ret.push_str("\" \"");
                    ret.push_str(i.name);
                    ret.push_str("\" (func (type ");
                    ret.push_str(&format!("{}", i.ty));
                    ret.push_str(")))");
                }
            }
            Section::Export(exports) => {
                let ret = printer.result_mut();
                for e in exports {
                    let e = e.context("failed to parse export")?;
                    ret.push_str("\n  (@interface export ");
                    ret.push_str("\"");
                    ret.push_str(e.name);
                    ret.push_str("\" (func ");
                    ret.push_str(&format!("{}", e.func));
                    ret.push_str("))");
                }
            }
            Section::Func(funcs) => {
                let ret = printer.result_mut();
                for f in funcs {
                    let f = f.context("failed to parse func")?;
                    ret.push_str("\n  (@interface func (type ");
                    ret.push_str(&format!("{}", f.ty));
                    ret.push_str(")");
                    for instr in f.instrs() {
                        let instr = instr.context("failed to parse instruction")?;
                        ret.push_str("\n    ");
                        push_instr(ret, &instr);
                    }
                    ret.push_str(")");
                }
            }
            Section::Implement(implements) => {
                for i in implements {
                    let i = i.context("failed to parse implement")?;
                    printer.result_mut().push_str("\n  (@interface implement (func ");
                    printer.print_func_idx(i.core_func)?;
                    printer.result_mut().push_str(") (func ");
                    printer.result_mut().push_str(&format!("{}", i.adapter_func));
                    printer.result_mut().push_str("))");
                }
            }
        }
    }
    return Ok(());

    fn push_ty(ret: &mut String, param: &ValType) {
        match param {
            ValType::S8 => ret.push_str("s8"),
            ValType::S16 => ret.push_str("s16"),
            ValType::S32 => ret.push_str("s32"),
            ValType::S64 => ret.push_str("s64"),
            ValType::U8 => ret.push_str("u8"),
            ValType::U16 => ret.push_str("u16"),
            ValType::U32 => ret.push_str("u32"),
            ValType::U64 => ret.push_str("u64"),
            ValType::F32 => ret.push_str("f32"),
            ValType::F64 => ret.push_str("f64"),
            ValType::String => ret.push_str("string"),
        }
    }

    fn push_instr(ret: &mut String, instr: &Instruction) {
        use std::fmt::Write;
        use Instruction::*;

        match instr {
            ArgGet(i) => write!(ret, "arg.get {}", i).unwrap(),
            CallCore(i) => write!(ret, "call-core {}", i).unwrap(),
            End => ret.push_str("end"),
        }
    }
}
