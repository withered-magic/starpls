use crate::{
    grammar::{arguments::*, expressions::*, statements::*},
    Parser,
    SyntaxKind::*,
    T,
};

mod arguments;
mod expressions;
mod statements;

pub(crate) fn module(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        statement(p);
    }
    m.complete(p, MODULE);
}
