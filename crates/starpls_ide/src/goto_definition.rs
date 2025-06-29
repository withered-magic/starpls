use starpls_common::Db;
use starpls_common::File;
use starpls_common::InFile;
use starpls_hir::LoadItem;
use starpls_hir::Name;
use starpls_hir::ScopeDef;
use starpls_hir::Semantics;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};
use starpls_syntax::match_ast;
use starpls_syntax::SyntaxToken;
use starpls_syntax::T;

use crate::util::pick_best_token;
use crate::Database;
use crate::FilePosition;
use crate::LocationLink;
use crate::ResolvedPath;

struct GotoDefinitionHandler<'a> {
    sema: Semantics<'a>,
    file: File,
    token: SyntaxToken,
    skip_re_exports: bool,
}

impl<'a> GotoDefinitionHandler<'a> {
    fn new(
        db: &'a Database,
        FilePosition { file_id, pos }: FilePosition,
        skip_re_exports: bool,
    ) -> Option<Self> {
        let sema = Semantics::new(db);
        let file = db.get_file(file_id)?;
        let parse = sema.parse(file);
        let token = pick_best_token(parse.syntax(db).token_at_offset(pos), |kind| match kind {
            T![ident] => 2,
            T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
            kind if kind.is_trivia_token() => 0,
            _ => 1,
        })?;

        Some(Self {
            sema,
            file,
            token,
            skip_re_exports,
        })
    }

    fn handle_goto_definition(&self) -> Option<Vec<LocationLink>> {
        let parent = self.token.parent()?;

        match_ast! {
            match parent {
                ast::NameRef(name_ref) => self.handle_name_ref(name_ref, self.skip_re_exports),
                ast::Name(name) => {
                    let parent = name.syntax().parent()?;
                    match_ast! {
                        match parent {
                            ast::DotExpr(dot_expr) => self.handle_dot_expr(dot_expr),
                            ast::KeywordArgument(arg) => self.handle_keyword_argument(arg),
                            _ => None
                        }
                    }
                },
                ast::LoadModule(load_module) => self.handle_load_module(load_module),
                ast::LoadItem(load_item) => self.handle_load_item(load_item, self.skip_re_exports),
                ast::LiteralExpr(lit) => self.handle_literal_expr(lit),
                _ => None
            }
        }
    }

    fn handle_name_ref(
        &self,
        name_ref: ast::NameRef,
        skip_re_exports: bool,
    ) -> Option<Vec<LocationLink>> {
        let name = Name::from_ast_name_ref(name_ref.clone());
        let scope = self.sema.scope_for_expr(
            self.file,
            &ast::Expression::cast(name_ref.syntax().clone())?,
        )?;
        Some(
            scope
                .resolve_name(&name)
                .into_iter()
                .flat_map(|def| match def {
                    ScopeDef::LoadItem(load_item) => {
                        if skip_re_exports {
                            self.try_resolve_re_export(&load_item)
                        } else {
                            let def = self.sema.def_for_load_item(&load_item)?;
                            self.def_to_location_link(def)
                        }
                    }
                    _ => self.def_to_location_link(def),
                })
                .collect(),
        )
    }

    // Re-exporting symbols is a common pattern in Starlark, but the default
    // behavior in this scenario isn't ideal from a UX perspective - the user
    // might need to issue multiple "Go to Definition" commands to get to
    // the actual definition of a re-exported symbol.
    fn try_resolve_re_export(&self, load_item: &LoadItem) -> Option<LocationLink> {
        let def = self.sema.def_for_load_item(load_item)?;
        self.try_resolve_assign_from_load_item(&def)
            .and_then(|load_item| self.try_resolve_re_export(&load_item))
            .or_else(|| self.def_to_location_link(def))
    }

