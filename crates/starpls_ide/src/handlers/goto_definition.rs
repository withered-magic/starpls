use crate::{util::pick_best_token, Database, FilePosition, Location};
use starpls_common::{parse, Db};
use starpls_hir::{source_map, Declaration, Name, Resolver};
use starpls_syntax::{
    ast::{self, AstNode, AstPtr},
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
        let ptr = AstPtr::new(&ast::Expression::cast(name_ref.syntax().clone())?);
        let source_map = source_map(db, file);
        let expr = source_map.expr_map.get(&ptr).cloned()?;
        let name = Name::from_ast_node(name_ref);

        let resolver = Resolver::new_for_expr(db, file, expr);
        return Some(
            resolver
                .resolve_name(&name)?
                .into_iter()
                .flat_map(|decl| match decl {
                    Declaration::Function { id, .. } => {
                        source_map.stmt_map_back.get(&id).map(|ptr| Location {
                            file_id,
                            range: ptr.syntax_node_ptr().text_range(),
                        })
                    }
                    Declaration::Variable { id, .. } => {
                        source_map.expr_map_back.get(&id).map(|ptr| Location {
                            file_id,
                            range: ptr.syntax_node_ptr().text_range(),
                        })
                    }
                    Declaration::Parameter { id, .. } => {
                        source_map.param_map_back.get(&id).map(|ptr| Location {
                            file_id,
                            range: ptr.syntax_node_ptr().text_range(),
                        })
                    }
                    Declaration::LoadItem { id } => {
                        source_map.load_item_map_back.get(&id).map(|ptr| Location {
                            file_id,
                            range: ptr.syntax_node_ptr().text_range(),
                        })
                    }
                    _ => None,
                })
                .collect(),
        );
    }
    None
}
