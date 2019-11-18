use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

fn main() {
    std::env::set_current_dir("../..").unwrap();
    test_helpers::run("tests".as_ref(), run);
}

fn run(path: &Path) -> Result<String> {
    let test = Test::read_from(path)?;
    let binary = if path.extension().and_then(|s| s.to_str()) == Some("wat") {
        wat::parse_file(path).map_err(|e| e.into())
    } else {
        wit_text::parse_file(path)
    };
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
                return Ok(format!("{:?}", e))
            } else {
                return Err(e)
            }
        }
    };
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
                    return Err(e)
                }
            }
        }
    }
    let wit = wit_printer::print_bytes(&binary)?;
    let roundtrip = wit_text::parse_str(&wit)?;
    if roundtrip != binary {
        bail!(
            "round-trip serialization of this text file failed:\n\n\
             tried to serialize:\n    {}",
            wit.replace("\n", "\n    ")
        );
    }
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
