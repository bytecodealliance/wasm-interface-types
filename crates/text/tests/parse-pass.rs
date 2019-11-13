use anyhow::{bail, Context};
use std::path::Path;
use wit_parser::*;

fn main() {
    test_helpers::run("tests/parse-pass".as_ref(), "wit.ok", run);
}

fn run(path: &Path) -> anyhow::Result<String> {
    let binary = wit_text::parse_file(path)?;
    let wit = match find_wit_section(&binary)? {
        Some(bytes) => bytes,
        None => return Ok(String::new()),
    };

    let text = stringify(wit).context("failed to parse interface types section")?;
    let roundtrip =
        wit_text::parse_str(&format!("(module {})", text)).context("failed to parse round-trip")?;
    if Some(wit) != find_wit_section(&roundtrip)? {
        bail!(
            "round-trip serialization of this text file failed:\n\n\
             tried to serialize:\n    {}",
            text.replace("\n", "\n    ")
        );
    }
    Ok(text)
}

fn find_wit_section(bytes: &[u8]) -> anyhow::Result<Option<&[u8]>> {
    let mut result = None;
    let mut parser = wasmparser::ModuleReader::new(bytes)?;
    while !parser.eof() {
        let section = parser.read()?;
        match section.code {
            wasmparser::SectionCode::Custom {
                name: "wasm-interface-types",
                ..
            } => {}
            _ => continue,
        }
        if result.is_some() {
            bail!("found two sections with interface types");
        }
        let range = section.get_binary_reader().range();
        result = Some(&bytes[range.start..range.end]);
    }
    Ok(result)
}

fn stringify(bytes: &[u8]) -> anyhow::Result<String> {
    let mut parser = Parser::new(bytes).context("failed to parse header")?;
    let mut ret = String::new();
    while !parser.is_empty() {
        match parser.section().context("failed to parse section")? {
            Section::Type(types) => {
                for ty in types {
                    let ty = ty.context("failed to parse type")?;
                    ret.push_str("(@interface type (func");
                    for param in ty.params.iter() {
                        ret.push_str(" (param ");
                        push_ty(&mut ret, param);
                        ret.push_str(")");
                    }
                    for result in ty.results.iter() {
                        ret.push_str(" (result ");
                        push_ty(&mut ret, result);
                        ret.push_str(")");
                    }
                    ret.push_str("))\n");
                }
            }
            Section::Import(imports) => {
                for i in imports {
                    let i = i.context("failed to parse import")?;
                    ret.push_str("(@interface import ");
                    ret.push_str("\"");
                    ret.push_str(i.module);
                    ret.push_str("\" \"");
                    ret.push_str(i.name);
                    ret.push_str("\" (func (type ");
                    ret.push_str(&format!("{}", i.ty));
                    ret.push_str(")))\n");
                }
            }
            Section::Export(exports) => {
                for e in exports {
                    let e = e.context("failed to parse export")?;
                    ret.push_str("(@interface export ");
                    ret.push_str("\"");
                    ret.push_str(e.name);
                    ret.push_str("\" (func ");
                    ret.push_str(&format!("{}", e.func));
                    ret.push_str("))\n");
                }
            }
            Section::Func(funcs) => {
                for f in funcs {
                    let f = f.context("failed to parse func")?;
                    ret.push_str("(@interface func (type ");
                    ret.push_str(&format!("{}", f.ty));
                    ret.push_str(")");
                    for instr in f.instrs() {
                        let instr = instr.context("failed to parse instruction")?;
                        ret.push_str("\n  ");
                        push_instr(&mut ret, &instr);
                    }
                    ret.push_str(")\n");
                }
            }
        }
    }
    return Ok(ret);

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
