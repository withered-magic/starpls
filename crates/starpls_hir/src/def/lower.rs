use starpls_common::{line_index, Diagnostic, Diagnostics, File, FileRange, Severity};
use starpls_syntax::{
    ast::{self, AstNode, AstPtr, AstToken, SyntaxNodePtr},
    SyntaxToken, TextRange,
};

use crate::{
    def::{
        Argument, CompClause, DictEntry, Expr, ExprId, ExprPtr, Function, Literal, LoadItem,
        LoadItemId, LoadItemPtr, LoadStmt, Module, ModuleSourceMap, Name, Param, ParamId, ParamPtr,
        Stmt, StmtId, StmtPtr,
    },
    typeck::FunctionTypeRef,
    Db, TypeRef,
};

pub(super) fn lower_module(
    db: &dyn Db,
    file: File,
    syntax: ast::Module,
) -> (Module, ModuleSourceMap) {
    let root = AstPtr::new(&syntax);
    LoweringContext {
        db,
        file,
        module: Default::default(),
        source_map: ModuleSourceMap {
            root,
            expr_map: Default::default(),
            expr_map_back: Default::default(),
            stmt_map: Default::default(),
            stmt_map_back: Default::default(),
            param_map: Default::default(),
            param_map_back: Default::default(),
            load_item_map: Default::default(),
            load_item_map_back: Default::default(),
        },
    }
    .lower(syntax)
}

struct LoweringContext<'a> {
    db: &'a dyn Db,
    file: File,
    module: Module,
    source_map: ModuleSourceMap,
}

impl<'a> LoweringContext<'a> {
    fn lower(mut self, syntax: ast::Module) -> (Module, ModuleSourceMap) {
        let line_index = line_index(self.db, self.file);
        self.module.type_ignore_comment_lines = syntax
            .type_ignore_comment_positions()
            .map(|pos| line_index.line_col(pos).line)
            .collect();

        let mut top_level = Vec::new();
        for statement in syntax.statements() {
            let stmt = self.lower_stmt(statement.clone());
            top_level.push(stmt);
            match &self.module.stmts[stmt] {
                Stmt::If { .. } => Diagnostics::push(
                    self.db,
                    Diagnostic {
                        message: "Starlark does not allow top-level if statements".to_string(),
                        severity: Severity::Error,
                        range: FileRange {
                            file_id: self.file.id(self.db),
                            range: statement.syntax().text_range(),
                        },
                    },
                ),
                Stmt::For { .. } => Diagnostics::push(
                    self.db,
                    Diagnostic {
                        message: "Starlark does not allow top-level for statements".to_string(),
                        severity: Severity::Error,
                        range: FileRange {
                            file_id: self.file.id(self.db),
                            range: statement.syntax().text_range(),
                        },
                    },
                ),
                _ => {}
            }
        }
        self.module.top_level = top_level.into_boxed_slice();
        (self.module, self.source_map)
    }

