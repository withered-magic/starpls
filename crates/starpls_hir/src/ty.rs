// use crate::{
//     def::{Declaration, ExprId, Expression, Module, Name, Statement, StmtId},
//     Db,
// };
// use rustc_hash::FxHashMap;
// use std::collections::hash_map::Entry;

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ModuleInfo {
//     declarations: FxHashMap<Name, Vec<Declaration>>,
// }

// pub(crate) fn bind_module(db: &dyn Db, module: Module) -> ModuleInfo {
//     BindingContext {
//         db,
//         module: &module,
//         declarations: Default::default(),
//     }
//     .bind()
// }

// struct BindingContext<'a> {
//     db: &'a dyn Db,
//     module: &'a Module,
//     declarations: FxHashMap<Name, Vec<Declaration>>,
// }

// impl<'a> BindingContext<'a> {
//     fn bind(mut self) -> ModuleInfo {
//         for statement_id in self.module.top_level.iter().copied() {
//             self.bind_statement(statement_id);
//         }
//         ModuleInfo {
//             declarations: self.declarations,
//         }
//     }

//     fn bind_statements(&mut self, statements: &Box<[StmtId]>) {
//         for statement_id in statements.iter().copied() {
//             self.bind_statement(statement_id);
//         }
//     }

//     fn bind_statement(&mut self, stmt: StmtId) {
//         match &self.module.stmts[stmt] {
//             Statement::Def {
//                 name, statements, ..
//             } => {
//                 if !name.is_missing(self.db) {
//                     self.add_declaration(*name, Declaration::Function { id: stmt });
//                     self.bind_statements(statements);
//                 }
//             }
//             Statement::If {
//                 if_statements,
//                 elif_statement,
//                 else_statements,
//                 ..
//             } => {
//                 self.bind_statements(if_statements);
//                 if let Some(statement_id) = elif_statement {
//                     self.bind_statement(*statement_id);
//                 }
//                 self.bind_statements(else_statements);
//             }
//             Statement::For { statements, .. } => self.bind_statements(statements),
//             Statement::Assign { lhs, .. } => self.collect_declarations_from_assignment(*lhs),
//             _ => {}
//         }
//     }

//     fn collect_declarations_from_assignment(&mut self, expression: ExprId) {
//         match &self.module.expressions[expression] {
//             Expression::Name { name } => {
//                 self.add_declaration(*name, Declaration::Variable { id: expression });
//             }
//             Expression::Tuple { expressions } | Expression::List { expressions } => {
//                 for expression in expressions.iter().cloned() {
//                     self.collect_declarations_from_assignment(expression);
//                 }
//             }
//             _ => (),
//         }
//     }

//     fn add_declaration(&mut self, name: Name, declaration: Declaration) {
//         match self.declarations.entry(name) {
//             Entry::Occupied(mut entry) => {
//                 entry.get_mut().push(declaration);
//             }
//             Entry::Vacant(entry) => {
//                 entry.insert(vec![declaration]);
//             }
//         }
//     }
// }
