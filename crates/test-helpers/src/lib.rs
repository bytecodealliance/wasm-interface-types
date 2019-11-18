//! A small test framework to execute a test function over all files in a
//! directory.
//!
//! Each file in the directory has its own `CHECK-ALL` annotation indicating the
//! expected output of the test. That can be automatically updated with
//! `BLESS=1` in the environment. Otherwise the test are checked against the
//! listed expectation.

use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(dir: &Path, run: fn(&Path) -> Result<String>) {
    let mut tests = Vec::new();
    find_tests(dir, &mut tests);
    let filter = std::env::args().nth(1);

    let bless = env::var("BLESS").is_ok();
    let tests = tests
        .iter()
        .filter(|test| {
            if let Some(filter) = &filter {
                if let Some(s) = test.file_name().and_then(|s| s.to_str()) {
                    if !s.contains(filter) {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();

    println!("\nrunning {} tests\n", tests.len());

    let errors = tests
        .par_iter()
        .filter_map(|test| run_test(test, bless, run).err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("error: {:?}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!("test result: ok. {} passed\n", tests.len());
}

fn run_test(test: &Path, bless: bool, run: fn(&Path) -> anyhow::Result<String>) -> Result<()> {
    (|| -> Result<_> {
        let expected = FileCheck::from_file(test)?;
        let actual = run(test)?;
        expected.check(&actual, bless)?;
        Ok(())
    })()
    .context(format!("test failed - {}", test.display()))?;
    Ok(())
}

fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
    for f in path.read_dir().unwrap() {
        let f = f.unwrap();
        if f.file_type().unwrap().is_dir() {
            find_tests(&f.path(), tests);
            continue;
        }
        match f.path().extension().and_then(|s| s.to_str()) {
            Some("wit") => {}
            Some("wat") => {}
            _ => continue,
        }
        tests.push(f.path());
    }
}

pub enum FileCheck {
    Exhaustive(String, PathBuf),
    None(PathBuf),
}

impl FileCheck {
    pub fn from_file(path: &Path) -> Result<FileCheck> {
        let contents = fs::read_to_string(path)?;
        let mut iter = contents.lines();
        while let Some(line) = iter.next() {
            if line.starts_with("(; CHECK-ALL:") {
                let mut pattern = String::new();
                while let Some(line) = iter.next() {
                    if line == ";)" {
                        break;
                    }
                    pattern.push_str(line);
                    pattern.push_str("\n");
                }
                while pattern.ends_with("\n") {
                    pattern.pop();
                }
                if iter.next().is_some() {
                    bail!("CHECK-ALL must be at the end of the file");
                }
                return Ok(FileCheck::Exhaustive(pattern, path.to_path_buf()));
            }
        }
        Ok(FileCheck::None(path.to_path_buf()))
    }

    pub fn check(&self, output: &str, bless: bool) -> Result<()> {
        let output = normalize(output);
        match self {
            FileCheck::Exhaustive(_, path) | FileCheck::None(path) if bless => {
                update_output(path, &output)
            }
            FileCheck::Exhaustive(pattern, _) => {
                if output == *pattern {
                    return Ok(());
                }
                bail!(
                    "expected\n    {}\n\nactual\n    {}",
                    pattern.replace("\n", "\n    "),
                    output.replace("\n", "\n    ")
                );
            }
            FileCheck::None(_) => {
                bail!(
                    "no test assertions were found in this file, but you can \
                     rerun tests with `BLESS=1` to automatically add assertions \
                     to this file"
                );
            }
        }
    }
}

fn update_output(path: &Path, output: &str) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let start = contents.find("(; CHECK-ALL:").unwrap_or(contents.len());

    let mut new_output = String::new();
    for line in output.lines() {
        new_output.push_str(line);
        new_output.push_str("\n");
    }
    let new = format!(
        "{}\n\n(; CHECK-ALL:\n{}\n;)\n",
        contents[..start].trim(),
        new_output.trim_end()
    );
    fs::write(path, new)?;
    Ok(())
}

fn normalize(output: &str) -> String {
    let mut ret = String::new();
    for line in output.lines() {
        // `anyhow` prints out some helpful information about RUST_LIB_BACKTRACE
        // but only on nightly right now, so normalize that away.
        if line.contains("RUST_LIB_BACKTRACE") || line.contains("Stack backtrace:") {
            continue;
        }
        ret.push_str(line);
        ret.push_str("\n");
    }
    while ret.ends_with("\n") {
        ret.pop();
    }
    return ret;
}
