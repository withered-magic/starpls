use crate::{util::pick_best_token, Database, FilePosition, LocationLink, ResolvedPath};
use starpls_common::{parse as parse_query, Db};
use starpls_hir::{Name, ScopeDef, Semantics};
use starpls_syntax::{
    ast::{self, AstNode},
    T,
};

pub(crate) fn goto_definition(
    db: &Database,
    FilePosition { file_id, pos }: FilePosition,
) -> Option<Vec<LocationLink>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let parse = parse_query(db, file);
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
                        let def = sema.def_for_load_item(&load_item)?;
                        let range = def.value.syntax_node_ptr(db, def.file)?.text_range();
                        Some(LocationLink::Local {
                            origin_selection_range: None,
                            target_range: range.clone(),
                            target_selection_range: range.clone(),
                            target_file_id: def.file.id(db),
                        })
                    }
                    _ => def
                        .syntax_node_ptr(db, file)
                        .map(|ptr| LocationLink::Local {
                            origin_selection_range: None,
                            target_range: ptr.text_range(),
                            target_selection_range: ptr.text_range(),
                            target_file_id: file_id,
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
                            let range = name.syntax().text_range();
                            vec![LocationLink::Local {
                                origin_selection_range: None,
                                target_range: range.clone(),
                                target_selection_range: range,
                                target_file_id: struct_call_expr.file.id(db),
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
                            Some(vec![LocationLink::Local {
                                origin_selection_range: None,
                                target_range: syntax.text_range(),
                                target_selection_range: syntax.text_range(),
                                target_file_id: provider_fields.file.id(db),
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
        return Some(vec![LocationLink::Local {
            origin_selection_range: Some(token.text_range()),
            target_range: Default::default(),
            target_selection_range: Default::default(),
            target_file_id: file.id(db),
        }]);
    }

    if let Some(load_item) = ast::LoadItem::cast(parent.clone()) {
        let load_item = sema.resolve_load_item(file, &load_item)?;
        let def = sema.def_for_load_item(&load_item)?;
        let range = def.value.syntax_node_ptr(db, def.file)?.text_range();
        return Some(vec![LocationLink::Local {
            origin_selection_range: None,
            target_range: range.clone(),
            target_selection_range: range,
            target_file_id: def.file.id(db),
        }]);
    }

    if let Some(lit) = ast::LiteralExpr::cast(parent) {
        let value = match lit.kind() {
            ast::LiteralKind::String(s) => s.value()?,
            _ => return None,
        };
        let resolved_path = db.resolve_path(&value, file.dialect(db), file_id).ok()??;
        return match resolved_path {
            ResolvedPath::Source { path } => path.try_exists().ok()?.then(|| {
                vec![LocationLink::External {
                    origin_selection_range: Some(token.text_range()),
                    target_path: path,
                }]
            }),
            ResolvedPath::BuildTarget {
                build_file: build_file_id,
                target,
                ..
            } => {
                let build_file = db.get_file(build_file_id)?;
                let parse = parse_query(db, build_file).syntax(db);
                let call_expr = parse
                    .children()
                    .filter_map(ast::CallExpr::cast)
                    .find(|expr| {
                        expr.arguments()
                            .into_iter()
                            .flat_map(|args| args.arguments())
                            .any(|arg| match arg {
                                ast::Argument::Keyword(arg) => {
                                    arg.name()
                                        .and_then(|name| name.name())
                                        .map(|name| name.text() == "name")
                                        .unwrap_or_default()
                                        && arg
                                            .expr()
                                            .and_then(|expr| match expr {
                                                ast::Expression::Literal(expr) => Some(expr),
                                                _ => None,
                                            })
                                            .and_then(|expr| match expr.kind() {
                                                ast::LiteralKind::String(s) => {
                                                    s.value().map(|value| &*value == target)
                                                }
                                                _ => None,
                                            })
                                            .unwrap_or_default()
                                }
                                _ => false,
                            })
                    })?;
                let range = call_expr.syntax().text_range();
                Some(vec![LocationLink::Local {
                    origin_selection_range: Some(token.text_range()),
                    target_range: range.clone(),
                    target_selection_range: range,
                    target_file_id: build_file_id,
                }])
            }
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{AnalysisSnapshot, FilePosition, LocationLink};
    use starpls_bazel::APIContext;
    use starpls_common::Dialect;
    use starpls_test_util::parse_fixture;

    fn check_goto_definition(fixture: &str) {
        let (contents, pos, expected) = parse_fixture(fixture);
        let (snap, file_id) =
            AnalysisSnapshot::from_single_file(&contents, Dialect::Bazel, Some(APIContext::Bzl));
        let actual = snap
            .goto_definition(FilePosition { file_id, pos })
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|loc| match loc {
                LocationLink::Local { target_range, .. } => target_range,
                _ => panic!("expected local location"),
            })
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
