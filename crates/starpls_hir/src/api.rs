use crate::{
    def::{
        self,
        resolver::Resolver,
        scope::{self, module_scopes, ParameterDef},
        Function as HirDefFunction, LoadItemId, Stmt,
    },
    module, source_map,
    typeck::{
        builtins::BuiltinFunction, intrinsics::IntrinsicFunction, resolve_type_ref, ParamInner,
        Substitution, Tuple, Ty, TypeRef,
    },
    Db, ExprId, Name, TyKind,
};
use starpls_common::{parse, Diagnostic, Diagnostics, File};
use starpls_syntax::{
    ast::{self, AstNode, AstPtr, SyntaxNodePtr},
    TextSize,
};

pub use crate::typeck::{Field, Param};

pub fn diagnostics_for_file(db: &dyn Db, file: File) -> impl Iterator<Item = Diagnostic> {
    module_scopes::accumulated::<Diagnostics>(db, file).into_iter()
}

pub struct Semantics<'a> {
    db: &'a dyn Db,
}

impl<'a> Semantics<'a> {
    pub fn new(db: &'a dyn Db) -> Self {
        Self { db }
    }

    pub fn callable_for_def(&self, file: File, stmt: ast::DefStmt) -> Option<Callable> {
        let ptr = AstPtr::new(&ast::Statement::cast(stmt.syntax().clone())?);
        let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
        match &module(self.db, file)[*stmt] {
            Stmt::Def { func, .. } => Some((*func).into()),
            _ => None,
        }
    }

    pub fn resolve_type(&self, type_: &ast::NamedType) -> Option<Type> {
        Some(
            resolve_type_ref(self.db, &TypeRef::from_str_opt(type_.name()?.text()))
                .0
                .into(),
        )
    }

    pub fn resolve_call_expr(&self, file: File, expr: &ast::CallExpr) -> Option<Callable> {
        let ty = self.type_of_expr(file, &expr.callee()?)?;
        Some(match ty.ty.kind() {
            TyKind::Function(func) => (*func).into(),
            TyKind::IntrinsicFunction(func, subst) => {
                Callable(CallableInner::IntrinsicFunction(*func, Some(subst.clone())))
            }
            TyKind::BuiltinFunction(func) => (*func).into(),
            TyKind::Rule(_) => Callable(CallableInner::Rule(ty.ty.clone())),
            _ => return None,
        })
    }

    pub fn type_of_expr(&self, file: File, expr: &ast::Expression) -> Option<Type> {
        let ptr = AstPtr::new(expr);
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        Some(self.db.infer_expr(file, *expr).into())
    }

    pub fn type_of_param(&self, file: File, param: &ast::Parameter) -> Option<Type> {
        let ptr = AstPtr::new(param);
        let param = source_map(self.db, file).param_map.get(&ptr)?;
        Some(self.db.infer_param(file, *param).into())
    }

    pub fn resolve_load_stmt(&self, file: File, load_stmt: &ast::LoadStmt) -> Option<File> {
        let ptr = AstPtr::new(&ast::Statement::Load(load_stmt.clone()));
        let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
        let load_stmt = match module(self.db, file)[*stmt] {
            Stmt::Load { load_stmt, .. } => load_stmt,
            _ => return None,
        };
        self.db.resolve_load_stmt(file, load_stmt)
    }

    pub fn scope_for_module(&self, file: File) -> SemanticsScope {
        let resolver = Resolver::new_for_module(self.db, file);
        SemanticsScope { resolver }
    }

    pub fn scope_for_expr(&self, file: File, expr: &ast::Expression) -> Option<SemanticsScope> {
        let ptr = AstPtr::new(expr);
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        let resolver = Resolver::new_for_expr(self.db, file, *expr);
        Some(SemanticsScope { resolver })
    }

    pub fn scope_for_offset(&self, file: File, offset: TextSize) -> SemanticsScope {
        let resolver = Resolver::new_for_offset(self.db, file, offset);
        SemanticsScope { resolver }
    }

    pub fn resolve_call_expr_active_param(
        &self,
        file: File,
        expr: &ast::CallExpr,
        active_arg: usize,
    ) -> Option<usize> {
        let ptr = AstPtr::new(&ast::Expression::Call(expr.clone()));
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        self.db
            .resolve_call_expr_active_param(file, *expr, active_arg)
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    id: Option<(File, ExprId)>,
}

impl Variable {
    pub fn is_user_defined(&self) -> bool {
        self.id.is_some()
    }
}

#[derive(Clone, Debug)]
pub struct LoadItem {
    file: File,
    id: LoadItemId,
}

impl LoadItem {
    pub fn load_stmt(&self, db: &dyn Db) -> Option<ast::LoadStmt> {
        source_map(db, self.file)
            .load_item_map_back
            .get(&self.id)
            .and_then(|ptr| ptr.try_to_node(&parse(db, self.file).syntax(db)))
            .and_then(|node| node.syntax().parent())
            .and_then(ast::LoadStmt::cast)
    }

