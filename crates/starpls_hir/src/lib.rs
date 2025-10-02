use std::sync::Arc;

use def::resolver::Resolver;
use def::scope;
use def::scope::module_scopes;
use def::scope::FunctionDef;
use def::scope::ParameterDef;
use def::Function;
use def::LoadItemId;
use def::Stmt;
use smallvec::SmallVec;
use starpls_bazel::Builtins;
use starpls_common::parse;
use starpls_common::Diagnostic;
use starpls_common::Diagnostics;
use starpls_common::Dialect;
use starpls_common::File;
use starpls_common::FileId;
use starpls_common::InFile;
use starpls_common::Parse;
use starpls_syntax::ast;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::AstPtr;
use starpls_syntax::ast::SyntaxNodePtr;
use starpls_syntax::TextSize;
use starpls_syntax::T;
use typeck::builtins::BuiltinFunction;
use typeck::intrinsics::IntrinsicFunction;
use typeck::resolve_type_ref;
use typeck::with_tcx;
use typeck::Field;
use typeck::FieldInner;
use typeck::Macro;
use typeck::Provider;
use typeck::Rule;
use typeck::RuleParam;
use typeck::Substitution;
use typeck::TagClass;
use typeck::TagParam;
use typeck::Tuple;

use crate::def::ExprId;
use crate::def::Module;
use crate::def::ModuleSourceMap;
pub use crate::def::Name;
pub use crate::display::DisplayWithDb;
pub use crate::display::DisplayWithDbWrapper;
pub use crate::test_database::Fixture;
pub use crate::typeck::builtins::BuiltinDefs;
pub use crate::typeck::Cancelled;
pub use crate::typeck::GlobalContext;
pub use crate::typeck::InferenceOptions;
pub use crate::typeck::Ty;
pub use crate::typeck::TyContext;
use crate::typeck::TyKind;
use crate::typeck::TypeRef;

mod def;
mod display;
mod test_database;
mod typeck;

#[salsa::tracked]
pub(crate) struct ModuleInfo {
    pub file: File,
    #[return_ref]
    pub module: Module,
    #[return_ref]
    pub source_map: ModuleSourceMap,
}

#[salsa::jar(db = Db)]
pub struct Jar(
    lower,
    ModuleInfo,
    def::Function,
    def::LoadStmt,
    def::InternedString,
    def::codeflow::CodeFlowGraphResult,
    def::codeflow::code_flow_graph,
    def::scope::ModuleScopes,
    def::scope::module_scopes,
    def::scope::module_scopes_query,
    typeck::builtins::BuiltinDefs,
    typeck::builtins::BuiltinFunction,
    typeck::builtins::BuiltinGlobals,
    typeck::builtins::BuiltinProvider,
    typeck::builtins::BuiltinProviders,
    typeck::builtins::BuiltinType,
    typeck::builtins::BuiltinTypes,
    typeck::builtins::builtin_globals_query,
    typeck::builtins::builtin_providers_query,
    typeck::builtins::builtin_types_query,
    typeck::builtins::CommonAttributes,
    typeck::builtins::common_attributes_query,
    typeck::intrinsics::Intrinsics,
    typeck::intrinsics::IntrinsicClass,
    typeck::intrinsics::IntrinsicFieldTypes,
    typeck::intrinsics::IntrinsicFunction,
    typeck::intrinsics::IntrinsicFunctions,
    typeck::intrinsics::intrinsic_types,
    typeck::intrinsics::intrinsic_field_types,
    typeck::intrinsics::intrinsic_functions,
);

/// Documentation for the `Target` type defined by Bazel.
/// TODO(withered-magic): Find a better place to put this.
const TARGET_DOC: &str = "The BUILD target for a dependency. Appears in the fields of `ctx.attr` corresponding to dependency attributes (`label` or `label_list`).";

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {
    fn gcx(&self) -> &GlobalContext;

    // Legacy dialect methods (for backward compatibility)
    fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins, rules: Builtins);
    fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs;

    fn set_bazel_prelude_file(&mut self, file_id: FileId);
    fn get_bazel_prelude_file(&self) -> Option<FileId>;
    fn set_all_workspace_targets(&mut self, targets: Vec<String>);
    fn get_all_workspace_targets(&self) -> Arc<Vec<String>>;
}