    fn lower_stmt(&mut self, stmt: ast::Statement) -> StmtId {
        let ptr = AstPtr::new(&stmt);
        let statement = match stmt {
            ast::Statement::Def(node) => {
                let name = self.lower_name_opt(node.name());
                let spec = self.lower_func_type_opt(node.spec());
                let doc = node.doc().and_then(|doc| doc.value());
                let params = self.lower_params_opt(
                    node.parameters(),
                    spec.as_ref().map(|spec| &spec.0[..]).unwrap_or(&[]),
                    &doc,
                );
                let stmts = self.lower_suite_opt(node.suite());
                let func = Function::new(
                    self.db,
                    self.file,
                    name,
                    spec.map(|spec| spec.1),
                    doc,
                    ptr.syntax_node_ptr(),
                    params,
                );
                let stmt = self.alloc_stmt(Stmt::Def { func, stmts }, ptr);
                for param in func.params(self.db).iter() {
                    self.module.param_to_def_stmt.insert(*param, stmt);
                }
                return stmt;
            }
            ast::Statement::If(stmt) => {
                let test = self.lower_expr_opt(stmt.test());
                let if_stmts = self.lower_suite_opt(stmt.if_suite());
                let elif_stmt = stmt
                    .elif_stmt()
                    .map(|elif_stmt| self.lower_stmt(ast::Statement::If(elif_stmt)));
                let else_stmts = self.lower_suite_opt(stmt.else_suite());
                Stmt::If {
                    test,
                    if_stmts,
                    elif_stmt,
                    else_stmts,
                }
            }
            ast::Statement::For(syntax) => {
                let iterable = self.lower_expr_opt(syntax.iterable());
                let targets = self.lower_loop_variables_opt(syntax.targets());
                let stmts = self.lower_suite_opt(syntax.suite());
                Stmt::For {
                    iterable,
                    targets,
                    stmts,
                }
            }
            ast::Statement::Return(syntax) => {
                let expr = self.lower_expr_maybe(syntax.expr());
                Stmt::Return { expr }
            }
            ast::Statement::Break(_) => Stmt::Break,
            ast::Statement::Continue(_) => Stmt::Continue,
            ast::Statement::Pass(_) => Stmt::Pass,
            ast::Statement::Assign(stmt) => {
                let lhs = self.lower_expr_opt(stmt.lhs());
                let rhs = self.lower_expr_opt(stmt.rhs());
                let op = stmt.assign_op_info().map(|info| info.1);
                let type_ref = self.lower_type_comment_opt(stmt.type_comment());
                Stmt::Assign {
                    lhs,
                    rhs,
                    op,
                    type_ref,
                }
            }
            ast::Statement::Load(stmt) => {
                let ptr = SyntaxNodePtr::new(&stmt.syntax());
                let module = self.lower_string_opt(stmt.module().and_then(|module| module.name()));
                let load_stmt = LoadStmt::new(self.db, module, ptr);
                let items = self.lower_load_items(load_stmt, stmt.items());
                Stmt::Load { load_stmt, items }
            }
            ast::Statement::Expr(stmt) => {
                let expr = self.lower_expr(stmt);
                Stmt::Expr { expr }
            }
        };
        self.alloc_stmt(statement, ptr)
    }

    fn lower_expr_opt(&mut self, syntax: Option<ast::Expression>) -> ExprId {
        match syntax {
            Some(syntax) => self.lower_expr(syntax),
            None => self.lower_expr_missing(),
        }
    }

    fn lower_expr_maybe(&mut self, syntax: Option<ast::Expression>) -> Option<ExprId> {
        syntax.map(|syntax| self.lower_expr(syntax))
    }

    fn lower_expr_missing(&mut self) -> ExprId {
        self.module.exprs.alloc(Expr::Missing)
    }

