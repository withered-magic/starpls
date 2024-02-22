use crate::{util::pick_best_token, Database, FilePosition, Location};
use starpls_common::{parse, Db};
use starpls_hir::{Name, Semantics};
use starpls_syntax::{
    ast::{self, AstNode},
    T,
};

pub(crate) fn goto_definition(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Vec<Location>> {
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let parent = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?
    .parent()?;

    // For now, we only handle identifiers.
    if let Some(name_ref) = ast::NameRef::cast(parent) {
        let name = Name::from_ast_node(name_ref.clone());
        let sema = Semantics::new(db);
        let scope =
            sema.scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;
        return Some(
            scope
                .resolve_name(&name)?
                .into_iter()
                .flat_map(|def| {
                    def.syntax_node_ptr(db, file).map(|ptr| Location {
                        file_id,
                        range: ptr.text_range(),
                    })
                })
                .collect(),
        );
    }
    None
}
