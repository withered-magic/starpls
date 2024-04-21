use crate::Database;
use starpls_bazel::APIContext;
use starpls_common::{parse, Db, File, FileId};
use starpls_hir::{ScopeDef, Semantics};
use starpls_syntax::{
    ast::{self, AstNode},
    TextRange,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolTag {
    Deprecated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub tags: Option<Vec<SymbolTag>>,
    pub range: TextRange,
    pub selection_range: TextRange,
    pub children: Option<Vec<DocumentSymbol>>,
}

pub(crate) fn document_symbols(db: &Database, file_id: FileId) -> Option<Vec<DocumentSymbol>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let scope = sema.scope_for_module(file);
    let mut symbols = scope
        .names()
        .filter_map(|(name, def)| {
            let range = def.syntax_node_ptr(db, file)?.text_range();
            Some(DocumentSymbol {
                name: name.as_str().to_string(),
                detail: None,
                kind: match def {
                    ScopeDef::Callable(_) => SymbolKind::Function,
                    ScopeDef::Variable(_) => SymbolKind::Variable,
                    _ => return None,
                },
                tags: None,
                range: range.clone(),
                selection_range: range,
                children: None,
            })
        })
        .collect();
    if file.api_context(db) == Some(APIContext::Build) {
        add_target_symbols(db, file, &mut symbols);
    }
    Some(symbols)
}

fn add_target_symbols(db: &Database, file: File, acc: &mut Vec<DocumentSymbol>) {
    let root = parse(db, file).syntax(db);
    let targets = root.children().filter_map(|child| {
        let expr = ast::CallExpr::cast(child)?;
        let range = expr.syntax().text_range();
        let name = expr
            .arguments()
            .into_iter()
            .flat_map(|args| args.arguments())
            .find_map(|arg| match arg {
                ast::Argument::Keyword(arg) => {
                    if arg.name()?.name()?.text() != "name" {
                        return None;
                    }
                    let lit = match arg.expr()? {
                        ast::Expression::Literal(lit) => lit,
                        _ => return None,
                    };
                    match lit.kind() {
                        ast::LiteralKind::String(s) => s.value(),
                        _ => None,
                    }
                }
                _ => None,
            })?;
        Some(DocumentSymbol {
            name: format!(":{}", name),
            detail: None,
            kind: SymbolKind::Variable,
            tags: None,
            range: range.clone(),
            selection_range: range,
            children: None,
        })
    });
    acc.extend(targets);
}
