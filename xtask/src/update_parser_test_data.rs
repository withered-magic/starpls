use anyhow::anyhow;
use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::util::project_root;

/// A contiguous block of comments in a Rust source file.
#[derive(Default, Debug)]
struct CommentBlock {
    lines: Vec<String>,
}

/// The kind of parser test. There are two kinds: Tests that expect no parser errors are located under the `ok`
/// directory, and tests expecting parser errors are located under the `err` directory.
#[derive(Debug, PartialEq, Eq)]
enum TestKind {
    Ok,
    Err,
}

/// A parser test, as derived from a `CommentBlock`.
#[derive(Debug)]
struct Test {
    kind: TestKind,
    name: String,
    text: String,
}

/// Extracts comment blocks from a Rust source file.
fn extract_comment_blocks(text: &str) -> Vec<CommentBlock> {
    let comment_prefix = "// ";
    let lines = text.lines().map(str::trim_start);
    let mut blocks = Vec::new();
    let mut current_block = CommentBlock::default();

    // Process the source file line-by-line. If we see a comment, add it to the intermediate block.
    // Subsequent comment lines are also added to the intermediate block until a non-comment line is reached,
    // at which point the intermediate block's contents are pushed to our accumulator, and the intermediate
    // block is reset.
    for line in lines {
        if line.starts_with(comment_prefix) {
            current_block
                .lines
                .push(line[comment_prefix.len()..].to_string());
        } else if !current_block.lines.is_empty() {
            blocks.push(mem::take(&mut current_block));
        }
    }

    // If the last processed line was a comment, we might have a non-empty intermediate block. If so, simply add it
    // to our accumulator as well.
    if !current_block.lines.is_empty() {
        blocks.push(current_block);
    }

    blocks
}

fn add_tests_from_comment_blocks(
    tests: &mut HashMap<String, Test>,
    blocks: &[CommentBlock],
) -> anyhow::Result<()> {
    for block in blocks {
        if block.lines.is_empty() {
            continue;
        }

        // Try to find a test header, e.g. "test first_example".
        let mut lines = block.lines.iter().map(|line| line.as_str());
        let header = loop {
            match lines.next() {
                Some(line) => {
                    let mut parts = line.trim_start().split_ascii_whitespace();
                    match (parts.next(), parts.next()) {
                        (Some("test"), Some(name)) => break Some((TestKind::Ok, name)),
                        (Some("test_err"), Some(name)) => break Some((TestKind::Err, name)),
                        _ => (),
                    }
                }
                None => break None,
            }
        };

        // If this comment block doesn't have a test header, continue.
        let (kind, name) = match header {
            Some(header) => header,
            None => continue,
        };

        // Check for an existing test with the given name.
        if tests.contains_key(name) {
            return Err(anyhow!("duplicate test name: {}", name));
        }

        let text = lines.collect::<Vec<_>>().join("\n");
        if !text.is_empty() {
            tests.insert(
                name.to_string(),
                Test {
                    kind,
                    name: name.to_string(),
                    text,
                },
            );
        }
    }

    Ok(())
}

pub(crate) fn run(filters: &[String]) -> anyhow::Result<()> {
    let update_patterns: HashSet<&str> = filters.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
    let mut tests: HashMap<String, Test> = HashMap::new();
    let source_idr = project_root().join("starpls_parser/src");

    Ok(())
}
