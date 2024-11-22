use crate::grammar::*;
use crate::syntax_kind::SyntaxKindSet;

pub(crate) const ARGUMENT_START: SyntaxKindSet =
    EXPR_START.union(SyntaxKindSet::new(&[T![**], T![*]]));

pub(crate) fn arguments(p: &mut Parser) {
    argument(p);
    while p.at(T![,]) && ARGUMENT_START.contains(p.nth(1)) {
        p.bump(T![,]);
        argument(p);
    }
    p.eat(T![,]);
}

pub(crate) fn argument(p: &mut Parser) {
    // test test_arguments_all
    // f(x, *args, y=1, **kwargs)
    let m = p.start();
    match p.current() {
        T![*] => {
            p.bump(T![*]);
            test(p);
            m.complete(p, UNPACKED_LIST_ARGUMENT);
        }
        T![**] => {
            p.bump(T![**]);
            test(p);
            m.complete(p, UNPACKED_DICT_ARGUMENT);
        }
        T![ident] if p.nth(1) == T![=] => {
            name(p);
            p.bump(T![=]);
            test(p);
            m.complete(p, KEYWORD_ARGUMENT);
        }
        _ => {
            test(p);
            m.complete(p, SIMPLE_ARGUMENT);
        }
    }
}
