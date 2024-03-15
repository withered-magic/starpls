use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db as _};
use starpls_hir::{DisplayWithDb, Semantics};
use starpls_syntax::{
    ast::{self, AstNode, Direction},
    T,
};
use std::fmt::Write;

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
        // '(', ')', and ',' are typically the main tokens in a call expression that are not part of
        // one of the arguments.
        T!['('] | T![')'] | T![,] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;

    // Find the argument node containing the current token.
    let expr = token.parent_ancestors().find_map(ast::CallExpr::cast)?;
    let func = sema.resolve_call_expr(file, &expr)?;
    let params = func.params(db);
    let param_labels: Vec<String> = params
        .iter()
        .map(|(param, ty)| {
            let mut s = String::new();
            if param.is_args_list(db) {
                s.push('*');
            } else if param.is_kwargs_dict(db) {
                s.push_str("**");
            }
            if let Some(name) = param.name(db) {
                s.push_str(name.as_str());
                s.push_str(": ");
                let _ = write!(&mut s, "{}", ty.display(db));
            }
            s
        })
        .collect();

    // Construct the labels for the function signature.
    // TODO(withered-magic): Some of this logic is duplicated from the `DisplayWithDb` implementation on `TyKind`.
    let mut label = String::new();
    label.push_str("def ");
    label.push_str(func.name(db).as_str());
    label.push('(');

    for (index, param_label) in param_labels.iter().enumerate() {
        if index > 0 {
            label.push_str(", ");
        }
        label.push_str(&param_label);
    }

    label.push_str(") -> None");

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
            label,
            documentation: None,
            parameters: Some(
                param_labels
                    .into_iter()
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
