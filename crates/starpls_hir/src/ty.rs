use crate::def::{Expression, ExpressionId, Module, Name, Statement, StatementId};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleInfo {
    declarations: FxHashMap<Name, Vec<ExpressionId>>,
}

pub(crate) fn bind_module(module: Module) -> ModuleInfo {
    BindingContext {
        module: &module,
        declarations: Default::default(),
    }
    .bind()
}

struct BindingContext<'a> {
    module: &'a Module,
    declarations: FxHashMap<Name, Vec<ExpressionId>>,
}

impl<'a> BindingContext<'a> {
    fn bind(mut self) -> ModuleInfo {
        for statement_id in self.module.top_level.iter().copied() {
            self.bind_statement(statement_id);
        }
        ModuleInfo {
            declarations: self.declarations,
        }
    }

    fn bind_statements(&mut self, statements: &Box<[StatementId]>) {
        for statement_id in statements.iter().copied() {
            self.bind_statement(statement_id);
        }
    }

    fn bind_statement(&mut self, statement_id: StatementId) {
        match &self.module.statements[statement_id] {
            Statement::Def { statements, .. } => self.bind_statements(statements),
            Statement::If {
                if_statements,
                elif_statement,
                else_statements,
                ..
            } => {
                self.bind_statements(if_statements);
                if let Some(statement_id) = elif_statement {
                    self.bind_statement(*statement_id);
                }
                self.bind_statements(else_statements);
            }
            Statement::For { statements, .. } => self.bind_statements(statements),
            Statement::Assign { lhs, rhs, op } => self.collect_declarations_from_assignment(*lhs),
            _ => {}
        }
    }

    fn collect_declarations_from_assignment(&mut self, expression: ExpressionId) {
        match &self.module.expressions[expression] {
            Expression::Name { name } => match self.declarations.entry(*name) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(expression);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![expression]);
                }
            },
            Expression::Tuple { expressions } | Expression::List { expressions } => {
                for expression in expressions.iter().cloned() {
                    self.collect_declarations_from_assignment(expression);
                }
            }
            _ => (),
        }
    }
}
