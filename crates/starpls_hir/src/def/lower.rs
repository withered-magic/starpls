use crate::{
    def::{
        Argument, CompClause, DictEntry, Expression, ExpressionId, Module, Name, Parameter,
        Statement, StatementId,
    },
    Db,
};
use starpls_syntax::ast::{self, LoopVariables};

pub(super) fn lower_module(db: &dyn Db, syntax: ast::Module) -> Module {
    LoweringContext {
        db: db,
        module: Module {
            expressions: Default::default(),
            statements: Default::default(),
        },
    }
    .lower(syntax)
}

struct LoweringContext<'a> {
    db: &'a dyn Db,
    module: Module,
}

impl<'a> LoweringContext<'a> {
    pub(crate) fn lower(mut self, syntax: ast::Module) -> Module {
        for statement in syntax.statements() {
            self.lower_statement(statement);
        }
        self.module
    }

    fn lower_statement(&mut self, statement: ast::Statement) -> StatementId {
        let statement = match statement {
            ast::Statement::Def(stmt) => {
                let name = self.lower_name_opt(stmt.name());
                let parameters = self.lower_parameters_opt(stmt.parameters());
                let statements = self.lower_suite_opt(stmt.suite());
                Statement::Def {
                    name,
                    parameters,
                    statements,
                }
            }
            ast::Statement::If(stmt) => {
                let test = self.lower_expression_opt(stmt.test());
                let if_statements = self.lower_suite_opt(stmt.if_suite());
                let elif_statement = stmt
                    .elif_stmt()
                    .map(|elif_stmt| self.lower_statement(elif_stmt));
                let else_statements = self.lower_suite_opt(stmt.else_suite());
                Statement::If {
                    test,
                    if_statements,
                    elif_statement,
                    else_statements,
                }
            }
            ast::Statement::For(syntax) => {
                let iterable = self.lower_expression_opt(syntax.iterable());
                let targets = self.lower_loop_variables_opt(syntax.targets());
                let statements = self.lower_suite_opt(syntax.suite());
                Statement::For {
                    iterable,
                    targets,
                    statements,
                }
            }
            ast::Statement::Return(syntax) => {
                let expr = self.lower_expression_maybe(syntax.expr());
                Statement::Return { expr }
            }
            ast::Statement::Break(_) => Statement::Break,
            ast::Statement::Continue(_) => Statement::Continue,
            ast::Statement::Pass(_) => Statement::Pass,
            ast::Statement::Assign(stmt) => {
                let lhs = self.lower_expression_opt(stmt.lhs());
                let rhs = self.lower_expression_opt(stmt.rhs());
                Statement::Assign { lhs, rhs }
            }
            ast::Statement::Load(_) => Statement::Load {
                items: Vec::new().into_boxed_slice(),
            },
            ast::Statement::Expr(stmt) => {
                let expr = self.lower_expression(stmt);
                Statement::Expr { expr }
            }
        };
        self.alloc_statement(statement)
    }

    fn lower_expression_opt(&mut self, syntax: Option<ast::Expression>) -> ExpressionId {
        match syntax {
            Some(syntax) => self.lower_expression(syntax),
            None => self.lower_expression_missing(),
        }
    }

    fn lower_expression_maybe(&mut self, syntax: Option<ast::Expression>) -> Option<ExpressionId> {
        syntax.map(|syntax| self.lower_expression(syntax))
    }

    fn lower_expression_missing(&mut self) -> ExpressionId {
        self.alloc_expression(Expression::Missing)
    }

