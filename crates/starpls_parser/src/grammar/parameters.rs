use crate::{grammar::*, syntax_kind::SyntaxKindSet};

pub(crate) const PARAMETER_START: SyntaxKindSet = SyntaxKindSet::new(&[T![ident], T![*], T![**]]);

/// Grammar: `Parameters = Parameter {',' Parameter}.`
pub(crate) fn parameters(p: &mut Parser) {
    // let m = p.start();
    parameter(p);
    while p.at(T![,]) && PARAMETER_START.contains(p.nth(1)) {
        p.bump(T![,]);
        parameter(p);
    }
    // m.complete(p, PARAMETERS);
    p.eat(T![,]);
}

/// Grammar: `Parameter  = identifier | identifier '=' Test | '*' | '*' identifier | '**' identifier`
pub(crate) fn parameter(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        T![*] => {
            p.bump(T![*]);
            name(p);
            m.complete(p, ARGS_LIST_PARAMETER);
        }
        T![**] => {
            p.bump(T![**]);
            if name(p).is_none() {
                p.error("Expected identifier")
            }
            m.complete(p, KWARGS_LIST_PARAMETER);
        }
        T![ident] => {
            assert!(name(p).is_some());
            if p.eat(T![=]) {
                if p.at_kinds(EXPR_START) {
                    test(p);
                } else {
                    p.error("Expected expression")
                }
            }
            m.complete(p, SIMPLE_PARAMETER);
        }
        _ => unreachable!(),
    }
}
