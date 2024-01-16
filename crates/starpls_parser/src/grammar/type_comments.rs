use crate::{grammar::*, marker::CompletedMarker, syntax_kind::SyntaxKindSet, SyntaxKind};

const TYPE_START: SyntaxKindSet = SyntaxKindSet::new(&[T![ident], T!['(']]);

const EMPTY: SyntaxKindSet = SyntaxKindSet::new(&[]);

pub(crate) fn types(p: &mut Parser, stop: Option<SyntaxKind>) {
    let cond = |p: &mut Parser| match stop {
        Some(stop) => !p.at(EOF) && !p.at(stop),
        None => !p.at(EOF),
    };
    type_(p);
    while cond(p) {
        if !p.eat(T![,]) {
            p.error_recover_until("Expected \",\"", EMPTY);
            break;
        }
        if !p.at_kinds(TYPE_START) {
            p.error_recover_until("Expected type", EMPTY);
            break;
        }
        type_(p);
    }
}

pub(crate) fn type_(p: &mut Parser) -> Option<CompletedMarker> {
    Some(match p.current() {
        T![None] => {
            let m = p.start();
            p.bump(T![None]);
            m.complete(p, NONE_TYPE)
        }
        T![ident] => {
            let m = p.start();
            p.bump(T![ident]);
            m.complete(p, NAMED_TYPE)
        }
        T!['('] => function_type(p),
        _ => {
            p.error_recover_until("Expected type", EMPTY);
            return None;
        }
    })
}

pub(crate) fn function_type(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(T!['(']);
    if p.at_kinds(TYPE_START) {
        let m = p.start();
        types(p, Some(T![')']));
        m.complete(p, PARAMETER_TYPES);
    }
    if !p.eat(T![')']) {
        p.error_recover_until("\"(\" was not closed", EMPTY);
        return m.complete(p, FUNCTION_TYPE);
    }
    if !p.eat(ARROW) {
        p.error_recover_until("Expected \"->\"", EMPTY);
        return m.complete(p, FUNCTION_TYPE);
    }
    type_(p);
    m.complete(p, FUNCTION_TYPE)
}