    fn lower_expr(&mut self, expr: ast::Expression) -> ExprId {
        let ptr = AstPtr::new(&expr);
        let expr = match expr {
            ast::Expression::Name(node) => {
                let name = self.lower_name_ref_opt(Some(node));
                Expr::Name { name }
            }
            ast::Expression::Literal(node) => {
                let literal = Literal::from_ast_literal(self.db, &node.kind());
                Expr::Literal { literal }
            }
            ast::Expression::If(node) => {
                let if_expr = self.lower_expr_opt(node.if_expr());
                let test = self.lower_expr_opt(node.test());
                let else_expr = self.lower_expr_opt(node.else_expr());
                Expr::If {
                    if_expr,
                    test,
                    else_expr,
                }
            }
            ast::Expression::Unary(node) => {
                let expr = self.lower_expr_opt(node.expr());
                let op = node.unary_op_info().map(|info| info.1);
                Expr::Unary { expr, op }
            }
            ast::Expression::Binary(node) => {
                let lhs = self.lower_expr_opt(node.lhs());
                let rhs = self.lower_expr_opt(node.rhs());
                let op = node.binary_op_info().map(|info| info.1);
                Expr::Binary { lhs, rhs, op }
            }
            ast::Expression::Lambda(node) => {
                let params = self.lower_params_opt(node.parameters(), &[], &None);
                let body = self.lower_expr_opt(node.body());
                Expr::Lambda { params, body }
            }
            ast::Expression::List(node) => {
                let exprs = node
                    .elements()
                    .map(|element| self.lower_expr(element))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Expr::List { exprs }
            }
            ast::Expression::ListComp(node) => {
                let expr = self.lower_expr_opt(node.expr());
                let comp_clauses = self.lower_comp_clauses(node.comp_clauses());
                Expr::ListComp { expr, comp_clauses }
            }
            ast::Expression::Dict(node) => {
                let entries = self.lower_entries(node.entries());
                Expr::Dict { entries }
            }
            ast::Expression::DictComp(node) => {
                let entry = node
                    .entry()
                    .map(|entry| {
                        let key = self.lower_expr_opt(entry.key());
                        let value = self.lower_expr_opt(entry.value());
                        DictEntry { key, value }
                    })
                    .unwrap_or_else(|| {
                        let key = self.lower_expr_missing();
                        let value = self.lower_expr_missing();
                        DictEntry { key, value }
                    });
                let comp_clauses = self.lower_comp_clauses(node.comp_clauses());
                Expr::DictComp {
                    entry,
                    comp_clauses,
                }
            }
            ast::Expression::Tuple(node) => {
                let exprs = node
                    .elements()
                    .map(|element| self.lower_expr(element))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Expr::Tuple { exprs }
            }
            ast::Expression::Paren(node) => {
                let expr = self.lower_expr_opt(node.expr());
                Expr::Paren { expr }
            }
            ast::Expression::Dot(node) => {
                let field = self.lower_name_opt(node.field());
                let expr = self.lower_expr_opt(node.expr());
                Expr::Dot { expr, field }
            }
            ast::Expression::Call(node) => {
                let callee = self.lower_expr_opt(node.callee());
                let args = self.lower_args_opt(node.arguments());
                let impl_fn_name = args.iter().find_map(|arg| match arg {
                    Argument::Keyword { name, expr } if name.as_str() == "implementation" => {
                        match &self.module.exprs[*expr] {
                            Expr::Name { name } => Some(name.clone()),
                            _ => None,
                        }
                    }
                    _ => None,
                });
                let expr = self.alloc_expr(Expr::Call { callee, args }, ptr);
                if let Some(name) = impl_fn_name {
                    self.module.call_expr_with_impl_fn.insert(name, expr);
                }
                return expr;
            }
            ast::Expression::Index(node) => {
                let lhs = self.lower_expr_opt(node.lhs());
                let index = self.lower_expr_opt(node.index());
                Expr::Index { lhs, index }
            }
            ast::Expression::Slice(node) => {
                let lhs = self.lower_expr_opt(node.expr());
                let start = self.lower_expr_maybe(node.start());
                let end = self.lower_expr_maybe(node.end());
                let step = self.lower_expr_maybe(node.step());
                Expr::Slice {
                    lhs,
                    start,
                    end,
                    step,
                }
            }
        };
        self.alloc_expr(expr, ptr)
    }

    fn lower_params_opt(
        &mut self,
        syntax: Option<ast::Parameters>,
        spec_type_refs: &[TypeRef],
        doc: &Option<Box<str>>,
    ) -> Box<[ParamId]> {
        let mut params = Vec::new();

        // Support Google-style parameter documentation, e.g.
        //
        // Args:
        //     x: The first argument
        //     y: The second argument
        //
        // This may be extended to support other styles in the future.
        let find_doc = |name: &str| {
            let prefix = format!("{}:", name);
            doc.as_ref().and_then(|doc| {
                doc.lines().find_map(|line| {
                    let line = line.trim().trim_start_matches('*');
                    if line.starts_with(&prefix) {
                        Some(line[prefix.len()..].to_string().into_boxed_str())
                    } else {
                        None
                    }
                })
            })
        };

        for (i, param) in syntax
            .iter()
            .flat_map(|params| params.parameters())
            .enumerate()
        {
            let ptr = AstPtr::new(&param);
            let type_ref = self
                .lower_type_comment_opt(param.type_comment())
                .map(|res| res.0)
                .or(spec_type_refs.get(i).cloned());
            let param = match param {
                ast::Parameter::Simple(param) => {
                    let name = self.lower_name_opt(param.name());
                    let doc = find_doc(name.as_str());
                    let default = self.lower_expr_maybe(param.default());
                    Param::Simple {
                        name,
                        default,
                        type_ref,
                        doc,
                    }
                }
                ast::Parameter::ArgsList(param) => {
                    let name = self.lower_name_opt(param.name());
                    let doc = find_doc(name.as_str());
                    Param::ArgsList {
                        name,
                        type_ref,
                        doc,
                    }
                }
                ast::Parameter::KwargsDict(param) => {
                    let name = self.lower_name_opt(param.name());
                    let doc = find_doc(name.as_str());
                    Param::KwargsDict {
                        name,
                        type_ref,
                        doc,
                    }
                }
            };
            params.push(self.alloc_param(param, ptr));
        }
        params.into_boxed_slice()
    }

