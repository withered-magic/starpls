use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
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
use starpls_common::Dialect;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::Severity;
use starpls_ide::Analysis;
use starpls_ide::AnalysisSnapshot;
use starpls_ide::Change;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::bazel::BazelContext;
use crate::commands::InferenceOptions;
use crate::document::DefaultFileLoader;
use crate::document::PathInterner;
use crate::document::{self};
use crate::server::load_bazel_builtins;

#[derive(Args, Default)]
pub(crate) struct CheckCommand {
    /// Paths to typecheck.
    pub(crate) paths: Vec<String>,

    /// Path to the Bazel output base.
    #[clap(long = "output_base")]
    pub(crate) output_base: Option<String>,

    /// Specify patterns of files/directories to ignore.
    #[clap(long = "ignore_pattern")]
    pub(crate) ignore_patterns: Vec<String>,

    #[clap(long = "ext")]
    pub(crate) extensions: Vec<String>,

    #[command(flatten)]
    pub(crate) inference_options: InferenceOptions,
}

impl CheckCommand {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        let bazel_client = Arc::new(BazelCLI::default());
        let bazel_cx = BazelContext::new(&*bazel_client)
            .map_err(|err| anyhow!("failed to initialize Bazel context: {}", err))?;
        let builtins = load_bazel_builtins();
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

        analysis.set_builtin_defs(builtins, bazel_cx.rules);

        // Strip off the leading "." from each of the specified extensions.
        // This works better when filtering against files with .extension().
        let extensions = self
            .extensions
            .iter()
            .map(|ext| match ext.strip_prefix('.') {
                Some(ext) => ext,
                None => ext,
            })
            .chain(["star", "sky"])
            .collect::<Vec<_>>();

        let checker = Checker::new(
            analysis,
            bazel_cx.info,
            interner,
            self.paths,
            self.ignore_patterns,
            &extensions,
        )?;
        checker.report_diagnostics()
    }
}

struct FileMetadata {
    path: PathBuf,
    contents: String,
}

struct Checker {
    analysis: Analysis,
    bazel_info: BazelInfo,
    interner: Arc<PathInterner>,
    files: HashMap<FileId, FileMetadata>,
    ignored_files: HashSet<PathBuf>,
}

fn diagnostic_to_message<'a>(
    diagnostic: &'a Diagnostic,
    metadata: &'a FileMetadata,
) -> Message<'a> {
    let start: usize = diagnostic.range.range.start().into();
    let end: usize = diagnostic.range.range.end().into();
    let level = match diagnostic.severity {
        Severity::Info => Level::Info,
        Severity::Warning => Level::Warning,
        Severity::Error => Level::Error,
    };
    level.title(&diagnostic.message).snippet(
        Snippet::source(&metadata.contents)
            .origin(metadata.path.as_os_str().to_str().unwrap_or(""))
            .fold(true)
            .line_start(1)
            .annotation(level.span(start..end)),
    )
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            // Don't consider lone "." as a hidden entry.
            s.starts_with('.') && s != "."
        })
        .unwrap_or(false)
}

impl Checker {
    fn new(
        analysis: Analysis,
        bazel_info: BazelInfo,
        interner: Arc<PathInterner>,
        paths: Vec<String>,
        ignore_patterns: Vec<String>,
        extensions: &[&str],
    ) -> anyhow::Result<Self> {
        let mut checker = Self {
            analysis,
            interner,
            bazel_info,
            files: Default::default(),
            ignored_files: Default::default(),
        };
        let mut change = Change::default();

        for path in paths {
            for entry in WalkDir::new(&path).into_iter().filter_entry(|e| {
                !is_hidden(e)
                    && !ignore_patterns
                        .iter()
                        .any(|pat| e.file_name().to_str().map(|s| s == pat).unwrap_or(false))
            }) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let is_explicit = entry.path().as_os_str().to_str() == Some(path.as_str());
                    checker.load_file(&mut change, entry.path(), is_explicit, extensions)?;
                }
            }
        }

        checker.analysis.apply_change(change);
        Ok(checker)
    }

    fn load_file(
        &mut self,
        change: &mut Change,
        path: &Path,
        is_explicit: bool,
        extensions: &[&str],
    ) -> anyhow::Result<()> {
        let canonical_path = PathBuf::from(&path).canonicalize()?;
        if self.interner.lookup_by_path_buf(&canonical_path).is_some() {
            return Ok(());
        }

        let (dialect, api_context) = match document::dialect_and_api_context_for_workspace_path(
            &self.bazel_info.workspace,
            &canonical_path,
        ) {
            Some(res) => res,
            None => bail!("Failed to determine Starlark dialect for file: {:?}", path),
        };

        // Only process files that match any of the file extensions passed via the command line.
        // This always includes ".star" and ".sky" files.
        if dialect == Dialect::Standard
            && !path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        {
            if is_explicit {
                self.ignored_files.insert(path.to_path_buf());
            }
            return Ok(());
        }

        let contents = fs::read_to_string(&canonical_path)?;

        let info = api_context.map(|api_context| FileInfo::Bazel {
            api_context,
            is_external: canonical_path.starts_with(&self.bazel_info.output_base),
        });

        let file_id = self.interner.intern_path(canonical_path);
        change.create_file(file_id, dialect, info, contents.clone());
        self.files.insert(
            file_id,
            FileMetadata {
                path: path.to_path_buf(),
                contents,
            },
        );

        Ok(())
    }

    fn report_diagnostics_for_file(
        &self,
        snapshot: &AnalysisSnapshot,
        file_id: FileId,
        metadata: &FileMetadata,
        num_errors: &mut usize,
        num_warnings: &mut usize,
        num_infos: &mut usize,
    ) -> anyhow::Result<()> {
        let renderer = Renderer::styled();
        for diagnostic in snapshot.diagnostics(file_id)? {
            match diagnostic.severity {
                Severity::Info => *num_infos += 1,
                Severity::Warning => *num_warnings += 1,
                Severity::Error => *num_errors += 1,
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
        let mut num_warnings = 0;
        let mut num_infos = 0;

        let mut ignored_files = self.ignored_files.iter().collect::<Vec<_>>();
        ignored_files.sort();

        for path in ignored_files {
            anstream::print!(
                "{}\n\n",
                Renderer::styled().render(
                    Level::Warning.title(&format!("non-Starlark file {:?} was ignored", path))
                )
            );
            num_warnings += 1;
        }

        let mut files = self.files.iter().collect::<Vec<_>>();
        files.sort_by_key(|(file_id, _)| **file_id);

        for (file_id, metadata) in files {
            self.report_diagnostics_for_file(
                &snapshot,
                *file_id,
                metadata,
                &mut num_errors,
                &mut num_warnings,
                &mut num_infos,
            )?;
        }

        if num_errors > 0 {
            if num_warnings > 0 {
                anstream::println!(
                    "{}",
                    Renderer::styled().render(Level::Error.title(&format!(
                        "failed with {} errors and {} warnings",
                        num_errors, num_warnings
                    )))
                );
            } else {
                anstream::println!(
                    "{}",
                    Renderer::styled()
                        .render(Level::Error.title(&format!("failed with {} errors", num_errors)))
                );
            }
            std::process::exit(1);
        }
        if num_warnings > 0 {
            anstream::println!(
                "{}",
                Renderer::styled().render(
                    Level::Warning.title(&format!("passed with {} warnings", num_warnings))
                )
            );
        }

        Ok(())
    }
}
