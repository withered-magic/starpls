use crate::{
    grammar::*,
    marker::{CompletedMarker, Marker},
    Parser, SyntaxKind, SyntaxKindSet, T,
};

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

/// Grammar: `BinaryExpr = Test {Binop Test} .`
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

/// Grammar: `Test = IfExpr | PrimaryExpr | UnaryExpr | BinaryExpr | LambdaExpr .`
pub(crate) fn test(p: &mut Parser) -> Option<CompletedMarker> {
    if_expr(p)
}

/// Grammar: `IfExpr = Test 'if' Test 'else' Test .`
fn if_expr(p: &mut Parser) -> Option<CompletedMarker> {
    let mut completed_marker = or_expr(p)?;
    if !p.at(T![if]) {
        return Some(completed_marker);
    }
    let m = completed_marker.precede(p);
    p.bump(T![if]);
    test(p);
    if !p.eat(T![else]) {
        p.error_recover_until("Expected \"else\"", STMT_RECOVERY);
        return Some(m.complete(p, IF_EXPR));
    }
    test(p);
    Some(m.complete(p, IF_EXPR))
}

fn or_expr(p: &mut Parser) -> Option<CompletedMarker> {
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

/// Grammar: `UnaryExpr = '+' Test | '-' Test | '~' Test | 'not' Test .`
fn unary_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if [T![+], T![-], T![~], T![not]].contains(&p.current()) {
        let m = p.start();
        p.bump_any();
        unary_expr(p);
        return Some(m.complete(p, UNARY_EXPR));
    }
    primary_expr(p)
}

/// Parses a function call, subscript expression, or member access.
pub(crate) fn primary_expr(p: &mut Parser) -> Option<CompletedMarker> {
    let mut m = match operand_expr(p) {
        Some(m) => m,
        None => return None,
    };

    loop {
        let next = match p.current() {
            T!['('] => call_expr,
            T![.] => dot_expr,
            T!['['] => slice_or_index_expr,
            _ => return Some(m),
        };
        let pred = m.precede(p);
        m = next(p, pred)
    }
}

/// Grammar: `CallSuffix  = '(' [Arguments [',']] ')' .`
fn call_expr(p: &mut Parser, m: Marker) -> CompletedMarker {
    p.bump(T!['(']);
    if ARGUMENT_START.contains(p.current()) {
        arguments(p);
    }
    // If we aren't at the closing paren, recover to the next newline.
    if !p.eat(T![')']) {
        p.error_recover_until("\"(\" was not closed", STMT_RECOVERY);
    }
    m.complete(p, CALL_EXPR)
}

/// Grammar: `DotSuffix = '.' identifier .`
fn dot_expr(p: &mut Parser, m: Marker) -> CompletedMarker {
    p.bump(T![.]);
    if !p.eat(T![ident]) {
        p.error_recover_until("Expected member name", STMT_RECOVERY);
    }
    m.complete(p, DOT_EXPR)
}

fn slice_or_index_expr(p: &mut Parser, m: Marker) -> CompletedMarker {
    let mut kind = INDEX_EXPR;
    p.bump(T!['[']);
    if p.eat(T![']']) {
        p.error("Slice expression cannot be empty");
        return m.complete(p, kind);
    }
    if p.at_kinds(EXPR_START) {
        tuple_or_paren_expr(p, false);
    }
    if p.eat(T![:]) {
        kind = SLICE_EXPR;
        if p.at_kinds(EXPR_START) {
            test(p);
        }
        if p.eat(T![:]) {
            if p.at_kinds(EXPR_START) {
                test(p);
            }
        }
    }
    if !p.eat(T![']']) {
        p.error_recover_until("\"[\" was not closed", STMT_RECOVERY);
    }
    m.complete(p, kind)
}

/// Grammar: `Operand = identifier | int | float | string | bytes | ListExpr | ListComp | DictExpr | DictComp | '(' [Expression [',']] ')' .`
fn operand_expr(p: &mut Parser) -> Option<CompletedMarker> {
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
        T!['('] => tuple_or_paren_expr(p, true),
        T!['['] => list_expr_or_comp(p),
        T!['{'] => dict_expr_or_comp(p),
        T![lambda] => lambda_expr(p),
        _ => {
            p.error_recover_until("Expected expression", STMT_RECOVERY);
            return None;
        }
    })
}

/// Grammar: `LambdaExpr = 'lambda' [Parameters] ':' Test .`
fn lambda_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![lambda]);
    if p.at_kinds(PARAMETER_START) {
        parameters(p);
    }
    if !p.eat(T![:]) {
        p.error_recover_until("Expected \":\"", STMT_RECOVERY);
        return m.complete(p, LAMBDA_EXPR);
    }
    test(p);
    m.complete(p, LAMBDA_EXPR)
}

