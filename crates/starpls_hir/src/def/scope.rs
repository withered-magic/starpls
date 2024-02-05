use crate::{
    def::{
        CompClause, Declaration, Expr, ExprId, Function, LoadItem, Param, ParamId, Stmt, StmtId,
    },
    lower, Db, Module, ModuleInfo, ModuleSourceMap, Name,
};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use starpls_common::{Diagnostic, Diagnostics, File, FileRange, Severity};
use std::collections::{hash_map::Entry, VecDeque};

pub(crate) type ScopeId = Id<Scope>;

#[salsa::tracked]
pub struct ModuleScopes {
    #[return_ref]
    pub(crate) scopes: Scopes,
}

#[salsa::tracked]
pub(crate) fn module_scopes_query(db: &dyn Db, info: ModuleInfo) -> ModuleScopes {
    let scopes = Scopes::new_for_module(db, info);
    ModuleScopes::new(db, scopes)
}

#[salsa::tracked]
pub fn module_scopes(db: &dyn Db, file: File) -> ModuleScopes {
    let info = lower(db, file);
    module_scopes_query(db, info)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub(crate) declarations: FxHashMap<Name, Vec<Declaration>>,
    pub(crate) parent: Option<ScopeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeHirId {
    Module,
    Expr(ExprId),
    Stmt(StmtId),
}

impl From<ExprId> for ScopeHirId {
    fn from(value: ExprId) -> Self {
        Self::Expr(value)
    }
}

impl From<StmtId> for ScopeHirId {
    fn from(value: StmtId) -> Self {
        Self::Stmt(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scopes {
    pub(crate) scopes: Arena<Scope>,
    pub(crate) scopes_by_hir_id: FxHashMap<ScopeHirId, ScopeId>,
}

struct DeferredScope {
    parent: ScopeId,
    data: FunctionData,
}

struct FunctionData {
    func: Function,
    params: Box<[ParamId]>,
    stmts: Box<[StmtId]>,
}

impl Scopes {
    fn new_for_module(db: &dyn Db, info: ModuleInfo) -> Self {
        ScopeCollector {
            db,
            deferred: VecDeque::new(),
            file: info.file(db),
            module: info.module(db),
            source_map: info.source_map(db),
            scopes: Scopes {
                scopes: Default::default(),
                scopes_by_hir_id: Default::default(),
            },
        }
        .collect()
    }

    fn alloc_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: Some(parent),
        })
    }

    fn add_decl(&mut self, scope: ScopeId, name: Name, decl: Declaration) {
        match self.scopes[scope].declarations.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(decl);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![decl]);
            }
        }
    }

    pub fn scope_for_hir_id(&self, id: impl Into<ScopeHirId>) -> Option<ScopeId> {
        self.scopes_by_hir_id.get(&id.into()).copied()
    }

    pub(crate) fn scope_chain(&self, scope: Option<ScopeId>) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(scope, |scope| self.scopes[*scope].parent)
    }
}

struct ScopeCollector<'a> {
    db: &'a dyn Db,
    deferred: VecDeque<DeferredScope>,
    file: File,
    module: &'a Module,
    source_map: &'a ModuleSourceMap,
    scopes: Scopes,
}

