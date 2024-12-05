use memchr::memmem::Finder;
use starpls_common::Db;
use starpls_common::File;
use starpls_hir::Name;
use starpls_hir::ScopeDef;
use starpls_hir::Semantics;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};
use starpls_syntax::match_ast;
use starpls_syntax::TextSize;
use starpls_syntax::T;

use crate::util::pick_best_token;
use crate::Database;
use crate::FilePosition;
use crate::Location;

struct FindReferencesHandler<'a> {
    sema: &'a Semantics<'a>,
    file: File,
    name: Name,
    defs: Vec<ScopeDef>,
    locations: Vec<Location>,
}

impl<'a> FindReferencesHandler<'a> {
    fn handle(mut self) -> Vec<Location> {
        let name = self.name.clone();
        let finder = Finder::new(name.as_str());
        let offsets = finder
            .find_iter(self.file.contents(self.sema.db).as_bytes())
            .map(|index| {
                let offset: TextSize = index.try_into().unwrap();
                offset
            });

        for offset in offsets {
            let Some(parent) = self
                .sema
                .parse(self.file)
                .syntax(self.sema.db)
                .token_at_offset(offset)
                .find(|token| token.text() == self.name.as_str())
                .and_then(|token| token.parent())
            else {
                continue;
            };

            match_ast! {
                match parent {
                    ast::Name(name) => self.check_matches_name(name),
                    ast::NameRef(name_ref) => self.check_matches_name_ref(name_ref),
                    _ => ()
                }
            };
        }

        self.locations
    }

    fn check_matches_name(&mut self, node: ast::Name) {
        let Some(callable) = node
            .syntax()
            .parent()
            .and_then(ast::DefStmt::cast)
            .and_then(|def_stmt| self.sema.resolve_def_stmt(self.file, &def_stmt))
        else {
            return;
        };
        if self.defs.contains(&ScopeDef::Callable(callable)) {
            self.locations.push(Location {
                file_id: self.file.id(self.sema.db),
                range: node.syntax().text_range(),
            });
        }
    }

    fn check_matches_name_ref(&mut self, node: ast::NameRef) {
        let Some(scope) = ast::Expression::cast(node.syntax().clone())
            .and_then(|expr| self.sema.scope_for_expr(self.file, &expr))
        else {
            return;
        };
        for def in scope.resolve_name(&self.name).into_iter() {
            if self.defs.contains(&def) {
                self.locations.push(Location {
                    file_id: self.file.id(self.sema.db),
                    range: node.syntax().text_range(),
                });

                // Add the current location at most once.
                break;
            }
        }
    }
}

pub(crate) fn find_references(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Vec<Location>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let parse = sema.parse(file);
    let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;
    let node = token.parent()?;

    let (name, defs) = if let Some(node) = ast::NameRef::cast(node.clone()) {
        let name = Name::from_ast_name_ref(node.clone());
        let scope = sema.scope_for_expr(file, &ast::Expression::cast(node.syntax().clone())?)?;
        let defs = scope
            .resolve_name(&name)
            .into_iter()
            .flat_map(|def| match &def {
                ScopeDef::Variable(_) => Some(def),
                ScopeDef::Callable(ref callable) if callable.is_user_defined() => Some(def),
                _ => None,
            })
            .collect::<Vec<_>>();

        if defs.is_empty() {
            return None;
        }

        (name, defs)
    } else if let Some(node) = ast::Name::cast(node) {
        let def_stmt = ast::DefStmt::cast(node.syntax().parent()?)?;
        let callable = sema.resolve_def_stmt(file, &def_stmt)?;
        (
            Name::from_ast_name(node),
            vec![ScopeDef::Callable(callable)],
        )
    } else {
        return None;
    };

    Some(
        FindReferencesHandler {
            sema: &sema,
            file,
            name,
            defs,
            locations: vec![],
        }
        .handle(),
    )
}

#[cfg(test)]
mod tests {
    use crate::Analysis;
    use crate::FilePosition;

    fn check_find_references(fixture: &str) {
        let (analysis, fixture) = Analysis::from_single_file_fixture(fixture);
        let references = analysis
            .snapshot()
            .find_references(
                fixture
                    .cursor_pos
                    .map(|(file_id, pos)| FilePosition { file_id, pos })
                    .unwrap(),
            )
            .unwrap()
            .unwrap();

        let mut actual_locations = references
            .into_iter()
            .map(|location| (location.file_id, location.range))
            .collect::<Vec<_>>();
        actual_locations.sort_by_key(|(_, range)| (range.start()));
        actual_locations.sort_by_key(|(file_id, _)| *file_id);

        assert_eq!(fixture.selected_ranges, actual_locations);
    }

    #[test]
    fn test_variable() {
        check_find_references(
            r#"
abc = 123
#^^

a$0bc
#^^
"#,
        );
    }

    #[test]
    fn test_variable_with_function_definition() {
        check_find_references(
            r#"
def foo():
    #^^
    pass

f$0oo()
#^^
"#,
        );
    }

    #[test]
    fn test_function_definition() {
        check_find_references(
            r#"
def f$0oo():
    #^^
    pass

foo()
#^^
"#,
        );
    }

    #[test]
    fn test_redeclared_variable() {
        check_find_references(
            r#"
foo = 123
#^^
foo
#^^
foo = "abc"
#^^
f$0oo
#^^
"#,
        );
    }

    #[test]
    fn test_redeclared_function() {
        check_find_references(
            r#"
def foo():
    #^^
    pass
foo
#^^
foo = "abc"
#^^
f$0oo
#^^
"#,
        );
    }
}
