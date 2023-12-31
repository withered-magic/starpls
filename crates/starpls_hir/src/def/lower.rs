use crate::{
    def::{CompClause, Expression, ExpressionId, Module, Name, Parameter, Statement, StatementId},
    Db,
};
use starpls_syntax::ast;

pub(crate) fn lower(db: &dyn Db, syntax: ast::Module) -> Module {
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
                let targets = syntax
                    .targets()
                    .iter()
                    .flat_map(|loop_variables| loop_variables.exprs())
                    .map(|expression| self.lower_expression(expression))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let statements = self.lower_suite_opt(syntax.suite());
                Statement::For {
                    iterable,
                    targets,
                    statements,
                }
            }
            ast::Statement::Return(syntax) => {
                let expr = syntax
                    .expr()
                    .map(|expression| self.lower_expression(expression));
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
            ast::Statement::Load(_) => {
                todo!()
            }
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
            None => self.alloc_expression(Expression::Missing),
        }
    }

    fn lower_expression(&mut self, syntax: ast::Expression) -> ExpressionId {
        let expression = match syntax {
            ast::Expression::Name(expr) => {
                let name = self.lower_name_opt(Some(expr));
                Expression::Name { name }
            }
            ast::Expression::Literal(_) => todo!(),
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
            ast::Expression::ListComp(expr) => expr.com,
            ast::Expression::Dict(_) => todo!(),
            ast::Expression::DictComp(_) => todo!(),
            ast::Expression::Tuple(_) => todo!(),
            ast::Expression::Paren(_) => todo!(),
            ast::Expression::Dot(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::Index(_) => todo!(),
            ast::Expression::Slice(_) => todo!(),
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
                    Parameter::Simple {
                        name,
                        default: match param.default() {
                            syntax @ Some(_) => Some(self.lower_expression_opt(syntax)),
                            None => None,
                        },
                    }
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
        comp_clauses.map(|comp_clause| match comp_clause {
            ast::CompClause::For(comp_clause) => {
                let iterable = self.lower_expression_opt(comp_clause.iterable());
                let targets = comp_clause.targets()
                CompClause::For { iterable, targets: () }
            }
            ast::CompClause::If(comp_clause) => {}
        })
    }

    fn alloc_statement(&mut self, statement: Statement) -> StatementId {
        self.module.statements.alloc(statement)
    }

    fn alloc_expression(&mut self, expression: Expression) -> ExpressionId {
        self.module.expressions.alloc(expression)
    }
}