impl ScopeCollector<'_> {
    fn collect(mut self) -> Scopes {
        // Allocate the root module scope.
        let root = self.scopes.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: None,
        });

        // Compute scopes by walking the module HIR, starting at the top-level statements.
        let root = self.collect_stmts_defer(&self.module.top_level, root);
        self.scopes
            .scopes_by_hir_id
            .insert(ScopeHirId::Module, root);

        // Compute deferred scopes. This mainly applies to function definitions.
        while let Some(DeferredScope { parent, data }) = self.deferred.pop_front() {
            let scope = self.scopes.alloc_scope(parent);
            for param in data.params.into_iter().copied() {
                match &self.module.params[param] {
                    Param::Simple { name, .. }
                    | Param::ArgsList { name, .. }
                    | Param::KwargsDict { name, .. } => {
                        self.scopes.add_decl(
                            scope,
                            name.clone(),
                            Declaration::Parameter {
                                id: param,
                                func: Some(data.func),
                            },
                        );
                    }
                }
            }
            self.collect_stmts_defer(&data.stmts, scope);
        }

        self.scopes
    }

    fn collect_stmts_defer(&mut self, stmts: &Box<[StmtId]>, mut current: ScopeId) -> ScopeId {
        let mut deferred = VecDeque::new();
        for stmt in stmts.iter().copied() {
            self.collect_stmt(&mut deferred, stmt, &mut current);
        }
        while let Some(data) = deferred.pop_front() {
            self.deferred.push_back(DeferredScope {
                parent: current,
                data,
            });
        }
        current
    }

    fn collect_stmts(
        &mut self,
        deferred: &mut VecDeque<FunctionData>,
        stmts: &Box<[StmtId]>,
        current: &mut ScopeId,
    ) {
        for stmt in stmts.iter().copied() {
            self.collect_stmt(deferred, stmt, current);
        }
    }

    fn collect_stmt(
        &mut self,
        deferred: &mut VecDeque<FunctionData>,
        stmt: StmtId,
        current: &mut ScopeId,
    ) {
        match &self.module.stmts[stmt] {
            Stmt::Def { func, stmts } => {
                *current = self.scopes.alloc_scope(*current);
                self.scopes.add_decl(
                    *current,
                    func.name(self.db).clone(),
                    Declaration::Function {
                        id: stmt,
                        func: *func,
                    },
                );
                deferred.push_back(FunctionData {
                    params: func.params(self.db).clone(),
                    stmts: stmts.clone(),
                    func: *func,
                });
            }
            Stmt::If {
                if_stmts,
                elif_stmt,
                else_stmts,
                test,
            } => {
                self.collect_expr(*test, *current, None);
                self.collect_stmts(deferred, if_stmts, current);
                if let Some(elif_stmt) = elif_stmt {
                    self.collect_stmt(deferred, *elif_stmt, current);
                }
                self.collect_stmts(deferred, else_stmts, current);
            }
            Stmt::For {
                iterable,
                targets,
                stmts,
            } => {
                self.collect_expr(*iterable, *current, None);
                targets.iter().copied().for_each(|expr| {
                    self.collect_expr(expr, *current, Some(*iterable));
                });
                self.collect_stmts(deferred, stmts, current);
            }
            Stmt::Assign { lhs, rhs, .. } => {
                self.collect_expr(*rhs, *current, None);
                *current = self.scopes.alloc_scope(*current);
                self.collect_expr(*lhs, *current, Some(*rhs));
            }
            Stmt::Load { load_stmt, items } => {
                *current = self.scopes.alloc_scope(*current);
                for item in items.iter() {
                    let name: &str = match &self.module.load_items[*item] {
                        LoadItem::Direct { name, .. } => &name,
                        LoadItem::Aliased { alias, .. } => alias.as_str(),
                    };
                    self.scopes.add_decl(
                        *current,
                        Name::from_str(name),
                        Declaration::LoadItem {
                            id: *item,
                            load_stmt: *load_stmt,
                        },
                    )
                }
            }
            Stmt::Return { expr } => {
                if let Some(expr) = expr {
                    self.collect_expr(*expr, *current, None);
                }
            }
            Stmt::Expr { expr } => {
                self.collect_expr(*expr, *current, None);
            }
            _ => return,
        }
        self.scopes.scopes_by_hir_id.insert(stmt.into(), *current);
    }

    fn collect_expr(&mut self, expr: ExprId, current: ScopeId, source: Option<ExprId>) {
        if let Some(source) = source {
            // Possible assignment targets: NAME, LIST, TUPLE, PAREN, DOT, INDEX, SLICE.
            match &self.module[expr] {
                Expr::Name { name } => {
                    self.scopes.add_decl(
                        current,
                        name.clone(),
                        Declaration::Variable {
                            id: expr,
                            source: Some(source),
                        },
                    );
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::List { exprs } | Expr::Tuple { exprs } => {
                    exprs.iter().copied().for_each(|expr| {
                        self.collect_expr(expr, current, Some(source));
                    });
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::Paren { expr: paren_expr } => {
                    self.collect_expr(*paren_expr, current, Some(source));
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                hir_expr @ (Expr::Dot { .. } | Expr::Index { .. } | Expr::Slice { .. }) => {
                    hir_expr.walk_child_exprs(|expr| self.collect_expr(expr, current, None));
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::Missing => {}
                _ => Diagnostics::push(
                    self.db,
                    Diagnostic {
                        message: "Expression is not assignable".to_string(),
                        severity: Severity::Error,
                        range: FileRange {
                            file_id: self.file.id(self.db),
                            range: self
                                .source_map
                                .expr_map_back
                                .get(&expr)
                                .expect("expected expr to exist in source map")
                                .syntax_node_ptr()
                                .text_range(),
                        },
                    },
                ),
            }
        } else {
            match &self.module[expr] {
                Expr::Missing => {}
                Expr::Name { .. } => {
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::Lambda { params, body } => {
                    let scope = self.scopes.alloc_scope(current);
                    for param in params.into_iter().copied() {
                        match &self.module.params[param] {
                            Param::Simple { name, .. }
                            | Param::ArgsList { name, .. }
                            | Param::KwargsDict { name, .. } => {
                                self.scopes.add_decl(
                                    scope,
                                    name.clone(),
                                    Declaration::Parameter {
                                        id: param,
                                        func: None,
                                    },
                                );
                            }
                        }
                    }
                    self.collect_expr(*body, scope, None);
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::Tuple { exprs } => {
                    exprs.iter().copied().for_each(|expr| {
                        self.collect_expr(expr, current, source);
                    });
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::Paren { expr: paren_expr } => {
                    self.collect_expr(*paren_expr, current, source);
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::List { exprs } => {
                    exprs.iter().copied().for_each(|expr| {
                        self.collect_expr(expr, current, source);
                    });
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::DictComp {
                    entry,
                    comp_clauses,
                } => {
                    let mut comp = current;
                    self.collect_comp_clauses(comp_clauses, &mut comp);
                    self.collect_expr(entry.key, comp, None);
                    self.collect_expr(entry.value, comp, None);
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                Expr::ListComp {
                    expr: list_expr,
                    comp_clauses,
                } => {
                    let mut comp = current;
                    self.collect_comp_clauses(comp_clauses, &mut comp);
                    self.collect_expr(*list_expr, comp, None);
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
                hir_expr => {
                    hir_expr.walk_child_exprs(|expr| self.collect_expr(expr, current, None));
                    self.scopes.scopes_by_hir_id.insert(expr.into(), current);
                }
            }
        }
    }

    fn collect_comp_clauses(&mut self, comp_clauses: &Box<[CompClause]>, current: &mut ScopeId) {
        for comp_clause in comp_clauses.into_iter() {
            match comp_clause {
                CompClause::For { iterable, targets } => {
                    self.collect_expr(*iterable, *current, None);
                    *current = self.scopes.alloc_scope(*current);
                    targets.iter().copied().for_each(|expr| {
                        self.collect_expr(expr, *current, Some(*iterable));
                    });
                }
                CompClause::If { test } => {
                    self.collect_expr(*test, *current, None);
                }
            }
        }
    }
}
