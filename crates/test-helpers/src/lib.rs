//! A test suite to parse everything in `parse-fail` and assert that it matches
//! the `*.err` file it generates.
//!
//! Use `BLESS=1` in the environment to auto-update `*.err` files. Be sure to
//! look at the diff!

use anyhow::Context;
use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

pub fn run(dir: &Path, expectation: &str, run: fn(&Path) -> anyhow::Result<String>) {
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
        .filter_map(|test| run_test(test, bless, run, expectation).err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("error: {:?}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!("test result: ok. {} passed\n", tests.len());
}

fn run_test(
    test: &Path,
    bless: bool,
    run: fn(&Path) -> anyhow::Result<String>,
    expectation: &str,
) -> anyhow::Result<()> {
    let actual = run(test).context(format!(
        "test execution function failed for: {}",
        test.display()
    ))?;
    let assert = test.with_extension(expectation);
    if bless {
        std::fs::write(assert, actual)?;
        return Ok(());
    }

    // Ignore CRLF line ending and force always `\n`
    let expected = std::fs::read_to_string(assert)
        .unwrap_or(String::new())
        .replace("\r\n", "\n");

    // Compare normalize verisons which handles weirdness like path differences
    if normalize(&expected) == normalize(&actual) {
        return Ok(());
    }

    anyhow::bail!(
        "test outputs didn't match:\n\nexpected:\n\t{}\nactual:\n\t{}\n",
        tab(&expected),
        tab(&actual),
    );

    fn normalize(s: &str) -> String {
        s.replace("\\", "/")
    }

    fn tab(s: &str) -> String {
        s.replace("\n", "\n\t")
    }
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
            _ => continue,
        }
        tests.push(f.path());
    }
}
