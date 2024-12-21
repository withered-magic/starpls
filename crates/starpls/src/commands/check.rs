use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use annotate_snippets::Level;
use annotate_snippets::Message;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use anyhow::anyhow;
use anyhow::bail;
use clap::Args;
use starpls_bazel::client::BazelCLI;
use starpls_bazel::client::BazelInfo;
use starpls_common::Diagnostic;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::Severity;
use starpls_ide::Analysis;
use starpls_ide::AnalysisSnapshot;
use starpls_ide::Change;

use crate::bazel::BazelContext;
use crate::commands::InferenceOptions;
use crate::document::DefaultFileLoader;
use crate::document::PathInterner;
use crate::document::{self};

#[derive(Args, Default)]
pub(crate) struct CheckCommand {
    /// Paths to typecheck.
    pub(crate) paths: Vec<String>,

    /// Path to the Bazel output base.
    #[clap(long = "output_base")]
    pub(crate) output_base: Option<String>,

    #[command(flatten)]
    pub(crate) inference_options: InferenceOptions,
}

impl CheckCommand {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        let bazel_client = Arc::new(BazelCLI::default());
        let bazel_cx = BazelContext::new(&*bazel_client)
            .map_err(|err| anyhow!("failed to initialize Bazel context: {}", err))?;
        let (fetch_repo_sender, _) = crossbeam_channel::unbounded();
        let interner = Arc::new(PathInterner::default());
        let loader = DefaultFileLoader::new(
            bazel_client,
            interner.clone(),
            bazel_cx.info.workspace.clone(),
            bazel_cx.info.workspace_name.clone(),
            bazel_cx.info.output_base.join("external"),
            fetch_repo_sender,
            bazel_cx.bzlmod_enabled,
        );

        let mut analysis = Analysis::new(
            Arc::new(loader),
            starpls_ide::InferenceOptions {
                infer_ctx_attributes: self.inference_options.infer_ctx_attributes,
                use_code_flow_analysis: self.inference_options.use_code_flow_analysis,
                ..Default::default()
            },
        );
        analysis.set_builtin_defs(bazel_cx.builtins, bazel_cx.rules);

        let checker = Checker::new(analysis, bazel_cx.info, interner, self.paths)?;
        checker.report_diagnostics()
    }
}

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
            .annotation(level.span(start..end)),
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

    fn report_diagnostics(&self) -> anyhow::Result<()> {
        let snapshot = self.analysis.snapshot();
        let mut num_errors = 0;
        let mut files = self.files.iter().collect::<Vec<_>>();
        files.sort_by_key(|(file_id, _)| **file_id);

        for (file_id, metadata) in files {
            self.report_diagnostics_for_file(&snapshot, *file_id, metadata, &mut num_errors)?;
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
}
