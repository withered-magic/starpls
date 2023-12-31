use crate::{
    def::{Expression, ExpressionId, Module, Name, Parameter, Statement, StatementId},
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
                let parameters = stmt
                    .parameters()
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
                    .into_boxed_slice();
                let statements = self.lower_suite_opt(stmt.suite());
                Statement::Def {
                    name,
                    parameters,
                    statements,
                }
            }
            ast::Statement::If(_) => todo!(),
            ast::Statement::For(_) => todo!(),
            ast::Statement::Return(_) => todo!(),
            ast::Statement::Break(_) => todo!(),
            ast::Statement::Continue(_) => todo!(),
            ast::Statement::Pass(_) => todo!(),
            ast::Statement::Assign(_) => todo!(),
            ast::Statement::Load(_) => todo!(),
            ast::Statement::Expr(_) => todo!(),
        };
        self.alloc_statement(statement)
    }

    fn lower_expression_opt(&mut self, syntax: Option<ast::Expression>) -> ExpressionId {
        let expression = match syntax {
            Some(_) => todo!(),
            None => Expression::Missing,
        };
        self.alloc_expression(expression)
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

    fn alloc_statement(&mut self, statement: Statement) -> StatementId {
        self.module.statements.alloc(statement)
    }

    fn alloc_expression(&mut self, expression: Expression) -> ExpressionId {
        self.module.expressions.alloc(expression)
    }
}
