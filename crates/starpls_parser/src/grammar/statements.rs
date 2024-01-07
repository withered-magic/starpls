use crate::{grammar::*, syntax_kind::SyntaxKindSet, SyntaxKind};

pub(crate) const SMALL_STMT_START: SyntaxKindSet = EXPR_START.union(SyntaxKindSet::new(&[
    T![return],
    T![break],
    T![continue],
    T![pass],
    T![load],
]));

pub(crate) const STMT_RECOVERY: SyntaxKindSet = SyntaxKindSet::new(&[T!['\n']]);

const SUITE_START: SyntaxKindSet = SMALL_STMT_START.union(SyntaxKindSet::new(&[T!['\n']]));

/// Parses a statement or blank line.
///
/// Grammar: `File = {Statement | newline} eof .
/// Statement = DefStmt | IfStmt | ForStmt | SimpleStmt .`
pub(crate) fn statement(p: &mut Parser) {
    match p.current() {
        T![def] => def_stmt(p),
        T![if] => if_stmt(p, T![if]),
        T![for] => for_stmt(p),
        T![load] => load_stmt(p),
        kind if SMALL_STMT_START.contains(kind) => simple_stmt(p),

        // Blank lines are valid.
        T!['\n'] => p.bump_any(),

        // Handle unexpected indented blocks by wrapping them with an error node.
        INDENT => error_block(p),

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
    if name(p).is_none() {
        p.error_recover_until("Expected function name", STMT_RECOVERY);
        m.complete(p, DEF_STMT);
        return;
    }

    // Parse the parameter list. If we don't see an opening '(' but are at a ':', we can emit
    // an error for the missing parameter list and recover.
    if p.eat(T!['(']) {
        if p.at_kinds(PARAMETER_START) {
            parameters(p);
        }

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

    if p.at_kinds(SUITE_START) {
        suite(p);
    } else {
        p.error_recover_until("Expected statement suite", STMT_RECOVERY);
    }

    m.complete(p, DEF_STMT);
}

/// Parses an `if` statement.
///
/// Gramar: `IfStmt = 'if' Test ':' Suite {'elif' Test ':' Suite} ['else' ':' Suite] .`
pub(crate) fn if_stmt(p: &mut Parser, if_or_elif: SyntaxKind) {
    let m = p.start();
    p.bump(if_or_elif);

    if test(p).is_none() {
        m.complete(p, IF_STMT);
        return;
    }

    if !p.eat(T![:]) {
        p.error_recover_until("Expected \":\"", STMT_RECOVERY);
        m.complete(p, IF_STMT);
        return;
    }

    if p.at_kinds(SUITE_START) {
        suite(p);
    } else {
        p.error_recover_until("Expected statement suite", STMT_RECOVERY);
    }

    match p.current() {
        T![elif] => if_stmt(p, T![elif]),
        T![else] => {
            p.bump(T![else]);
            if p.eat(T![:]) {
                if p.at_kinds(SUITE_START) {
                    suite(p);
                } else {
                    p.error_recover_until("Expected statement suite", STMT_RECOVERY);
                }
            } else {
                p.error_recover_until("Expected \":\"", STMT_RECOVERY);
            }
        }
        _ => (),
    }

    m.complete(p, IF_STMT);
}

/// Parses a `for` statement.
///
/// Grammar: `ForStmt = 'for' LoopVariables 'in' Expression ':' Suite .`
pub(crate) fn for_stmt(p: &mut Parser) {
    // test test_for_stmt_full
    // for i, value in enumerate(values):
    //     print(i, value)
    let m = p.start();
    p.bump(T![for]);

    // test_err test_for_stmt_missing_loop_variables
    // for
    if !PRIMARY_EXPR_START.contains(p.current()) {
        p.error_recover_until("Expected loop variables", STMT_RECOVERY);
        m.complete(p, FOR_STMT);
        return;
    }

    loop_variables(p);

    // test_err test_for_stmt_missing_in
    // for x * y
    if !p.eat(T![in]) {
        p.error_recover_until("Expected \"in\"", STMT_RECOVERY);
        m.complete(p, FOR_STMT);
        return;
    }

    // test_err test_for_stmt_missing_iterable
    // for x in:
    if !p.at_kinds(EXPR_START) {
        p.error_recover_until("Expected expression", STMT_RECOVERY);
        m.complete(p, FOR_STMT);
        return;
    }

    tuple_or_paren_expr(p, false);

    // test_err test_for_stmt_missing_colon
    // for x in y
    if !p.eat(T![:]) {
        p.error_recover_until("Expected \":\"", STMT_RECOVERY);
        m.complete(p, IF_STMT);
        return;
    }

    // test_err test_for_stmt_missing_suite
    // for x in y:
    if p.at_kinds(SUITE_START) {
        suite(p);
    } else {
        p.error_recover_until("Expected statement suite", STMT_RECOVERY);
    }

    m.complete(p, FOR_STMT);
}

/// Parses a semicolon-delimited list of small statements.
///
/// Grammar: `SimpleStmt = SmallStmt {';' SmallStmt} [';'] '\n' .`
pub(crate) fn simple_stmt(p: &mut Parser) {
    small_stmt(p);
    while p.at(T![;]) && SMALL_STMT_START.contains(p.nth(1)) {
        p.bump(T![;]);
        small_stmt(p);
    }

    p.eat(T![;]);

    // Simple statements need to end with a newline. If we aren't at one, recover to it,
    // discarding all tokens in-between.
    if !p.at(EOF) && !p.at(DEDENT) && !p.eat(T!['\n']) {
        p.error_recover_until("Expected newline", STMT_RECOVERY);
    }
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

/// Grammar: `ReturnStmt = 'return' [Expression] .`
pub(crate) fn return_stmt(p: &mut Parser) {
    // test test_return_stmt
    // return
    // return 1 + 2
    let m = p.start();
    p.bump(T![return]);
    if p.at_kinds(EXPR_START) {
        tuple_or_paren_expr(p, false);
    }
    m.complete(p, RETURN_STMT);
}

/// Grammar: `BreakStmt = 'break' .`
pub(crate) fn break_stmt(p: &mut Parser) {
    // test test_break_stmt
    // break
    let m = p.start();
    p.bump(T![break]);
    m.complete(p, BREAK_STMT);
}

/// Grammar: `ContinueStmt = 'continue' .`
pub(crate) fn continue_stmt(p: &mut Parser) {
    // test test_continue_stmt
    // continue
    let m = p.start();
    p.bump(T![continue]);
    m.complete(p, CONTINUE_STMT);
}

/// Grammar: `PassStmt = 'pass' .`
pub(crate) fn pass_stmt(p: &mut Parser) {
    // test test_pass_stmt
    // pass
    let m = p.start();
    p.bump(T![pass]);
    m.complete(p, PASS_STMT);
}

/// Grammar: `LoadStmt = 'load' '(' string {',' [identifier '='] string} [','] ')' .`
pub(crate) fn load_stmt(p: &mut Parser) {
    let m = p.start();
    p.bump(T![load]);
    if !p.eat(T!['(']) {
        p.error_recover_until("Expected \"(\"", STMT_RECOVERY);
        m.complete(p, LOAD_STMT);
        return;
    }
    if !p.eat(STRING) {
        p.error_recover_until("Expected module name", STMT_RECOVERY);
        m.complete(p, LOAD_STMT);
        return;
    }
    while p.at(T![,]) && matches!(p.nth(1), T![ident] | STRING) {
        p.bump(T![,]);
        match p.current() {
            T![ident] => {
                let m = p.start();
                assert!(name_ref(p).is_some());
                if !p.eat(T![=]) {
                    p.error("Expected \"=\"");
                } else if !p.eat(STRING) {
                    p.error("Expected item name");
                }
                m.complete(p, ALIASED_LOAD_ITEM);
            }
            STRING => {
                let m = p.start();
                p.bump(STRING);
                m.complete(p, DIRECT_LOAD_ITEM);
            }
            _ => unreachable!(),
        }
    }
    p.eat(T![,]);
    if !p.eat(T![')']) {
        p.error_recover_until("\"(\" was not closed", STMT_RECOVERY);
        m.complete(p, LOAD_STMT);
        return;
    }
    m.complete(p, LOAD_STMT);
}

/// Grammar: `AssignStmt = Expression ('=' | '+=' | '-=' | '*=' | '/=' | '//=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>=') Expression .`
pub(crate) fn assign_or_expr_stmt(p: &mut Parser) {
    let mut completed_marker = tuple_or_paren_expr(p, false);

    if !matches!(
        p.current(),
        T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T!["//="]
            | T![%=]
            | T![&=]
            | T![|=]
            | T![^=]
            | T![<<=]
            | T![>>=]
    ) {
        return;
    }

    let m = completed_marker.precede(p);

    p.bump_any();

    if !p.at_kinds(EXPR_START) {
        p.error_recover_until("Expected expression", STMT_RECOVERY);
        m.complete(p, ASSIGN_STMT);
        return;
    }

    tuple_or_paren_expr(p, false);
    m.complete(p, ASSIGN_STMT);
}

/// Grammar: `Suite = [newline indent {Statement} outdent] | SimpleStmt .`
fn suite(p: &mut Parser) {
    // test test_suite_full
    // def f(x, y): print(x); print(y)
    // def g(x, y):
    //     print(x)
    //     print(y)
    let m = p.start();
    match p.current() {
        T!['\n'] => {
            p.bump(T!['\n']);
            if p.eat(INDENT) {
                while !p.at(EOF) && !p.at(DEDENT) {
                    statement(p);
                }
                if !p.eat(DEDENT) {
                    p.error("Expected dedentation");
                }
            } else {
                p.error("Expected indentation");
            }
        }
        _ if p.at_kinds(SMALL_STMT_START) => {
            simple_stmt(p);
        }
        _ => unreachable!(),
    }
    m.complete(p, SUITE);
}

/// Grammar: `LoopVariables = PrimaryExpr {',' PrimaryExpr} .`
pub(crate) fn loop_variables(p: &mut Parser) {
    let m = p.start();
    primary_expr(p);
    while p.at(T![,]) && PRIMARY_EXPR_START.contains(p.nth(1)) {
        p.bump(T![,]);
        primary_expr(p);
    }
    m.complete(p, LOOP_VARIABLES);
}

fn error_block(p: &mut Parser) {
    // test_err test_error_block
    // x = 1
    //     y = 2
    //     z = 3
    // a = 4
    let m = p.start();
    let mut level = 1;
    p.error("Unexpected indentation");
    p.bump(INDENT);

    while !p.at(EOF) {
        let kind = p.current();
        p.bump_any();
        match kind {
            INDENT => level += 1,
            DEDENT => {
                level -= 1;
                if level == 0 {
                    break;
                }
            }
            _ => (),
        }
    }

    m.complete(p, ERROR);
}