    fn lower_expression(&mut self, syntax: ast::Expression) -> ExpressionId {
        let expression = match syntax {
            ast::Expression::Name(expr) => {
                let name = self.lower_name_opt(Some(expr));
                Expression::Name { name }
            }
            ast::Expression::Literal(_) => Expression::Literal,
            ast::Expression::If(expr) => {
                let if_expression = self.lower_expression_opt(expr.if_expr());
                let test = self.lower_expression_opt(expr.test());
                let else_expression = self.lower_expression_opt(expr.else_expr());
                Expression::If {
                    if_expression,
                    test,
                    else_expression,
                }
            }
            ast::Expression::Unary(expr) => {
                let expression = self.lower_expression_opt(expr.expr());
                Expression::Unary { expression }
            }
            ast::Expression::Binary(expr) => {
                let lhs = self.lower_expression_opt(expr.lhs());
                let rhs = self.lower_expression_opt(expr.rhs());
                Expression::Binary { lhs, rhs }
            }
            ast::Expression::Lambda(expr) => {
                let parameters = self.lower_parameters_opt(expr.parameters());
                let body = self.lower_expression_opt(expr.body());
                Expression::Lambda { parameters, body }
            }
            ast::Expression::List(expr) => {
                let elements = expr
                    .elements()
                    .map(|element| self.lower_expression(element))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Expression::List { elements }
            }
            ast::Expression::ListComp(expr) => {
                let expression = self.lower_expression_opt(expr.expr());
                let comp_clauses = self.lower_comp_clauses(expr.comp_clauses());
                Expression::ListComp {
                    expression,
                    comp_clauses,
                }
            }
            ast::Expression::Dict(expr) => {
                let entries = self.lower_entries(expr.entries());
                Expression::Dict { entries }
            }
            ast::Expression::DictComp(expr) => {
                let entry = expr
                    .entry()
                    .map(|entry| {
                        let key = self.lower_expression_opt(entry.key());
                        let value = self.lower_expression_opt(entry.value());
                        DictEntry { key, value }
                    })
                    .unwrap_or_else(|| {
                        let key = self.lower_expression_missing();
                        let value = self.lower_expression_missing();
                        DictEntry { key, value }
                    });
                let comp_clauses = self.lower_comp_clauses(expr.comp_clauses());
                Expression::DictComp {
                    entry,
                    comp_clauses,
                }
            }
            ast::Expression::Tuple(expr) => {
                let expressions = expr
                    .elements()
                    .map(|element| self.lower_expression(element))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Expression::Tuple { expressions }
            }
            ast::Expression::Paren(expr) => {
                let expression = self.lower_expression_opt(expr.expr());
                Expression::Paren { expression }
            }
            ast::Expression::Dot(expr) => {
                let field = self.lower_name_opt(expr.field());
                let expression = self.lower_expression_opt(expr.expr());
                Expression::Dot { expression, field }
            }
            ast::Expression::Call(expr) => {
                let callee = self.lower_expression_opt(expr.callee());
                let arguments = self.lower_arguments_opt(expr.arguments());
                Expression::Call { callee, arguments }
            }
            ast::Expression::Index(expr) => {
                let lhs = self.lower_expression_opt(expr.lhs());
                let index = self.lower_expression_opt(expr.index());
                Expression::Index { lhs, index }
            }
            ast::Expression::Slice(expr) => {
                let start = self.lower_expression_maybe(expr.start());
                let end = self.lower_expression_maybe(expr.end());
                let step = self.lower_expression_maybe(expr.step());
                Expression::Slice { start, end, step }
            }
        };
        self.alloc_expression(expression)
    }

    fn lower_parameters_opt(&mut self, syntax: Option<ast::Parameters>) -> Box<[Parameter]> {
        syntax
            .iter()
            .flat_map(|parameters| parameters.parameters())
            .map(|parameter| match parameter {
                ast::Parameter::Simple(param) => {
                    let name = self.lower_name_opt(param.name());
                    let default = self.lower_expression_maybe(param.default());
                    Parameter::Simple { name, default }
                }
                ast::Parameter::ArgsList(param) => Parameter::ArgsList {
                    name: self.lower_name_opt(param.name()),
                },
                ast::Parameter::KwargsList(param) => Parameter::KwargsList {
                    name: self.lower_name_opt(param.name()),
                },
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_arguments_opt(&mut self, syntax: Option<ast::Arguments>) -> Box<[Argument]> {
        syntax
            .iter()
            .flat_map(|arguments| arguments.arguments())
            .map(|argument| match argument {
                ast::Argument::Simple(arg) => {
                    let expr = self.lower_expression_opt(arg.expr());
                    Argument::Simple { expr }
                }
                ast::Argument::Keyword(arg) => {
                    let name = self.lower_name_opt(arg.name());
                    let expr = self.lower_expression_opt(arg.expr());
                    Argument::Keyword { name, expr }
                }
                ast::Argument::UnpackedList(arg) => {
                    let expr = self.lower_expression_opt(arg.expr());
                    Argument::UnpackedList { expr }
                }
                ast::Argument::UnpackedDict(arg) => {
                    let expr = self.lower_expression_opt(arg.expr());
                    Argument::UnpackedDict { expr }
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_name_opt(&mut self, syntax: Option<ast::Name>) -> Name {
        syntax
            .and_then(|name| name.name())
            .as_ref()
            .map(|token| token.text())
            .map_or_else(
                || Name::missing(self.db),
                |text| Name::from_str(self.db, text),
            )
    }

    fn lower_suite_opt(&mut self, syntax: Option<ast::Suite>) -> Box<[StatementId]> {
        syntax
            .iter()
            .flat_map(|suite| suite.statements())
            .map(|statement| self.lower_statement(statement))
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
                    let iterable = self.lower_expression_opt(comp_clause.iterable());
                    let targets = self.lower_loop_variables_opt(comp_clause.targets());
                    CompClause::For { iterable, targets }
                }
                ast::CompClause::If(comp_clause) => {
                    let test = self.lower_expression_opt(comp_clause.test());
                    CompClause::If { test }
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_loop_variables_opt(
        &mut self,
        loop_variables: Option<LoopVariables>,
    ) -> Box<[ExpressionId]> {
        loop_variables
            .iter()
            .flat_map(|loop_variables| loop_variables.exprs())
            .map(|expression| self.lower_expression(expression))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn lower_entries(&mut self, entries: impl Iterator<Item = ast::DictEntry>) -> Box<[DictEntry]> {
        entries
            .map(|entry| {
                let key = self.lower_expression_opt(entry.key());
                let value = self.lower_expression_opt(entry.value());
                DictEntry { key, value }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn alloc_statement(&mut self, statement: Statement) -> StatementId {
        self.module.statements.alloc(statement)
    }

    fn alloc_expression(&mut self, expression: Expression) -> ExpressionId {
        self.module.expressions.alloc(expression)
    }
}
