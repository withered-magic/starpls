use crate::{
    grammar::{arguments::*, expressions::*, parameters::*, statements::*, type_comments::*},
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

pub(crate) fn type_list(p: &mut Parser) {
    let m = p.start();
    types(p, None);
    m.complete(p, TYPE_LIST);
}
