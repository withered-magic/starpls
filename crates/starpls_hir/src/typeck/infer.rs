use std::sync::Arc;

use either::Either;
use starpls_common::line_index;
use starpls_common::parse;
use starpls_common::Diagnostic;
use starpls_common::DiagnosticTag;
use starpls_common::File;
use starpls_common::FileRange;
use starpls_common::InFile;
use starpls_common::Severity;
use starpls_syntax::ast::ArithOp;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::AstPtr;
use starpls_syntax::ast::BinaryOp;
use starpls_syntax::ast::BitwiseOp;
use starpls_syntax::ast::LogicOp;
use starpls_syntax::ast::UnaryOp;
use starpls_syntax::ast::{self};
use starpls_syntax::TextRange;

use crate::def::codeflow::code_flow_graph;
use crate::def::codeflow::CodeFlowGraph;
use crate::def::codeflow::FlowNode;
use crate::def::codeflow::FlowNodeId;
use crate::def::resolver::Export;
use crate::def::resolver::Resolver;
use crate::def::scope::module_scopes;
use crate::def::scope::ExecutionScopeId;
use crate::def::scope::FunctionDef;
use crate::def::scope::LoadItemDef;
use crate::def::scope::ParameterDef;
use crate::def::scope::ScopeDef;
use crate::def::scope::ScopeHirId;
use crate::def::scope::VariableDef;
use crate::def::Argument;
use crate::def::Expr;
use crate::def::ExprId;
use crate::def::InternedString;
use crate::def::Literal;
use crate::def::LoadItem;
use crate::def::LoadItemId;
use crate::def::LoadStmt;
use crate::def::Param;
use crate::def::ParamId;
use crate::def::Stmt;
use crate::def::StmtId;
use crate::display::DisplayWithDb;
use crate::module;
use crate::source_map;
use crate::typeck::assign_tys;
use crate::typeck::builtins::builtin_types;
use crate::typeck::call::Slot;
use crate::typeck::call::SlotProvider;
use crate::typeck::call::Slots;
use crate::typeck::intrinsics::IntrinsicFunctionParam;
use crate::typeck::intrinsics::IntrinsicTypes;
use crate::typeck::resolve_builtin_type_ref;
use crate::typeck::resolve_type_ref;
use crate::typeck::resolve_type_ref_opt;
use crate::typeck::CodeFlowCacheKey;
use crate::typeck::DictLiteral;
use crate::typeck::FileExprId;
use crate::typeck::FileLoadItemId;
use crate::typeck::FileLoadStmt;
use crate::typeck::FileParamId;
use crate::typeck::Protocol;
use crate::typeck::Provider;
use crate::typeck::RuleKind;
use crate::typeck::Struct;
use crate::typeck::Substitution;
use crate::typeck::Tuple;
use crate::typeck::Ty;
use crate::typeck::TyContext;
use crate::typeck::TyData;
use crate::typeck::TyKind;
use crate::typeck::TypeRef;
use crate::typeck::TypecheckCancelled;
use crate::Name;

