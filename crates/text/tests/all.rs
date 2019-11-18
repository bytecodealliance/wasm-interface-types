use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

fn main() {
    test_helpers::run("../../tests".as_ref(), "wit.out", run);
}

fn run(path: &Path) -> Result<String> {
    let test = Test::read_from(path)?;
    let binary = match wit_text::parse_file(path) {
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
                bail!(e);
            }
        }
    };
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
                _ => {}
            }
        }
        Ok(ret)
    }
}
