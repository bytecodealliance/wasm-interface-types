//! Test suite which executes everything inside of the top-level `tests`
//! directory. Each file is its own test and has an assertion of the expected
//! output at the end of the test.

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

fn main() {
    std::env::set_current_dir("../..").unwrap();
    test_helpers::run("tests".as_ref(), run);
}

fn run(path: &Path) -> Result<String> {
    let test = Test::read_from(path)?;

    let binary = wit_text::parse_file(path);

    // Extract the binary bytes, handling `parse-fail` directives here
    let binary = match binary {
        Ok(binary) => {
            if test.parse_fail {
                bail!("successfully parsed {:?}", path);
            } else {
                binary
            }
        }
        Err(e) => {
            if test.parse_fail {
                return Ok(format!("{:?}", e));
            } else {
                return Err(e);
            }
        }
    };

    // Perform validation over the binary blob, if not explicitly disabled
    if !test.no_validate {
        match wit_validator::validate(&binary) {
            Ok(()) => {
                if test.validate_fail {
                    match wit_printer::print_bytes(&binary) {
                        Ok(s) => bail!("expected a validation failure: {}", s),
                        Err(_) => bail!("expected a validation failure"),
                    }
                }
            }
            Err(e) => {
                if test.validate_fail {
                    return Ok(format!("{:?}", e));
                } else {
                    return Err(e);
                }
            }
        }

        // If we validated correctly then `walrus` parsing should definitely
        // succeed
        let mut module = walrus::ModuleConfig::new()
            .on_parse(wit_walrus::on_parse)
            .parse(&binary)
            .context("failed walrus parsing")?;
        walrus::passes::gc::run(&mut module);
        let binary = module.emit_wasm();
        wit_validator::validate(&binary).context("failed walrus round trip validation")?;
    }

    // And if we got this far do a double-check that our printer can indeed be
    // parsed, and the binary representation matches our original binary
    // representation as well.
    let wit = wit_printer::print_bytes(&binary)?;
    let roundtrip = wit_text::parse_str(&wit)?;
    if roundtrip != binary {
        bail!(
            "round-trip serialization of this text file failed:\n\n\
             tried to serialize:\n    {}",
            wit.replace("\n", "\n    ")
        );
    }

    // And that all passed! Consider our success as the fully-pretty-printed
    // version of the module.
    Ok(wit)
}

#[derive(Default)]
struct Test {
    parse_fail: bool,
    no_validate: bool,
    validate_fail: bool,
}

impl Test {
    fn read_from(path: &Path) -> Result<Test> {
        let contents = fs::read_to_string(path)?;
        let mut ret = Test::default();
        for line in contents.lines() {
            if !line.starts_with(";;") {
                break;
            }
            let line = line[2..].trim();
            match line {
                "parse-fail" => ret.parse_fail = true,
                "no-validate" => ret.no_validate = true,
                "validate-fail" => ret.validate_fail = true,
                _ => {}
            }
        }
        Ok(ret)
    }
}
