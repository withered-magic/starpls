use crate::{
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};
use std::marker::PhantomData;

/// A trait that allows converting between untyped `SyntaxNode`s and typed AST nodes.
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

/// A trait that allows converting between untyped `SyntaxToken`s and typed AST tokens.
pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// A macro for defining AST nodes. The `AstNode` trait is automatically implemented.
macro_rules! ast_node {
    (
        $(#[doc = $doc:expr])*$node:ident => $kind:ident
        $(child $($child:ident -> $child_node:ident),+;)*
        $(children $($children:ident -> $children_node:ident),+;)*
    ) => {
        $(#[doc = $doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $node {
            pub(crate) syntax: SyntaxNode,
        }

        impl AstNode for $node {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == $kind
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }

        impl $node {
        $($(
            pub fn $child(&self) -> Option<$child_node> {
                child(&self.syntax)
            }
        )+)*

        $($(
            pub fn $children(&self) -> AstChildren<$children_node> {
                AstChildren::new(&self.syntax)
            }
        )+)*
        }
    };
}

pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    phantom: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            phantom: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}

fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|element| element.into_token())
        .find(|token| token.kind() == kind)
}

ast_node! {
    /// A Starlark module. This is typically the root of the AST.
    Module => MODULE
}

ast_node! {
    /// A function definition.
    DefStmt => DEF_STMT
}

ast_node! {
    /// An `if` statement.
    IfStmt => IF_STMT
}

ast_node! {
    /// A `for` statement.
    ForStmt => FOR_STMT
}

ast_node! {
    /// A list of one or more small statements, delimited with semicolons.
    SimpleStmt => SIMPLE_STMT
    children small_stmts -> SmallStmt;
}

ast_node! {
    ReturnStmt => RETURN_STMT
}

ast_node! {
    BreakStmt => BREAK_STMT
}

ast_node! {
    ContinueStmt => CONTINUE_STMT
}

ast_node! {
    PassStmt => PASS_STMT
}

/// A statement.
pub enum Statement {
    Def(DefStmt),
    If(IfStmt),
    For(ForStmt),
    Simple(SimpleStmt),
}

impl AstNode for Statement {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, DEF_STMT | IF_STMT | FOR_STMT | SIMPLE_STMT)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match syntax.kind() {
            DEF_STMT => Self::Def(DefStmt { syntax }),
            IF_STMT => Self::If(IfStmt { syntax }),
            FOR_STMT => Self::For(ForStmt { syntax }),
            SIMPLE_STMT => Self::Simple(SimpleStmt { syntax }),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Statement::Def(DefStmt { syntax }) => syntax,
            Statement::If(IfStmt { syntax }) => syntax,
            Statement::For(ForStmt { syntax }) => syntax,
            Statement::Simple(SimpleStmt { syntax }) => syntax,
        }
    }
}

/// A small statement.
pub enum SmallStmt {
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Pass(PassStmt),
    // Assign(AssignStmt),
}
