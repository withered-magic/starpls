use either::Either;
use id_arena::Arena;
use id_arena::Id;
use rustc_hash::FxHashMap;
use starpls_common::File;

use crate::def::scope::module_scopes;
use crate::def::scope::ExecutionScopeId;
use crate::def::scope::ScopeHirId;
use crate::def::scope::Scopes;
use crate::def::CompClause;
use crate::def::Expr;
use crate::def::Stmt;
use crate::def::StmtId;
use crate::lower;
use crate::Db;
use crate::ExprId;
use crate::Module;
use crate::Name;

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
    Loop {
        antecedents: Vec<FlowNodeId>,
    },
    Call {
        expr: ExprId,
        antecedent: FlowNodeId,
    },
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CodeFlowGraph {
    pub(crate) flow_nodes: Arena<FlowNode>,
    pub(crate) hir_to_flow_node: FxHashMap<ScopeHirId, FlowNodeId>,
}

#[allow(unused)]
struct CodeFlowLoweringContext<'a> {
    module: &'a Module,
    scopes: &'a Scopes,
    result: CodeFlowGraph,
    curr_node: FlowNodeId,
    unreachable_node: FlowNodeId,
    curr_break_target: Option<FlowNodeId>,
    curr_continue_target: Option<FlowNodeId>,
}

impl<'a> CodeFlowLoweringContext<'a> {
    fn new(module: &'a Module, scopes: &'a Scopes) -> Self {
        let mut flow_nodes = Arena::new();
        let unreachable_node = flow_nodes.alloc(FlowNode::Unreachable);
        let curr_node = flow_nodes.alloc(FlowNode::Start);
        let cfg = CodeFlowGraph {
            flow_nodes,
            hir_to_flow_node: Default::default(),
        };
        CodeFlowLoweringContext {
            module,
            scopes,
            result: cfg,
            curr_node,
            unreachable_node,
            curr_break_target: None,
            curr_continue_target: None,
        }
    }

    fn lower_stmts(&mut self, stmts: &[StmtId]) {
        // Lower each statement in the list, stopping if we see unreachable code.
        for stmt in stmts {
            self.lower_stmt(*stmt);

            // If we find ourselves at an unreachable flow node, all remaining statements
            // are unreachable. Unreachable statements in general are not represented
            // in the code flow graph, so we can simply exit here.
            if self.curr_node == self.unreachable_node {
                break;
            }
        }
    }

    fn lower_stmt(&mut self, stmt: StmtId) {
        self.result
            .hir_to_flow_node
            .insert(stmt.into(), self.curr_node);

        match &self.module[stmt] {
            Stmt::Assign { lhs, rhs, .. } => {
                self.lower_assignment_target_and_source(*lhs, *rhs);
            }

            Stmt::Def { stmts, .. } => {
                self.with_new_start_node(|this| {
                    this.lower_stmts(stmts);
                    stmt
                });
                self.result
                    .hir_to_flow_node
                    .insert(stmt.into(), self.curr_node);
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
                        self.lower_stmts(else_stmts);
                        self.push_antecedent(post_if_node, self.curr_node);
                    }
                    _ => {
                        self.push_antecedent(post_if_node, pre_if_node);
                    }
                }

                self.curr_node = self.finish_branch_or_loop_node(post_if_node);
            }

            Stmt::Return { expr: Some(expr) } => {
                self.lower_expr(*expr);
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
                    self.lower_assignment_target_and_source(*target, *iterable);
                }

                let pre_for_node = self.new_flow_node(FlowNode::Loop {
                    antecedents: Vec::new(),
                });
                let post_for_node = self.new_flow_node(FlowNode::Branch {
                    antecedents: Vec::new(),
                });

                // Save the previous `break` and `continue` targets, and update them to point
                // to the pre-`for` and post-`for` nodes that we just allocated.
                let prev_break_target = self.curr_break_target;
                let prev_continue_target = self.curr_continue_target;
                self.curr_break_target = Some(post_for_node);
                self.curr_continue_target = Some(pre_for_node);

                // Lower the actual `for` statement body.
                self.push_antecedent(pre_for_node, self.curr_node);
                self.curr_node = pre_for_node;
                self.lower_stmts(stmts);

                // Wire up the pre-`for` and post-`for` nodes.
                self.push_antecedent(pre_for_node, self.curr_node);
                self.push_antecedent(post_for_node, pre_for_node);
                self.curr_node = self.finish_branch_or_loop_node(post_for_node);

