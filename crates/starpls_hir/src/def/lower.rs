use crate::{
    def::{
        Argument, CompClause, DictEntry, Expr, ExprId, ExprPtr, Literal, LoadItem, Module,
        ModuleSourceMap, Name, Param, ParamId, ParamPtr, Stmt, StmtId, StmtPtr,
    },
    Db,
};
use starpls_syntax::ast::{self, AstPtr, LoopVariables};

pub(super) fn lower_module(db: &dyn Db, syntax: ast::Module) -> (Module, ModuleSourceMap) {
    let root = AstPtr::new(&syntax);
    LoweringContext {
        db,
        module: Module {
            exprs: Default::default(),
            stmts: Default::default(),
            params: Default::default(),
            top_level: Default::default(),
        },
        source_map: ModuleSourceMap {
            root,
            expr_map: Default::default(),
            expr_map_back: Default::default(),
            stmt_map: Default::default(),
            stmt_map_back: Default::default(),
            param_map: Default::default(),
            param_map_back: Default::default(),
        },
    }
    .lower(syntax)
}

struct LoweringContext<'a> {
    db: &'a dyn Db,
    module: Module,
    source_map: ModuleSourceMap,
}

impl<'a> LoweringContext<'a> {
    fn lower(mut self, syntax: ast::Module) -> (Module, ModuleSourceMap) {
        let mut top_level = Vec::new();
        for statement in syntax.statements() {
            top_level.push(self.lower_stmt(statement));
        }
        self.module.top_level = top_level.into_boxed_slice();
        (self.module, self.source_map)
    }

    fn lower_stmt(&mut self, stmt: ast::Statement) -> StmtId {
        let ptr = AstPtr::new(&stmt);
        let statement = match stmt {
            ast::Statement::Def(syntax) => {
                let name = self.lower_name_opt(syntax.name());
                let params = self.lower_params_opt(syntax.parameters());
                let stmts = self.lower_suite_opt(syntax.suite());
                Stmt::Def {
                    name,
                    params,
                    stmts,
                }
            }
            ast::Statement::If(stmt) => {
                let test = self.lower_expr_opt(stmt.test());
                let if_stmts = self.lower_suite_opt(stmt.if_suite());
                let elif_stmt = stmt.elif_stmt().map(|elif_stmt| self.lower_stmt(elif_stmt));
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
                Stmt::Assign { lhs, rhs, op }
            }
            ast::Statement::Load(stmt) => {
                let items = self.lower_load_items(stmt.items());
                Stmt::Load { items }
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
                let literal = node.kind().into();
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
                let params = self.lower_params_opt(node.parameters());
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
                Expr::Call { callee, args }
            }
            ast::Expression::Index(node) => {
                let lhs = self.lower_expr_opt(node.lhs());
                let index = self.lower_expr_opt(node.index());
                Expr::Index { lhs, index }
            }
            ast::Expression::Slice(node) => {
                let start = self.lower_expr_maybe(node.start());
                let end = self.lower_expr_maybe(node.end());
                let step = self.lower_expr_maybe(node.step());
                Expr::Slice { start, end, step }
            }
        };
        self.alloc_expr(expr, ptr)
    }

    fn lower_params_opt(&mut self, syntax: Option<ast::Parameters>) -> Box<[ParamId]> {
        let mut params = Vec::new();
        for param in syntax.iter().flat_map(|params| params.parameters()) {
            let ptr = AstPtr::new(&param);
            let param = match param {
                ast::Parameter::Simple(param) => {
                    let name = self.lower_name_opt(param.name());
                    let default = self.lower_expr_maybe(param.default());
                    Param::Simple {
                        name,
                        default,
                        type_ref: None,
                    }
                }
                ast::Parameter::ArgsList(param) => Param::ArgsList {
                    name: self.lower_name_opt(param.name()),
                    type_ref: None,
                },
                ast::Parameter::KwargsList(param) => Param::KwargsList {
                    name: self.lower_name_opt(param.name()),
                    type_ref: None,
                },
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

    fn lower_loop_variables_opt(&mut self, loop_variables: Option<LoopVariables>) -> Box<[ExprId]> {
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
        load_items: impl Iterator<Item = ast::LoadItem>,
    ) -> Box<[LoadItem]> {
        load_items
            .map(|load_item| match load_item {
                ast::LoadItem::Direct(_item) => {
                    let name = String::new().into_boxed_str();
                    LoadItem::Direct { name }
                }
                ast::LoadItem::Aliased(item) => {
                    let alias = self.lower_name_opt(item.alias());
                    let name = String::new().into_boxed_str();
                    LoadItem::Aliased { alias, name }
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
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
}

impl From<ast::LiteralKind> for Literal {
    fn from(value: ast::LiteralKind) -> Self {
        match value {
            ast::LiteralKind::Int(_) => Literal::Int,
            ast::LiteralKind::Float(_) => Literal::Float,
            ast::LiteralKind::String(_) => Literal::String,
            ast::LiteralKind::Bytes(_) => Literal::Bytes,
            ast::LiteralKind::Bool(_) => Literal::Bool,
            ast::LiteralKind::None => Literal::None,
        }
    }
}