    fn try_resolve_assign_from_load_item(&self, def: &ScopeDef) -> Option<LoadItem> {
        let InFile { file, value: ptr } = def.syntax_node_ptr(self.sema.db)?;
        let syntax = ptr.try_to_node(&self.sema.parse(file).syntax(self.sema.db))?;
        if !ast::NameRef::can_cast(syntax.kind()) {
            return None;
        }
        let assign_stmt = ast::AssignStmt::cast(syntax.parent()?)?;
        let name_ref = match assign_stmt.rhs()? {
            ast::Expression::Name(name_ref) => name_ref,
            _ => return None,
        };
        let name = Name::from_ast_name_ref(name_ref.clone());
        let scope = self
            .sema
            .scope_for_expr(file, &ast::Expression::cast(name_ref.syntax().clone())?)?;
        scope
            .resolve_name(&name)
            .into_iter()
            .find_map(|def| match def {
                ScopeDef::LoadItem(load_item) => Some(load_item),
                _ => None,
            })
    }

    fn handle_dot_expr(&self, dot_expr: ast::DotExpr) -> Option<Vec<LocationLink>> {
        let ty = self.sema.type_of_expr(self.file, &dot_expr.expr()?)?;

        if let Some(strukt) = ty.try_as_inline_struct() {
            // Check for struct field definition.
            let struct_call_expr = strukt.call_expr(self.sema.db)?;
            struct_call_expr
                .value
                .arguments()
                .into_iter()
                .flat_map(|args| args.arguments())
                .find_map(|arg| match arg {
                    ast::Argument::Keyword(kwarg) => {
                        let name = kwarg.name()?;
                        (name.name()?.text() == self.token.text()).then(|| {
                            let range = name.syntax().text_range();
                            vec![LocationLink::Local {
                                origin_selection_range: None,
                                target_range: range,
                                target_selection_range: range,
                                target_file_id: struct_call_expr.file.id(self.sema.db),
                            }]
                        })
                    }
                    _ => None,
                })
        } else if let Some(provider_fields) = ty.provider_fields_source(self.sema.db) {
            // Check for provider field definition. This only handles the case where the provider
            // fields are specified in a dictionary literal.
            return self.find_name_in_dict_expr(provider_fields);
        } else {
            None
        }
    }

    fn handle_keyword_argument(&self, arg: ast::KeywordArgument) -> Option<Vec<LocationLink>> {
        let call_expr = arg
            .syntax()
            .parent()
            .and_then(ast::Arguments::cast)
            .and_then(|args| args.syntax().parent())
            .and_then(ast::CallExpr::cast)?;
        let callable = self.sema.resolve_call_expr(self.file, &call_expr)?;

        // If the callable is a rule, link to the dictionary where its attributes are declared.
        if let Some(attrs_expr) = callable.rule_attrs_source(self.sema.db) {
            return self.find_name_in_dict_expr(attrs_expr);
        }

        let (param, _) = callable
            .params(self.sema.db)
            .into_iter()
            .find(|(param, _)| {
                param.name(self.sema.db).as_ref().map(|name| name.as_str())
                    == arg
                        .name()
                        .and_then(|name| name.name())
                        .as_ref()
                        .map(|name| name.text())
            })?;

        let InFile { file, value: ptr } = param.syntax_node_ptr(self.sema.db)?;
        let range = ptr.text_range();
        Some(vec![LocationLink::Local {
            origin_selection_range: None,
            target_range: range,
            target_selection_range: range,
            target_file_id: file.id(self.sema.db),
        }])
    }

    fn handle_load_module(&self, load_module: ast::LoadModule) -> Option<Vec<LocationLink>> {
        let load_stmt = ast::LoadStmt::cast(load_module.syntax().parent()?)?;
        let file = self.sema.resolve_load_stmt(self.file, &load_stmt)?;
        Some(vec![LocationLink::Local {
            origin_selection_range: Some(self.token.text_range()),
            target_range: Default::default(),
            target_selection_range: Default::default(),
            target_file_id: file.id(self.sema.db),
        }])
    }

    fn handle_load_item(
        &self,
        load_item: ast::LoadItem,
        skip_re_exports: bool,
    ) -> Option<Vec<LocationLink>> {
        let load_item = self.sema.resolve_load_item(self.file, &load_item)?;
        if skip_re_exports {
            self.try_resolve_re_export(&load_item)
        } else {
            let def = self.sema.def_for_load_item(&load_item)?;
            self.def_to_location_link(def)
        }
        .map(|loc| vec![loc])
    }

