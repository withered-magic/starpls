use crate::{util::pick_best_token, Database, FilePosition, Location};
use starpls_common::{parse, Db, File};
use starpls_hir::{LoadItem, Name, ScopeDef, Semantics};
use starpls_syntax::{
    ast::{self, AstNode},
    TextRange, TextSize, T,
};

pub(crate) fn goto_definition(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Vec<Location>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let parent = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?
    .parent()?;

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let name = Name::from_ast_node(name_ref.clone());
        let scope =
            sema.scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;
        Some(
            scope
                .resolve_name(&name)?
                .into_iter()
                .flat_map(|def| match def {
                    ScopeDef::LoadItem(load_item) => {
                        let loaded_file = resolve_load_item(db, &sema, file, load_item)?;
                        let loaded_file_id = loaded_file.id(db);
                        sema.scope_for_module(loaded_file)
                            .resolve_name(&name)
                            .map(|defs| defs.into_iter())
                            .and_then(|mut defs| defs.next())
                            .and_then(|def| def.syntax_node_ptr(db, loaded_file))
                            .map(|ptr| Location {
                                file_id: loaded_file_id,
                                range: ptr.text_range(),
                            })
                    }
                    _ => def.syntax_node_ptr(db, file).map(|ptr| Location {
                        file_id,
                        range: ptr.text_range(),
                    }),
                })
                .collect(),
        )
    } else if let Some(load_module) = ast::LoadModule::cast(parent) {
        let load_stmt = ast::LoadStmt::cast(load_module.syntax().parent()?)?;
        let file = sema.resolve_load_stmt(file, &load_stmt)?;
        Some(vec![Location {
            file_id: file.id(db),
            range: TextRange::new(TextSize::new(0), TextSize::new(1)),
        }])
    } else {
        None
    }
}

fn resolve_load_item(
    db: &Database,
    sema: &Semantics,
    file: File,
    load_item: LoadItem,
) -> Option<File> {
    load_item
        .load_stmt(db)
        .and_then(|load_stmt| sema.resolve_load_stmt(file, &load_stmt))
}
