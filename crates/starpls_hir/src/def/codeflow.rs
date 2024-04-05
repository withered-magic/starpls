use either::Either;
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use starpls_common::File;

use crate::{
    def::{
        scope::{module_scopes, ExecutionScopeId, ScopeHirId, Scopes},
        CompClause, Expr, Stmt, StmtId,
    },
    lower, Db, ExprId, Module, Name,
};

#[allow(unused)]
pub(crate) mod pretty;

pub(crate) type FlowNodeId = Id<FlowNode>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FlowNode {
    Start,
    Assign {
        expr: ExprId,
        name: Name,
        execution_scope: ExecutionScopeId,
        source: ExprId,
        antecedent: FlowNodeId,
    },
    Branch {
        antecedents: Vec<FlowNodeId>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CodeFlowGraph {
    pub(crate) flow_nodes: Arena<FlowNode>,
    pub(crate) expr_to_node: FxHashMap<ExprId, FlowNodeId>,
    pub(crate) hir_to_flow_node: FxHashMap<ScopeHirId, FlowNodeId>,
}

#[allow(unused)]
struct CodeFlowLowerCtx<'a> {
    module: &'a Module,
    scopes: &'a Scopes,
    result: CodeFlowGraph,
    curr_node: FlowNodeId,
    curr_break_target: Option<FlowNodeId>,
    curr_continue_target: Option<FlowNodeId>,
}

impl<'a> CodeFlowLowerCtx<'a> {
    fn new(module: &'a Module, scopes: &'a Scopes) -> Self {
        let mut flow_nodes = Arena::new();
        let curr_node = flow_nodes.alloc(FlowNode::Start);
        let cfg = CodeFlowGraph {
            flow_nodes,
            expr_to_node: Default::default(),
            hir_to_flow_node: Default::default(),
        };
        CodeFlowLowerCtx {
            module,
            scopes,
            result: cfg,
            curr_node,
            curr_break_target: None,
            curr_continue_target: None,
        }
    }

    fn lower_stmts(&mut self, stmts: &[StmtId]) {
        for stmt in stmts {
            self.lower_stmt(*stmt);
        }
    }

    fn lower_stmt(&mut self, stmt: StmtId) {
        match &self.module[stmt] {
            Stmt::Assign { lhs, rhs, .. } => {
                self.lower_assignment_target(*lhs, *rhs);
            }
            Stmt::Def { stmts, .. } => {
                self.with_new_start_node(|this| {
                    this.lower_stmts(stmts);
                    stmt
                });
            }
            Stmt::If {
                test,
                if_stmts,
                elif_or_else_stmts,
            } => {
                self.lower_expr(*test);

                let pre_if_node = self.curr_node;
                let post_if_node = self.new_flow_node(FlowNode::Branch {
                    antecedents: Vec::new(),
                });
                self.lower_stmts(if_stmts);
                self.push_antecedent(post_if_node, self.curr_node);
                match elif_or_else_stmts {
                    Some(Either::Left(elif_stmt)) => {
                        self.curr_node = pre_if_node;
                        self.lower_stmt(*elif_stmt);
                        self.push_antecedent(post_if_node, self.curr_node);
                    }
                    Some(Either::Right(else_stmts)) => {
                        self.curr_node = pre_if_node;
                        self.lower_stmts(&else_stmts);
                        self.push_antecedent(post_if_node, self.curr_node);
                    }
                    _ => {
                        self.push_antecedent(post_if_node, pre_if_node);
                    }
                }

                self.curr_node = post_if_node;
            }
            Stmt::Return { expr } => {
                if let Some(expr) = expr {
                    self.lower_expr(*expr);
                }
            }
            Stmt::Expr { expr } => {
                self.lower_expr(*expr);
            }
            Stmt::For {
                iterable,
                targets,
                stmts,
            } => {
                for target in targets.iter() {
                    self.lower_assignment_target(*target, *iterable);
                }
                self.lower_stmts(stmts);
            }
            _ => {}
        }
    }

    fn lower_expr(&mut self, expr: ExprId) {
        match &self.module[expr] {
            Expr::Name { .. } => {
                self.result.expr_to_node.insert(expr, self.curr_node);
            }
            Expr::DictComp {
                entry,
                comp_clauses,
            } => {
                self.lower_comp_clauses(comp_clauses);
                self.lower_expr(entry.key);
                self.lower_expr(entry.value);
            }
            Expr::ListComp { expr, comp_clauses } => {
                self.lower_comp_clauses(comp_clauses);
                self.lower_expr(*expr);
            }
            expr => expr.walk_child_exprs(|expr| {
                self.lower_expr(expr);
            }),
        }
    }

    fn lower_assignment_target(&mut self, expr: ExprId, source: ExprId) {
        self.lower_expr(source);
        match &self.module[expr] {
            Expr::Name { ref name } => {
                let assign_node = self.new_flow_node(FlowNode::Assign {
                    expr,
                    name: name.clone(),
                    execution_scope: self.scopes.execution_scope_for_expr(expr).unwrap(),
                    source,
                    antecedent: self.curr_node,
                });
                self.curr_node = assign_node;
                self.result.expr_to_node.insert(expr, self.curr_node);
            }
            Expr::Paren { expr } => {
                self.lower_assignment_target(*expr, source);
            }
            Expr::Tuple { exprs } | Expr::List { exprs } => {
                for expr in exprs.iter() {
                    self.lower_assignment_target(*expr, source);
                }
            }
            expr => expr.walk_child_exprs(|expr| {
                self.lower_expr(expr);
            }),
        }
    }

    fn lower_comp_clauses(&mut self, comp_clauses: &[CompClause]) {
        for comp_clause in comp_clauses.into_iter() {
            match comp_clause {
                CompClause::For { iterable, targets } => {
                    self.lower_expr(*iterable);
                    for target in targets.iter() {
                        self.lower_assignment_target(*target, *iterable);
                    }
                }
                CompClause::If { test } => {
                    self.lower_expr(*test);
                }
            }
        }
    }

    fn new_flow_node(&mut self, data: FlowNode) -> FlowNodeId {
        self.result.flow_nodes.alloc(data)
    }

    fn push_antecedent(&mut self, this: FlowNodeId, antecedent: FlowNodeId) {
        match self.result.flow_nodes[this] {
            FlowNode::Branch {
                ref mut antecedents,
            } => {
                if !antecedents.contains(&antecedent) {
                    antecedents.push(antecedent);
                }
            }
            _ => unreachable!(),
        }
    }

    fn with_new_start_node<F, T>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self) -> T,
        T: Into<ScopeHirId>,
    {
        let saved_curr_node = self.curr_node;
        self.curr_node = self.new_flow_node(FlowNode::Start);
        let hir = f(self).into();
        self.result.hir_to_flow_node.insert(hir, self.curr_node);
        self.curr_node = saved_curr_node;
    }
}