    pub fn name(&self, db: &dyn Db) -> Name {
        match &module(db, self.file).load_items[self.id] {
            def::LoadItem::Direct { name, .. } | def::LoadItem::Aliased { name, .. } => {
                Name::from_str(&name)
            }
        }
    }
}

#[derive(Debug)]
pub enum ScopeDef {
    Callable(Callable),
    Variable(Variable),
    Parameter(Param),
    LoadItem(LoadItem),
}

impl ScopeDef {
    // TODO(withered-magic): All `ScopeDef` variants should probably store the `File` somehow
    // so we don't have to pass it in as a parameter.
    pub fn syntax_node_ptr(&self, db: &dyn Db, file: File) -> Option<SyntaxNodePtr> {
        let source_map = source_map(db, file);
        match self {
            ScopeDef::Callable(Callable(CallableInner::HirDef(func))) => Some(func.ptr(db)),
            ScopeDef::Variable(Variable {
                id: Some((_, expr)),
            }) => source_map
                .expr_map_back
                .get(expr)
                .map(|ptr| ptr.syntax_node_ptr()),
            ScopeDef::Parameter(param) => param.syntax_node_ptr(db),
            ScopeDef::LoadItem(LoadItem { id, .. }) => source_map
                .load_item_map_back
                .get(id)
                .map(|ptr| ptr.syntax_node_ptr()),
            _ => None,
        }
    }

    pub fn ty(&self, db: &dyn Db) -> Type {
        match self {
            ScopeDef::Variable(Variable {
                id: Some((file, expr)),
            }) => db.infer_expr(*file, *expr),
            ScopeDef::LoadItem(LoadItem { file, id }) => db.infer_load_item(*file, *id),
            _ => Ty::unknown(),
        }
        .into()
    }

    pub fn is_user_defined(&self) -> bool {
        match self {
            ScopeDef::Callable(it) => it.is_user_defined(),
            ScopeDef::Variable(it) => it.is_user_defined(),
            _ => true,
        }
    }
}

impl From<scope::ScopeDef> for ScopeDef {
    fn from(value: scope::ScopeDef) -> Self {
        match value {
            scope::ScopeDef::Function(it) => ScopeDef::Callable(it.into()),
            scope::ScopeDef::IntrinsicFunction(it) => {
                ScopeDef::Callable(Callable(CallableInner::IntrinsicFunction(it, None)))
            }
            scope::ScopeDef::BuiltinFunction(it) => ScopeDef::Callable(it.into()),
            scope::ScopeDef::Variable(it) => ScopeDef::Variable(Variable {
                id: Some((it.file, it.expr)),
            }),
            scope::ScopeDef::BuiltinVariable(_) => ScopeDef::Variable(Variable { id: None }),
            scope::ScopeDef::Parameter(ParameterDef {
                func: parent,
                index,
            }) => ScopeDef::Parameter(Param(ParamInner::Param { parent, index })),
            scope::ScopeDef::LoadItem(it) => ScopeDef::LoadItem(LoadItem {
                file: it.file,
                id: it.load_item,
            }),
        }
    }
}

pub struct SemanticsScope<'a> {
    resolver: Resolver<'a>,
}

impl SemanticsScope<'_> {
    pub fn names(&self) -> impl Iterator<Item = (Name, ScopeDef)> {
        self.resolver
            .names()
            .into_iter()
            .map(|(name, def)| (name, def.into()))
    }

    pub fn resolve_name(&self, name: &Name) -> Option<Vec<ScopeDef>> {
        self.resolver
            .resolve_name(&name)
            .map(|defs| defs.into_iter().map(|def| def.into()).collect())
    }
}

#[derive(Clone, Debug)]
pub struct Type {
    pub(crate) ty: Ty,
}

impl Type {
    pub fn is_function(&self) -> bool {
        matches!(
            self.ty.kind(),
            TyKind::Function(_) | TyKind::BuiltinFunction(_) | TyKind::IntrinsicFunction(_, _)
        )
    }

    pub fn is_callable(&self) -> bool {
        self.is_function() || matches!(self.ty.kind(), TyKind::Rule(_))
    }

    pub fn is_unknown(&self) -> bool {
        self.ty.kind() == &TyKind::Unknown
    }

