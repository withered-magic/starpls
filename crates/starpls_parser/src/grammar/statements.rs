use crate::{grammar::*, syntax_kind::SyntaxKindSet};

pub(crate) const SMALL_STMT_START: SyntaxKindSet = EXPR_START.union(SyntaxKindSet::new(&[
    T![return],
    T![break],
    T![continue],
    T![pass],
    T![load],
]));

pub(crate) const STMT_RECOVERY: SyntaxKindSet = SyntaxKindSet::new(&[T!['\n']]);

/// Parses a statement or blank line.
///
/// Grammar: `File = {Statement | newline} eof .
/// Statement = DefStmt | IfStmt | ForStmt | SimpleStmt .`
pub(crate) fn statement(p: &mut Parser) {
    match p.current() {
        T![def] => def_stmt(p),
        T![if] => if_stmt(p),
        T![for] => for_stmt(p),
        kind if SMALL_STMT_START.contains(kind) => simple_stmt(p),

        // Blank lines are valid.
        T!['\n'] => p.bump_any(),

        // Recover to the next newline, leaving it to be processed by the next call to `statement()`.
        _ => p.error_recover_until("Expected statement", STMT_RECOVERY),
    }
}

/// Parses a function definition.
///
/// Grammar: `DefStmt = 'def' identifier '(' [Parameters [',']] ')' ':' Suite .`
pub(crate) fn def_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![def]);

    // Parse the function name.
    if !p.eat(T![ident]) {
        p.error_recover_until("Expected function name", STMT_RECOVERY);
        m.complete(p, DEF_STMT);
        return;
    }

    // Parse the parameter list. If we don't see an opening '(' but are at a ':', we can emit
    // an error for the missing parameter list and recover.
    if p.eat(T!['(']) {
        if !p.eat(T![')']) {
            p.error_recover_until("\"(\" was not closed", STMT_RECOVERY);
            m.complete(p, DEF_STMT);
            return;
        }
    } else {
        if p.current() != T![:] {
            p.error_recover_until("Expected parameter list", STMT_RECOVERY);
            m.complete(p, DEF_STMT);
            return;
        }
        p.error("Expected parameter list")
    }

    if !p.eat(T![:]) {
        p.error_recover_until("Expected \":\"", STMT_RECOVERY);
        m.complete(p, DEF_STMT);
        return;
    }

    m.complete(p, DEF_STMT);
}

/// Parses an `if` statement.
///
/// Gramar: `IfStmt = 'if' Test ':' Suite {'elif' Test ':' Suite} ['else' ':' Suite] .`
pub(crate) fn if_stmt(p: &mut Parser) {
    p.bump(T![if]);
}

/// Parses a `for` statement.
///
/// Grammar: `ForStmt = 'for' LoopVariables 'in' Expression ':' Suite .`
pub(crate) fn for_stmt(p: &mut Parser) {
    p.bump(T![for]);
}

/// Parses a semicolon-delimited list of small statements.
///
/// Grammar: `SimpleStmt = SmallStmt {';' SmallStmt} [';'] '\n' .`
pub(crate) fn simple_stmt(p: &mut Parser) {
    let m = p.start();
    small_stmt(p);
    while p.at(T![;]) && SMALL_STMT_START.contains(p.nth(1)) {
        p.bump(T![;]);
        small_stmt(p);
    }

    p.eat(T![;]);

    // Simple statements need to end with a newline. If we aren't at one, recover to it,
    // discarding all tokens in-between.
    if !p.at(EOF) && !p.eat(T!['\n']) {
        p.error_recover("Expected newline", STMT_RECOVERY);
    }

    m.complete(p, SIMPLE_STMT);
}

/// Parses a small statement.
///
/// Grammar: `SmallStmt = ReturnStmt | BreakStmt | ContinueStmt | PassStmt | AssignStmt | ExprStmt | LoadStmt .`
pub(crate) fn small_stmt(p: &mut Parser) {
    match p.current() {
        T![return] => return_stmt(p),
        T![break] => break_stmt(p),
        T![continue] => continue_stmt(p),
        T![pass] => pass_stmt(p),
        kind if EXPR_START.contains(kind) => assign_or_expr_stmt(p),

        // Guaranteed by `simple_stmt` and `small_stmt` that we will match one of the above cases.
        _ => unreachable!(),
    }
}

pub(crate) fn return_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![return]);
    m.complete(p, RETURN_STMT);
}

pub(crate) fn break_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![break]);
    m.complete(p, BREAK_STMT);
}

pub(crate) fn continue_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![continue]);
    m.complete(p, CONTINUE_STMT);
}

pub(crate) fn pass_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![pass]);
    m.complete(p, PASS_STMT);
}

pub(crate) fn assign_or_expr_stmt(p: &mut Parser) {
    or_expr(p);
}
