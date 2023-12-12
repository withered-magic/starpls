use std::error::Error;
use tree_sitter::{InputEdit, Parser, Point, Tree};

struct Reparser {
    ts_parser: Parser,
    tree: Option<Tree>,
}

impl Reparser {
    fn new(ts_parser: Parser) -> Self {
        Self {
            ts_parser,
            tree: None,
        }
    }

    fn reparse(&mut self, input: &str, edits: Option<Vec<InputEdit>>) -> Option<Tree> {
        // If edits have been specified, apply them to the old tree and use the old tree during reparsing.
        // Otherwise, discard the old tree and parse from scratch.
        let mut old_tree: Option<&Tree> = None;
        if let Some(edits) = edits {
            if let Some(old_tree) = self.tree.as_mut() {
                for edit in edits {
                    old_tree.edit(&edit);
                }
            }
            old_tree = self.tree.as_ref();
        }

        self.tree = self.ts_parser.parse(input, old_tree);
        self.tree.as_ref().cloned()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_starlark::language())?;

    let source_code = "def test(): pass";
    let mut tree = parser.parse(source_code, None).unwrap();

    println!("{}", tree.root_node().to_sexp());

    let new_source_code = "def test(x, y): pass";

    tree.edit(&InputEdit {
        start_byte: 9,
        old_end_byte: 9,
        new_end_byte: 12,
        start_position: Point::new(0, 9),
        old_end_position: Point::new(0, 9),
        new_end_position: Point::new(0, 12),
    });

    let new_tree = parser.parse(new_source_code, Some(&tree)).unwrap();

    println!("{}", new_tree.root_node().to_sexp());

    Ok(())
}
