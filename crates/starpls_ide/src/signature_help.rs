use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db as _};
use starpls_hir::{DisplayWithDb, Semantics};
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
    pub documentation: Option<String>,
    pub parameters: Option<Vec<ParameterInfo>>,
    pub active_parameter: Option<usize>,
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
        T!['('] | T![')'] | T![,] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;

    // Find the argument node containing the current token.
    let expr = token.parent_ancestors().find_map(ast::CallExpr::cast)?;
    let func = sema.resolve_call_expr(file, &expr)?;

    let label = format!("{}", func.ty(db).display(db));
    let start = label.find('(')? + 1;
    let end = label.find(") -> ")?;
    let param_labels = label[start..end].split(", ").map(|s| s.to_string());
    let parent = token.parent()?;

    let active_parameter = if ast::CallExpr::can_cast(parent.kind()) {
        expr.arguments()
            .map(|args| args.arguments().count())
            .unwrap_or(0)
    } else {
        let syntax = match token.parent_ancestors().find_map(ast::Argument::cast) {
            Some(arg) => arg.syntax().clone(),
            _ => parent,
        };
        let active_arg = syntax
            .siblings(Direction::Prev)
            .skip(1)
            .filter_map(ast::Argument::cast)
            .count();
        sema.resolve_call_expr_active_param(file, &expr, active_arg)
            .unwrap_or(99)
    };

    Some(SignatureHelp {
        signatures: vec![SignatureInfo {
            label: format!("{}", func.ty(db).display(db)),
            documentation: None,
            parameters: Some(
                param_labels
                    .map(|label| ParameterInfo {
                        label,
                        documentation: None,
                    })
                    .collect(),
            ),
            active_parameter: Some(active_parameter),
        }],
    })
}
