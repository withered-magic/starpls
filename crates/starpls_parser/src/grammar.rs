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

pub(crate) fn type_(p: &mut Parser) {
    union_type(p);
    if !p.at(EOF) {
        p.error_recover_until("Unexpected token", SyntaxKindSet::new(&[]));
    }
}
