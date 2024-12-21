use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use annotate_snippets::Level;
use annotate_snippets::Message;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use anyhow::bail;
use log::debug;
use log::info;
use starpls_bazel::client::BazelCLI;
use starpls_bazel::client::BazelClient;
use starpls_bazel::client::BazelInfo;
use starpls_common::Diagnostic;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::Severity;
use starpls_ide::Analysis;
use starpls_ide::AnalysisSnapshot;
use starpls_ide::Change;

use crate::document::DefaultFileLoader;
use crate::document::PathInterner;
use crate::document::{self};
use crate::server::load_bazel_build_language;
use crate::server::load_bazel_builtins;

struct FileMetadata {
    path: String,
    contents: String,
}

struct Checker {
    analysis: Analysis,
    bazel_info: BazelInfo,
    interner: Arc<PathInterner>,
    files: HashMap<FileId, FileMetadata>,
}

fn diagnostic_to_message<'a>(
    diagnostic: &'a Diagnostic,
    metadata: &'a FileMetadata,
) -> Message<'a> {
    let start: usize = diagnostic.range.range.start().into();
    let end: usize = diagnostic.range.range.end().into();
    let level = match diagnostic.severity {
        Severity::Warning => Level::Warning,
        Severity::Error => Level::Error,
    };

    level.title(&diagnostic.message).snippet(
        Snippet::source(&metadata.contents)
            .origin(&metadata.path)
            .fold(true)
            .line_start(1)
            .annotation(Level::Error.span(start..end)),
    )
}

impl Checker {
    fn new(
        analysis: Analysis,
        bazel_info: BazelInfo,
        interner: Arc<PathInterner>,
        paths: Vec<String>,
    ) -> anyhow::Result<Self> {
        let mut checker = Self {
            analysis,
            interner,
            bazel_info,
            files: HashMap::default(),
        };

        let mut change = Change::default();
        for path in paths {
            checker.load_file(&mut change, path)?;
        }

        checker.analysis.apply_change(change);

        Ok(checker)
    }

    fn load_file(&mut self, change: &mut Change, path: String) -> anyhow::Result<()> {
        let canonical_path = PathBuf::from(&path).canonicalize()?;
        if self.interner.lookup_by_path_buf(&canonical_path).is_some() {
            return Ok(());
        }

        let contents = fs::read_to_string(&canonical_path)?;
        let (dialect, api_context) = match document::dialect_and_api_context_for_workspace_path(
            &self.bazel_info.workspace,
            &canonical_path,
        ) {
            Some(res) => res,
            None => bail!("Failed to determine Starlark dialect for file: {}", path),
        };

        let info = api_context.map(|api_context| FileInfo::Bazel {
            api_context,
            is_external: canonical_path.starts_with(&self.bazel_info.output_base),
        });

        let file_id = self.interner.intern_path(canonical_path);
        change.create_file(file_id, dialect, info, contents.clone());
        self.files.insert(file_id, FileMetadata { path, contents });

        Ok(())
    }

    fn report_diagnostics_for_file(
        &self,
        snapshot: &AnalysisSnapshot,
        file_id: FileId,
        metadata: &FileMetadata,
        num_errors: &mut usize,
    ) -> anyhow::Result<()> {
        let renderer = Renderer::styled();
        for diagnostic in snapshot.diagnostics(file_id)? {
            if diagnostic.severity == Severity::Error {
                *num_errors += 1;
            }
            anstream::print!(
                "{}\n\n",
                renderer.render(diagnostic_to_message(&diagnostic, metadata))
            );
        }
        Ok(())
    }

    fn report_diagnostics(&self, snapshot: &AnalysisSnapshot) -> anyhow::Result<()> {
        let mut num_errors = 0;
        let mut files = self.files.iter().collect::<Vec<_>>();
        files.sort_by_key(|(file_id, _)| **file_id);

        for (file_id, metadata) in files {
            self.report_diagnostics_for_file(snapshot, *file_id, metadata, &mut num_errors)?;
        }

        if num_errors > 0 {
            anstream::println!(
                "{}",
                Renderer::styled()
                    .render(Level::Error.title(&format!("failed with {} errors", num_errors)))
            );
            std::process::exit(1);
        }

        Ok(())
    }

    fn snapshot(&self) -> AnalysisSnapshot {
        self.analysis.snapshot()
    }
}

pub(crate) fn run_check(paths: Vec<String>, output_base: Option<String>) -> anyhow::Result<()> {
    let bazel_client = Arc::new(BazelCLI::default());
    let bazel_info = bazel_client.info()?;
    let external_output_base = output_base
        .map(PathBuf::from)
        .unwrap_or_else(|| bazel_info.output_base.join("external"));

    let bzlmod_enabled = bazel_info
        .workspace
        .join("MODULE.bazel")
        .try_exists()
        .unwrap_or(false)
        && {
            debug!("checking for `bazel mod dump_repo_mapping` capability");
            match bazel_client.dump_repo_mapping("") {
                Ok(_) => true,
                Err(_) => {
                    info!("installed Bazel version doesn't support `bazel mod dump_repo_mapping`, disabling bzlmod support");
                    false
                }
            }
        };

    let (fetch_repo_sender, _) = crossbeam_channel::unbounded();
    let builtins = load_bazel_builtins()?;
    let rules = load_bazel_build_language(&*bazel_client)?;
    let interner = Arc::new(PathInterner::default());
    let loader = DefaultFileLoader::new(
        bazel_client,
        interner.clone(),
        bazel_info.workspace.clone(),
        bazel_info.workspace_name.clone(),
        external_output_base.clone(),
        fetch_repo_sender,
        bzlmod_enabled,
    );
    let mut analysis = Analysis::new(Arc::new(loader), Default::default());
    analysis.set_builtin_defs(builtins, rules);

    let checker = Checker::new(analysis, bazel_info, interner, paths)?;
    let snapshot = checker.snapshot();
    checker.report_diagnostics(&snapshot)
}