    pub fn is_user_defined_function(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Function(_))
    }

    pub fn params(&self, db: &dyn Db) -> Vec<(Param, Type)> {
        match self.ty.params(db) {
            Some(params) => params.map(|(param, ty)| (param, ty.into())).collect(),
            None => Vec::new(),
        }
    }

    pub fn doc(&self, db: &dyn Db) -> Option<String> {
        Some(match self.ty.kind() {
            TyKind::BuiltinFunction(func) => func.doc(db).clone(),
            TyKind::BuiltinType(type_) => type_.doc(db).clone(),
            TyKind::Function(func) => return func.doc(db).map(|doc| doc.to_string()),
            TyKind::IntrinsicFunction(func, _) => func.doc(db).clone(),
            TyKind::Rule(rule) => return rule.doc.as_ref().map(Box::to_string),
            _ => return None,
        })
    }

    pub fn fields(&self, db: &dyn Db) -> Vec<(Field, Type)> {
        let fields = match self.ty.fields(db) {
            Some(fields) => fields,
            None => return Vec::new(),
        };

        fields.map(|(name, ty)| (name, ty.into())).collect()
    }

    pub fn known_keys(&self) -> Option<Vec<String>> {
        self.ty.known_keys().map(|known_keys| {
            known_keys
                .iter()
                .map(|(name, _)| name.to_string())
                .collect()
        })
    }

    pub fn dict_value_ty(&self) -> Option<Type> {
        match self.ty.kind() {
            TyKind::Dict(_, value_ty, _) => Some(value_ty.clone().into()),
            _ => None,
        }
    }

    pub fn variable_tuple_element_ty(&self) -> Option<Type> {
        match self.ty.kind() {
            TyKind::Tuple(Tuple::Variable(ty)) => Some(ty.clone().into()),
            _ => None,
        }
    }
}

impl From<Ty> for Type {
    fn from(ty: Ty) -> Self {
        Self { ty }
    }
}

#[derive(Clone, Debug)]
pub struct Callable(CallableInner);

impl Callable {
    pub fn name(&self, db: &dyn Db) -> Name {
        match self.0 {
            CallableInner::HirDef(func) => func.name(db),
            CallableInner::IntrinsicFunction(func, _) => func.name(db),
            CallableInner::BuiltinFunction(func) => func.name(db),
            CallableInner::Rule(_) => Name::new_inline("rule"),
        }
    }

    pub fn params(&self, db: &dyn Db) -> Vec<(Param, Type)> {
        self.ty(db).params(db)
    }

    pub fn ty(&self, db: &dyn Db) -> Type {
        match self.0 {
            CallableInner::HirDef(func) => TyKind::Function(func).intern(),
            CallableInner::IntrinsicFunction(func, ref subst) => TyKind::IntrinsicFunction(
                func,
                subst
                    .clone()
                    .unwrap_or_else(|| Substitution::new_identity(func.num_vars(db))),
            )
            .intern(),
            CallableInner::BuiltinFunction(func) => TyKind::BuiltinFunction(func).intern(),
            CallableInner::Rule(ref ty) => ty.clone().into(),
        }
        .into()
    }

    pub fn ret_ty(&self, db: &dyn Db) -> Type {
        self.ty(db)
            .ty
            .ret_ty(db)
            .expect("expected return type")
            .into()
    }

    pub fn doc(&self, db: &dyn Db) -> Option<String> {
        match self.0 {
            CallableInner::HirDef(func) => func.doc(db).map(|doc| doc.to_string()),
            CallableInner::BuiltinFunction(func) => Some(func.doc(db).clone()),
            CallableInner::IntrinsicFunction(func, _) => Some(func.doc(db).clone()),
            CallableInner::Rule(ref ty) => match ty.kind() {
                TyKind::Rule(rule) => rule.doc.as_ref().map(Box::to_string),
                _ => None,
            },
        }
    }

    pub fn is_user_defined(&self) -> bool {
        matches!(self.0, CallableInner::HirDef(_))
    }

    pub fn is_rule(&self) -> bool {
        matches!(self.0, CallableInner::Rule(_))
    }
}

impl From<HirDefFunction> for Callable {
    fn from(func: HirDefFunction) -> Self {
        Self(CallableInner::HirDef(func))
    }
}

impl From<BuiltinFunction> for Callable {
    fn from(func: BuiltinFunction) -> Self {
        Self(CallableInner::BuiltinFunction(func))
    }
}

#[derive(Clone, Debug)]
enum CallableInner {
    HirDef(HirDefFunction),
    IntrinsicFunction(IntrinsicFunction, Option<Substitution>),
    BuiltinFunction(BuiltinFunction),
    Rule(Ty),
}
