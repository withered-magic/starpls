use crate::{parse, StrStep, StrWithTokens};
use expect_test::{expect_file, ExpectFile};
use std::{
    env,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

fn check(input: &str, expected: ExpectFile) {
    let str_with_tokens = StrWithTokens::new(input);
    let output = parse(&str_with_tokens.to_input());

    // Render the parse tree, including trivia tokens.
    let mut buf = String::new();
    let mut indent = String::new();
    let mut errors = Vec::new();

    str_with_tokens.build_with_trivia(output, &mut |step| match step {
        StrStep::Start { kind } => {
            writeln!(buf, "{indent}{kind:?}").unwrap();
        }
        StrStep::Finish => {
            indent.pop();
            indent.pop();
        }
        StrStep::Token { kind, text } => {
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        StrStep::Error { message, pos } => errors.push((message, pos)),
    });

    for (message, pos) in errors {
        writeln!(buf, "error {pos}: {message}").unwrap();
    }

    expected.assert_eq(&buf);
}

#[test]
fn test_parse_ok() {
    for test_case in collect_test_cases("test_data/ok").unwrap() {
        check(&test_case.input, expect_file![test_case.expect_file]);
    }
}

#[test]
fn test_parse_error() {
    for test_case in collect_test_cases("test_data/err").unwrap() {
        check(&test_case.input, expect_file![test_case.expect_file]);
    }
}

#[derive(Debug)]
struct TestCase {
    input: String,
    expect_file: PathBuf,
}

fn collect_test_cases(dir: &'static str) -> Result<Vec<TestCase>, Box<dyn std::error::Error>> {
    let mut test_cases = Vec::new();

    // Check for a test filter.
    let filter = env::var("TEST_FILTER").ok();

    // Determine the test data directory.
    let crate_root =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    let crate_root = Path::new(&crate_root);
    let test_data_dir = crate_root.join(dir);

    for entry in fs::read_dir(test_data_dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        // Skip non-Starlark files.
        if entry_path.extension().unwrap_or_default() != "star" || !entry.file_type()?.is_file() {
            continue;
        }

        // If a filter was specified, check if the test name (the base name of the file without the extension) matches it.
        let stripped = entry_path.with_extension("");
        let test_name = match stripped.file_name().and_then(|name| name.to_str()) {
            Some(test_name) => test_name,
            None => continue,
        };
        match filter {
            Some(ref filter) => {
                if !test_name.contains(filter) {
                    continue;
                }
            }
            None => (),
        }

        // For a Starlark source file `source.star`, the corresponding expect file is `source.rast`.
        let input = fs::read_to_string(&entry_path)?;
        let expect_file = stripped.with_extension("rast");

        test_cases.push(TestCase { input, expect_file })
    }

    Ok(test_cases)
}
