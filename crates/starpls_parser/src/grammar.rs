use crate::{
    grammar::{arguments::*, expressions::*, parameters::*, statements::*, type_comments::*},
    syntax_kind::SyntaxKindSet,
    Parser,
    SyntaxKind::*,
    T,
};

mod arguments;
mod expressions;
mod parameters;
mod statements;
mod type_comments;

pub(crate) fn module(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        statement(p);
    }
    m.complete(p, MODULE);
}

pub(crate) fn type_comment_body(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        T![ignore] => {
            let m = p.start();
            p.bump(T![ignore]);
            m.complete(p, IGNORE_TYPE);
        }
        T!['('] => {
            function_type(p);
        }
        _ => union_type(p),
    }

    if p.at(T!['(']) {
        function_type(p);
    } else {
        union_type(p);
    }

    // We only parse one type, so if there's any remaining tokens, add them
    // to an error node.
    if !p.at(EOF) {
        p.error_recover_until("Unexpected token", SyntaxKindSet::new(&[]));
    }
    m.complete(p, TYPE_COMMENT_BODY);
}
