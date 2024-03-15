use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db as _};
use starpls_hir::Semantics;
use starpls_syntax::{
    ast::{self, AstNode, Direction},
    T,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureInfo {
    pub label: String,
    pub documentation: String,
    pub parameters: Option<Vec<ParameterInfo>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterInfo {
    pub label: String,
    pub documentation: Option<String>,
}

pub(crate) fn signature_help(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<SignatureHelp> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;

    // Find the argument node containing the current token.
    let arg = token.parent_ancestors().find_map(ast::Argument::cast)?;
    let expr = ast::CallExpr::cast(arg.syntax().parent()?.parent()?)?;
    let active_arg = arg
        .syntax()
        .siblings(Direction::Prev)
        .skip(1)
        .filter_map(ast::Argument::cast)
        .count();

    let active_param = sema.resolve_call_expr_active_param(file, &expr, active_arg);

    None
}
