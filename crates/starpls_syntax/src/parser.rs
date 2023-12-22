use std::marker::PhantomData;

use crate::{ast::AstNode, SyntaxNode};
use rowan::{GreenNode, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub message: String,
    pub range: TextRange,
}

/// The result of parsing a Starlark module and constructing a Rowan syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    fn new(green: GreenNode) -> Self {
        Parse {
            green,
            _ty: PhantomData,
        }
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    pub fn tree(&self) -> T {
        T::cast(self.syntax()).unwrap()
    }
}
