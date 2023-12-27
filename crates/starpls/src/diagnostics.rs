use lsp_types::Diagnostic;
use starpls_common::FileId;
use std::{collections::HashMap, mem};

#[derive(Default)]
pub(crate) struct DiagnosticsManager {
    diagnostics: HashMap<FileId, Vec<lsp_types::Diagnostic>>,
    files_with_changed_diagnostics: Vec<FileId>,
}

impl DiagnosticsManager {
    pub(crate) fn set_diagnostics(&mut self, file_id: FileId, mut diagnostics: Vec<Diagnostic>) {
        self.diagnostics
            .entry(file_id)
            .and_modify(|current_diagnostics| {
                if current_diagnostics.len() != diagnostics.len()
                    && current_diagnostics.iter().zip(diagnostics.iter()).any(
                        |(current_diagnostic, diagnostic)| {
                            !is_diagnostic_equal(current_diagnostic, diagnostic)
                        },
                    )
                {
                    *current_diagnostics = mem::take(&mut diagnostics);
                }
            })
            .or_insert(diagnostics);
        self.files_with_changed_diagnostics.push(file_id);
    }
}

fn is_diagnostic_equal(left: &Diagnostic, right: &Diagnostic) -> bool {
    left.source == right.source
        && left.severity == right.severity
        && left.range == right.range
        && left.message == right.message
}