    fn lower_args_opt(&mut self, syntax: Option<ast::Arguments>) -> Box<[Argument]> {
        let args = syntax
            .iter()
            .flat_map(|arguments| arguments.arguments())
            .map(|argument| match argument {
                ast::Argument::Simple(arg) => {
                    let expr = self.lower_expr_opt(arg.expr());
                    Argument::Simple { expr }
                }
                ast::Argument::Keyword(arg) => {
                    let name = self.lower_name_opt(arg.name());
                    let expr = self.lower_expr_opt(arg.expr());
                    Argument::Keyword { name, expr }
                }
                ast::Argument::UnpackedList(arg) => {
                    let expr = self.lower_expr_opt(arg.expr());
                    Argument::UnpackedList { expr }
                }
                ast::Argument::UnpackedDict(arg) => {
                    let expr = self.lower_expr_opt(arg.expr());
                    Argument::UnpackedDict { expr }
                }
            })
            .collect::<Vec<_>>();
        args.into_boxed_slice()
    }

    fn lower_name_opt(&mut self, syntax: Option<ast::Name>) -> Name {
        syntax
            .and_then(|name| name.name())
            .as_ref()
            .map(|token| token.text())
            .map_or_else(|| Name::missing(), |text| Name::from_str(text))
    }

    fn lower_name_ref_opt(&mut self, syntax: Option<ast::NameRef>) -> Name {
        syntax
            .and_then(|name| name.name())
            .as_ref()
            .map(|token| token.text())
            .map_or_else(|| Name::missing(), |text| Name::from_str(text))
    }

