use crate::{grammar::*, syntax_kind::SyntaxKindSet};

const TYPE_START: SyntaxKindSet = SyntaxKindSet::new(&[T![ident], T!['(']]);

const EMPTY: SyntaxKindSet = SyntaxKindSet::new(&[]);

pub(crate) fn types(p: &mut Parser) {
    type_(p);
    while !p.at(EOF) {
        if !p.eat(T![,]) {
            p.error_recover_until("Expected \",\"", EMPTY);
            break;
        }
        if !p.at_kinds(TYPE_START) {
            p.error_recover_until("Expected type", EMPTY);
        }
        type_(p);
    }
}

pub(crate) fn type_(p: &mut Parser) {
    let m = p.start();
    if !p.eat(T![ident]) {
        p.error_recover_until("Expected type", EMPTY);
    }
    m.complete(p, TYPE);
}
