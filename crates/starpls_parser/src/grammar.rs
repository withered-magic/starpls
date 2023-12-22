use crate::{Parser, SyntaxKind::*, SyntaxKindSet, T};

/// Set of all tokens that can start a primary expression.
pub(crate) const PRIMARY_EXPR_START: SyntaxKindSet = SyntaxKindSet::new(&[
    INT,
    FLOAT,
    STRING,
    BYTES,
    T![ident],
    // tuples
    T!['('],
    // lists and list comprehensions
    T!['['],
    // dicts and dict comprehensions
    T!['{'],
]);

/// Set of all possible tokens that can start an expression.
pub(crate) const EXPR_START: SyntaxKindSet = PRIMARY_EXPR_START.union(SyntaxKindSet::new(&[
    // if expression, e.g. `x if cond else y`
    T![if],
    // unary expressions
    T![+],
    T![-],
    T![~],
    T![not],
    // lambda expression
    T![lambda],
]));

pub(crate) fn module(p: &mut Parser) {}
