use crate::{
    grammar::{arguments::*, expressions::*, parameters::*, statements::*},
    Parser,
    SyntaxKind::*,
    T,
};

mod arguments;
mod expressions;
mod parameters;
mod statements;

pub(crate) fn module(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        statement(p);
    }
    m.complete(p, MODULE);
}
