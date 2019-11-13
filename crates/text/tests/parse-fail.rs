//! A test suite to parse everything in `parse-fail` and assert that it matches
//! the `*.err` file it generates.
//!
//! Use `BLESS=1` in the environment to auto-update `*.err` files. Be sure to
//! look at the diff!

fn main() {
    test_helpers::run("tests/parse-fail".as_ref(), "wit.err", |path| {
        match wit_text::parse_file(path) {
            Ok(_) => anyhow::bail!("{} parsed successfully", path.display()),
            Err(e) => Ok(e.to_string() + "\n"),
        }
    })
}