    fn handle_literal_expr(&self, lit: ast::LiteralExpr) -> Option<Vec<LocationLink>> {
        let value = match lit.kind() {
            ast::LiteralKind::String(s) => s.value()?,
            _ => return None,
        };
        let resolved_path = self
            .sema
            .db
            .resolve_path(
                &value,
                self.file.dialect(self.sema.db),
                self.file.id(self.sema.db),
            )
            .ok()??;

        match resolved_path {
            ResolvedPath::Source { path } => path.try_exists().ok()?.then(|| {
                vec![LocationLink::External {
                    origin_selection_range: Some(self.token.text_range()),
                    target_path: path,
                }]
            }),
            ResolvedPath::BuildTarget {
                build_file: build_file_id,
                target,
                ..
            } => {
                let build_file = self.sema.db.get_file(build_file_id)?;
                let parse = self.sema.parse(build_file).syntax(self.sema.db);
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
                                                    s.value().map(|value| *value == target)
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
                    origin_selection_range: Some(self.token.text_range()),
                    target_range: range,
                    target_selection_range: range,
                    target_file_id: build_file_id,
                }])
            }
        }
    }

    fn find_name_in_dict_expr(
        &self,
        dict_expr: InFile<ast::DictExpr>,
    ) -> Option<Vec<LocationLink>> {
        dict_expr.value.entries().find_map(|entry| {
            entry
                .key()
                .as_ref()
                .and_then(|entry| match entry {
                    ast::Expression::Literal(lit) => Some((lit.syntax(), lit.kind())),
                    _ => None,
                })
                .and_then(|(syntax, kind)| match kind {
                    ast::LiteralKind::String(s)
                        if s.value().as_deref() == Some(self.token.text()) =>
                    {
                        Some(vec![LocationLink::Local {
                            origin_selection_range: None,
                            target_range: syntax.text_range(),
                            target_selection_range: syntax.text_range(),
                            target_file_id: dict_expr.file.id(self.sema.db),
                        }])
                    }
                    _ => None,
                })
        })
    }

    fn def_to_location_link(&self, def: ScopeDef) -> Option<LocationLink> {
        let location = match def {
            ScopeDef::Callable(_) => {
                let InFile { file, value: ptr } = def.syntax_node_ptr(self.sema.db)?;
                let def_stmt = ptr
                    .try_to_node(&self.sema.parse(file).syntax(self.sema.db))
                    .and_then(ast::DefStmt::cast)?;
                let range = def_stmt.name()?.syntax().text_range();
                LocationLink::Local {
                    origin_selection_range: None,
                    target_range: range,
                    target_selection_range: range,
                    target_file_id: file.id(self.sema.db),
                }
            }
            _ => {
                let InFile { file, value: ptr } = def.syntax_node_ptr(self.sema.db)?;
                let range = ptr.text_range();
                LocationLink::Local {
                    origin_selection_range: None,
                    target_range: range,
                    target_selection_range: range,
                    target_file_id: file.id(self.sema.db),
                }
            }
        };
        Some(location)
    }
}

pub(crate) fn goto_definition(
    db: &Database,
    pos: FilePosition,
    skip_re_exports: bool,
) -> Option<Vec<LocationLink>> {
    GotoDefinitionHandler::new(db, pos, skip_re_exports)?.handle_goto_definition()
}

#[cfg(test)]
mod tests {
    use starpls_bazel::APIContext;
    use starpls_common::Dialect;
    use starpls_common::FileInfo::Bazel;
    use starpls_hir::Fixture;

    use crate::Analysis;
    use crate::FilePosition;
    use crate::LocationLink;

    fn check_goto_definition(fixture: &str) {
        let (analysis, fixture) = Analysis::from_single_file_fixture(fixture);
        check_goto_definition_from_fixture(analysis, fixture, false);
    }

