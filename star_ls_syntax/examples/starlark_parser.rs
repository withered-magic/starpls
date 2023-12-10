use std::error::Error;
use tree_sitter::{InputEdit, Parser, Point};

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
