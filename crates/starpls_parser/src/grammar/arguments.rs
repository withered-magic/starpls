use crate::{grammar::*, syntax_kind::SyntaxKindSet};

pub(crate) const ARGUMENT_START: SyntaxKindSet =
    EXPR_START.union(SyntaxKindSet::new(&[T![**], T![*]]));

pub(crate) fn arguments(p: &mut Parser) {
    let m = p.start();
    argument(p);
    while p.at(T![,]) && ARGUMENT_START.contains(p.nth(1)) {
        p.bump(T![,]);
        argument(p);
    }
    m.complete(p, ARGUMENTS);
    p.eat(T![,]);
}

pub(crate) fn argument(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        T![*] => {
            p.bump(T![*]);
            or_expr(p);
            m.complete(p, UNPACKED_LIST_ARGUMENT);
        }
        T![**] => {
            p.bump(T![**]);
            or_expr(p);
            m.complete(p, UNPACKED_DICT_ARGUMENT);
        }
        T![ident] if p.nth(1) == T![=] => {
            p.bump(T![ident]);
            p.bump(T![=]);
            or_expr(p);
            m.complete(p, KEYWORD_ARGUMENT);
        }
        _ => {
            or_expr(p);
            m.complete(p, SIMPLE_ARGUMENT);
        }
    }
}