pub(crate) fn lower_to_code_flow_graph(module: &Module, scopes: &Scopes) -> CodeFlowGraph {
    let mut cx = CodeFlowLowerCtx::new(module, scopes);
    cx.lower_stmts(&module.top_level);
    cx.result
        .hir_to_flow_node
        .insert(ScopeHirId::Module, cx.curr_node);
    cx.result
}

#[salsa::tracked]
pub(crate) struct CodeFlowGraphResult {
    #[return_ref]
    pub(crate) cfg: CodeFlowGraph,
}

#[allow(unused)]
#[salsa::tracked]
pub(crate) fn code_flow_graph(db: &dyn Db, file: File) -> CodeFlowGraphResult {
    let info = lower(db, file);
    let scopes = module_scopes(db, file);
    let cfg = lower_to_code_flow_graph(info.module(db), scopes.scopes(db));
    CodeFlowGraphResult::new(db, cfg)
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use starpls_common::{Dialect, FileId};

    use super::*;
    use crate::test_database::TestDatabase;

    fn check(input: &str, expect: Expect) {
        let db = TestDatabase::default();
        let file_id = FileId(0);
        let file = File::new(&db, file_id, Dialect::Standard, None, input.to_string());
        let res = code_flow_graph(&db, file);
        let cfg = res.cfg(&db);
        expect.assert_eq(&cfg.pretty_print());
    }

    #[test]
    fn test_empty() {
        check(
            r#""#,
            expect![[r#"
            def main():
                'bb0: {
                    data: Start
                    antecedents: []
                }

        "#]],
        );
    }

    #[test]
    fn test_assign() {
        check(
            r#"
x = 1
y = "a"
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Start
                        antecedents: []
                    }

                    'bb1: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("x"), execution_scope: Module, source: Id { idx: 1 }, antecedent: Id { idx: 0 } }
                        antecedents: ['bb0]
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 2 }, name: Name("y"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

            "#]],
        );
    }

    #[test]
    fn test_if_only() {
        check(
            r#"
if x > 0:
    y = 1
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Start
                        antecedents: []
                    }

                    'bb1: {
                        data: Branch { antecedents: [Id { idx: 2 }, Id { idx: 0 }] }
                        antecedents: ['bb2, 'bb0]
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 3 }, name: Name("y"), execution_scope: Module, source: Id { idx: 4 }, antecedent: Id { idx: 0 } }
                        antecedents: ['bb0]
                    }

            "#]],
        );
    }

    #[test]
    fn test_separate_execution_scope() {
        check(
            r#"
def f():
    x = 1
    y = 2

x = 3
y = 4
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Start
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("x"), execution_scope: Def(Id { idx: 2 }), source: Id { idx: 1 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 2 }, name: Name("y"), execution_scope: Def(Id { idx: 2 }), source: Id { idx: 3 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 5 }, antecedent: Id { idx: 0 } }
                        antecedents: ['bb0]
                    }

                    'bb5: {
                        data: Assign { expr: Id { idx: 6 }, name: Name("y"), execution_scope: Module, source: Id { idx: 7 }, antecedent: Id { idx: 4 } }
                        antecedents: ['bb4]
                    }

            "#]],
        );
    }

    #[test]
    fn test_list_comp() {
        check(
            r#"
nums = [x for x in range(10)]        
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Start
                        antecedents: []
                    }

                    'bb1: {
                        data: Assign { expr: Id { idx: 5 }, name: Name("x"), execution_scope: Comp(Id { idx: 6 }), source: Id { idx: 4 }, antecedent: Id { idx: 0 } }
                        antecedents: ['bb0]
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("nums"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

            "#]],
        )
    }

    #[test]
    fn test_for_stmt() {
        check(
            r#"
for x, y in [[1, 2], [3, 4]]:
    nums = [(x * y * i) for i in range(5)]
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Start
                        antecedents: []
                    }

                    'bb1: {
                        data: Assign { expr: Id { idx: 7 }, name: Name("x"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 0 } }
                        antecedents: ['bb0]
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 8 }, name: Name("y"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 19 }, name: Name("i"), execution_scope: Comp(Id { idx: 20 }), source: Id { idx: 18 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Assign { expr: Id { idx: 9 }, name: Name("nums"), execution_scope: Module, source: Id { idx: 20 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

            "#]],
        );
    }
}