/// Grammar: `Expression = Test {',' Test} .`
pub(crate) fn tuple_or_paren_expr(p: &mut Parser, is_enclosed_in_parens: bool) -> CompletedMarker {
    let m = p.start();

    if is_enclosed_in_parens {
        p.bump(T!['(']);
        if p.eat(T![')']) {
            return m.complete(p, TUPLE_EXPR);
        }
    } else {
        assert!(p.at_kinds(EXPR_START));
    }

    let completed_marker = test(p);
    if !is_enclosed_in_parens && !p.at(T![,]) {
        m.abandon(p);
        return completed_marker.expect(
            "first expression in tuple_or_paren_expr must parse if not enclosed in parens",
        );
    }

    let mut num_parsed = 1;
    let mut has_trailing_comma = false;

    while p.at(T![,]) && EXPR_START.contains(p.nth(1)) {
        p.bump(T![,]);
        test(p);
        num_parsed += 1;
    }

    if is_enclosed_in_parens {
        has_trailing_comma = p.eat(T![,]);
    }

    if is_enclosed_in_parens && !p.eat(T![')']) {
        p.error_recover_until("\"(\" was not closed", STMT_RECOVERY);
        num_parsed += 1;
    }

    let kind = if num_parsed == 1 && !has_trailing_comma {
        PAREN_EXPR
    } else {
        TUPLE_EXPR
    };

    m.complete(p, kind)
}

/// Grammar: `ListExpr = '[' [Expression [',']] ']' . ListComp = '[' Test {CompClause} ']'.`
fn list_expr_or_comp(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    let mut kind = LIST_EXPR;
    p.bump(T!['[']);
    if p.eat(T![']']) {
        return m.complete(p, kind);
    }
    test(p);
    match p.current() {
        // CompClause = 'for' LoopVariables 'in' Test | 'if' Test .
        T![for] | T![if] => {
            kind = LIST_COMP;
            loop {
                match p.current() {
                    T![for] => {
                        let m = p.start();
                        p.bump(T![for]);
                        if !PRIMARY_EXPR_START.contains(p.current()) {
                            p.error_recover_until("Expected loop variables", STMT_RECOVERY);
                            m.complete(p, COMP_CLAUSE_FOR);
                            break;
                        }
                        loop_variables(p);
                        if !p.eat(T![in]) {
                            p.error_recover_until("Expected \"in\"", STMT_RECOVERY);
                            m.complete(p, COMP_CLAUSE_FOR);
                            break;
                        }
                        test(p);
                        m.complete(p, COMP_CLAUSE_FOR);
                    }
                    T![if] => {
                        let m = p.start();
                        p.bump(T![if]);
                        test(p);
                        m.complete(p, COMP_CLAUSE_IF);
                    }
                    _ => break,
                }
            }
        }
        _ => {
            while p.at(T![,]) && EXPR_START.contains(p.nth(1)) {
                p.bump(T![,]);
                test(p);
            }
            p.eat(T![,]);
        }
    }
    if !p.eat(T![']']) {
        p.error_recover_until("\"[\" was not closed", STMT_RECOVERY);
    }
    m.complete(p, kind)
}

/// Grammar: `DictExpr = '{' [Entries [',']] '}' . DictComp = '{' Entry {CompClause} '}' .`
fn dict_expr_or_comp(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    let mut kind = DICT_EXPR;
    p.bump(T!['{']);
    if p.eat(T!['}']) {
        return m.complete(p, kind);
    }
    entry(p);
    match p.current() {
        // CompClause = 'for' LoopVariables 'in' Test | 'if' Test .
        T![for] | T![if] => {
            kind = LIST_COMP;
            loop {
                match p.current() {
                    T![for] => {
                        let m = p.start();
                        p.bump(T![for]);
                        if !PRIMARY_EXPR_START.contains(p.current()) {
                            p.error_recover_until("Expected loop variables", STMT_RECOVERY);
                            m.complete(p, COMP_CLAUSE_FOR);
                            break;
                        }
                        loop_variables(p);
                        if !p.eat(T![in]) {
                            p.error_recover_until("Expected \"in\"", STMT_RECOVERY);
                            m.complete(p, COMP_CLAUSE_FOR);
                            break;
                        }
                        test(p);
                        m.complete(p, COMP_CLAUSE_FOR);
                    }
                    T![if] => {
                        let m = p.start();
                        p.bump(T![if]);
                        test(p);
                        m.complete(p, COMP_CLAUSE_IF);
                    }
                    _ => break,
                }
            }
        }
        _ => {
            while p.at(T![,]) && EXPR_START.contains(p.nth(1)) {
                p.bump(T![,]);
                entry(p);
            }
            p.eat(T![,]);
        }
    }
    if !p.eat(T!['}']) {
        p.error_recover_until("\"{\" was not closed", STMT_RECOVERY);
    }
    m.complete(p, kind)
}

fn entry(p: &mut Parser) {
    let m = p.start();
    test(p);
    if !p.eat(T![:]) {
        p.error_recover_until("Expected \":\"", STMT_RECOVERY);
    } else {
        test(p);
    }
    m.complete(p, DICT_ENTRY);
}
