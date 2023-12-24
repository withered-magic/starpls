use crate::{parse, StrStep, StrWithTokens};
use expect_test::ExpectFile;
use std::fmt::Write;

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
