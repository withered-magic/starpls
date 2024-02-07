use crate::{
    def::{
        resolver::{Export, Resolver},
        Argument, Expr, ExprId, Literal, LoadItem, LoadItemId, LoadStmt, Param, ParamId, Stmt,
    },
    display::DisplayWithDb,
    module, source_map,
    typeck::{
        assign_tys,
        call::{Slot, SlotProvider, Slots},
        intrinsics::{IntrinsicFunctionParam, IntrinsicTypes},
        resolve_type_ref, resolve_type_ref_opt, FileExprId, FileParamId, Substitution, Tuple, Ty,
        TyCtxt, TyKind, TypeRef, TypecheckCancelled,
    },
    Declaration, Name,
};
use smallvec::SmallVec;
use starpls_common::{line_index, parse, Diagnostic, Dialect, File, FileRange, Severity};
use starpls_syntax::{
    ast::{self, ArithOp, AstNode, AstPtr, BinaryOp, UnaryOp},
    TextRange,
};

use super::{FileLoadItemId, FileLoadStmt};

impl TyCtxt<'_> {
    pub fn infer_all_exprs(&mut self, file: File) {
        for (expr, _) in module(self.db, file).exprs.iter() {
            self.infer_expr(file, expr);
        }
    }

    pub fn infer_all_params(&mut self, file: File) {
        for (param, _) in module(self.db, file).params.iter() {
            self.infer_param(file, param);
        }
    }

    pub fn infer_all_load_items(&mut self, file: File) {
        let module = module(self.db, file);

        for stmt in module.top_level.iter().copied() {
            if let Stmt::Load { load_stmt, items } = &module.stmts[stmt] {
                self.resolve_load_stmt(file, *load_stmt);
                for load_item in items.iter().copied() {
                    self.infer_load_item(file, *load_stmt, load_item);
                }
            }
        }
    }

    pub fn diagnostics_for_file(&self, file: File) -> Vec<Diagnostic> {
        let line_index = line_index(self.db, file);
        let module = module(self.db, file);
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

    pub fn infer_expr(&mut self, file: File, expr: ExprId) -> Ty {
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
                let resolver = Resolver::new_for_expr(db, file, expr);
                resolver
                    .resolve_name(name)
                    .and_then(|decls| decls.into_iter().last())
                    .map(|decl| match decl {
                        Declaration::Variable { id, source } => self
                            .cx
                            .type_of_expr
                            .get(&FileExprId::new(file, id))
                            .cloned()
                            .unwrap_or_else(|| {
                                source
                                    .and_then(|source| {
                                        self.infer_source_expr_assign(file, source);
                                        self.cx
                                            .type_of_expr
                                            .get(&FileExprId::new(file, id))
                                            .cloned()
                                    })
                                    .unwrap_or_else(|| self.unknown_ty())
                            }),
                        Declaration::Function { func, .. } => func.ty(),
                        Declaration::IntrinsicFunction { func } => {
                            TyKind::IntrinsicFunction(func, Substitution::new_identity(0)).intern()
                        }
                        Declaration::BuiltinFunction { func } => {
                            TyKind::BuiltinFunction(func).intern()
                        }
                        Declaration::BuiltinVariable { type_ref } => {
                            resolve_type_ref(db, &type_ref).0
                        }
                        Declaration::Parameter { id, .. } => self.infer_param(file, id),
                        Declaration::LoadItem { id, load_stmt } => {
                            self.infer_load_item(file, load_stmt, id)
                        }
                    })
                    .unwrap_or_else(|| {
                        self.add_expr_diagnostic(
                            file,
                            expr,
                            format!("\"{}\" is not defined", name.as_str()),
                        );
                        self.unbound_ty()
                    })
            }
            Expr::List { exprs } => {
                // Determine the full type of the list. If all of the specified elements are of the same type T, then
                // we assign the list the type `list[T]`. Otherwise, we assign it the type `list[Unknown]`.
                TyKind::List(self.get_common_type(file, exprs.iter().cloned(), self.unknown_ty()))
                    .intern()
            }
            Expr::ListComp { expr, .. } => TyKind::List(self.infer_expr(file, *expr)).intern(),
            Expr::Dict { entries } => {
                // Determine the dict's key type. For now, if all specified entries have the key type `T`, then we also
                // use the type `T` as the dict's key tpe. Otherwise, we use `Any` as the key type.
                // TODO(withered-magic): Eventually, we should use a union type here.
                let key_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.key),
                    self.any_ty(),
                );

                // Similarly, determine the dict's value type.
                let value_ty = self.get_common_type(
                    file,
                    entries.iter().map(|entry| entry.value),
                    self.unknown_ty(),
                );
                TyKind::Dict(key_ty, value_ty).intern()
            }
            Expr::DictComp { entry, .. } => {
                let key_ty = self.infer_expr(file, entry.key);
                let value_ty = self.infer_expr(file, entry.value);
                TyKind::Dict(key_ty, value_ty).intern()
            }
            Expr::Literal { literal } => match literal {
                Literal::Int(_) => self.int_ty(),
                Literal::Float => self.float_ty(),
                Literal::String(_) => self.string_ty(),
                Literal::Bytes => self.bytes_ty(),
                Literal::Bool(_) => self.bool_ty(),
                Literal::None => self.none_ty(),
            },
            Expr::Unary {
                op,
                expr: unary_expr,
            } => op
                .as_ref()
                .map(|op| self.infer_unary_expr(file, expr, *unary_expr, op.clone()))
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

                // Special-casing for "Any", "Unknown", "Unbound", invalid field
                // names, and Bazel `struct`s.
                // TODO(withered-magic): Is there a better way to handle `struct`s here?
                if receiver_ty.is_any() {
                    return self.any_ty();
                }

                if receiver_ty.is_unknown() || field.is_missing() {
                    return self.unknown_ty();
                }

                // Special handling for Bazel structs, which can have arbitrary fields.
                if let TyKind::BuiltinType(type_) = receiver_ty.kind() {
                    if type_.name(db).as_str() == "struct" {
                        return self.unknown_ty();
                    }
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
                        self.add_expr_diagnostic_ty(
                            file,
                            expr,
                            format!(
                                "Cannot access field \"{}\" for type \"{}\"",
                                field.as_str(),
                                receiver_ty.display(db)
                            ),
                        )
                    })
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
                    TyKind::List(ty) => (&int_ty, ty, "list"),
                    TyKind::Dict(key_ty, value_ty) => (key_ty, value_ty, "dict"),
                    TyKind::String => (&int_ty, &string_ty, "string"),
                    TyKind::Bytes => (&int_ty, &int_ty, "bytes"),
                    TyKind::Range => (&int_ty, &int_ty, "range"),
                    TyKind::Any => return self.any_ty(),
                    TyKind::Unknown => return self.unknown_ty(),
                    _ => {
                        let return_ty = self.add_expr_diagnostic_ty(
                            file,
                            expr,
                            format!("Type \"{}\" is not indexable", lhs_ty.display(db).alt()),
                        );
                        return self.set_expr_type(file, expr, return_ty);
                    }
                };

                if assign_tys(db, &index_ty, target) {
                    value.clone()
                } else {
                    self.add_expr_diagnostic_ty(
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
                let mut saw_keyword_arg = false;
                let callee_ty = self.infer_expr(file, *callee);
                let args_with_ty: SmallVec<[(Argument, Ty); 5]> =
                    args.iter()
                        .cloned()
                        .map(|arg| {
                            let arg_ty = match &arg {
                                Argument::Simple { expr } => {
                                    if saw_keyword_arg {
                                        self.add_expr_diagnostic(
                                        file,
                                        *expr,
                                        String::from(
                                            "Positional argument cannot follow keyword arguments",
                                        ),
                                    );
                                    }
                                    self.infer_expr(file, *expr)
                                }
                                Argument::Keyword { expr, .. } => {
                                    saw_keyword_arg = true;
                                    self.infer_expr(file, *expr)
                                }
                                Argument::UnpackedList { expr }
                                | Argument::UnpackedDict { expr } => self.infer_expr(file, *expr),
                            };
                            (arg, arg_ty)
                        })
                        .collect();

                match callee_ty.kind() {
                    TyKind::Function(func) => {
                        let module = module(db, func.file(db));
                        let params = func.params(db).iter().copied();
                        let mut slots: Slots = params
                            .clone()
                            .map(|param| module[param].clone())
                            .collect::<Vec<_>>()[..]
                            .into();
                        let errors = slots.assign_args(&args_with_ty);

                        for error in errors {
                            self.add_expr_diagnostic(file, error.expr, error.message);
                        }

                        let mut missing_params = Vec::new();

                        // Validate argument types.
                        for (param, slot) in params.zip(slots.into_inner()) {
                            let hir_param = &module[param];
                            let param_ty = resolve_type_ref_opt(db, hir_param.type_ref());

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
                                SlotProvider::Single(expr, ty) => {
                                    if !assign_tys(db, &ty, &param_ty) {
                                        self.add_expr_diagnostic(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
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

                            self.add_expr_diagnostic(file, expr, message);
                        }

                        func.ret_type_ref(db)
                            .map(|type_ref| resolve_type_ref(db, &type_ref).0)
                            .unwrap_or_else(|| self.unknown_ty())
                    }
                    TyKind::IntrinsicFunction(func, subst) => {
                        let params = func.params(db);
                        let mut slots: Slots = params[..].into();
                        let errors = slots.assign_args(&args_with_ty);

                        for error in errors {
                            self.add_expr_diagnostic(file, error.expr, error.message);
                        }

                        // Validate argument types.
                        for (param, slot) in params.iter().zip(slots.into_inner()) {
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
                                        self.add_expr_diagnostic(
                                            file,
                                            expr,
                                            format!(
                                                "Missing expected argument of type \"{}\"",
                                                param_ty.display(db)
                                            ),
                                        );
                                    }
                                }
                                SlotProvider::Single(expr, ty) => {
                                    if !assign_tys(db, &ty, &param_ty) {
                                        self.add_expr_diagnostic(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
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

                        func.ret_ty(db).substitute(&subst.args)
                    }
                    TyKind::BuiltinFunction(func) => {
                        let params = func.params(db);
                        let mut slots: Slots = params[..].into();
                        let errors = slots.assign_args(&args_with_ty);

                        for error in errors {
                            self.add_expr_diagnostic(file, error.expr, error.message);
                        }

                        let mut missing_params = Vec::new();

                        // Validate argument types.
                        for (param, slot) in params.iter().zip(slots.into_inner()) {
                            let param_ty = resolve_type_ref_opt(db, param.type_ref());
                            let mut validate_provider = |provider| match provider {
                                SlotProvider::Missing => {
                                    if param.is_mandatory() {
                                        let name = param.name();
                                        if !name.is_missing() {
                                            missing_params.push(name.clone());
                                        }
                                    }
                                }
                                SlotProvider::Single(expr, ty) => {
                                    if !assign_tys(db, &ty, &param_ty) {
                                        self.add_expr_diagnostic(file, expr, format!("Argument of type \"{}\" cannot be assigned to parameter of type \"{}\"", ty.display(self.db).alt(), param_ty.display(self.db).alt()));
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

                            self.add_expr_diagnostic(file, expr, message);
                        }

                        resolve_type_ref(db, &func.ret_type_ref(db)).0
                    }
                    TyKind::Unknown | TyKind::Any | TyKind::Unbound => self.unknown_ty(),
                    _ => self.add_expr_diagnostic_ty(
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
            _ => self.any_ty(),
        };
        self.set_expr_type(file, expr, ty)
    }

    fn infer_unary_expr(&mut self, file: File, parent: ExprId, expr: ExprId, op: UnaryOp) -> Ty {
        let db = self.db;
        let ty = self.infer_expr(file, expr);
        let mut unknown = || {
            self.add_expr_diagnostic_ty(
                file,
                parent,
                format!(
                    "Operator \"{}\" is not supported for type \"{}\"",
                    op,
                    ty.display(db)
                ),
            )
        };

        // Special handling for "Any".
        if ty.is_any() {
            return self.any_ty();
        }

        // Special handling for "Unknown" and "Unbound".
        if ty.is_unknown() {
            return self.unknown_ty();
        }

        let kind = ty.kind();
        match op {
            UnaryOp::Arith(_) => match kind {
                TyKind::Int => self.int_ty(),
                TyKind::Float => self.float_ty(),
                _ => unknown(),
            },
            UnaryOp::Inv => match kind {
                TyKind::Int => self.int_ty(),
                _ => unknown(),
            },
            UnaryOp::Not => self.bool_ty(),
        }
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
            self.add_expr_diagnostic_ty(
                file,
                parent,
                format!(
                    "Operator \"{}\" not supported for types \"{}\" and \"{}\"",
                    op,
                    lhs_kind.display(db),
                    rhs_kind.display(db)
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
            // TODO(withered-magic): Handle string interoplation with "%".
            BinaryOp::Arith(op) => match (lhs_kind, rhs_kind, op) {
                (TyKind::String, TyKind::String, ArithOp::Add | ArithOp::Mod) => self.string_ty(),
                (TyKind::List(target), TyKind::List(source), ArithOp::Add)
                    if assign_tys(db, source, target) =>
                {
                    lhs
                }
                (TyKind::Int, TyKind::Int, _) => self.int_ty(),
                (TyKind::Float, TyKind::Int, _)
                | (TyKind::Int, TyKind::Float, _)
                | (TyKind::Float, TyKind::Float, _) => self.float_ty(),
                _ => unknown(),
            },
            BinaryOp::Bitwise(_) => match (lhs_kind, rhs_kind) {
                (TyKind::Int, TyKind::Int) => self.int_ty(),
                _ => unknown(),
            },
            _ => self.bool_ty(),
        }
    }

    fn infer_source_expr_assign(&mut self, file: File, source: ExprId) {
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
            let expected_ty = match &module(db, file)[*source_map.stmt_map.get(&ptr).unwrap()] {
                Stmt::Assign { type_ref, .. } => type_ref.as_ref().and_then(|type_ref| {
                    let (expected_ty, errors) = resolve_type_ref(db, &type_ref.0);
                    if errors.is_empty() {
                        Some(expected_ty)
                    } else {
                        // Add TypeRef resolution errors.
                        for error in errors.iter() {
                            self.add_diagnostic_for_range(file, type_ref.1, error);
                        }
                        None
                    }
                }),
                _ => None,
            };

            if let Some(lhs) = node.lhs() {
                let lhs_ptr = AstPtr::new(&lhs);
                let expr = source_map.expr_map.get(&lhs_ptr).unwrap();
                self.assign_expr_source_ty(file, source, *expr, source_ty, expected_ty);
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
            TyKind::List(ty) => ty.clone(),
            TyKind::Dict(key_ty, _) => key_ty.clone(),
            TyKind::Tuple(_) | TyKind::Any => self.any_ty(),
            TyKind::Range => self.int_ty(),
            TyKind::StringElems => self.string_ty(),
            TyKind::BytesElems => self.int_ty(),
            TyKind::Unknown => self.unknown_ty(),
            _ => {
                self.add_expr_diagnostic(
                    file,
                    source,
                    format!("Type \"{}\" is not iterable", source_ty.display(db)),
                );
                for expr in targets.iter() {
                    self.assign_expr_unknown_rec(file, *expr);
                }
                return;
            }
        };
        if targets.len() == 1 {
            self.assign_expr_source_ty(file, targets[0], targets[0], sub_ty, None);
        } else {
            self.assign_exprs_source_ty(file, source, &targets, sub_ty);
        }
    }

    fn assign_expr_source_ty(
        &mut self,
        file: File,
        root: ExprId,
        expr: ExprId,
        source_ty: Ty,
        expected_ty: Option<Ty>,
    ) {
        match module(self.db, file).exprs.get(expr).unwrap() {
            Expr::Name { .. } => {
                // If we have an expected type from a type comment, use that.
                // We also emit any error if the source and expected types aren't compatible.
                if let Some(expected_ty) = expected_ty {
                    if !assign_tys(self.db, &source_ty, &expected_ty) {
                        self.add_expr_diagnostic(
                            file,
                            root,
                            format!(
                                "Expected value of type \"{}\"",
                                expected_ty.display(self.db)
                            ),
                        )
                    }
                    self.set_expr_type(file, expr, expected_ty);
                } else {
                    self.set_expr_type(file, expr, source_ty);
                }
            }
            Expr::List { exprs } | Expr::Tuple { exprs } => {
                self.assign_exprs_source_ty(file, root, exprs, source_ty);
            }
            Expr::Paren { expr } => self.assign_expr_source_ty(file, root, *expr, source_ty, None),
            _ => {}
        }
    }

    fn assign_exprs_source_ty(
        &mut self,
        file: File,
        root: ExprId,
        exprs: &[ExprId],
        source_ty: Ty,
    ) {
        match source_ty.kind() {
            TyKind::List(ty) | TyKind::Tuple(Tuple::Variable(ty)) => {
                for expr in exprs.iter().copied() {
                    self.assign_expr_source_ty(file, root, expr, ty.clone(), None);
                }
            }
            TyKind::Tuple(Tuple::Simple(tys)) => {
                let mut pairs = exprs.iter().copied().zip(tys.iter());
                while let Some((expr, ty)) = pairs.next() {
                    self.assign_expr_source_ty(file, root, expr, ty.clone(), None);
                }
                if exprs.len() != tys.len() {
                    if exprs.len() > tys.len() {
                        for expr in &exprs[tys.len()..] {
                            self.assign_expr_unknown_rec(file, *expr);
                        }
                    }
                    self.add_expr_diagnostic(
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
                    self.assign_expr_source_ty(file, root, expr, self.unknown_ty(), None);
                }
            }
            _ => {
                self.add_expr_diagnostic(
                    file,
                    root,
                    format!("Type \"{}\" is not iterable", source_ty.display(self.db)),
                );
                for expr in exprs.iter() {
                    self.assign_expr_unknown_rec(file, *expr);
                }
                return;
            }
        };
    }

    fn assign_expr_unknown_rec(&mut self, file: File, expr: ExprId) {
        self.set_expr_type(file, expr, self.unknown_ty());
        module(self.db, file)[expr].walk_child_exprs(|expr| {
            self.assign_expr_unknown_rec(file, expr);
        })
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
                exprs
                    .map(|expr| self.infer_expr(file, expr))
                    .all(|ty| ty == first_ty)
                    .then_some(first_ty)
            })
            .unwrap_or(default)
    }

    fn add_expr_diagnostic<T: Into<String>>(&mut self, file: File, expr: ExprId, message: T) {
        let range = match source_map(self.db, file).expr_map_back.get(&expr) {
            Some(ptr) => ptr.syntax_node_ptr().text_range(),
            None => return,
        };
        self.add_diagnostic_for_range(file, range, message);
    }

    fn add_expr_diagnostic_ty<T: Into<String>>(
        &mut self,
        file: File,
        expr: ExprId,
        message: T,
    ) -> Ty {
        self.add_expr_diagnostic(file, expr, message);
        self.unknown_ty()
    }

    fn add_diagnostic_for_range<T: Into<String>>(
        &mut self,
        file: File,
        range: TextRange,
        message: T,
    ) {
        self.cx.diagnostics.push(Diagnostic {
            message: message.into(),
            severity: Severity::Error,
            range: FileRange {
                file_id: file.id(self.db),
                range,
            },
        });
    }

    pub fn infer_param(&mut self, file: File, param: ParamId) -> Ty {
        if let Some(ty) = self.cx.type_of_param.get(&FileParamId::new(file, param)) {
            return ty.clone();
        }

        let ty = match &module(self.db, file)[param] {
            Param::Simple { type_ref, .. } => type_ref
                .as_ref()
                .map(|type_ref| self.lower_param_type_ref(file, param, &type_ref))
                .unwrap_or_else(|| self.unknown_ty()),
            Param::ArgsList { type_ref, .. } => TyKind::Tuple(Tuple::Variable(
                type_ref
                    .as_ref()
                    .map(|type_ref| self.lower_param_type_ref(file, param, type_ref))
                    .unwrap_or_else(|| self.unknown_ty()),
            ))
            .intern(),
            Param::KwargsDict { type_ref, .. } => TyKind::Dict(
                self.string_ty(),
                type_ref
                    .as_ref()
                    .map(|type_ref| self.lower_param_type_ref(file, param, type_ref))
                    .unwrap_or_else(|| self.unknown_ty()),
            )
            .intern(),
        };

        self.cx
            .type_of_param
            .insert(FileParamId::new(file, param), ty.clone());
        ty
    }

    fn lower_param_type_ref(&mut self, file: File, param: ParamId, type_ref: &TypeRef) -> Ty {
        let (ty, errors) = resolve_type_ref(self.db, type_ref);

        // TODO(withered-magic): This will eventually need to handle diagnostics
        // for other places that type comments can appear.
        for error in errors {
            if let Some(ptr) = source_map(self.db, file).param_map_back.get(&param) {
                self.add_diagnostic_for_range(file, ptr.syntax_node_ptr().text_range(), error);
            }
        }

        ty
    }

    pub(crate) fn infer_load_item(
        &mut self,
        file: File,
        load_stmt: LoadStmt,
        load_item: LoadItemId,
    ) -> Ty {
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

        // TODO(withered-magic): Next step is to support resolving Bazel source files.
        if file.dialect(db) != Dialect::Standard {
            self.cx
                .type_of_load_item
                .insert(FileLoadItemId::new(file, load_item), self.unknown_ty());
            return self.unknown_ty();
        }

        let ty = match &module(db, file).load_items[load_item] {
            LoadItem::Direct { name } | LoadItem::Aliased { name, .. } => {
                self.resolve_load_stmt(file, load_stmt)
                    .map(|loaded_file| {
                        // Check for potential circular imports, including importing the current file.
                        if file == loaded_file {
                            self.add_diagnostic_for_range(
                                file,
                                range(),
                                "Cannot load the current file",
                            );
                            return self.unknown_ty();
                        } else if self
                            .cx
                            .load_resolution_stack
                            .iter()
                            .find(|(entry_file, _)| loaded_file == *entry_file)
                            .is_some()
                        {
                            let mut message = String::from("Detected circular import\n");
                            for (_, load_stmt) in self.cx.load_resolution_stack.iter() {
                                message.push_str("- ");
                                message.push_str(&load_stmt.module(db));
                                message.push('\n');
                            }
                            message.push_str("- ");
                            message.push_str(&load_stmt.module(db));
                            message.push('\n');

                            // Use a range here to avoid having to allocate.
                            for i in 0..self.cx.load_resolution_stack.len() {
                                let (file, load_stmt) = self.cx.load_resolution_stack[i].clone();
                                self.add_diagnostic_for_range(
                                    file,
                                    load_stmt.ptr(db).text_range(),
                                    message.clone(),
                                )
                            }

                            // Also add the current (importing) file.
                            self.add_diagnostic_for_range(
                                file,
                                load_stmt.ptr(db).text_range(),
                                message,
                            );

                            return self.unknown_ty();
                        }

                        // Add the current file to the load resolution stack.
                        self.push_load_resolution(file, load_stmt, |tcx| {
                            tcx.infer_all_load_items(loaded_file);

                            match Resolver::resolve_export_in_file(
                                db,
                                loaded_file,
                                &Name::from_str(name),
                            ) {
                                Some(Export::Variable { expr }) => {
                                    tcx.infer_expr(loaded_file, expr)
                                }
                                Some(Export::Function { func }) => func.ty(),
                                None => {
                                    tcx.add_diagnostic_for_range(
                                        file,
                                        range(),
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

    fn resolve_load_stmt(&mut self, file: File, load_stmt: LoadStmt) -> Option<File> {
        let id = FileLoadStmt::new(file, load_stmt);

        if let Some(loaded_file) = self.cx.resolved_load_stmts.get(&id) {
            return *loaded_file;
        }

        let module = load_stmt.module(self.db);
        let res = match self
            .db
            .load_file(&module, file.dialect(self.db), file.id(self.db))
        {
            Ok(loaded_file) => Some(loaded_file),
            Err(_err) => {
                self.add_diagnostic_for_range(
                    file,
                    load_stmt.ptr(self.db).text_range(),
                    format!("Could not resolve module \"{}\"", load_stmt.module(self.db)),
                );
                None
            }
        };

        self.cx.resolved_load_stmts.insert(id, res.clone());
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
