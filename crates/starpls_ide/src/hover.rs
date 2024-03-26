use crate::{
    util::{deindent_doc, pick_best_token},
    Database, FilePosition,
};
use starpls_common::{parse, Db as _};
use starpls_hir::{DisplayWithDb, Semantics};
use starpls_syntax::{
    ast::{self, AstNode},
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
    let sema = Semantics::new(db);
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
    if let Some(expr) = ast::NameRef::cast(parent.clone()) {
        let ty = sema.type_of_expr(file, &expr.clone().into())?;
        let mut text = String::from("```python\n");

        // Handle special `def` formatting for function types.
        if ty.is_function() {
            text.push_str("(function) ");
        } else {
            text.push_str("(variable) ");
            text.push_str(expr.name()?.text());
            text.push_str(": ");
        }

        write!(&mut text, "{}", ty.display(db)).unwrap();
        text.push_str("\n```\n");

        if let Some(doc) = ty.doc(db) {
            text.push_str(&deindent_doc(&doc));
            text.push('\n');
        }

        return Some(text.into());
    } else if let Some(name) = ast::Name::cast(parent.clone()) {
        let parent = name.syntax().parent()?;
        let name_token = name.name()?;
        let name_text = name_token.text();
        if let Some(expr) = ast::DotExpr::cast(parent.clone()) {
            let ty = sema.type_of_expr(file, &expr.expr()?.into())?;
            let fields = ty.fields(db);
            let (field, field_ty) = fields.into_iter().find_map(|(field, ty)| {
                if field.name(db).as_str() == name_text {
                    Some((field, ty))
                } else {
                    None
                }
            })?;

            // Handle special `def` formatting for methods.
            let mut text = String::from("```python\n");
            if field_ty.is_function() {
                text.push_str("(method) ");
            } else {
                text.push_str("(field) ");
                text.push_str(name_text);
                text.push_str(": ");
            }
            write!(&mut text, "{}", field_ty.display(db)).unwrap();
            text.push_str("\n```\n");

            let doc = field.doc(db);
            if !doc.is_empty() {
                text.push_str(&doc);
                text.push('\n');
            }

            return Some(text.into());
        } else if let Some(stmt) = ast::DefStmt::cast(parent.clone()) {
            let func = sema.function_for_def(file, stmt)?;
            let mut text = String::from("```python\n(function) ");
            write!(text, "{}\n```\n", func.ty(db).display(db)).ok()?;
            if let Some(doc) = func.doc(db) {
                text.push_str(&deindent_doc(&doc));
                text.push('\n');
            }
            return Some(text.into());
        } else if let Some(param) = ast::Parameter::cast(parent.clone()) {
            let ty = sema.type_of_param(file, &param)?;
            return Some(
                format!(
                    "```python\n(parameter) {}: {}\n```\n",
                    param.name()?,
                    ty.display(db)
                )
                .into(),
            );
        } else if let Some(arg) = ast::Argument::cast(parent) {
            let call = arg
                .syntax()
                .parent()
                .and_then(|parent| ast::Arguments::cast(parent))
                .and_then(|args| args.syntax().parent())
                .and_then(|parent| ast::CallExpr::cast(parent))?;
            let func = sema.resolve_call_expr(file, &call)?;
            let (name, param) = func.params(db).into_iter().find_map(|(param, _)| {
                let name = param.name(db)?;
                if name.as_str() == name_text {
                    Some((name, param))
                } else {
                    None
                }
            })?;

            let mut text = format!(
                "```python\n(parameter) {}: {}\n```\n",
                name.as_str(),
                param.ty(db).display(db)
            );

            if let Some(doc) = param.doc(db) {
                if !doc.is_empty() {
                    text.push_str(&doc);
                    text.push('\n');
                }
            }
            return Some(text.into());
        }
    } else if let Some(type_) = ast::NamedType::cast(parent) {
        let ty = sema.resolve_type(&type_)?;
        let mut text = format!("```python\n(type) {}\n```\n", ty.display(db));
        if let Some(doc) = ty.doc(db) {
            text.push_str(&doc);
            text.push('\n');
        }
        return Some(text.into());
    }

    None
}
