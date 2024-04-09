use crate::{util::pick_best_token, Database, FilePosition, Location};
use starpls_common::{parse, Db, File, InFile};
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
    let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;
    let parent = token.parent()?;

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let name = Name::from_ast_node(name_ref.clone());
        let scope =
            sema.scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;
        return Some(
            scope
                .resolve_name(&name)?
                .into_iter()
                .flat_map(|def| match def {
                    ScopeDef::LoadItem(load_item) => {
                        let def = scope_def_for_load_item(db, &sema, file, &load_item)?;
                        Some(Location {
                            file_id: def.file.id(db),
                            range: def.value.syntax_node_ptr(db, def.file)?.text_range(),
                        })
                    }
                    _ => def.syntax_node_ptr(db, file).map(|ptr| Location {
                        file_id,
                        range: ptr.text_range(),
                    }),
                })
                .collect(),
        );
    }

    if let Some(name) = ast::Name::cast(parent.clone()) {
        let dot_expr = ast::DotExpr::cast(name.syntax().parent()?)?;
        let ty = sema.type_of_expr(file, &dot_expr.expr()?)?;

        // Check for struct field definition.
        if let Some(struct_) = ty.try_as_struct() {
            let struct_call_expr = struct_.call_expr(db)?;
            return struct_call_expr
                .value
                .arguments()
                .into_iter()
                .flat_map(|args| args.arguments())
                .find_map(|arg| match arg {
                    ast::Argument::Keyword(kwarg) => {
                        let name = kwarg.name()?;
                        (name.name()?.text() == token.text()).then(|| {
                            vec![Location {
                                file_id: struct_call_expr.file.id(db),
                                range: name.syntax().text_range(),
                            }]
                        })
                    }
                    _ => None,
                });
        }

        // Check for provider field definition. This only handles the case where the provider
        // fields are specified in a dictionary literal.
        if let Some(provider_fields) = ty.provider_fields_source(db) {
            return provider_fields.value.entries().find_map(|entry| {
                entry
                    .key()
                    .as_ref()
                    .and_then(|entry| match entry {
                        ast::Expression::Literal(lit) => Some((lit.syntax(), lit.kind())),
                        _ => None,
                    })
                    .and_then(|(syntax, kind)| match kind {
                        ast::LiteralKind::String(s)
                            if s.value().as_deref() == Some(token.text()) =>
                        {
                            Some(vec![Location {
                                file_id: provider_fields.file.id(db),
                                range: syntax.text_range(),
                            }])
                        }
                        _ => None,
                    })
            });
        }
    }

    if let Some(load_module) = ast::LoadModule::cast(parent.clone()) {
        let load_stmt = ast::LoadStmt::cast(load_module.syntax().parent()?)?;
        let file = sema.resolve_load_stmt(file, &load_stmt)?;
        return Some(vec![Location {
            file_id: file.id(db),
            range: TextRange::new(TextSize::new(0), TextSize::new(1)),
        }]);
    }

    if let Some(load_item) = ast::LoadItem::cast(parent) {
        let load_item = sema.resolve_load_item(file, &load_item)?;
        let def = scope_def_for_load_item(db, &sema, file, &load_item)?;
        return Some(vec![Location {
            file_id: def.file.id(db),
            range: def.value.syntax_node_ptr(db, def.file)?.text_range(),
        }]);
    }

    None
}

fn scope_def_for_load_item(
    db: &Database,
    sema: &Semantics,
    file: File,
    load_item: &LoadItem,
) -> Option<InFile<ScopeDef>> {
    let load_stmt = load_item.load_stmt(db)?;
    let loaded_file = sema.resolve_load_stmt(file, &load_stmt)?;
    sema.scope_for_module(loaded_file)
        .resolve_name(&load_item.name(db))?
        .into_iter()
        .next()
        .map(|def| InFile {
            file: loaded_file,
            value: def,
        })
}

#[cfg(test)]
mod tests {
    use crate::{AnalysisSnapshot, FilePosition};
    use starpls_test_util::parse_fixture;

    fn check_goto_definition(fixture: &str) {
        let (contents, pos, expected) = parse_fixture(fixture);
        let (snap, file_id) = AnalysisSnapshot::from_single_file(&contents);
        let actual = snap
            .goto_definition(FilePosition { file_id, pos })
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|loc| loc.range)
            .collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simple() {
        check_goto_definition(
            r#"
foo = 1
#^^
f$0oo
"#,
        )
    }

    #[test]
    fn test_global_variable() {
        check_goto_definition(
            r#"
GLOBAL_LIST = [1, 2, 3]
#^^^^^^^^^^
def f():
    print(GLOBAL$0_LIST)
"#,
        )
    }

    #[test]
    fn test_param() {
        check_goto_definition(
            r#"
def f(abc):
      #^^
      a$0bc
"#,
        )
    }

    #[test]
    fn test_struct_field() {
        check_goto_definition(
            r#"
s = struct(foo = "bar")
           #^^

s.f$0oo
"#,
        )
    }

    #[test]
    fn test_provider_field() {
        check_goto_definition(
            r#"
GoInfo = provider(
    fields = {
        "foo": "The foo field",
        #^^^^
    },
)
info = GoInfo(foo = 123)
info.fo$0o
"#,
        )
    }
}