impl TyContext<'_> {
    fn infer_all_exprs(&mut self, file: File) {
        for (expr, _) in module(self.db, file).exprs.iter() {
            self.infer_expr(file, expr);
        }
    }

    fn infer_all_params(&mut self, file: File) {
        for (param, _) in module(self.db, file).params.iter() {
            self.infer_param(file, param);
        }
    }

    fn walk_stmts(&mut self, file: File, stmts: &[StmtId]) {
        if stmts.is_empty() {
            return;
        }

        if !self.shared_state.options.use_code_flow_analysis {
            for stmt in stmts.iter() {
                self.walk_stmt(file, *stmt);
            }
            return;
        }

        let cfg = code_flow_graph(self.db, file).cfg(self.db);
        let mut prev_flow_node_or_unreachable = {
            self.walk_stmt(file, stmts[0]);
            if let Some(flow_node_id) = cfg.hir_to_flow_node.get(&ScopeHirId::Stmt(stmts[0])) {
                Either::Left(*flow_node_id)
            } else {
                Either::Right(stmts[0])
            }
        };

        for stmt in stmts[1..].iter() {
            self.walk_stmt(file, *stmt);

            if let Either::Left(prev_flow_node) = prev_flow_node_or_unreachable {
                // Check for reachability.
                match cfg.hir_to_flow_node.get(&ScopeHirId::Stmt(*stmt)) {
                    Some(flow_node_id) => {
                        prev_flow_node_or_unreachable =
                            if self.exists_flow_path(cfg, file, *flow_node_id, prev_flow_node) {
                                Either::Left(*flow_node_id)
                            } else {
                                Either::Right(*stmt)
                            }
                    }
                    None => prev_flow_node_or_unreachable = Either::Right(*stmt),
                }
            }
        }

        if let Either::Right(unreachable_start) = prev_flow_node_or_unreachable {
            let unreachable_end = stmts[stmts.len() - 1];
            let source_map = source_map(self.db, file);
            if let Some((start, end)) =
                source_map
                    .stmt_map_back
                    .get(&unreachable_start)
                    .and_then(|start| {
                        source_map.stmt_map_back.get(&unreachable_end).map(|end| {
                            (
                                start.syntax_node_ptr().text_range().start(),
                                end.syntax_node_ptr().text_range().end(),
                            )
                        })
                    })
            {
                self.add_diagnostic_for_range(
                    file,
                    Severity::Warning,
                    TextRange::new(start, end),
                    Some(vec![DiagnosticTag::Unnecessary]),
                    "Code is unreachable".to_string(),
                );
            }
        }
    }

    fn walk_stmt(&mut self, file: File, stmt: StmtId) {
        let module = module(self.db, file);

        match &module[stmt] {
            Stmt::Load { load_stmt, items } => {
                self.resolve_load_stmt(file, *load_stmt);
                for load_item in items.iter().copied() {
                    self.infer_load_item(file, load_item);
                }
            }
            Stmt::Assign { lhs, rhs, .. } => match &module[*lhs] {
                Expr::Index { .. } => {
                    let lhs_ty = self.infer_expr(file, *lhs);
                    let rhs_ty = self.infer_expr(file, *rhs);
                    if !assign_tys(self.db, &rhs_ty, &lhs_ty) {
                        self.add_expr_diagnostic_error(
                            file,
                            *lhs,
                            format!(
                                "Cannot use value of type \"{}\" as type \"{}\" in assignment",
                                rhs_ty.display(self.db).alt(),
                                lhs_ty.display(self.db).alt()
                            ),
                        );
                    }
                }
                Expr::Dot { expr, field } => {
                    let ty = self.infer_expr(file, *expr);
                    if matches!(
                        ty.kind(),
                        TyKind::Struct(_) | TyKind::Provider(_) | TyKind::BuiltinType(_, _)
                    ) {
                        self.add_expr_diagnostic_error(
                            file,
                            *lhs,
                            format!(
                                "Cannot assign to field \"{}\" for immutable type \"{}\"",
                                field.as_str(),
                                ty.display(self.db).alt()
                            ),
                        );
                    } else if let Some(name) = ty
                        .fields(self.db)
                        .and_then(|mut fields| fields.find(|(el, _)| &el.name(self.db) == field))
                        .and_then(|(field, ty)| {
                            if matches!(
                                ty.kind(),
                                TyKind::Function(_)
                                    | TyKind::BuiltinFunction(_)
                                    | TyKind::IntrinsicFunction(_, _)
                            ) {
                                Some(field.name(self.db))
                            } else {
                                None
                            }
                        })
                    {
                        self.add_expr_diagnostic_error(
                            file,
                            *lhs,
                            format!(
                                "Cannot reassign to method \"{}\" of type \"{}\"",
                                name,
                                ty.display(self.db).alt()
                            ),
                        );
                    }
                }
                _ => {}
            },
            Stmt::Def { stmts, .. } | Stmt::For { stmts, .. } => {
                self.walk_stmts(file, stmts);
            }
            Stmt::If {
                if_stmts,
                elif_or_else_stmts,
                ..
            } => {
                self.walk_stmts(file, if_stmts);
                match elif_or_else_stmts {
                    Some(Either::Left(stmt)) => self.walk_stmt(file, *stmt),
                    Some(Either::Right(stmts)) => self.walk_stmts(file, stmts),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn report_unused_definitions(&mut self, file: File) {
        for (expr, name) in module(self.db, file).exprs.iter().filter_map(|(id, expr)| {
            if let Expr::Name { name } = expr {
                Some((id, name))
            } else {
                None
            }
        }) {
            // Don't report assignments to the placeholder "_" as unused.
            if name.as_str() == "_" {
                continue;
            }

            if let Some(false) = self.cx.definition_is_used.get(&InFile {
                file,
                value: Either::Left(expr),
            }) {
                self.add_expr_diagnostic_warning(
                    file,
                    expr,
                    Some(vec![DiagnosticTag::Unnecessary]),
                    format!("\"{}\" is not accessed", name.as_str()),
                );
            }
        }

        for (stmt, name) in module(self.db, file)
            .stmts
            .iter()
            .filter_map(|(id, stmt)| match stmt {
                Stmt::Def { func, .. } => {
                    let name = func.name(self.db);
                    if name.is_missing() {
                        None
                    } else {
                        Some((id, name))
                    }
                }
                _ => None,
            })
        {
            if !self.cx.definition_is_used.contains_key(&InFile {
                file,
                value: Either::Right(stmt),
            }) {
                fn maybe_add_diagnostic(
                    tcx: &mut TyContext,
                    file: File,
                    stmt: StmtId,
                    name: &Name,
                ) -> Option<()> {
                    // Don't report exported functions as unused.
                    let scopes = module_scopes(tcx.db, file).scopes(tcx.db);
                    let scope = scopes.scopes_by_hir_id.get(&ScopeHirId::Stmt(stmt))?;
                    if scopes.scopes[*scope].execution_scope == ExecutionScopeId::Module
                        && !name.as_str().starts_with('_')
                    {
                        return None;
                    }

                    let ptr = source_map(tcx.db, file).stmt_map_back.get(&stmt)?;
                    let node = ptr
                        .syntax_node_ptr()
                        .try_to_node(&parse(tcx.db, file).syntax(tcx.db))?;
                    let def_stmt = ast::DefStmt::cast(node)?;
                    let name_node = def_stmt.name()?;

                    tcx.add_diagnostic_for_range(
                        file,
                        Severity::Warning,
                        name_node.syntax().text_range(),
                        Some(vec![DiagnosticTag::Unnecessary]),
                        format!("\"{}\" is not accessed", name.as_str()),
                    );
                    Some(())
                }

                maybe_add_diagnostic(self, file, stmt, &name);
            }
        }
    }

    pub fn diagnostics_for_file(&mut self, file: File) -> Vec<Diagnostic> {
        let module = module(self.db, file);

        self.infer_all_exprs(file);
        self.infer_all_params(file);
        self.walk_stmts(file, &module.top_level);

        if !self.shared_state.options.allow_unused_definitions {
            self.report_unused_definitions(file);
        }

        let line_index = line_index(self.db, file);
        self.cx
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                if diagnostic.range.file_id != file.id(self.db) {
                    return false;
                }
                let start_line = line_index.line_col(diagnostic.range.range.start()).line;
                let end_line = line_index.line_col(diagnostic.range.range.end()).line;
                (start_line..=end_line)
                    .all(|line| !module.type_ignore_comment_lines.contains(&line))
            })
            .cloned()
            .collect()
    }

    fn unwind_if_cancelled(&self) {
        if self.shared_state.cancelled.load() {
            TypecheckCancelled.throw();
        }
    }

    pub(crate) fn infer_expr(&mut self, file: File, expr: ExprId) -> Ty {
        if let Some(ty) = self
            .cx
            .type_of_expr
            .get(&FileExprId::new(file, expr))
            .cloned()
        {
            return ty;
        }

        self.unwind_if_cancelled();

        let db = self.db;
        let curr_module = module(db, file);
        let ty = match &curr_module[expr] {
            Expr::Name { name } => {
                let ty = self
                    .infer_name(file, name, expr)
                    .unwrap_or_else(|| self.unbound_ty());

                // Report unbound and possibly unbound variables.
                if ty.is_unbound() {
                    self.add_expr_diagnostic_error(
                        file,
                        expr,
                        format!("\"{}\" is not defined", name.as_str()),
                    );
                } else if ty.is_possibly_unbound() {
                    self.add_expr_diagnostic_error(
                        file,
                        expr,
                        format!("\"{}\" is possibly unbound", name.as_str()),
                    )
                }
                ty
            }
            Expr::List { exprs } => {
                // Determine the full type of the list. If all of the specified elements are of the same type T, then
                // we assign the list the type `list[T]`. Otherwise, we assign it the type `list[Unknown]`.
                TyKind::List(self.get_common_type(file, exprs.iter().cloned(), self.unknown_ty()))
                    .intern()
            }
            Expr::ListComp { expr, .. } => TyKind::List(self.infer_expr(file, *expr)).intern(),
            Expr::Dict { entries } => {
                let key_ty = match entries.len() {
                    0 => Ty::unknown(),
                    len if len > 32 => {
                        return self.set_expr_type(
                            file,
                            expr,
                            Ty::dict(Ty::unknown(), Ty::unknown(), None),
                        );
                    }
                    _ => Ty::union(entries.iter().map(|entry| self.infer_expr(file, entry.key))),
                };
                let value_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.value),
                    self.unknown_ty(),
                );

                // Determine the list of known string keys from the entries.
                let known_keys = entries
                    .iter()
                    .filter_map(|entry| match &curr_module[entry.key] {
                        Expr::Literal {
                            literal: Literal::String(s),
                        } => Some((*s, self.infer_expr(file, entry.value))),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                TyKind::Dict(
                    key_ty,
                    value_ty,
                    Some(Arc::new(DictLiteral {
                        expr: Some(InFile { file, value: expr }),
                        known_keys: known_keys.into_boxed_slice(),
                    })),
                )
                .intern()
            }
            Expr::DictComp { entry, .. } => {
                let key_ty = self.infer_expr(file, entry.key);
                let value_ty = self.infer_expr(file, entry.value);
                TyKind::Dict(key_ty, value_ty, None).intern()
            }
            Expr::Literal { literal } => match literal {
                Literal::Int(x) => TyKind::Int(i64::try_from(*x).ok()).intern(),
                Literal::Float => self.float_ty(),
                Literal::String(s) => TyKind::String(Some(*s)).intern(),
                Literal::Bytes => self.bytes_ty(),
                Literal::Bool(b) => TyKind::Bool(Some(*b)).intern(),
                Literal::None => self.none_ty(),
            },
            Expr::Unary {
                op,
                expr: unary_expr,
            } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(file, expr, *unary_expr, *op))
                .unwrap_or_else(|| self.unknown_ty()),
            Expr::Binary { lhs, rhs, op } => op
                .as_ref()
                .map(|op| self.infer_binary_expr(file, expr, *lhs, *rhs, op.clone()))
                .unwrap_or_else(|| self.unknown_ty()),
            Expr::Dot {
                expr: dot_expr,
                field,
            } => {
                let receiver_ty = self.infer_expr(file, *dot_expr);
                match receiver_ty.kind() {
                    TyKind::Unknown
                    | TyKind::Unbound
                    | TyKind::Any
                    | TyKind::ProviderInstance(Provider::Custom(_)) => self.unknown_ty(),
                    _ => {
                        if field.is_missing() {
                            return self.unknown_ty();
                        }

                        receiver_ty
                            .fields(db)
                            .and_then(|mut fields| {
                                fields.find_map(|(f, ty)| {
                                    if &f.name(db) == field {
                                        Some(ty.clone())
                                    } else {
                                        None
                                    }
                                })
                            })
                            .unwrap_or_else(|| {
                                match receiver_ty.kind() {
                                    TyKind::Struct(Some(Struct::FieldSignature { ty })) => {
                                        return ty.clone()
                                    }
                                    TyKind::Struct(Some(Struct::RuleAttributes {
                                        rule_kind,
                                        attrs,
                                    })) => {
                                        return attrs
                                            .attrs
                                            .iter()
                                            .find_map(|(name, attr)| {
                                                if name == field {
                                                    attr.as_ref()
                                                        .map(|attr| attr.resolved_ty(rule_kind))
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap_or_else(|| self.unknown_ty());
                                    }
                                    TyKind::Struct(_) | TyKind::ProviderInstance(_) => {
                                        return self.unknown_ty()
                                    }
                                    _ => {}
                                }

                                self.add_expr_diagnostic_warning_ty(
                                    file,
                                    expr,
                                    format!(
                                        "Cannot access field \"{}\" for type \"{}\"",
                                        field.as_str(),
                                        receiver_ty.display(db).alt()
                                    ),
                                )
                            })
                    }
                }
            }
            Expr::Index { lhs, index } => {
                let lhs_ty = self.infer_expr(file, *lhs);
                let index_ty = self.infer_expr(file, *index);
                let int_ty = self.int_ty();
                let string_ty = self.string_ty();

                // Tuples, Lists, dictionaries, strings, byte literals, and range values, as
                // well as the `Indexable` and `SetIndexable` protocols, support indexing.
                let (target, value, name) = match lhs_ty.kind() {
                    TyKind::Tuple(Tuple::Variable(ty)) => (&int_ty, ty, "tuple"),
                    TyKind::Tuple(Tuple::Simple(tys)) => {
                        let return_ty = match index_ty.kind() {
                            TyKind::Int(Some(x)) => match tys.get(*x as usize) {
                                Some(ty) => ty.clone(),
                                None => self.add_expr_diagnostic_error_ty(
                                    file,
                                    expr,
                                    format!(
                                        "Index {} is out of range for type {}",
                                        x,
                                        lhs_ty.display(db).alt()
                                    ),
                                ),
                            },
                            TyKind::Int(None) => Ty::union(tys.iter().cloned()),
                            _ => self.add_expr_diagnostic_error_ty(
                                file,
                                expr,
                                format!(
                                    "Cannot index tuple with type \"{}\"",
                                    index_ty.display(db).alt()
                                ),
                            ),
                        };

                        return self.set_expr_type(file, expr, return_ty);
                    }
                    TyKind::List(ty) => (&int_ty, ty, "list"),
                    TyKind::Dict(key_ty, value_ty, _) => (key_ty, value_ty, "dict"),
                    TyKind::String(_) => (&int_ty, &string_ty, "string"),
                    TyKind::Bytes => (&int_ty, &int_ty, "bytes"),
                    TyKind::Range => (&int_ty, &int_ty, "range"),
                    kind => {
                        let return_ty = match (kind, index_ty.kind()) {
                            (
                                TyKind::Any | TyKind::Unknown | TyKind::Target,
                                TyKind::Provider(provider),
                            ) => Some(TyKind::ProviderInstance(provider.clone()).intern()),
                            (TyKind::Any | TyKind::Unknown, _) => Some(Ty::unknown()),
                            (TyKind::BuiltinType(ty, _), _) => match ty.indexable_by(db) {
                                Some((expected_index_ty, return_ty)) => {
                                    let expected_index_ty =
                                        resolve_builtin_type_ref(db, &expected_index_ty).0;
                                    let return_ty = resolve_builtin_type_ref(db, &return_ty).0;
                                    if assign_tys(db, &index_ty, &expected_index_ty) {
                                        Some(return_ty)
                                    } else {
                                        None
                                    }
                                }
                                None => None,
                            },
                            _ => None,
                        }
                        .unwrap_or_else(|| {
                            self.add_expr_diagnostic_warning_ty(
                                file,
                                expr,
                                format!("Type \"{}\" is not indexable", lhs_ty.display(db).alt()),
                            )
                        });

                        return self.set_expr_type(file, expr, return_ty);
                    }
                };

                if assign_tys(db, &index_ty, target) {
                    value.clone()
                } else {
                    self.add_expr_diagnostic_warning_ty(
                        file,
                        *lhs,
                        format!(
                            "Cannot index {} with type \"{}\"",
                            name,
                            index_ty.display(db).alt()
                        ),
                    )
                }
            }
            Expr::Call { callee, args } => {
                let mut saw_keyword = false;
                let mut saw_unpacked_dict = false;
                let callee_ty = self.infer_expr(file, *callee);
                let arg_tys: Vec<_> = args
                    .iter()
                    .map(|arg| match arg {
                        Argument::Simple { expr } => {
                            if saw_keyword {
                                self.add_expr_diagnostic_error(
                                    file,
                                    *expr,
                                    String::from(
                                        "Positional argument cannot follow keyword arguments",
                                    ),
                                );
                            }
                            if saw_unpacked_dict {
                                self.add_expr_diagnostic_error(
                                    file,
                                    *expr,
                                    String::from(
                                        "Positional argument cannot follow keyword argument unpacking",
                                    ),
                                );
                            }
                            self.infer_expr(file, *expr)
                        }
                        Argument::Keyword { expr, .. } => {
                            saw_keyword = true;
                            self.infer_expr(file, *expr)
                        }
                        Argument::UnpackedList { expr } => {
                            if saw_keyword {
                                self.add_expr_diagnostic_error(
                                    file,
                                    *expr,
                                    String::from(
                                        "Unpacked iterable argument cannot follow keyword arguments",
                                    ),
                                );
                            }
                            if saw_unpacked_dict {
                                self.add_expr_diagnostic_error(
                                    file,
                                    *expr,
                                    String::from(
                                        "Unpacked iterable argument cannot follow keyword argument unpacking",
                                    ),
                                );
                            }
                            self.infer_expr(file, *expr)
                        }
                        Argument::UnpackedDict { expr } => {
                            saw_unpacked_dict = true;
                            self.infer_expr(file, *expr)
                        },
                    })
                    .collect();
                let args_with_ty = args.iter().zip(arg_tys.iter());

                // TODO(withered-magic): This is hilariously non-DRY, should probably clean this up at some point.
                match callee_ty.kind() {
                    TyKind::Function(def) => {
                        let module = module(db, def.func().file(db));
                        let params = def.func().params(db).iter().copied();
                        let mut slots: Slots = params
                            .clone()
                            .map(|param| module[param].clone())
                            .collect::<Vec<_>>()[..]
                            .into();
                        let errors = slots.assign_args(args, None).0;

                        for error in errors {
                            self.add_expr_diagnostic_error(file, error.expr, error.message);
                        }

                        let mut missing_params = Vec::new();

                        // Validate argument types.
                        for (param, slot) in params.zip(slots.slots) {
                            let hir_param = &module[param];
                            let param_ty =
                                resolve_type_ref_opt(self, hir_param.type_ref(), def.stmt());

                            // TODO(withered-magic): Deduplicate the following logic for
                            // validating providers, as it's currently shared between
                            // the handlers for `Function`s, `IntrinsicFunction`s, and
                            // `BuiltinFunction`s.
                            let mut validate_provider = |provider| match provider {
                                SlotProvider::Missing => {
                                    if !hir_param.is_optional() {
                                        let name = hir_param.name();
                                        if !name.is_missing() {
                                            missing_params.push(name.clone());
                                        }
                                    }
                                }
                                SlotProvider::Single(expr, index) => {
                                    let ty = &arg_tys[index];
                                    if !assign_tys(db, ty, &param_ty) {
                                        self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
                                    }
                                }
                                _ => {}
                            };

                            match slot {
                                Slot::Positional { provider } | Slot::Keyword { provider, .. } => {
                                    validate_provider(provider);
                                }
                                Slot::ArgsList { providers, .. }
                                | Slot::KwargsDict { providers } => {
                                    providers.into_iter().for_each(validate_provider);
                                }
                            }
                        }

                        // Emit diagnostic for missing parameters.
                        if !missing_params.is_empty() {
                            let mut message = String::from("Argument missing for parameter(s) ");
                            for (i, name) in missing_params.into_iter().enumerate() {
                                if i > 0 {
                                    message.push_str(", ");
                                }
                                message.push('"');
                                message.push_str(name.as_str());
                                message.push('"');
                            }

                            self.add_expr_diagnostic_error(file, expr, message);
                        }

                        def.func()
                            .ret_type_ref(db)
                            .map(|type_ref| resolve_type_ref(self, &type_ref, def.stmt()).0)
                            .unwrap_or_else(|| self.unknown_ty())
                    }
                    TyKind::IntrinsicFunction(func, subst) => {
                        let params = func.params(db);
                        let mut slots: Slots = params[..].into();
                        let errors = slots.assign_args(args, None).0;

                        for error in errors {
                            self.add_expr_diagnostic_error(file, error.expr, error.message);
                        }

                        // Validate argument types.
                        for (param, slot) in params.iter().zip(slots.slots) {
                            let param_ty = match param {
                                IntrinsicFunctionParam::Positional { ty, .. }
                                | IntrinsicFunctionParam::Keyword { ty, .. }
                                | IntrinsicFunctionParam::ArgsList { ty } => ty.clone(),
                                IntrinsicFunctionParam::KwargsDict => self.any_ty(),
                            }
                            .substitute(&subst.args);

                            let mut validate_provider = |provider| match provider {
                                SlotProvider::Missing => {
                                    if !param.is_optional() {
                                        self.add_expr_diagnostic_error(
                                            file,
                                            expr,
                                            format!(
                                                "Missing expected argument of type \"{}\"",
                                                param_ty.display(db).alt()
                                            ),
                                        );
                                    }
                                }
                                SlotProvider::Single(expr, index) => {
                                    let ty = &arg_tys[index];
                                    if !assign_tys(db, ty, &param_ty) {
                                        self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
                                    }
                                    if let IntrinsicFunctionParam::Keyword {
                                        name,
                                        deprecated,
                                        ..
                                    } = param
                                    {
                                        if *deprecated {
                                            let source_map = source_map(self.db, file);
                                            let arg_name_node = source_map
                                                .expr_map_back
                                                .get(&expr)
                                                .and_then(|arg_value_ptr| {
                                                    arg_value_ptr.syntax_node_ptr().try_to_node(
                                                        &parse(self.db, file).syntax(self.db),
                                                    )
                                                })
                                                .and_then(|arg_value_node| arg_value_node.parent())
                                                .and_then(ast::KeywordArgument::cast)
                                                .and_then(|keyword_arg| keyword_arg.name());
                                            if let Some(arg_name_node) = arg_name_node {
                                                self.add_diagnostic_for_range(
                                                    file,
                                                    Severity::Info,
                                                    arg_name_node.syntax().text_range(),
                                                    Some(vec![DiagnosticTag::Deprecated]),
                                                    format!(
                                                        "Argument \"{}\" is deprecated",
                                                        name.as_str()
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            };

                            match slot {
                                Slot::Positional { provider } | Slot::Keyword { provider, .. } => {
                                    validate_provider(provider)
                                }
                                Slot::ArgsList { providers, .. }
                                | Slot::KwargsDict { providers } => {
                                    providers.into_iter().for_each(validate_provider);
                                }
                            }
                        }

                        func.maybe_unique_ret_type(db, args_with_ty)
                            .unwrap_or_else(|| func.ret_ty(db).substitute(&subst.args))
                    }
                    TyKind::BuiltinFunction(func) => {
                        let params = func.params(db);
                        let mut slots: Slots = params[..].into();
                        let errors = slots.assign_args(args, None).0;

                        for error in errors {
                            self.add_expr_diagnostic_error(file, error.expr, error.message);
                        }

                        let mut missing_params = Vec::new();

                        // Validate argument types.
                        for (param, slot) in params.iter().zip(slots.slots) {
                            let param_ty = resolve_type_ref_opt(self, param.type_ref(), None);
                            let mut validate_provider = |provider| match provider {
                                SlotProvider::Missing => {
                                    if param.is_mandatory() {
                                        let name = param.name();
                                        if !name.is_missing() {
                                            missing_params.push(name.clone());
                                        }
                                    }
                                }
                                SlotProvider::Single(expr, index) => {
                                    let ty = &arg_tys[index];
                                    if !assign_tys(db, ty, &param_ty) {
                                        self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
                                    }
                                }
                                _ => {}
                            };

                            match slot {
                                Slot::Positional { provider } | Slot::Keyword { provider, .. } => {
                                    validate_provider(provider)
                                }
                                Slot::ArgsList { providers, .. }
                                | Slot::KwargsDict { providers } => {
                                    providers.into_iter().for_each(validate_provider);
                                }
                            }
                        }

                        // Emit diagnostic for missing parameters.
                        if !missing_params.is_empty() {
                            let mut message = String::from("Argument missing for parameter(s) ");
                            for (i, name) in missing_params.into_iter().enumerate() {
                                if i > 0 {
                                    message.push_str(", ");
                                }
                                message.push('"');
                                message.push_str(name.as_str());
                                message.push('"');
                            }

                            self.add_expr_diagnostic_error(file, expr, message);
                        }

                        func.maybe_unique_ret_type(self, file, expr, args_with_ty)
                            .unwrap_or_else(|| {
                                resolve_type_ref(self, func.ret_type_ref(db), None).0
                            })
                    }
                    TyKind::Rule(rule) => {
                        let mut slots = Slots::from_rule(db, rule);
                        let mut missing_attrs = Vec::new();
                        slots.assign_args(args, None);

                        // Validate argument types.
                        for ((name, attr), slot) in rule.attrs(db).zip(slots.slots) {
                            let expected_ty = attr.expected_ty();
                            if let Slot::Keyword { provider, .. } = slot {
                                match provider {
                                    SlotProvider::Single(expr, index) => {
                                        let ty = &arg_tys[index];
                                        if !assign_tys(db, ty, &expected_ty) {
                                            self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), expected_ty.display(self.db).alt()));
                                        }
                                    }
                                    SlotProvider::Missing => {
                                        if attr.mandatory {
                                            missing_attrs.push(name);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // Emit diagnostic for missing attributes.
                        if !missing_attrs.is_empty() {
                            let mut message = String::from("Argument missing for attribute(s) ");
                            for (i, name) in missing_attrs.iter().enumerate() {
                                if i > 0 {
                                    message.push_str(", ");
                                }
                                message.push('"');
                                message.push_str(name.as_str());
                                message.push('"');
                            }

                            self.add_expr_diagnostic_error(file, expr, message);
                        }

                        self.none_ty()
                    }
                    TyKind::Provider(provider) | TyKind::ProviderRawConstructor(_, provider) => {
                        TyKind::ProviderInstance(provider.clone()).intern()
                    }
                    TyKind::Tag(tag_class) => {
                        // TODO(withered-magic): Much of this logic is duplicated from handling `TyKind::Rule` above.
                        let mut slots = Slots::from_tag_class(tag_class);
                        slots.assign_args(args, None);

                        let mut missing_attrs = Vec::new();

                        // Validate argument types.
                        for (data, slot) in tag_class
                            .attrs
                            .iter()
                            .flat_map(|attrs| attrs.iter())
                            .zip(slots.slots)
                        {
                            let expected_ty = data.attr.expected_ty();
                            if let Slot::Keyword { provider, .. } = slot {
                                match provider {
                                    SlotProvider::Single(expr, index) => {
                                        let ty = &arg_tys[index];
                                        if !assign_tys(db, ty, &expected_ty) {
                                            self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), expected_ty.display(self.db).alt()));
                                        }
                                    }
                                    SlotProvider::Missing => {
                                        if data.attr.mandatory {
                                            missing_attrs.push(&data.name);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // Emit diagnostic for missing parameters.
                        if !missing_attrs.is_empty() {
                            let mut message = String::from("Argument missing for attribute(s) ");
                            for (i, name) in missing_attrs.iter().enumerate() {
                                if i > 0 {
                                    message.push_str(", ");
                                }
                                message.push('"');
                                message.push_str(name.as_str());
                                message.push('"');
                            }

                            self.add_expr_diagnostic_error(file, expr, message);
                        }

                        self.none_ty()
                    }
                    TyKind::Macro(makro) => {
                        let mut slots = Slots::from_macro(makro);
                        let mut missing_attrs = Vec::new();
                        slots.assign_args(args, None);

                        // Check for any disallowed attributes.
                        for arg in args.iter() {
                            eprintln!("{:?}", arg);
                            match arg {
                                Argument::Keyword { name, expr }
                                    if makro.disallowed_attrs().any(|n| n == name) =>
                                {
                                    self.add_expr_diagnostic_error(
                                        file,
                                        *expr,
                                        format!("Cannot set attribute \"{}\"", name.as_str()),
                                    );
                                }
                                _ => {}
                            }
                        }

                        // Validate attribute types.
                        for ((name, attr), slot) in makro.attrs().zip(slots.slots) {
                            let expected_ty = attr.expected_ty();
                            if let Slot::Keyword { provider, .. } = slot {
                                match provider {
                                    SlotProvider::Single(expr, index) => {
                                        let ty = &arg_tys[index];
                                        if !assign_tys(db, ty, &expected_ty) {
                                            self.add_expr_diagnostic_error(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), expected_ty.display(self.db).alt()));
                                        }
                                    }
                                    SlotProvider::Missing => {
                                        if attr.mandatory {
                                            missing_attrs.push(name);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // Emit diagnostic for missing attributes.
                        if !missing_attrs.is_empty() {
                            let mut message = String::from("Argument missing for attribute(s) ");
                            for (i, name) in missing_attrs.iter().enumerate() {
                                if i > 0 {
                                    message.push_str(", ");
                                }
                                message.push('"');
                                message.push_str(name.as_str());
                                message.push('"');
                            }

                            self.add_expr_diagnostic_error(file, expr, message);
                        }

                        self.none_ty()
                    }
                    TyKind::Unknown | TyKind::Any | TyKind::Unbound => self.unknown_ty(),
                    _ => self.add_expr_diagnostic_warning_ty(
                        file,
                        expr,
                        format!("Type \"{}\" is not callable", callee_ty.display(db).alt()),
                    ),
                }
            }
            Expr::Tuple { exprs } => TyKind::Tuple(Tuple::Simple(
                exprs
                    .iter()
                    .map(|expr| self.infer_expr(file, *expr))
                    .collect(),
            ))
            .intern(),
            Expr::If {
                if_expr,
                test,
                else_expr,
            } => {
                self.infer_expr(file, *test);
                Ty::union(
                    [
                        self.infer_expr(file, *if_expr),
                        self.infer_expr(file, *else_expr),
                    ]
                    .into_iter(),
                )
            }
            Expr::Slice {
                lhs,
                start,
                end,
                step,
            } => {
                let mut check_slice_component = |expr| {
                    let ty = self.infer_expr(file, expr);
                    if !assign_tys(db, &ty, &self.int_ty()) && !assign_tys(db, &ty, &self.none_ty())
                    {
                        self.add_expr_diagnostic_error(
                            file,
                            expr,
                            "`start`, `stop`, and `step` operands must be integers or `None`",
                        )
                    }
                };

                start.map(&mut check_slice_component);
                end.map(&mut check_slice_component);
                step.map(&mut check_slice_component);

                let lhs_ty = self.infer_expr(file, *lhs);
                match lhs_ty.kind() {
                    TyKind::String(_) => self.string_ty(),
                    TyKind::Bytes => self.bytes_ty(),
                    TyKind::Tuple(Tuple::Simple(tys)) => Ty::union(tys.iter().cloned()),
                    TyKind::Tuple(Tuple::Variable(ty)) => Ty::list(ty.clone()),
                    TyKind::Range => Ty::list(self.int_ty()),
                    TyKind::List(ty) | TyKind::Protocol(Protocol::Sequence(ty)) => {
                        Ty::list(ty.clone())
                    }
                    TyKind::Unknown | TyKind::Any => self.unknown_ty(),
                    _ => self.add_expr_diagnostic_warning_ty(
                        file,
                        expr,
                        format!(
                            "Cannot slice expression of type \"{}\"",
                            lhs_ty.display(db).alt()
                        ),
                    ),
                }
            }
            Expr::Paren { expr } => self.infer_expr(file, *expr),
            Expr::Lambda { func, .. } => {
                TyKind::Function(FunctionDef::Lambda { func: *func }).intern()
            }
            _ => self.unknown_ty(),
        };
        self.set_expr_type(file, expr, ty)
    }

    fn infer_unary_expr(&mut self, file: File, parent: ExprId, expr: ExprId, op: UnaryOp) -> Ty {
        let ty = self.infer_expr(file, expr);
        match self.check_unary_expr(&ty, op) {
            Ok(ty) => ty,
            Err(()) => self.add_expr_diagnostic_error_ty(
                file,
                parent,
                format!(
                    "Operator \"{}\" is not supported for type \"{}\"",
                    op,
                    ty.display(self.db).alt()
                ),
            ),
        }
    }

    fn check_unary_expr(&self, ty: &Ty, op: UnaryOp) -> Result<Ty, ()> {
        Ok(match (op, ty.kind()) {
            (UnaryOp::Arith(_) | UnaryOp::Inv, TyKind::Int(_)) => self.int_ty(),
            (UnaryOp::Arith(_), TyKind::Float) => self.float_ty(),
            (UnaryOp::Not, _) => self.bool_ty(),
            (_, TyKind::Unknown | TyKind::Any) => self.unknown_ty(),
            (op, TyKind::Union(tys)) => {
                let mut mapped_tys = Vec::with_capacity(tys.len());
                for ty in tys.iter() {
                    mapped_tys.push(self.check_unary_expr(ty, op)?);
                }
                Ty::union(mapped_tys.into_iter())
            }
            _ => return Err(()),
        })
    }

    fn infer_binary_expr(
        &mut self,
        file: File,
        parent: ExprId,
        lhs: ExprId,
        rhs: ExprId,
        op: BinaryOp,
    ) -> Ty {
        let db = self.db;
        let lhs = self.infer_expr(file, lhs);
        let rhs = self.infer_expr(file, rhs);
        let lhs_kind = lhs.kind();
        let rhs_kind = rhs.kind();
        let mut unknown = || {
            self.add_expr_diagnostic_warning_ty(
                file,
                parent,
                format!(
                    "Operator \"{}\" not supported for types \"{}\" and \"{}\"",
                    op,
                    lhs_kind.display(db).alt(),
                    rhs_kind.display(db).alt()
                ),
            )
        };

        match (lhs_kind, rhs_kind) {
            (TyKind::Any | TyKind::Unknown, _) | (_, TyKind::Any | TyKind::Unknown) => {
                return self.unknown_ty()
            }
            _ => {}
        }

        match op {
            BinaryOp::Arith(op) => match (lhs_kind, rhs_kind, op) {
                (TyKind::String(Some(s1)), TyKind::String(Some(s2)), ArithOp::Add) => {
                    let s1 = &s1.value(db);
                    let s2 = &s2.value(db);
                    let mut s = String::with_capacity(s1.len() + s2.len());
                    s.push_str(s1);
                    s.push_str(s2);
                    let interned = InternedString::new(db, s.into_boxed_str());
                    TyKind::String(Some(interned)).intern()
                }
                (TyKind::String(_), TyKind::String(_), ArithOp::Add)
                | (TyKind::String(_), _, ArithOp::Mod) => self.string_ty(), // concatenation, string interpolcation
                (TyKind::Bytes, TyKind::Bytes, ArithOp::Add) => self.bytes_ty(), // concatenation
                (
                    TyKind::List(ty1)
                    | TyKind::Protocol(Protocol::Sequence(ty1) | Protocol::Iterable(ty1)),
                    TyKind::List(ty2)
                    | TyKind::Protocol(Protocol::Sequence(ty2) | Protocol::Iterable(ty2)),
                    ArithOp::Add,
                ) => Ty::list(Ty::union([ty1.clone(), ty2.clone()].into_iter())),
                (TyKind::String(_), TyKind::Int(_), ArithOp::Mul)
                | (TyKind::Int(_), TyKind::String(_), ArithOp::Mul) => self.string_ty(),
                (TyKind::Tuple(Tuple::Simple(tys)), TyKind::Int(_), ArithOp::Mul)
                | (TyKind::Int(_), TyKind::Tuple(Tuple::Simple(tys)), ArithOp::Mul) => {
                    TyKind::Tuple(Tuple::Variable(Ty::union(tys.iter().cloned()))).intern()
                }
                (
                    TyKind::Bytes | TyKind::List(_) | TyKind::Tuple(Tuple::Variable(_)),
                    TyKind::Int(_),
                    ArithOp::Mul,
                ) => lhs,
                (
                    TyKind::Int(_),
                    TyKind::Bytes | TyKind::List(_) | TyKind::Tuple(Tuple::Variable(_)),
                    ArithOp::Mul,
                ) => rhs,
                (TyKind::Int(Some(x1)), TyKind::Int(Some(x2)), ArithOp::Add) => {
                    TyKind::Int(Some(x1 + x2)).intern()
                }
                (TyKind::Int(_), TyKind::Int(_), _) => self.int_ty(),
                (TyKind::Float, TyKind::Int(_), _)
                | (TyKind::Int(_), TyKind::Float, _)
                | (TyKind::Float, TyKind::Float, _) => self.float_ty(),
                _ => unknown(),
            },
            BinaryOp::Bitwise(op) => match (lhs_kind, rhs_kind, op) {
                (TyKind::Int(_), TyKind::Int(_), _) => self.int_ty(),
                (
                    TyKind::Dict(lhs_key_ty, lhs_value_ty, _),
                    TyKind::Dict(rhs_key_ty, rhs_value_ty, _),
                    BitwiseOp::Or,
                ) => Ty::dict(
                    Ty::union([lhs_key_ty.clone(), rhs_key_ty.clone()].into_iter()),
                    Ty::union([lhs_value_ty.clone(), rhs_value_ty.clone()].into_iter()),
                    None,
                ),
                _ => unknown(),
            },
            BinaryOp::MemberOp(_) => {
                if !matches!(
                    rhs_kind,
                    TyKind::List(_)
                        | TyKind::Tuple(_)
                        | TyKind::Dict(_, _, _)
                        | TyKind::String(_)
                        | TyKind::Bytes
                        | TyKind::Protocol(Protocol::Sequence(_))
                        | TyKind::Target
                ) {
                    self.add_expr_diagnostic_warning(
                        file,
                        parent,
                        None,
                        format!(
                            "Operator \"{}\" not supported for types \"{}\" and \"{}\"",
                            op,
                            lhs_kind.display(db).alt(),
                            rhs_kind.display(db).alt()
                        ),
                    );
                }
                self.bool_ty()
            }
            BinaryOp::Logic(LogicOp::Or) => match (lhs_kind, rhs_kind) {
                // TODO(withered-magic): Strip None from optional types once we implement narrowing.
                (TyKind::Bool(Some(lhs)), TyKind::Bool(Some(rhs))) => {
                    TyKind::Bool(Some(*lhs || *rhs)).intern()
                }
                (TyKind::Bool(Some(false)) | TyKind::Int(Some(0)) | TyKind::None, _) => rhs,
                (TyKind::Tuple(Tuple::Simple(tys)), _) if tys.is_empty() => rhs,
                _ => Ty::union([lhs, rhs].into_iter()),
            },
            BinaryOp::Logic(LogicOp::And) => match (lhs_kind, rhs_kind) {
                (TyKind::Bool(Some(lhs)), TyKind::Bool(Some(rhs))) => {
                    TyKind::Bool(Some(*lhs && *rhs)).intern()
                }
                (TyKind::Bool(Some(false)) | TyKind::Int(Some(0)) | TyKind::None, _) => lhs,
                (TyKind::Tuple(Tuple::Simple(tys)), _) if tys.is_empty() => lhs,
                _ => Ty::union([lhs, rhs].into_iter()),
            },
            _ => self.bool_ty(),
        }
    }

    fn infer_assign(
        &mut self,
        file: File,
        expr: ExprId,
        source: Option<ExprId>,
        expected_ty: Option<Ty>,
        execution_scope: ExecutionScopeId,
    ) -> Ty {
        let cached_ty = self
            .cx
            .type_of_expr
            .get(&FileExprId::new(file, expr))
            .cloned();
        cached_ty.unwrap_or_else(|| {
            source
                .and_then(|source| {
                    self.infer_source_expr_assign(file, source, expected_ty, execution_scope);
                    self.cx
                        .type_of_expr
                        .get(&FileExprId::new(file, expr))
                        .cloned()
                })
                .unwrap_or_else(|| self.unknown_ty())
        })
    }

    fn infer_source_expr_assign(
        &mut self,
        file: File,
        source: ExprId,
        expected_ty: Option<Ty>,
        execution_scope: ExecutionScopeId,
    ) {
        let key = FileExprId::new(file, source);
        if self.cx.source_assign_done.contains(&key) {
            return;
        }
        self.infer_source_expr_assign_inner(file, source, expected_ty, execution_scope);
        self.cx.source_assign_done.insert(key);
    }

    fn infer_source_expr_assign_inner(
        &mut self,
        file: File,
        source: ExprId,
        expected_ty: Option<Ty>,
        execution_scope: ExecutionScopeId,
    ) {
        // Find the parent assignment node. This can be either an assignment statement (`x = 0`), a `for` statement (`for x in 1, 2, 3`), or
        // a for comp clause in a list/dict comprehension (`[x + 1 for x in [1, 2, 3]]`).
        let db = self.db;
        let source_map = source_map(db, file);
        let source_ptr = match source_map.expr_map_back.get(&source) {
            Some(ptr) => ptr,
            _ => return,
        };
        let parent = source_ptr
            .to_node(&parse(db, file).syntax(db))
            .syntax()
            .parent()
            .unwrap();

        // Convert "Unbound" to "Unknown" in assignments to avoid confusion.
        let mut source_ty = self.infer_expr(file, source);
        if matches!(source_ty.kind(), TyKind::Unbound) {
            source_ty = self.unknown_ty();
        }

        // Handle standard assigments, e.g. `x, y = 1, 2`.
        if let Some(node) = ast::AssignStmt::cast(parent.clone()) {
            let ptr = AstPtr::new(&ast::Statement::Assign(node.clone()));
            let stmt = *source_map.stmt_map.get(&ptr).unwrap();
            let expected_ty = expected_ty.or_else(|| {
                match &module(db, file)[stmt] {
                    Stmt::Assign { type_ref, .. } => type_ref.as_ref().and_then(|type_ref| {
                        let (expected_ty, errors) =
                            resolve_type_ref(self, &type_ref.0, Some(InFile { file, value: stmt }));
                        if errors.is_empty() {
                            Some(expected_ty)
                        } else {
                            // Add TypeRef resolution errors.
                            for error in errors.iter() {
                                self.add_diagnostic_for_range(
                                    file,
                                    Severity::Error,
                                    type_ref.1,
                                    None,
                                    error,
                                );
                            }
                            None
                        }
                    }),
                    _ => None,
                }
            });

            if let Some(lhs) = node.lhs() {
                let lhs_ptr = AstPtr::new(&lhs);
                let expr = source_map.expr_map.get(&lhs_ptr).unwrap();
                self.assign_expr_source_ty(
                    file,
                    source,
                    *expr,
                    source_ty,
                    expected_ty,
                    execution_scope,
                );
                return;
            }
        }

        // Handle assignments in "for" statements and comphrehensions.
        // e.g. `for x in 1, 2, 3` or `[x*y for x in range(5) for y in range(5)]`
        let targets = ast::ForStmt::cast(parent.clone())
            .and_then(|stmt| stmt.targets())
            .or_else(|| {
                ast::CompClauseFor::cast(parent).and_then(|comp_clause| comp_clause.targets())
            });

        let targets = match targets {
            Some(targets) => targets
                .exprs()
                .map(|expr| source_map.expr_map.get(&AstPtr::new(&expr)).unwrap())
                .copied()
                .collect::<Vec<_>>(),
            None => return,
        };

        let sub_ty = match source_ty.kind() {
            TyKind::List(ty)
            | TyKind::Tuple(Tuple::Variable(ty))
            | TyKind::Protocol(Protocol::Iterable(ty) | Protocol::Sequence(ty)) => ty.clone(),
            TyKind::Tuple(Tuple::Simple(tys)) => Ty::union(tys.iter().cloned()),
            TyKind::Dict(key_ty, _, _) => key_ty.clone(),
            TyKind::Any => self.any_ty(),
            TyKind::Range => self.int_ty(),
            TyKind::StringElems => self.string_ty(),
            TyKind::BytesElems => self.int_ty(),
            TyKind::Unknown => self.unknown_ty(),
            _ => {
                self.add_expr_diagnostic_warning(
                    file,
                    source,
                    None,
                    format!("Type \"{}\" is not iterable", source_ty.display(db).alt()),
                );
                for expr in targets.iter() {
                    self.assign_expr_unknown_rec(file, *expr, execution_scope);
                }
                return;
            }
        };
        if targets.len() == 1 {
            self.assign_expr_source_ty(file, targets[0], targets[0], sub_ty, None, execution_scope);
        } else {
            self.assign_exprs_source_ty(file, source, &targets, sub_ty, execution_scope);
        }
    }

    pub(crate) fn infer_name(
        &mut self,
        file: File,
        name: &Name,
        usage: impl Into<ScopeHirId>,
    ) -> Option<Ty> {
        let hir_id = usage.into();
        let resolver = Resolver::new_for_hir_execution_scope(self.db, file, hir_id);
        let expr_scope = resolver.scope_for_hir_id(hir_id)?;
        let curr_execution_scope = resolver.execution_scope_for_hir_id(hir_id)?;
        let (def_execution_scope, effective_ty) = match resolver.resolve_name(name) {
            Some((def_execution_scope, defs)) => {
                let mut var_defs = Vec::new();

                // The type of a named expression may already be unambigiously known, e.g. in the case of
                // annotated variable declarations or function definitions. In this case, we always use
                // this known type.
                let mut known_ty = None;
                for def in defs.skip_while(|def| def.scope > expr_scope) {
                    let ty = match def.def {
                        ScopeDef::Variable(VariableDef { file, expr, source }) => {
                            if def_execution_scope != ExecutionScopeId::Module
                                || name.as_str().starts_with('_')
                            {
                                if let ScopeHirId::Expr(usage_expr) = hir_id {
                                    let key = InFile {
                                        file: *file,
                                        value: Either::Left(*expr),
                                    };
                                    if usage_expr != *expr {
                                        self.cx.definition_is_used.insert(key, true);
                                    }
                                }
                            }

                            if self.shared_state.options.use_code_flow_analysis {
                                var_defs.push((file, *expr, *source));
                                continue;
                            } else {
                                self.infer_assign(*file, *expr, *source, None, def_execution_scope)
                            }
                        }
                        ScopeDef::Function(def) => {
                            if let Some(stmt) = def.stmt() {
                                self.cx.definition_is_used.insert(
                                    InFile {
                                        file: stmt.file,
                                        value: Either::Right(stmt.value),
                                    },
                                    true,
                                );
                            }
                            TyKind::Function(def.clone()).intern()
                        }
                        ScopeDef::Parameter(ParameterDef { func, index }) => {
                            self.infer_param(file, func.params(self.db)[*index])
                        }
                        ScopeDef::LoadItem(LoadItemDef { load_item, .. }) => {
                            self.infer_load_item(file, *load_item)
                        }
                        // This should be unreachable.
                        _ => continue,
                    };

                    known_ty.get_or_insert(ty);
                }

                // Determine the effective type of the named expression. The effective type is the union of the types
                // of all the values assigned to the name in question. For example, given:
                //
                // x = 1
                // x = "one"
                //
                // then the effective type of `x` would be `int | string`. This type will be further narrowed by
                // the code-flow analysis done below. NOTE: Even if the type has already been determined from the above
                // logic (i.e. `known_ty` is `Some`), we still compute the effective type if only to infer types
                // for the relevant assignment statements.
                let effective_ty = Ty::union(var_defs.into_iter().map(|(file, expr, source)| {
                    self.infer_assign(*file, expr, source, known_ty.clone(), def_execution_scope)
                }));

                if known_ty.is_some() {
                    return known_ty;
                }

                (def_execution_scope, effective_ty)
            }

            None => {
                return Some(match resolver.resolve_name_in_prelude_or_builtins(name)? {
                    ScopeDef::IntrinsicFunction(func) => {
                        TyKind::IntrinsicFunction(func, Substitution::new_identity(0)).intern()
                    }
                    ScopeDef::BuiltinFunction(func) => TyKind::BuiltinFunction(func).intern(),
                    ScopeDef::BuiltinVariable(type_ref) => {
                        resolve_builtin_type_ref(self.db, &type_ref).0
                    }
                    // Handle symbols declared/loaded in the prelude file.
                    ScopeDef::Variable(def) => self.infer_expr(def.file, def.expr),
                    ScopeDef::Function(def) => TyKind::Function(def.clone()).intern(),
                    ScopeDef::LoadItem(def) => self.infer_load_item(def.file, def.load_item),
                    _ => return None,
                });
            }
        };

        // TODO(withered-magic): We should eventually apply narrowing to the effective type here.
        let start_ty = if def_execution_scope != curr_execution_scope {
            effective_ty.clone()
        } else {
            self.unbound_ty()
        };

        // See if we can narrow the effective type further through code-flow analysis. If not, then
        // fall back to the effective type.
        Some(
            self.infer_name_from_code_flow(file, name, hir_id, curr_execution_scope, &start_ty)
                .unwrap_or(effective_ty),
        )
    }

    fn infer_name_from_code_flow(
        &mut self,
        file: File,
        name: &Name,
        usage: impl Into<ScopeHirId>,
        execution_scope: ExecutionScopeId,
        start_ty: &Ty,
    ) -> Option<Ty> {
        // If an expression is missing its corresponding node in the code flow graph, that
        // means the expression is unreachable. We use the `Never` type to represent this case.
        let cfg = code_flow_graph(self.db, file).cfg(self.db);
        let start_node = match cfg.hir_to_flow_node.get(&usage.into()) {
            Some(start_node) => start_node,
            None => return Some(TyKind::Never.intern()),
        };
        self.infer_ref_from_flow_node(cfg, file, execution_scope, name, start_ty, *start_node)
    }

    /// Returning `None` here means that code-flow analysis failed and that a fallback type should
    /// be returned instead.
    fn infer_ref_from_flow_node(
        &mut self,
        cfg: &CodeFlowGraph,
        file: File,
        execution_scope: ExecutionScopeId,
        name: &Name,
        start_ty: &Ty,
        start_node: FlowNodeId,
    ) -> Option<Ty> {
        if let Some(res) =
            self.read_cached_ref_type_at_flow_node(file, execution_scope, name, start_node)
        {
            return res;
        }

        let mut curr_node_id = start_node;
        let res = 'outer: loop {
            let curr_node = &cfg.flow_nodes[curr_node_id];
            let curr_node_ty = match &curr_node {
                FlowNode::Start => start_ty.clone(),
                FlowNode::Assign {
                    expr,
                    name: node_name,
                    source,
                    antecedent,
                    execution_scope: assign_execution_scope,
                } => {
                    // We need to do the extra check for the execution scope here to handle execution scopes from things
                    // like list/dict comprehensions.
                    if name != node_name || execution_scope != *assign_execution_scope {
                        curr_node_id = *antecedent;
                        continue;
                    }

                    self.infer_source_expr_assign(file, *source, None, execution_scope);
                    self.cx
                        .type_of_expr
                        .get(&FileExprId::new(file, *expr))
                        .cloned()
                        .unwrap_or_else(Ty::never)
                }
                FlowNode::Branch { antecedents } => {
                    let mut antecedent_tys = Vec::with_capacity(antecedents.len());
                    for antecedent in antecedents {
                        match self.infer_ref_from_flow_node(
                            cfg,
                            file,
                            execution_scope,
                            name,
                            start_ty,
                            *antecedent,
                        ) {
                            Some(antecedent_ty) => {
                                antecedent_tys.push(antecedent_ty);
                            }
                            None => break 'outer None,
                        }
                    }
                    Ty::union(antecedent_tys.into_iter())
                }
                FlowNode::Loop { .. } => break 'outer None, // TODO(withered-magic): Correctly handle loops.
                FlowNode::Call { expr, antecedent } => {
                    let ty = self.infer_expr(file, *expr);
                    if matches!(ty.kind(), TyKind::Never) {
                        ty
                    } else {
                        curr_node_id = *antecedent;
                        continue;
                    }
                }
                FlowNode::Unreachable { .. } => Ty::never(),
            };

            break Some(curr_node_ty);
        };

        self.cache_ref_type_at_flow_node(file, execution_scope, name, start_node, res)
    }

    fn exists_flow_path(
        &mut self,
        cfg: &CodeFlowGraph,
        file: File,
        from_node: FlowNodeId,
        to_node: FlowNodeId,
    ) -> bool {
        if from_node == to_node {
            true
        } else {
            match &cfg.flow_nodes[from_node] {
                FlowNode::Assign { antecedent, .. } => {
                    self.exists_flow_path(cfg, file, *antecedent, to_node)
                }
                FlowNode::Branch { antecedents } => antecedents
                    .iter()
                    .any(|antecedent| self.exists_flow_path(cfg, file, *antecedent, to_node)),
                FlowNode::Loop { .. } => true,
                FlowNode::Call { expr, antecedent } => {
                    if self.infer_expr(file, *expr) == Ty::never() {
                        false
                    } else {
                        self.exists_flow_path(cfg, file, *antecedent, to_node)
                    }
                }
                _ => false,
            }
        }
    }

    fn read_cached_ref_type_at_flow_node(
        &self,
        file: File,
        execution_scope: ExecutionScopeId,
        name: &Name,
        flow_node: FlowNodeId,
    ) -> Option<Option<Ty>> {
        self.cx
            .flow_node_type_cache
            .get(&CodeFlowCacheKey {
                file,
                execution_scope,
                name: name.clone(),
                flow_node,
            })
            .cloned()
    }

    fn cache_ref_type_at_flow_node(
        &mut self,
        file: File,
        execution_scope: ExecutionScopeId,
        name: &Name,
        flow_node: FlowNodeId,
        res: Option<Ty>,
    ) -> Option<Ty> {
        self.cx.flow_node_type_cache.insert(
            CodeFlowCacheKey {
                file,
                execution_scope,
                name: name.clone(),
                flow_node,
            },
            res.clone(),
        );
        res
    }

    fn assign_expr_source_ty(
        &mut self,
        file: File,
        root: ExprId,
        expr: ExprId,
        source_ty: Ty,
        expected_ty: Option<Ty>,
        execution_scope: ExecutionScopeId,
    ) {
        match module(self.db, file).exprs.get(expr).unwrap() {
            Expr::Name { name } => {
                if execution_scope != ExecutionScopeId::Module || name.as_str().starts_with('_') {
                    self.cx
                        .definition_is_used
                        .entry(InFile {
                            file,
                            value: Either::Left(expr),
                        })
                        .or_insert(false);
                }
                // If we have an expected type from a type comment, use that.
                // We also emit any error if the source and expected types aren't compatible.
                if let Some(expected_ty) = expected_ty {
                    if !assign_tys(self.db, &source_ty, &expected_ty) {
                        self.add_expr_diagnostic_error(
                            file,
                            root,
                            format!(
                                "Expression of type \"{}\" cannot be assigned to variable of type \"{}\"",
                                source_ty.display(self.db).alt(),
                                expected_ty.display(self.db).alt()
                            ),
                        )
                    }
                    self.set_expr_type(file, expr, expected_ty);
                } else {
                    self.set_expr_type(file, expr, source_ty);
                }
            }
            Expr::List { exprs } | Expr::Tuple { exprs } => {
                self.assign_exprs_source_ty(file, root, exprs, source_ty, execution_scope);
            }
            Expr::Paren { expr } => {
                self.assign_expr_source_ty(file, root, *expr, source_ty, None, execution_scope)
            }
            _ => {}
        }
    }

    fn assign_exprs_source_ty(
        &mut self,
        file: File,
        root: ExprId,
        exprs: &[ExprId],
        source_ty: Ty,
        execution_scope: ExecutionScopeId,
    ) {
        match source_ty.kind() {
            TyKind::List(ty) | TyKind::Tuple(Tuple::Variable(ty)) => {
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, root, expr, ty.clone(), None, execution_scope);
                }
            }
            TyKind::Tuple(Tuple::Simple(tys)) => {
                let pairs = exprs.iter().copied().zip(tys.iter());
                for (expr, ty) in pairs {
                    self.assign_expr_source_ty(file, root, expr, ty.clone(), None, execution_scope);
                }
                if exprs.len() != tys.len() {
                    if exprs.len() > tys.len() {
                        for expr in &exprs[tys.len()..] {
                            self.assign_expr_unknown_rec(file, *expr, execution_scope);
                        }
                    }
                    self.add_expr_diagnostic_error(
                        file,
                        root,
                        format!(
                            "Tuple size mismatch, {} on left-hand side and {} on right-hand side",
                            exprs.len(),
                            tys.len(),
                        ),
                    );
                }
            }
            TyKind::Any | TyKind::Unknown => {
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(
                        file,
                        root,
                        expr,
                        self.unknown_ty(),
                        None,
                        execution_scope,
                    );
                }
            }
            _ => {
                self.add_expr_diagnostic_warning(
                    file,
                    root,
                    None,
                    format!(
                        "Type \"{}\" is not iterable",
                        source_ty.display(self.db).alt()
                    ),
                );
                for expr in exprs.iter() {
                    self.assign_expr_unknown_rec(file, *expr, execution_scope);
                }
            }
        }
    }

    fn assign_expr_unknown_rec(
        &mut self,
        file: File,
        expr: ExprId,
        execution_scope: ExecutionScopeId,
    ) {
        let module = module(self.db, file);
        let node = &module[expr];
        self.set_expr_type(file, expr, self.unknown_ty());

        match node {
            Expr::Name { name }
                if execution_scope != ExecutionScopeId::Module
                    || name.as_str().starts_with('_') =>
            {
                self.cx
                    .definition_is_used
                    .entry(InFile {
                        file,
                        value: Either::Left(expr),
                    })
                    .or_insert(false);
            }
            _ => node.walk_child_exprs(|expr| {
                self.assign_expr_unknown_rec(file, expr, execution_scope);
            }),
        }
    }

    fn set_expr_type(&mut self, file: File, expr: ExprId, ty: Ty) -> Ty {
        self.cx
            .type_of_expr
            .insert(FileExprId::new(file, expr), ty.clone());
        ty
    }

    fn get_common_type(
        &mut self,
        file: File,
        mut exprs: impl Iterator<Item = ExprId>,
        default: Ty,
    ) -> Ty {
        let first = exprs.next();
        first
            .map(|first| self.infer_expr(file, first))
            .and_then(|first_ty| {
                let first_ty = first_ty.normalize();
                exprs
                    .map(|expr| self.infer_expr(file, expr))
                    .all(|ty| Ty::eq(&ty.normalize(), &first_ty))
                    .then_some(first_ty)
            })
            .unwrap_or(default)
    }

    fn add_expr_diagnostic_warning<T: Into<String>>(
        &mut self,
        file: File,
        expr: ExprId,
        tags: Option<Vec<DiagnosticTag>>,
        message: T,
    ) {
        self.add_expr_diagnostic_with_severity(file, expr, Severity::Warning, tags, message)
    }

    fn add_expr_diagnostic_error<T: Into<String>>(&mut self, file: File, expr: ExprId, message: T) {
        self.add_expr_diagnostic_with_severity(file, expr, Severity::Error, None, message)
    }

    fn add_expr_diagnostic_with_severity<T: Into<String>>(
        &mut self,
        file: File,
        expr: ExprId,
        severity: Severity,
        tags: Option<Vec<DiagnosticTag>>,
        message: T,
    ) {
        let range = match source_map(self.db, file).expr_map_back.get(&expr) {
            Some(ptr) => ptr.syntax_node_ptr().text_range(),
            None => return,
        };
        self.add_diagnostic_for_range(file, severity, range, tags, message);
    }

    fn add_expr_diagnostic_error_ty<T: Into<String>>(
        &mut self,
        file: File,
        expr: ExprId,
        message: T,
    ) -> Ty {
        self.add_expr_diagnostic_error(file, expr, message);
        self.unknown_ty()
    }

    fn add_expr_diagnostic_warning_ty<T: Into<String>>(
        &mut self,
        file: File,
        expr: ExprId,
        message: T,
    ) -> Ty {
        self.add_expr_diagnostic_warning(file, expr, None, message);
        self.unknown_ty()
    }

    fn add_diagnostic_for_range<T: Into<String>>(
        &mut self,
        file: File,
        severity: Severity,
        range: TextRange,
        tags: Option<Vec<DiagnosticTag>>,
        message: T,
    ) {
        self.cx.diagnostics.push(Diagnostic {
            message: message.into(),
            severity,
            range: FileRange {
                file_id: file.id(self.db),
                range,
            },
            tags,
        });
    }

    pub(crate) fn infer_param(&mut self, file: File, param: ParamId) -> Ty {
        if let Some(ty) = self.cx.type_of_param.get(&FileParamId::new(file, param)) {
            return ty.clone();
        }

        let ty = self
            .shared_state
            .options
            .infer_ctx_attributes
            .then(|| self.infer_param_from_rule_usage(file, param))
            .and_then(|ty| ty)
            .unwrap_or_else(|| {
                let module = module(self.db, file);
                let usage = module.param_to_def_stmt.get(&param).map(|(stmt, _)| *stmt);
                match &module[param] {
                    Param::Simple { type_ref, .. } => type_ref
                        .as_ref()
                        .map(|type_ref| self.lower_param_type_ref(file, param, type_ref, usage))
                        .unwrap_or_else(|| self.unknown_ty()),
                    Param::ArgsList { type_ref, .. } => TyKind::Tuple(Tuple::Variable(
                        type_ref
                            .as_ref()
                            .map(|type_ref| self.lower_param_type_ref(file, param, type_ref, usage))
                            .unwrap_or_else(|| self.unknown_ty()),
                    ))
                    .intern(),
                    Param::KwargsDict { type_ref, .. } => TyKind::Dict(
                        self.string_ty(),
                        type_ref
                            .as_ref()
                            .map(|type_ref| self.lower_param_type_ref(file, param, type_ref, usage))
                            .unwrap_or_else(|| self.unknown_ty()),
                        None,
                    )
                    .intern(),
                }
            });

        self.cx
            .type_of_param
            .insert(FileParamId::new(file, param), ty.clone());
        ty
    }

    fn infer_param_from_rule_usage(&mut self, file: File, param: ParamId) -> Option<Ty> {
        let module = module(self.db, file);
        let name = match module[module.param_to_def_stmt.get(&param)?.0] {
            Stmt::Def { func, .. } if func.params(self.db).len() == 1 => func.name(self.db),
            _ => return None,
        };
        match self
            .infer_expr(file, *module.call_expr_with_impl_fn.get(&name)?)
            .kind()
        {
            TyKind::Rule(rule) => {
                let ty = builtin_types(self.db, file.dialect(self.db))
                    .types(self.db)
                    .get(match rule.kind {
                        RuleKind::Build => "ctx",
                        RuleKind::Repository => "repository_ctx",
                    })?;
                match (ty.kind(), &rule.attrs) {
                    (TyKind::BuiltinType(ty, _), Some(attrs)) => Some(
                        TyKind::BuiltinType(
                            *ty,
                            Some(TyData::Attributes(rule.kind.clone(), attrs.clone())),
                        )
                        .intern(),
                    ),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn lower_param_type_ref(
        &mut self,
        file: File,
        param: ParamId,
        type_ref: &TypeRef,
        usage: Option<StmtId>,
    ) -> Ty {
        let (ty, errors) = usage.map_or_else(
            || resolve_builtin_type_ref(self.db, type_ref),
            |usage| resolve_type_ref(self, type_ref, Some(InFile { file, value: usage })),
        );

        // TODO(withered-magic): This will eventually need to handle diagnostics
        // for other places that type comments can appear.
        for error in errors {
            if let Some(ptr) = source_map(self.db, file).param_map_back.get(&param) {
                self.add_diagnostic_for_range(
                    file,
                    Severity::Warning,
                    ptr.syntax_node_ptr().text_range(),
                    None,
                    error,
                );
            }
        }

        ty
    }

    pub fn infer_load_item(&mut self, file: File, load_item: LoadItemId) -> Ty {
        if let Some(ty) = self
            .cx
            .type_of_load_item
            .get(&FileLoadItemId::new(file, load_item))
        {
            return ty.clone();
        }

        let db = self.db;
        let range = || {
            let ptr = source_map(db, file)
                .load_item_map_back
                .get(&load_item)
                .unwrap();
            ptr.syntax_node_ptr().text_range()
        };

        let ty = match &module(db, file).load_items[load_item] {
            LoadItem::Direct { name, load_stmt }
            | LoadItem::Aliased {
                name, load_stmt, ..
            } => {
                self.resolve_load_stmt(file, *load_stmt)
                    .map(|loaded_file| {
                        // Check for potential circular imports, including importing the current file.
                        if file == loaded_file {
                            self.add_diagnostic_for_range(
                                file,
                                Severity::Warning,
                                range(),
                                None,
                                "Cannot load the current file",
                            );
                            return self.unknown_ty();
                        }

                        if self
                            .cx
                            .load_resolution_stack
                            .iter()
                            .any(|(entry_file, _)| loaded_file == *entry_file)
                        {
                            let mut message = String::from("Detected circular import\n");
                            for (_, load_stmt) in self.cx.load_resolution_stack.iter() {
                                message.push_str("- ");
                                message.push_str(load_stmt.module(db));
                                message.push('\n');
                            }
                            message.push_str("- ");
                            message.push_str(load_stmt.module(db));
                            message.push('\n');

                            // Use a range here to avoid having to allocate.
                            for i in 0..self.cx.load_resolution_stack.len() {
                                let (file, load_stmt) = self.cx.load_resolution_stack[i];
                                self.add_diagnostic_for_range(
                                    file,
                                    Severity::Warning,
                                    load_stmt.ptr(db).text_range(),
                                    None,
                                    message.clone(),
                                )
                            }

                            // Also add the current (importing) file.
                            self.add_diagnostic_for_range(
                                file,
                                Severity::Warning,
                                load_stmt.ptr(db).text_range(),
                                None,
                                message,
                            );

                            return self.unknown_ty();
                        }

                        // Add the current file to the load resolution stack.
                        self.push_load_resolution(file, *load_stmt, |tcx| {
                            // TODO(withered-magic): This is potentially super slow.
                            // tcx.infer_all_load_items(loaded_file);

                            match Resolver::resolve_export_in_file(
                                db,
                                loaded_file,
                                &Name::from_str(name),
                            ) {
                                Some(Export::Variable(expr)) => {
                                    tcx.infer_expr(loaded_file, expr.expr)
                                }
                                Some(Export::Function(def)) => TyKind::Function(def).intern(),
                                None => {
                                    tcx.add_diagnostic_for_range(
                                        file,
                                        Severity::Warning,
                                        range(),
                                        None,
                                        format!(
                                            "Could not resolve symbol \"{}\" in module \"{}\"",
                                            name,
                                            load_stmt.module(db)
                                        ),
                                    );
                                    tcx.unknown_ty()
                                }
                            }
                        })
                    })
                    .unwrap_or_else(|| self.unknown_ty())
            }
        };

        self.cx
            .type_of_load_item
            .insert(FileLoadItemId::new(file, load_item), ty.clone());
        ty
    }

    pub fn resolve_load_stmt(&mut self, file: File, load_stmt: LoadStmt) -> Option<File> {
        let id = FileLoadStmt::new(file, load_stmt);

        if let Some(loaded_file) = self.cx.resolved_load_stmts.get(&id) {
            return *loaded_file;
        }

        let module = load_stmt.module(self.db);
        let res = match self
            .db
            .load_file(module, file.dialect(self.db), file.id(self.db))
        {
            Ok(Some(loaded_file)) => Some(loaded_file),
            Ok(None) => return None,
            Err(err) => {
                self.add_diagnostic_for_range(
                    file,
                    Severity::Warning,
                    load_stmt.ptr(self.db).text_range(),
                    None,
                    format!(
                        "Could not resolve module \"{}\": {}",
                        load_stmt.module(self.db),
                        err
                    ),
                );
                None
            }
        };

        self.cx.resolved_load_stmts.insert(id, res);
        res
    }

    fn push_load_resolution<F, T>(&mut self, file: File, load_stmt: LoadStmt, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.cx.load_resolution_stack.push((file, load_stmt));
        let res = f(self);
        self.cx.load_resolution_stack.pop();
        res
    }

    pub(crate) fn resolve_call_expr_active_param(
        &mut self,
        file: File,
        expr: ExprId,
        active_arg: usize,
    ) -> Option<usize> {
        let db = self.db;
        match &module(db, file)[expr] {
            Expr::Call { callee, args } => {
                // Determine args that are in invalid positions.
                let mut saw_keyword = false;
                let mut saw_unpacked_dict = false;
                for (index, arg) in args.iter().enumerate() {
                    match arg {
                        Argument::Simple { .. } | Argument::UnpackedList { .. } => {
                            if saw_keyword || saw_unpacked_dict && index == active_arg {
                                return None;
                            }
                        }
                        Argument::Keyword { .. } => saw_keyword = true,
                        Argument::UnpackedDict { .. } => saw_unpacked_dict = true,
                    }
                }

                if active_arg == args.len() && (saw_keyword || saw_unpacked_dict) {
                    return None;
                }

                let callee_ty = self.infer_expr(file, *callee);
                let mut slots: Slots = match callee_ty.kind() {
                    TyKind::Function(def) => {
                        let module = module(db, def.func().file(db));
                        let params = def.func().params(db).iter().copied();
                        params
                            .clone()
                            .map(|param| module[param].clone())
                            .collect::<Vec<_>>()[..]
                            .into()
                    }
                    TyKind::IntrinsicFunction(func, _) => func.params(db)[..].into(),
                    TyKind::BuiltinFunction(func) => func.params(db)[..].into(),
                    TyKind::Rule(rule) => Slots::from_rule(db, rule),
                    TyKind::Provider(provider) | TyKind::ProviderRawConstructor(_, provider) => {
                        Slots::from_provider(db, provider)
                    }
                    TyKind::Tag(tag_class) => Slots::from_tag_class(tag_class),
                    TyKind::Macro(makro) => Slots::from_macro(makro),
                    _ => return None,
                };

                slots.assign_args(args, Some(active_arg)).1
            }
            _ => None,
        }
    }

    fn types(&self) -> &IntrinsicTypes {
        self.intrinsics.types(self.db)
    }

    fn any_ty(&self) -> Ty {
        self.types().any.clone()
    }

    fn unbound_ty(&self) -> Ty {
        self.types().unbound.clone()
    }

    fn unknown_ty(&self) -> Ty {
        self.types().unknown.clone()
    }

    fn none_ty(&self) -> Ty {
        self.types().none.clone()
    }

    fn bool_ty(&self) -> Ty {
        self.types().bool.clone()
    }

    fn int_ty(&self) -> Ty {
        self.types().int.clone()
    }

    fn float_ty(&self) -> Ty {
        self.types().float.clone()
    }

    fn string_ty(&self) -> Ty {
        self.types().string.clone()
    }

    fn bytes_ty(&self) -> Ty {
        self.types().bytes.clone()
    }
}
