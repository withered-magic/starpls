use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db as _};
use starpls_hir::{lower, Db as _, DisplayWithDb};
use starpls_syntax::{
    ast::{self, AstNode, AstPtr},
    SyntaxKind::*,
    TextRange, T,
};
use std::fmt::Write;

mod docs;

pub struct Markup {
    pub value: String,
}

pub struct Hover {
    pub contents: Markup,
    pub range: Option<TextRange>,
}

impl From<String> for Hover {
    fn from(value: String) -> Self {
        Self {
            contents: Markup { value },
            range: None,
        }
    }
}

pub(crate) fn hover(db: &Database, FilePosition { file_id, pos }: FilePosition) -> Option<Hover> {
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;

    // Check for keyword hovers first.
    if token.kind().is_keyword() {
        let text = match token.kind() {
            BREAK => docs::BREAK_DOCS,
            CONTINUE => docs::CONTINUE_DOCS,
            DEF => docs::DEF_DOCS,
            FOR => docs::FOR_DOCS,
            IF => docs::IF_DOCS,
            LOAD => docs::LOAD_DOCS,
            PASS => docs::PASS_DOCS,
            RETURN => docs::RETURN_DOCS,
            _ => return None,
        };
        return Some(text.to_string().into());
    }

    // Otherwise, provide hover information for identifiers.
    let parent = token.parent()?;
    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let expr_ptr = AstPtr::new(&ast::Expression::cast(name_ref.syntax().clone())?);
        let expr = *lower(db, file).source_map(db).expr_map.get(&expr_ptr)?;
        let ty = db.infer_expr(file, expr);
        let mut text = String::new();
        text.push_str("```python\n");

        // Handle special `def` formatting for function types.
        if ty.is_user_defined_fn() {
            text.push_str("(function) ");
        } else {
            text.push_str("(variable) ");
            text.push_str(name_ref.name()?.text());
            text.push_str(": ");
        }

        write!(&mut text, "{}", ty.display(db)).unwrap();
        text.push_str("\n```\n");
        return Some(text.into());
    } else if let Some(name) = ast::Name::cast(parent.clone()) {
        let parent = name.syntax().parent()?;
        if let Some(dot_expr) = ast::DotExpr::cast(parent.clone()) {
            let receiver_ptr = AstPtr::new(&dot_expr.expr()?);
            let receiver_expr = *lower(db, file).source_map(db).expr_map.get(&receiver_ptr)?;
            let receiver_ty = db.infer_expr(file, receiver_expr);
            let field_ty = receiver_ty
                .fields(db)?
                .iter()
                .find_map(|(field_name, ty)| {
                    (field_name.as_str() == name.syntax().text()).then_some(ty.clone())
                })?;

            let mut text = String::new();
            text.push_str("```python\n(field) ");
            name.syntax().text().for_each_chunk(|s| text.push_str(s));
            text.push_str(": ");
            write!(&mut text, "{}", field_ty.display(db)).unwrap();
            text.push_str("\n```\n");
            return Some(text.into());
        } else if let Some(def_stmt) = ast::DefStmt::cast(parent.clone()) {
            let ptr = AstPtr::new(&ast::Statement::cast(def_stmt.syntax().clone())?);
            let info = lower(db, file);
            let def_stmt = info.source_map(db).stmt_map.get(&ptr)?;
            let func = info.module(db).function_for_stmt(*def_stmt)?;
            return Some(format!("```python\n(function) {}\n```\n", func.ty().display(db)).into());
        }
    }
    None
}
