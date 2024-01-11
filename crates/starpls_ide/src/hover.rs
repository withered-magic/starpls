use crate::{util::pick_best_token, Database, FilePosition};
use starpls_common::{parse, Db};
use starpls_hir::{lower, FileExprId, TyCtxtSnapshot};
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

pub(crate) fn hover(
    db: &Database,
    tcx: &TyCtxtSnapshot,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Hover> {
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
    if let Some(name_ref) = ast::NameRef::cast(parent) {
        let expr_ptr = AstPtr::new(&ast::Expression::cast(name_ref.syntax().clone())?);
        let expr = *lower(db, file).source_map(db).expr_map.get(&expr_ptr)?;
        let ty = tcx.type_of_expr(db, FileExprId { file, expr });
        let mut text = String::new();
        text.push_str("```text\n");
        text.push_str("(variable) ");
        name_ref
            .syntax()
            .text()
            .for_each_chunk(|s| text.push_str(s));
        text.push_str(": ");
        write!(&mut text, "{}", ty).unwrap();
        text.push_str("\n```\n");
        return Some(text.into());
    }

    None
}