/// Return the diagnostics accumulated by Salsa queries on the given file.
/// This does not include diagnostics from type inference, which are reported
/// by [`typeck::TyContext`] instead.
pub fn diagnostics_for_file(db: &dyn Db, file: File) -> impl Iterator<Item = Diagnostic> {
    module_scopes::accumulated::<Diagnostics>(db, file).into_iter()
}

pub struct Semantics<'a> {
    pub db: &'a dyn Db,
}

impl<'a> Semantics<'a> {
    pub fn new(db: &'a dyn Db) -> Self {
        Self { db }
    }

    pub fn parse(&self, file: File) -> Parse {
        parse(self.db, file)
    }

    pub fn callable_for_def(&self, file: File, node: ast::DefStmt) -> Option<Callable> {
        let ptr = AstPtr::new(&ast::Statement::cast(node.syntax().clone())?);
        let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
        match &module(self.db, file)[*stmt] {
            Stmt::Def { func, .. } => Some(
                FunctionDef::Def {
                    func: *func,
                    stmt: InFile { file, value: *stmt },
                }
                .into(),
            ),
            _ => None,
        }
    }

    pub fn resolve_path_type(&self, file: File, node: &ast::PathType) -> Option<Type> {
        let usage = node
            .syntax()
            .ancestors()
            .find_map(ast::TypeComment::cast)
            .and_then(|type_comment| {
                let parent = type_comment.syntax().parent()?;
                let ptr = if ast::Suite::can_cast(parent.kind()) {
                    let grandparent = parent.parent()?;
                    if ast::DefStmt::can_cast(grandparent.kind()) {
                        AstPtr::new(&ast::Statement::cast(grandparent)?)
                    } else {
                        return None;
                    }
                } else if let Some(assign_stmt) = type_comment
                    .syntax()
                    .siblings_with_tokens(ast::Direction::Prev)
                    .take_while(|el| !matches!(el.kind(), T!['\n'] | T![;]))
                    .filter_map(|el| el.into_node())
                    .find_map(ast::AssignStmt::cast)
                {
                    AstPtr::new(&ast::Statement::Assign(assign_stmt))
                } else {
                    return None;
                };

                let stmt = source_map(self.db, file).stmt_map.get(&ptr)?;
                Some(InFile { file, value: *stmt })
            });
        let segments = node
            .segments()
            .flat_map(|segment| segment.value())
            .map(|token| Name::from_str(token.text()))
            .collect::<SmallVec<_>>();
        Some(
            with_tcx(self.db, |tcx| {
                // TODO(withered-magic): The clone here is a bit ugly but should be fine.
                resolve_type_ref(tcx, &TypeRef::Path(segments.clone(), None), usage).0
            })
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
            TyKind::Rule(rule) => Callable(CallableInner::Rule(rule.clone())),
            TyKind::Provider(provider) => Callable(CallableInner::Provider(provider.clone())),
            TyKind::ProviderRawConstructor(name, provider) => Callable(
                CallableInner::ProviderRawConstructor(name.clone(), provider.clone()),
            ),
            TyKind::Tag(tag_class) => Callable(CallableInner::Tag(tag_class.clone())),
            TyKind::Macro(makro) => Callable(CallableInner::Macro(makro.clone())),
            _ => return None,
        })
    }

    pub fn resolve_def_stmt(&self, file: File, def_stmt: &ast::DefStmt) -> Option<Callable> {
        let module = module(self.db, file);
        let stmt = source_map(self.db, file)
            .stmt_map
            .get(&AstPtr::new(&ast::Statement::Def(def_stmt.clone())))?;
        let Stmt::Def { func, .. } = module[*stmt] else {
            return None;
        };
        Some(Callable(CallableInner::HirDef(FunctionDef::Def {
            func,
            stmt: InFile { file, value: *stmt },
        })))
    }

    pub fn type_of_expr(&self, file: File, expr: &ast::Expression) -> Option<Type> {
        let ptr = AstPtr::new(expr);
        let expr = source_map(self.db, file).expr_map.get(&ptr)?;
        Some(with_tcx(self.db, |tcx| tcx.infer_expr(file, *expr).into()))
    }

    pub fn resolve_param(&self, file: File, param: &ast::Parameter) -> Option<(Param, Type)> {
        let module = module(self.db, file);
        let param = source_map(self.db, file)
            .param_map
            .get(&AstPtr::new(param))?;
        let (func, index) = module
            .param_to_def_stmt
            .get(param)
            .and_then(|(stmt, index)| match module[*stmt] {
                Stmt::Def { func, .. } => Some((func, index)),
                _ => None,
            })?;
        Some((
            Param(ParamInner::Param {
                func,
                index: *index,
            }),
            with_tcx(self.db, |tcx| tcx.infer_param(file, *param)).into(),
        ))
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
            id: InFile {
                file,
                value: *load_item,
            },
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

    pub fn def_for_load_item(&self, load_item: &LoadItem) -> Option<ScopeDef> {
        let load_stmt = load_item.load_stmt(self.db)?;
        let loaded_file = self.resolve_load_stmt(load_item.id.file, &load_stmt)?;
        self.scope_for_module(loaded_file)
            .resolve_name(&load_item.name(self.db))
            .into_iter()
            .next()
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
        let mut defs: Vec<ScopeDef> = match self.resolver.resolve_name(name) {
            Some((_, defs)) => defs.map(|def| def.def.clone().into()).collect(),
            None => Vec::new(),
        };
        if defs.is_empty() {
            if let Some(def) = self.resolver.resolve_name_in_prelude_or_builtins(name) {
                defs.push(def.into());
            }
        }
        defs
    }
}

