use crate::{grammar::*, marker::CompletedMarker, Parser, SyntaxKind, SyntaxKindSet, T};

/// Set of all tokens that can start a primary expression.
pub(crate) const PRIMARY_EXPR_START: SyntaxKindSet = SyntaxKindSet::new(&[
    INT,
    FLOAT,
    STRING,
    BYTES,
    T![ident],
    // tuples
    // T!['('],
    // lists and list comprehensions
    // T!['['],
    // dicts and dict comprehensions
    // T!['{'],
]);

/// Set of all possible tokens that can start an expression.
pub(crate) const EXPR_START: SyntaxKindSet = PRIMARY_EXPR_START.union(SyntaxKindSet::new(&[
    // if expression, e.g. `x if cond else y`
    // T![if],
    // unary expressions
    T![+],
    T![-],
    T![~],
    T![not],
    // lambda expression
    // T![lambda],
]));

pub(crate) fn binary_expr(
    p: &mut Parser,
    tokens: &[SyntaxKind],
    next: fn(&mut Parser) -> Option<CompletedMarker>,
) -> Option<CompletedMarker> {
    let mut m = match next(p) {
        Some(m) => m,
        None => return None,
    };

    while tokens.contains(&p.current()) {
        let binary_marker = m.precede(p);
        p.bump_any();
        next(p);
        m = binary_marker.complete(p, BINARY_EXPR);
    }

    Some(m)
}

pub(crate) fn or_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![or]], and_expr)
}

fn and_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![and]], comparison_expr)
}

fn comparison_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(
        p,
        &[T![==], T![!=], T![<], T![>], T![<=], T![>=], T![in]],
        bitwise_or_expr,
    )
}

fn bitwise_or_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![|]], bitwise_xor_expr)
}

fn bitwise_xor_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![^]], bitwise_and_expr)
}

fn bitwise_and_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![&]], shift_expr)
}

fn shift_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![<<], T![>>]], term_expr)
}

fn term_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![+], T![-]], factor_expr)
}

fn factor_expr(p: &mut Parser) -> Option<CompletedMarker> {
    binary_expr(p, &[T![*], T![/], T!["//"], T![%]], unary_expr)
}

fn unary_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if [T![+], T![-], T![~], T![not]].contains(&p.current()) {
        let m = p.start();
        p.bump_any();
        unary_expr(p);
        return Some(m.complete(p, UNARY_EXPR));
    }
    primary_expr(p)
}

fn primary_expr(p: &mut Parser) -> Option<CompletedMarker> {
    Some(match p.current() {
        INT | FLOAT | STRING | BYTES | T![True] | T![False] | T![None] => {
            let m = p.start();
            p.bump_any();
            m.complete(p, LITERAL_EXPR)
        }
        T![ident] => {
            let m = p.start();
            p.bump(T![ident]);
            m.complete(p, IDENT_EXPR)
        }
        _ => {
            p.error_recover("Expected expression", STMT_RECOVERY);
            return None;
        }
    })
}
