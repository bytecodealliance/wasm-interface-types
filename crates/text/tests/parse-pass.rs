use anyhow::bail;
use std::path::Path;

fn main() {
    test_helpers::run("tests/parse-pass".as_ref(), "wit.ok", run);
}

fn run(path: &Path) -> anyhow::Result<String> {
    let binary = wit_text::parse_file(path)?;
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