                // Restore the previous `break` and `continue` targets.
                self.curr_break_target = prev_break_target;
                self.curr_continue_target = prev_continue_target;
            }

            Stmt::Continue => {
                if let Some(target) = &self.curr_continue_target {
                    self.push_antecedent(*target, self.curr_node);
                }
                self.curr_node = self.unreachable_node;
            }

            Stmt::Break => {
                if let Some(target) = &self.curr_break_target {
                    self.push_antecedent(*target, self.curr_node);
                }
                self.curr_node = self.unreachable_node;
            }

            _ => {}
        }
    }

    fn lower_expr(&mut self, expr: ExprId) {
        match &self.module[expr] {
            Expr::Name { .. } => {
                self.result
                    .hir_to_flow_node
                    .insert(expr.into(), self.curr_node);
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
            node @ Expr::Call { .. } => {
                node.walk_child_exprs(|expr| {
                    self.lower_expr(expr);
                });
                self.curr_node = self.new_flow_node(FlowNode::Call {
                    expr,
                    antecedent: self.curr_node,
                })
            }
            expr => expr.walk_child_exprs(|expr| {
                self.lower_expr(expr);
            }),
        }
    }

    fn lower_assignment_target_and_source(&mut self, expr: ExprId, source: ExprId) {
        self.lower_expr(source);
        self.lower_assignment_target(expr, source);
    }

    fn lower_assignment_target(&mut self, expr: ExprId, source: ExprId) {
        match &self.module[expr] {
            Expr::Name { ref name } => {
                self.curr_node = self.new_flow_node(FlowNode::Assign {
                    expr,
                    name: name.clone(),
                    execution_scope: self.scopes.execution_scope_for_hir_id(expr).unwrap(),
                    source,
                    antecedent: self.curr_node,
                });
                self.result
                    .hir_to_flow_node
                    .insert(expr.into(), self.curr_node);
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
        for comp_clause in comp_clauses.iter() {
            match comp_clause {
                CompClause::For { iterable, targets } => {
                    self.lower_expr(*iterable);
                    for target in targets.iter() {
                        self.lower_assignment_target_and_source(*target, *iterable);
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

    fn push_antecedent(&mut self, node: FlowNodeId, antecedent: FlowNodeId) {
        match self.result.flow_nodes[node] {
            FlowNode::Branch {
                ref mut antecedents,
            }
            | FlowNode::Loop {
                ref mut antecedents,
            } => {
                if antecedent != self.unreachable_node && !antecedents.contains(&antecedent) {
                    antecedents.push(antecedent);
                }
            }
            _ => unreachable!(),
        }
    }

    fn finish_branch_or_loop_node(&mut self, node: FlowNodeId) -> FlowNodeId {
        match self.result.flow_nodes[node] {
            FlowNode::Branch { ref antecedents } | FlowNode::Loop { ref antecedents } => {
                if antecedents.is_empty() {
                    self.unreachable_node
                } else {
                    node
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
    let mut cx = CodeFlowLoweringContext::new(module, scopes);
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
    use expect_test::expect;
    use expect_test::Expect;
    use starpls_common::Dialect;
    use starpls_common::FileId;

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
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
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
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("x"), execution_scope: Module, source: Id { idx: 1 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 2 }, name: Name("y"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
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
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Branch { antecedents: [Id { idx: 3 }, Id { idx: 1 }] }
                        antecedents: ['bb3, 'bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 3 }, name: Name("y"), execution_scope: Module, source: Id { idx: 4 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
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
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Start
                        antecedents: []
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("x"), execution_scope: Def(Id { idx: 2 }), source: Id { idx: 1 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Assign { expr: Id { idx: 2 }, name: Name("y"), execution_scope: Def(Id { idx: 2 }), source: Id { idx: 3 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

                    'bb5: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 5 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb6: {
                        data: Assign { expr: Id { idx: 6 }, name: Name("y"), execution_scope: Module, source: Id { idx: 7 }, antecedent: Id { idx: 5 } }
                        antecedents: ['bb5]
                    }

            "#]],
        );
    }

    #[test]
    fn test_list_comp() {
        check(
            r#"
nums = [x for x in [1, 2, 3]]
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 6 }, name: Name("x"), execution_scope: Comp(Id { idx: 7 }), source: Id { idx: 5 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("nums"), execution_scope: Module, source: Id { idx: 7 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
                    }

            "#]],
        )
    }

    #[test]
    fn test_for_stmt() {
        check(
            r#"
for x, y in [[1, 2, 3], [3, 4]]:
    nums = [(x * y * i) for i in [1, 2, 3]]
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 8 }, name: Name("x"), execution_scope: Module, source: Id { idx: 7 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Assign { expr: Id { idx: 9 }, name: Name("y"), execution_scope: Module, source: Id { idx: 7 }, antecedent: Id { idx: 2 } }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Loop { antecedents: [Id { idx: 3 }, Id { idx: 7 }] }
                        antecedents: ['bb3, 'bb7]
                    }

                    'bb5: {
                        data: Branch { antecedents: [Id { idx: 4 }] }
                        antecedents: ['bb4]
                    }

                    'bb6: {
                        data: Assign { expr: Id { idx: 21 }, name: Name("i"), execution_scope: Comp(Id { idx: 22 }), source: Id { idx: 20 }, antecedent: Id { idx: 4 } }
                        antecedents: ['bb4]
                    }

                    'bb7: {
                        data: Assign { expr: Id { idx: 10 }, name: Name("nums"), execution_scope: Module, source: Id { idx: 22 }, antecedent: Id { idx: 6 } }
                        antecedents: ['bb6]
                    }

            "#]],
        );
    }

    #[test]
    fn test_for_stmt_simple() {
        check(
            r#"
for x in [1, 2, 3]:
    pass
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }, Id { idx: 3 }] }
                        antecedents: ['bb2, 'bb3]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 3 }] }
                        antecedents: ['bb3]
                    }

            "#]],
        );
    }

    #[test]
    fn test_break_stmt() {
        check(
            r#"
for x in [1, 2, 3]:
    break
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }] }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 3 }] }
                        antecedents: ['bb3]
                    }

            "#]],
        );
    }

    #[test]
    fn test_break_stmt_with_unreachable() {
        check(
            r#"
for x in [1, 2, 3]:
    y = 1
    break
    z = 1

a = 1
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }] }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 5 }, Id { idx: 3 }] }
                        antecedents: ['bb5, 'bb3]
                    }

                    'bb5: {
                        data: Assign { expr: Id { idx: 5 }, name: Name("y"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

                    'bb6: {
                        data: Assign { expr: Id { idx: 9 }, name: Name("a"), execution_scope: Module, source: Id { idx: 10 }, antecedent: Id { idx: 4 } }
                        antecedents: ['bb4]
                    }

            "#]],
        );
    }

    #[test]
    fn test_break_stmt_nested() {
        check(
            r#"
for x in [1, 2, 3]:
    for y in [4, 5, 6]:
        break
        a = 1
    break
    b = 2
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }] }
                        antecedents: ['bb2]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 7 }, Id { idx: 3 }] }
                        antecedents: ['bb7, 'bb3]
                    }

                    'bb5: {
                        data: Assign { expr: Id { idx: 9 }, name: Name("y"), execution_scope: Module, source: Id { idx: 8 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

                    'bb6: {
                        data: Loop { antecedents: [Id { idx: 5 }] }
                        antecedents: ['bb5]
                    }

                    'bb7: {
                        data: Branch { antecedents: [Id { idx: 6 }] }
                        antecedents: ['bb6]
                    }

            "#]],
        );
    }

    #[test]
    fn test_continue_stmt() {
        check(
            r#"
for x in [1, 2, 3]:
    y = 1
    continue
    z = 2
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("x"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }, Id { idx: 5 }] }
                        antecedents: ['bb2, 'bb5]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 3 }] }
                        antecedents: ['bb3]
                    }

                    'bb5: {
                        data: Assign { expr: Id { idx: 5 }, name: Name("y"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

            "#]],
        )
    }

    #[test]
    fn test_if_else_continue() {
        check(
            r#"
for i in [1, 2, 3]:
    if 1 < 2:
        continue
    else:
        continue
    x = 123
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 4 }, name: Name("i"), execution_scope: Module, source: Id { idx: 3 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Loop { antecedents: [Id { idx: 2 }, Id { idx: 3 }] }
                        antecedents: ['bb2, 'bb3]
                    }

                    'bb4: {
                        data: Branch { antecedents: [Id { idx: 3 }] }
                        antecedents: ['bb3]
                    }

                    'bb5: {
                        data: Branch { antecedents: [] }
                        antecedents: []
                    }

            "#]],
        );
    }

    #[test]
    fn test_call() {
        check(
            r#"
x = 123
fail("foo")
y = 456
"#,
            expect![[r#"
                def main():
                    'bb0: {
                        data: Unreachable
                        antecedents: []
                    }

                    'bb1: {
                        data: Start
                        antecedents: []
                    }

                    'bb2: {
                        data: Assign { expr: Id { idx: 0 }, name: Name("x"), execution_scope: Module, source: Id { idx: 1 }, antecedent: Id { idx: 1 } }
                        antecedents: ['bb1]
                    }

                    'bb3: {
                        data: Call { expr: Id { idx: 4 }, antecedent: Id { idx: 2 } }
                        antecedents: []
                    }

                    'bb4: {
                        data: Assign { expr: Id { idx: 5 }, name: Name("y"), execution_scope: Module, source: Id { idx: 6 }, antecedent: Id { idx: 3 } }
                        antecedents: ['bb3]
                    }

            "#]],
        );
    }
}