/// A type. Mostly serves as a public API for [`typeck::Ty`].
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
                    | TyKind::Macro(_)
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
            TyKind::Function(def) => def.func().doc(db).map(|doc| doc.to_string()),
            TyKind::IntrinsicFunction(func, _) => Some(func.doc(db).clone()),
            TyKind::Rule(rule) => rule.doc.as_ref().map(Box::to_string),
            TyKind::Provider(provider) | TyKind::ProviderInstance(provider) => provider.doc(db),
            TyKind::ModuleExtension(module_extension)
            | TyKind::ModuleExtensionProxy(module_extension) => {
                module_extension.doc.as_ref().map(Box::to_string)
            }
            TyKind::Target => Some(TARGET_DOC.into()),
            TyKind::Macro(makro) => makro.doc.map(|doc| doc.value(db).to_string()),
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
        if let TyKind::Struct(Some(typeck::Struct::RuleAttributes { rule_kind, attrs })) =
            self.ty.kind()
        {
            fields.extend(attrs.attrs.iter().filter_map(|(name, attr)| {
                attr.as_ref().map(|attr| {
                    (
                        Field(FieldInner::StructField {
                            name: name.clone(),
                            doc: attr.doc.as_ref().map(|doc| doc.value(db).to_string()),
                        }),
                        attr.resolved_ty(rule_kind).into(),
                    )
                })
            }));
        }

        fields
    }

    pub fn provider_fields_source(&self, db: &dyn Db) -> Option<InFile<ast::DictExpr>> {
        match self.ty.kind() {
            TyKind::Provider(provider) | TyKind::ProviderInstance(provider) => {
                let dict_expr = match provider {
                    Provider::Builtin(_) => return None,
                    Provider::Custom(provider) => {
                        provider.fields.as_ref().and_then(|fields| fields.expr)?
                    }
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
                typeck::Struct::Inline { call_expr, .. } => Some(Struct {
                    call_expr: *call_expr,
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

/// A Bazel struct, created with the `struct()` function.
#[derive(Clone, Debug)]
pub struct Struct {
    /// The `struct()` call that created this struct.
    call_expr: InFile<ExprId>,
}

impl Struct {
    /// Returns the AST node corresponding to the `struct()` call that created this struct.
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

/// A variable definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    expr: Option<InFile<ExprId>>,
}

impl Variable {
    /// Whether the variable is user-defined; `false` in the case of
    /// variables from e.g. Bazel builtins.
    pub fn is_user_defined(&self) -> bool {
        self.expr.is_some()
    }
}

/// A callable value, e.g. a function, rule, etc.
/// The actual data is stored in [`CallableInner`], which wraps some
/// crate-internal data types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Callable(CallableInner);

impl Callable {
    pub fn name(&self, db: &dyn Db) -> Name {
        match self.0 {
            CallableInner::HirDef(ref def) => def.func().name(db),
            CallableInner::IntrinsicFunction(func, _) => func.name(db),
            CallableInner::BuiltinFunction(func) => func.name(db),
            CallableInner::Rule(_) => Name::new_inline("rule"),
            CallableInner::Provider(ref provider) => provider
                .name(db)
                .cloned()
                .unwrap_or_else(|| Name::new_inline("provider")),
            CallableInner::ProviderRawConstructor(ref name, _) => name.clone(),
            CallableInner::Tag(_) => Name::new_inline("tag"),
            CallableInner::Macro(_) => Name::new_inline("macro"),
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
            CallableInner::Rule(ref rule) => TyKind::Rule(rule.clone()).intern(),
            CallableInner::Provider(ref provider) => TyKind::Provider(provider.clone()).intern(),
            CallableInner::ProviderRawConstructor(ref name, ref provider) => {
                TyKind::ProviderRawConstructor(name.clone(), provider.clone()).intern()
            }
            CallableInner::Tag(ref tag_class) => TyKind::Tag(tag_class.clone()).intern(),
            CallableInner::Macro(ref makro) => TyKind::Macro(makro.clone()).intern(),
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
            CallableInner::HirDef(ref def) => def.func().doc(db).map(|doc| doc.to_string()),
            CallableInner::BuiltinFunction(func) => Some(func.doc(db).clone()),
            CallableInner::IntrinsicFunction(func, _) => Some(func.doc(db).clone()),
            CallableInner::Rule(ref rule) => rule.doc.as_ref().map(Box::to_string),
            CallableInner::Provider(ref provider)
            | CallableInner::ProviderRawConstructor(_, ref provider) => match provider {
                Provider::Builtin(provider) => Some(provider.doc(db).clone()),
                Provider::Custom(provider) => provider.doc.map(|doc| doc.value(db).to_string()),
            },
            CallableInner::Tag(ref tag_class) => {
                tag_class.doc.as_ref().map(|doc| doc.value(db).to_string())
            }
            CallableInner::Macro(ref makro) => {
                makro.doc.as_ref().map(|doc| doc.value(db).to_string())
            }
        }
    }

    pub fn file(&self) -> Option<File> {
        match self.0 {
            CallableInner::HirDef(ref def) => def.stmt().map(|stmt| stmt.file),
            _ => None,
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

    pub fn is_macro(&self) -> bool {
        matches!(self.0, CallableInner::Macro(_))
    }

    pub fn rule_attrs_source(&self, db: &dyn Db) -> Option<InFile<ast::DictExpr>> {
        let attrs_expr = match self.0 {
            CallableInner::Rule(ref rule) => rule.attrs.as_ref()?.expr?,
            _ => return None,
        };

        source_map(db, attrs_expr.file)
            .expr_map_back
            .get(&attrs_expr.value)
            .and_then(|ptr| ptr.clone().cast::<ast::DictExpr>())
            .and_then(|ptr| {
                Some(InFile {
                    file: attrs_expr.file,
                    value: ptr.try_to_node(&parse(db, attrs_expr.file).syntax(db))?,
                })
            })
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

/// Reperesents different types of callables.
#[derive(Clone, Debug, PartialEq, Eq)]
enum CallableInner {
    /// A user-defined function.
    HirDef(FunctionDef),

    // An intrinsic function, i.e. a function defined by the Starlark spec.
    IntrinsicFunction(IntrinsicFunction, Option<Substitution>),

    /// A builtin function (e.g. Bazel builtins).
    BuiltinFunction(BuiltinFunction),

    /// A Bazel rule.
    Rule(Rule),

    /// A Bazel provider.
    Provider(Provider),

    /// The raw constructor for a Bazel provider.
    /// See: https://bazel.build/rules/lib/globals/bzl.html#provider
    ProviderRawConstructor(Name, Provider),

    /// A Bazel tag.
    Tag(Arc<TagClass>),

    /// A Bazel symbolic macro.
    Macro(Macro),
}

/// A parameter for a function, rule, etc.
/// The actual data is stored in [`ParamInner`], which wraps some
/// crate-internal data types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param(pub(crate) ParamInner);

/// Reperesents parameters for different types of callables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ParamInner {
    /// A function or lambda parameter.
    Param { func: Function, index: usize },

    /// A parameter for an intrinsic function (i.e. a Starlark builtin).
    IntrinsicParam {
        parent: IntrinsicFunction,
        index: usize,
    },

    /// A parameter for a builtin function (e.g. Bazel builtins).
    BuiltinParam {
        parent: BuiltinFunction,
        index: usize,
    },

    /// Corresponds to a rule attribute.
    RuleParam(RuleParam),

    /// Corresponds to a provider field.
    ProviderParam { provider: Provider, index: usize },

    /// Corresponds to a tag attribute.
    TagParam(TagParam),
}

/// An item in a load statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadItem {
    id: InFile<LoadItemId>,
}

impl LoadItem {
    /// Returns the AST node for the corresponding load statement.
    pub fn load_stmt(&self, db: &dyn Db) -> Option<ast::LoadStmt> {
        source_map(db, self.id.file)
            .load_item_map_back
            .get(&self.id.value)
            .and_then(|ptr| ptr.try_to_node(&parse(db, self.id.file).syntax(db)))
            .and_then(|node| node.syntax().parent())
            .and_then(ast::LoadStmt::cast)
    }

    /// The name of the item being loaded by the load statement.
    pub fn name(&self, db: &dyn Db) -> Name {
        match &module(db, self.id.file).load_items[self.id.value] {
            def::LoadItem::Direct { name, .. } | def::LoadItem::Aliased { name, .. } => {
                Name::from_str(name)
            }
        }
    }
}

/// Represents the different types of definition present within a scope.
/// Mostly provides a nicer API for [`scope::ScopeDef`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScopeDef {
    /// A function definition.
    Callable(Callable),

    /// A variable definition.
    Variable(Variable),

    /// A parameter for a user-defined function.
    Parameter(Param),

    /// An item loaded by a `load` statement.
    LoadItem(LoadItem),
}

impl ScopeDef {
    pub fn syntax_node_ptr(&self, db: &dyn Db) -> Option<InFile<SyntaxNodePtr>> {
        match self {
            ScopeDef::Callable(Callable(CallableInner::HirDef(def))) => {
                Some(def.func().syntax_node_ptr(db))
            }
            ScopeDef::Variable(Variable { expr: Some(expr) }) => source_map(db, expr.file)
                .expr_map_back
                .get(&expr.value)
                .map(|ptr| InFile {
                    file: expr.file,
                    value: ptr.syntax_node_ptr(),
                }),
            ScopeDef::Parameter(param) => param.syntax_node_ptr(db),
            ScopeDef::LoadItem(LoadItem { id }) => source_map(db, id.file)
                .load_item_map_back
                .get(&id.value)
                .map(|ptr| InFile {
                    file: id.file,
                    value: ptr.syntax_node_ptr(),
                }),
            _ => None,
        }
    }

    pub fn ty(&self, db: &dyn Db) -> Type {
        match self {
            ScopeDef::Variable(Variable { expr: Some(expr) }) => {
                with_tcx(db, |tcx| tcx.infer_expr(expr.file, expr.value))
            }
            ScopeDef::Callable(callable) => return callable.ty(db),
            ScopeDef::LoadItem(LoadItem { id }) => {
                with_tcx(db, |tcx| tcx.infer_load_item(id.file, id.value))
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
                expr: Some(InFile {
                    file: it.file,
                    value: it.expr,
                }),
            }),
            scope::ScopeDef::BuiltinVariable(type_ref) => match type_ref {
                TypeRef::Provider(provider) => ScopeDef::Callable(Callable(
                    CallableInner::Provider(Provider::Builtin(provider)),
                )),
                _ => ScopeDef::Variable(Variable { expr: None }),
            },
            scope::ScopeDef::Parameter(ParameterDef { func, index }) => {
                ScopeDef::Parameter(Param(ParamInner::Param { func, index }))
            }
            scope::ScopeDef::LoadItem(def) => ScopeDef::LoadItem(LoadItem {
                id: InFile {
                    file: def.file,
                    value: def.load_item,
                },
            }),
        }
    }
}

#[salsa::tracked]
pub(crate) fn lower(db: &dyn Db, file: File) -> ModuleInfo {
    let parse = parse(db, file);
    let (module, source_map) = Module::new_with_source_map(db, file, parse.tree(db));
    ModuleInfo::new(db, file, module, source_map)
}

/// Shortcut to immediately access a `lower` query's `Module`.
pub(crate) fn module(db: &dyn Db, file: File) -> &Module {
    lower(db, file).module(db)
}

/// Shortcut to immediately access a `lower` query's `ModuleSourceMap`.
pub(crate) fn source_map(db: &dyn Db, file: File) -> &ModuleSourceMap {
    lower(db, file).source_map(db)
}
