use starpls_parser::SyntaxKind;

pub use {
    crate::{
        ast::Module,
        parser::{line_index, parse_module, Parse, SyntaxError},
    },
    line_index::LineIndex,
    rowan::TextRange,
};

mod ast;
mod parser;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StarlarkLanguage {}

impl rowan::Language for StarlarkLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.0.into()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<StarlarkLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<StarlarkLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<StarlarkLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<StarlarkLanguage>;
