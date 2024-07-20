use std::sync::Arc;

use starpls_common::{parse, Diagnostic, Diagnostics, File, InFile};
use starpls_syntax::{
    ast::{self, AstNode, AstPtr, SyntaxNodePtr},
    TextSize,
};

pub use crate::typeck::{Field, Param};
use crate::{
    def::{
        self,
        resolver::Resolver,
        scope::{self, module_scopes, FunctionDef, ParameterDef},
        LoadItemId, Stmt,
    },
    module, source_map,
    typeck::{
        self, builtins::BuiltinFunction, intrinsics::IntrinsicFunction, resolve_type_ref, with_tcx,
        FieldInner, ParamInner, Provider, Struct as DefStruct, Substitution, TagClass, Tuple, Ty,
        TypeRef,
    },
    Db, ExprId, Name, TyKind,
};

const TARGET_DOC: &str = "The BUILD target for a dependency. Appears in the fields of `ctx.attr` corresponding to dependency attributes (`label` or `label_list`).";

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
            Stmt::Def { func, .. } => Some(
                FunctionDef {
                    stmt: InFile { file, value: *stmt },
                    func: *func,
                }
                .into(),
            ),
            _ => None,
        }
    }

    pub fn resolve_type(&self, type_: &ast::NamedType) -> Option<Type> {
        Some(
            type_
                .name()
                .as_ref()
                .map(|name| name.text())
                .map(|text| {
                    with_tcx(self.db, |tcx| {
                        resolve_type_ref(tcx, &TypeRef::from_str_opt(text)).0
                    })
                })?
                .into(),
        )
    }

    pub fn resolve_call_expr(&self, file: File, expr: &ast::CallExpr) -> Option<Callable> {
        let ty = self.type_of_expr(file, &expr.callee()?)?;
        Some(match ty.ty.kind() {
            TyKind::Function(def) => def.clone().into(),
            TyKind::IntrinsicFunction(func, subst) => {
                Callable(CallableInner::IntrinsicFunction(*func, Some(subst.clone())))
            }
            TyKind::BuiltinFunction(func) => (*func).into(),
            TyKind::Rule(_) => Callable(CallableInner::Rule(ty.ty.clone())),
            TyKind::Provider(provider) => Callable(CallableInner::Provider(provider.clone())),
            TyKind::ProviderRawConstructor(name, provider) => Callable(
                CallableInner::ProviderRawConstructor(name.clone(), provider.clone()),
            ),
            TyKind::Tag(tag_class) => Callable(CallableInner::Tag(tag_class.clone())),
            _ => return None,
        })
    }

    pub fn type_of_expr(&self, file: File, expr: &ast::Expression) -> Option<Type> {
        let ptr = AstPtr::new(expr);
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        Some(with_tcx(self.db, |tcx| tcx.infer_expr(file, *expr).into()))
    }

    pub fn type_of_param(&self, file: File, param: &ast::Parameter) -> Option<Type> {
        let param = source_map(self.db, file)
            .param_map
            .get(&AstPtr::new(param))?;
        Some(with_tcx(self.db, |tcx| tcx.infer_param(file, *param)).into())
    }

    pub fn resolve_load_stmt(&self, file: File, load_stmt: &ast::LoadStmt) -> Option<File> {
        let ptr = AstPtr::new(&ast::Statement::Load(load_stmt.clone()));
        let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
        let load_stmt = match module(self.db, file)[*stmt] {
            Stmt::Load { load_stmt, .. } => load_stmt,
            _ => return None,
        };
        with_tcx(self.db, |tcx| tcx.resolve_load_stmt(file, load_stmt))
    }

    pub fn resolve_load_item(&self, file: File, load_item: &ast::LoadItem) -> Option<LoadItem> {
        let ptr = AstPtr::new(load_item);
        let load_item = source_map(self.db, file).load_item_map.get(&ptr)?;
        Some(LoadItem {
            file,
            id: *load_item,
        })
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
        with_tcx(self.db, |tcx| {
            tcx.resolve_call_expr_active_param(file, *expr, active_arg)
        })
    }

    pub fn def_for_load_item(&self, load_item: &LoadItem) -> Option<InFile<ScopeDef>> {
        let load_stmt = load_item.load_stmt(self.db)?;
        let loaded_file = self.resolve_load_stmt(load_item.file, &load_stmt)?;
        self.scope_for_module(loaded_file)
            .resolve_name(&load_item.name(self.db))
            .into_iter()
            .next()
            .map(|def| InFile {
                file: loaded_file,
                value: def,
            })
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

#[derive(Clone, Debug)]
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
            ScopeDef::Callable(Callable(CallableInner::HirDef(def))) => Some(def.func.ptr(db)),
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
            }) => with_tcx(db, |tcx| tcx.infer_expr(*file, *expr)),
            ScopeDef::Callable(callable) => return callable.ty(db),
            ScopeDef::LoadItem(LoadItem { file, id }) => {
                with_tcx(db, |tcx| tcx.infer_load_item(*file, *id))
            }
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
            scope::ScopeDef::BuiltinVariable(type_ref) => match type_ref {
                TypeRef::Provider(provider) => ScopeDef::Callable(Callable(
                    CallableInner::Provider(Provider::Builtin(provider)),
                )),
                _ => ScopeDef::Variable(Variable { id: None }),
            },
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

    pub fn exports(&self) -> impl Iterator<Item = (Name, ScopeDef)> {
        self.resolver
            .module_defs(true)
            .into_iter()
            .map(|(name, def)| (name, def.into()))
    }

    pub fn resolve_name(&self, name: &Name) -> Vec<ScopeDef> {
        let defs = match self.resolver.resolve_name(name) {
            Some((_, defs)) => defs,
            None => return Vec::new(),
        };
        let mut defs = defs.map(|def| def.def.clone().into()).collect::<Vec<_>>();
        if defs.is_empty() {
            if let Some(builtin_defs) = self.resolver.resolve_name_in_prelude_or_builtins(name) {
                defs.extend(builtin_defs.into_iter().map(|def| def.into()));
            }
        }
        defs
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
        self.is_function()
            || matches!(
                self.ty.kind(),
                TyKind::Rule(_)
                    | TyKind::Provider(_)
                    | TyKind::ProviderRawConstructor(_, _)
                    | TyKind::Tag(_)
            )
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
        match self.ty.kind() {
            TyKind::BuiltinFunction(func) => Some(func.doc(db).clone()),
            TyKind::BuiltinType(ty, _) => Some(ty.doc(db).clone()),
            TyKind::Function(def) => return def.func.doc(db).map(|doc| doc.to_string()),
            TyKind::IntrinsicFunction(func, _) => Some(func.doc(db).clone()),
            TyKind::Rule(rule) => rule.doc.as_ref().map(Box::to_string),
            TyKind::Provider(provider) => provider.doc(db),
            TyKind::ModuleExtension(module_extension)
            | TyKind::ModuleExtensionProxy(module_extension) => {
                module_extension.doc.as_ref().map(Box::to_string)
            }
            TyKind::Target => Some(TARGET_DOC.into()),
            _ => None,
        }
    }

    pub fn fields(&self, db: &dyn Db) -> Vec<(Field, Type)> {
        let fields = match self.ty.fields(db) {
            Some(fields) => fields,
            None => return Vec::new(),
        };

        let mut fields = fields
            .map(|(name, ty)| (name, ty.into()))
            .collect::<Vec<_>>();

        // TODO(withered-magic): This ideally should be handled in `Ty::fields()` instead.
        if let TyKind::Struct(Some(DefStruct::Attributes { attrs })) = self.ty.kind() {
            fields.extend(attrs.iter().map(|(name, attr)| {
                (
                    Field(FieldInner::StructField {
                        name: name.clone(),
                        doc: attr.doc.as_ref().map(|doc| doc.to_string()),
                    }),
                    attr.resolved_ty().into(),
                )
            }));
        }

        fields
    }

    pub fn provider_fields_source(&self, db: &dyn Db) -> Option<InFile<ast::DictExpr>> {
        match self.ty.kind() {
            TyKind::Provider(provider) | TyKind::ProviderInstance(provider) => {
                let dict_expr = match provider {
                    Provider::Builtin(_) => return None,
                    Provider::Custom(provider) => provider
                        .fields
                        .as_ref()
                        .and_then(|fields| fields.0.clone())?,
                };
                source_map(db, dict_expr.file)
                    .expr_map_back
                    .get(&dict_expr.value)
                    .and_then(|ptr| ptr.clone().cast::<ast::DictExpr>())
                    .and_then(|ptr| {
                        Some(InFile {
                            file: dict_expr.file,
                            value: ptr.try_to_node(&parse(db, dict_expr.file).syntax(db))?,
                        })
                    })
            }
            _ => None,
        }
    }

    pub fn known_keys(&self, db: &dyn Db) -> Option<Vec<String>> {
        self.ty.known_keys().map(|known_keys| {
            known_keys
                .iter()
                .map(|(name, _)| name.value(db).to_string())
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

    pub fn try_as_inline_struct(&self) -> Option<Struct> {
        match self.ty.kind() {
            TyKind::Struct(strukt) => strukt.as_ref().and_then(|strukt| match strukt {
                &typeck::Struct::Inline { ref call_expr, .. } => Some(Struct {
                    call_expr: call_expr.clone(),
                }),
                _ => None,
            }),
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
            CallableInner::HirDef(ref def) => def.func.name(db),
            CallableInner::IntrinsicFunction(func, _) => func.name(db),
            CallableInner::BuiltinFunction(func) => func.name(db),
            CallableInner::Rule(_) => Name::new_inline("rule"),
            CallableInner::Provider(ref provider) => provider
                .name(db)
                .cloned()
                .unwrap_or_else(|| Name::new_inline("provider")),
            CallableInner::ProviderRawConstructor(ref name, _) => name.clone(),
            CallableInner::Tag(_) => Name::new_inline("tag"),
        }
    }

    pub fn params(&self, db: &dyn Db) -> Vec<(Param, Type)> {
        self.ty(db).params(db)
    }

    pub fn ty(&self, db: &dyn Db) -> Type {
        match self.0 {
            CallableInner::HirDef(ref def) => TyKind::Function(def.clone()).intern(),
            CallableInner::IntrinsicFunction(func, ref subst) => TyKind::IntrinsicFunction(
                func,
                subst
                    .clone()
                    .unwrap_or_else(|| Substitution::new_identity(func.num_vars(db))),
            )
            .intern(),
            CallableInner::BuiltinFunction(func) => TyKind::BuiltinFunction(func).intern(),
            CallableInner::Rule(ref ty) => ty.clone().into(),
            CallableInner::Provider(ref provider) => TyKind::Provider(provider.clone()).intern(),
            CallableInner::ProviderRawConstructor(ref name, ref provider) => {
                TyKind::ProviderRawConstructor(name.clone(), provider.clone()).intern()
            }
            CallableInner::Tag(ref tag_class) => TyKind::Tag(tag_class.clone()).intern(),
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
            CallableInner::HirDef(ref def) => def.func.doc(db).map(|doc| doc.to_string()),
            CallableInner::BuiltinFunction(func) => Some(func.doc(db).clone()),
            CallableInner::IntrinsicFunction(func, _) => Some(func.doc(db).clone()),
            CallableInner::Rule(ref ty) => match ty.kind() {
                TyKind::Rule(rule) => rule.doc.as_ref().map(Box::to_string),
                _ => None,
            },
            CallableInner::Provider(ref provider)
            | CallableInner::ProviderRawConstructor(_, ref provider) => match provider {
                Provider::Builtin(provider) => Some(provider.doc(db).clone()),
                Provider::Custom(provider) => provider.doc.map(|doc| doc.value(db).to_string()),
            },
            CallableInner::Tag(ref tag_class) => tag_class.doc.as_ref().map(|doc| doc.to_string()),
        }
    }

    pub fn is_user_defined(&self) -> bool {
        matches!(self.0, CallableInner::HirDef(_))
    }

    pub fn is_rule(&self) -> bool {
        matches!(self.0, CallableInner::Rule(_))
    }

    pub fn is_tag(&self) -> bool {
        matches!(self.0, CallableInner::Tag(_))
    }
}

impl From<FunctionDef> for Callable {
    fn from(def: FunctionDef) -> Self {
        Self(CallableInner::HirDef(def))
    }
}

impl From<BuiltinFunction> for Callable {
    fn from(func: BuiltinFunction) -> Self {
        Self(CallableInner::BuiltinFunction(func))
    }
}

#[derive(Clone, Debug)]
enum CallableInner {
    HirDef(FunctionDef),
    IntrinsicFunction(IntrinsicFunction, Option<Substitution>),
    BuiltinFunction(BuiltinFunction),
    Rule(Ty),
    Provider(Provider),
    ProviderRawConstructor(Name, Provider),
    Tag(Arc<TagClass>),
}

#[derive(Clone, Debug)]
pub struct Struct {
    call_expr: InFile<ExprId>,
}

impl Struct {
    pub fn call_expr(&self, db: &dyn Db) -> Option<InFile<ast::CallExpr>> {
        let call_expr = source_map(db, self.call_expr.file)
            .expr_map_back
            .get(&self.call_expr.value)
            .cloned()?
            .cast::<ast::CallExpr>()?
            .try_to_node(&parse(db, self.call_expr.file).syntax(db))?;
        Some(InFile {
            file: self.call_expr.file,
            value: call_expr,
        })
    }
}
