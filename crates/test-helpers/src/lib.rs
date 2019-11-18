//! A test suite to parse everything in `parse-fail` and assert that it matches
//! the `*.err` file it generates.
//!
//! Use `BLESS=1` in the environment to auto-update `*.err` files. Be sure to
//! look at the diff!

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
    .context(format!(
        "test execution function failed for: {}",
        test.display()
    ))?;
    Ok(())
    // if bless {
    //     std::fs::write(assert, actual)?;
    //     return Ok(());
    // }

    // // Ignore CRLF line ending and force always `\n`
    // let expected = std::fs::read_to_string(assert)
    //     .unwrap_or(String::new())
    //     .replace("\r\n", "\n");
    //
    // // Compare normalize verisons which handles weirdness like path differences
    // if normalize(&expected) == normalize(&actual) {
    //     return Ok(());
    // }
    //
    // anyhow::bail!(
    //     "test outputs didn't match:\n\nexpected:\n\t{}\nactual:\n\t{}\n",
    //     tab(&expected),
    //     tab(&actual),
    // );
    //
    // fn normalize(s: &str) -> String {
    //     s.replace("\\", "/")
    // }
    //
    // fn tab(s: &str) -> String {
    //     s.replace("\n", "\n\t")
    // }
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
    Exhaustive(Vec<String>, PathBuf),
    None(PathBuf),
}

impl FileCheck {
    pub fn from_file(path: &Path) -> Result<FileCheck> {
        let contents = fs::read_to_string(path)?;
        let mut iter = contents.lines().map(str::trim);
        while let Some(line) = iter.next() {
            if line.starts_with("(; CHECK-ALL:") {
                let mut pattern = Vec::new();
                while let Some(line) = iter.next() {
                    if line == ";)" {
                        break;
                    }
                    pattern.push(line.to_string());
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
        let output_lines = output.lines().collect::<Vec<_>>();
        match self {
            FileCheck::Exhaustive(_, path) | FileCheck::None(path) if bless => {
                update_output(path, output)?;
            }
            FileCheck::Exhaustive(pattern, _) => {
                for (out_line, pat_line) in output_lines.iter().zip(pattern) {
                    if !matches(out_line, pat_line) {
                        self.missing_pattern(pattern, output)?;
                    }
                }
            }
            FileCheck::None(_) => {
                bail!(
                    "no test assertions were found in this file, but you can \
                     rerun tests with `BLESS=1` to automatically add assertions \
                     to this file"
                );
            }
        }

        Ok(())
    }

    fn missing_pattern(&self, pattern: &[String], output: &str) -> Result<()> {
        let pattern = pattern
            .iter()
            .enumerate()
            .map(|(i, l)| format!("    {}: {}", if i == 0 { "CHECK" } else { "NEXT" }, l,))
            .collect::<Vec<_>>()
            .join("\n");

        let output = output
            .lines()
            .map(|l| format!("    {}", l.trim_end()))
            .collect::<Vec<_>>()
            .join("\n");

        bail!(
            "\
             CHECK failed!\n\n\
             Did not find pattern\n\n\
             {}\n\n\
             in output\n\n\
             {}\n\n",
            pattern,
            output
        );
    }
}

fn matches(mut actual: &str, expected: &str) -> bool {
    actual = actual.trim();
    // skip a leading comment
    if actual.starts_with("(;") {
        actual = actual[actual.find(";)").unwrap() + 2..].trim();
    }
    actual.starts_with(expected.trim())
}

fn update_output(path: &Path, output: &str) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let start = contents.find("(; CHECK-ALL:").unwrap_or(contents.len());

    let mut new_output = String::new();
    for line in output.lines() {
        if !line.is_empty() {
            new_output.push_str("  ");
            new_output.push_str(line.trim_end());
        }
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
