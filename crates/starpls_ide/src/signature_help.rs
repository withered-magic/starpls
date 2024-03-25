use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db as _};
use starpls_hir::{DisplayWithDb, Semantics};
use starpls_syntax::{
    ast::{self, AstNode, Direction},
    T,
};
use std::fmt::Write;

const DEFAULT_ACTIVE_PARAMETER_INDEX: usize = 100;

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
            }
            match param.name(db) {
                Some(name) if !name.is_missing() => {
                    s.push_str(name.as_str());
                    if !ty.is_unknown() {
                        s.push_str(": ");
                        let _ = write!(&mut s, "{}", ty.display(db));
                    }
                }
                _ => {}
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

    label.push_str(") -> ");
    let _ = write!(&mut label, "{}", func.ret_ty(db).display(db));

    // Check if token's direct parent is an `Arguments` node. If so, that means we are at a ',', '(', or ')'.
    // The active parameter index is equal to the number of commas that we see to the left (including ourselves).
    // If the number of commas is greater than the number of arguments in the CallExpr, then
    // the active parameter is considered fake.
    let active_arg = if ast::Arguments::can_cast(token.parent()?.kind()) {
        token
            .siblings_with_tokens(Direction::Prev)
            .filter_map(|el| el.into_token())
            .filter(|token| token.kind() == T![,])
            .count()
    } else {
        // Otherwise, check if there is a parent `Argument` node. If so, the active parameter index
        // is equal to the number of `Argument`s to the left of us. The active parameter is never fake
        // in this scenario.
        let arg = token.parent_ancestors().find_map(ast::Argument::cast)?;
        arg.syntax()
            .siblings(Direction::Prev)
            .skip(1)
            .filter_map(ast::Argument::cast)
            .count()
    };

    let active_parameter = sema
        .resolve_call_expr_active_param(file, &expr, active_arg)
        .unwrap_or(DEFAULT_ACTIVE_PARAMETER_INDEX); // active_parameter defaults to 0, so we just add a crazy high value here to avoid a false positive

    Some(SignatureHelp {
        signatures: vec![SignatureInfo {
            label,
            documentation: func.doc(db),
            parameters: Some(
                params
                    .into_iter()
                    .zip(param_labels.into_iter())
                    .map(|((param, _), label)| ParameterInfo {
                        label,
                        documentation: param.doc(db),
                    })
                    .collect(),
            ),
            active_parameter: Some(active_parameter),
        }],
    })
}
