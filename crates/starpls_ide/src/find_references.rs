use memchr::memmem::Finder;
use starpls_common::parse;
use starpls_common::Db;
use starpls_hir::Name;
use starpls_hir::ScopeDef;
use starpls_hir::Semantics;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};
use starpls_syntax::TextSize;
use starpls_syntax::T;

use crate::util::pick_best_token;
use crate::Database;
use crate::FilePosition;
use crate::Location;

pub(crate) fn find_references(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Vec<Location>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;
    let node = token.parent()?;

    if let Some(name_ref) = ast::NameRef::cast(node) {
        let mut locations = vec![];
        let name = Name::from_ast_node(name_ref.clone());
        let scope =
            sema.scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;
        let var_defs = scope
            .resolve_name(&name)
            .into_iter()
            .flat_map(|def| match &def {
                ScopeDef::Variable(_) => Some(def),
                ScopeDef::Callable(ref callable) if callable.is_user_defined() => Some(def),
                _ => None,
            })
            .collect::<Vec<_>>();

        if var_defs.is_empty() {
            return None;
        }

        let finder = Finder::new(name.as_str());
        let offsets = finder.find_iter(file.contents(db).as_bytes()).map(|index| {
            let offset: TextSize = index.try_into().unwrap();
            offset
        });

        for offset in offsets {
            let Some(parent) = parse
                .syntax(db)
                .token_at_offset(offset)
                .find(|token| token.text() == name.as_str())
                .and_then(|token| token.parent())
            else {
                continue;
            };

            let text_range = parent.text_range();
            let Some(name_ref) = ast::NameRef::cast(parent) else {
                continue;
            };

            let scope =
                sema.scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;

            for def in scope.resolve_name(&name).into_iter() {
                if var_defs.contains(&def) {
                    locations.push(Location {
                        file_id,
                        range: text_range,
                    });
                }
            }
        }

        return Some(locations);
    }

    None
}
