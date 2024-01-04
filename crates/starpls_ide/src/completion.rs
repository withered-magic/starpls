//! Partially replicates the "completions" API in the LSP specification.

pub struct CompletionItem {
    label: String,
    kind: CompletionItemKind,
    mode: CompletionMode,
}

pub enum CompletionMode {
    InsertText(String),
    TextEdit {},
}

pub enum CompletionItemKind {
    Function,
    Variable,
}