    fn check_goto_definition_from_fixture(
        analysis: Analysis,
        fixture: Fixture,
        skip_re_exports: bool,
    ) {
        let actual = analysis
            .snapshot()
            .goto_definition(
                fixture
                    .cursor_pos
                    .map(|(file_id, pos)| FilePosition { file_id, pos })
                    .unwrap(),
                skip_re_exports,
            )
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|loc| match loc {
                LocationLink::Local {
                    target_range,
                    target_file_id,
                    ..
                } => (target_file_id, target_range),
                _ => panic!("expected local location"),
            })
            .collect::<Vec<_>>();
        assert_eq!(fixture.selected_ranges, actual);
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
    fn test_function() {
        check_goto_definition(
            r#"
def foo():
    #^^
    pass

f$0oo()
"#,
        );
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
    fn test_lambda_param() {
        check_goto_definition(
            r#"
lambda abc: print(a$0bc)
       #^^
"#,
        );
    }

    #[test]
    fn test_keyword_argument() {
        check_goto_definition(
            r#"
def foo(abc):
        #^^
        print(abc)

foo(a$0bc = 123)
"#,
        );
    }

    #[test]
    fn test_rule_attribute() {
        check_goto_definition(
            r#"
def _foo_impl(ctx):
    pass

foo = rule(
    implementation = _foo_impl,
    attrs = {
        "bar": attr.string(),
        #^^^^
    },
)

foo(
    name = "foo",
    b$0ar = "baz",
)
"#,
        );
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

    #[test]
    fn test_load_stmt() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_file(
            &mut analysis.db,
            "//:foo.bzl",
            r#"
def foo():
    #^^
    pass
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:bar.bzl",
            r#"
load("//:foo.bzl", "foo")

f$0oo()
"#,
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, false);
    }

    #[test]
    fn test_load_stmt_re_export() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_file(
            &mut analysis.db,
            "//:foo.bzl",
            r#"
foo = 123
#^^
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:bar.bzl",
            r#"
load("//:foo.bzl", _foo = "foo")

foo = _foo
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:baz.bzl",
            r#"
load("//:bar.bzl", "foo")

f$0oo
"#,
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, true);
    }

    #[test]
    fn test_load_stmt_re_export_load_item() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_file(
            &mut analysis.db,
            "//:foo.bzl",
            r#"
foo = 123
#^^
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:bar.bzl",
            r#"
load("//:foo.bzl", _foo = "foo")

foo = _foo
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:baz.bzl",
            r#"
load("//:bar.bzl", "f$0oo")
"#,
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, true);
    }

    #[test]
    fn test_load_stmt_re_export_short_circuit() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_file(
            &mut analysis.db,
            "//:foo.bzl",
            r#"
foo = 123
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:bar.bzl",
            r#"
load("//:foo.bzl", _foo = "bar")

foo = _foo
#^^
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:baz.bzl",
            r#"
load("//:bar.bzl", "foo")

f$0oo
"#,
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, true);
    }

    #[test]
    fn test_prelude_variable() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_prelude_file(
            &mut analysis.db,
            r#"
FOO = 123
#^^
"#,
        );
        fixture.add_file_with_options(
            &mut analysis.db,
            "BUILD.bazel",
            r#"
F$0OO
"#,
            Dialect::Bazel,
            Some(Bazel {
                api_context: APIContext::Build,
                is_external: false,
            }),
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, false);
    }

    #[test]
    fn test_prelude_function_definition() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_prelude_file(
            &mut analysis.db,
            r#"
def foo():
    #^^
    pass
"#,
        );
        fixture.add_file_with_options(
            &mut analysis.db,
            "BUILD.bazel",
            r#"
f$0oo()
"#,
            Dialect::Bazel,
            Some(Bazel {
                api_context: APIContext::Build,
                is_external: false,
            }),
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, false);
    }

    #[test]
    fn test_prelude_load_stmt() {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        fixture.add_prelude_file(
            &mut analysis.db,
            r#"
load("//:defs.bzl", "java_library")
"#,
        );
        fixture.add_file(
            &mut analysis.db,
            "//:defs.bzl",
            r#"
def java_library():
    #^^^^^^^^^^^
    pass
"#,
        );
        fixture.add_file_with_options(
            &mut analysis.db,
            "BUILD.bazel",
            r#"
j$0ava_library()
"#,
            Dialect::Bazel,
            Some(Bazel {
                api_context: APIContext::Build,
                is_external: false,
            }),
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);
        check_goto_definition_from_fixture(analysis, fixture, false);
    }
}