    fn lower_suite_opt(&mut self, syntax: Option<ast::Suite>) -> Box<[StmtId]> {
        syntax
            .iter()
            .flat_map(|suite| suite.statements())
            .map(|statement| self.lower_stmt(statement))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_comp_clauses(
        &mut self,
        comp_clauses: impl Iterator<Item = ast::CompClause>,
    ) -> Box<[CompClause]> {
        comp_clauses
            .map(|comp_clause| match comp_clause {
                ast::CompClause::For(comp_clause) => {
                    let iterable = self.lower_expr_opt(comp_clause.iterable());
                    let targets = self.lower_loop_variables_opt(comp_clause.targets());
                    CompClause::For { iterable, targets }
                }
                ast::CompClause::If(comp_clause) => {
                    let test = self.lower_expr_opt(comp_clause.test());
                    CompClause::If { test }
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_loop_variables_opt(
        &mut self,
        loop_variables: Option<ast::LoopVariables>,
    ) -> Box<[ExprId]> {
        loop_variables
            .iter()
            .flat_map(|loop_variables| loop_variables.exprs())
            .map(|expression| self.lower_expr(expression))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_entries(&mut self, entries: impl Iterator<Item = ast::DictEntry>) -> Box<[DictEntry]> {
        entries
            .map(|entry| {
                let key = self.lower_expr_opt(entry.key());
                let value = self.lower_expr_opt(entry.value());
                DictEntry { key, value }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_load_items(
        &mut self,
        load_stmt: LoadStmt,
        load_items: impl Iterator<Item = ast::LoadItem>,
    ) -> Box<[LoadItemId]> {
        load_items
            .map(|load_item| {
                let ptr = AstPtr::new(&load_item);
                let load_item = match load_item {
                    ast::LoadItem::Direct(item) => LoadItem::Direct {
                        name: self.lower_string_opt(item.name()),
                        load_stmt,
                    },
                    ast::LoadItem::Aliased(item) => {
                        let alias = self.lower_name_opt(item.alias());
                        LoadItem::Aliased {
                            alias,
                            name: self.lower_string_opt(item.name()),
                            load_stmt,
                        }
                    }
                };
                self.alloc_load_item(load_item, ptr)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_string_opt(&self, syntax: Option<SyntaxToken>) -> Box<str> {
        syntax
            .and_then(|name| ast::String::cast(name))
            .and_then(|name| name.value())
            .unwrap_or_else(|| String::new().into_boxed_str())
    }

    fn lower_type_comment_opt(
        &self,
        node: Option<ast::TypeComment>,
    ) -> Option<(TypeRef, TextRange)> {
        node.map(|node| {
            let range = node.syntax().text_range();
            (self.lower_type_comment(node), range)
        })
    }

    fn lower_type_comment(&self, node: ast::TypeComment) -> TypeRef {
        node.type_()
            .map(|type_| self.lower_type(type_))
            .unwrap_or_else(|| TypeRef::Unknown)
    }

    fn lower_func_type_opt(&self, node: Option<ast::FunctionType>) -> Option<FunctionTypeRef> {
        node.map(|func_type| {
            let params = match func_type.parameter_types() {
                Some(params) => params
                    .types()
                    .map(|param| self.lower_type_opt(param.type_()))
                    .collect(),
                None => vec![],
            };
            let ret_type_ref = func_type
                .ret_type()
                .map(|type_| self.lower_type(type_))
                .unwrap_or(TypeRef::Unknown);
            FunctionTypeRef(params, ret_type_ref)
        })
    }

    fn lower_type(&self, type_: ast::Type) -> TypeRef {
        match type_ {
            ast::Type::NamedType(type_) => type_.name().map(|name| {
                TypeRef::Name(
                    Name::from_str(name.text()),
                    type_.generic_arguments().map(|args| {
                        let args = args.types().map(|type_| self.lower_type(type_));
                        args.collect::<Vec<_>>().into_boxed_slice()
                    }),
                )
            }),
            ast::Type::UnionType(type_) => Some(TypeRef::Union(
                type_.types().map(|type_| self.lower_type(type_)).collect(),
            )),
            ast::Type::NoneType(_) => Some(TypeRef::Name(Name::new_inline("None"), None)),
            ast::Type::EllipsisType(_) => Some(TypeRef::Ellipsis),
        }
        .unwrap_or_else(|| TypeRef::Unknown)
    }

    fn lower_type_opt(&self, type_: Option<ast::Type>) -> TypeRef {
        type_
            .map(|type_| self.lower_type(type_))
            .unwrap_or(TypeRef::Unknown)
    }

    fn alloc_stmt(&mut self, stmt: Stmt, ptr: StmtPtr) -> StmtId {
        let id = self.module.stmts.alloc(stmt);
        self.source_map.stmt_map.insert(ptr.clone(), id);
        self.source_map.stmt_map_back.insert(id, ptr);
        id
    }

    fn alloc_expr(&mut self, expr: Expr, ptr: ExprPtr) -> ExprId {
        let id = self.module.exprs.alloc(expr);
        self.source_map.expr_map.insert(ptr.clone(), id);
        self.source_map.expr_map_back.insert(id, ptr);
        id
    }

    fn alloc_param(&mut self, param: Param, ptr: ParamPtr) -> ParamId {
        let id = self.module.params.alloc(param);
        self.source_map.param_map.insert(ptr.clone(), id);
        self.source_map.param_map_back.insert(id, ptr.clone());
        id
    }

    fn alloc_load_item(&mut self, load_item: LoadItem, ptr: LoadItemPtr) -> LoadItemId {
        let id = self.module.load_items.alloc(load_item);
        self.source_map.load_item_map.insert(ptr.clone(), id);
        self.source_map.load_item_map_back.insert(id, ptr.clone());
        id
    }
}
