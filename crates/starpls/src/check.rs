use std::{fmt::Write, fs, path::PathBuf, process, sync::Arc};

use anyhow::anyhow;
use rustc_hash::FxHashMap;
use starpls_bazel::client::{BazelCLI, BazelClient};
use starpls_common::Severity;
use starpls_ide::{Analysis, Change};

use crate::{
    document::{self, DefaultFileLoader, PathInterner},
    server::{load_bazel_build_language, load_bazel_builtins},
};

pub(crate) fn run_check(paths: Vec<String>, output_base: Option<String>) -> anyhow::Result<()> {
    let bazel_client = Arc::new(BazelCLI::default());
    let info = bazel_client.info()?;
    let external_output_base = output_base
        .map(PathBuf::from)
        .unwrap_or_else(|| info.output_base.join("external"));

    let bzlmod_enabled = info
        .workspace
        .join("MODULE.bazel")
        .try_exists()
        .unwrap_or(false)
        && {
            eprintln!("server: checking for `bazel mod dump_repo_mapping` capability");
            match bazel_client.dump_repo_mapping("") {
                Ok(_) => true,
                Err(_) => {
                    eprintln!("server: installed Bazel version doesn't support `bazel mod dump_repo_mapping`, disabling bzlmod support");
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
        info.workspace,
        external_output_base,
        fetch_repo_sender,
        bzlmod_enabled,
    );
    let mut analysis = Analysis::new(Arc::new(loader));
    let mut change = Change::default();
    let mut file_ids = Vec::new();
    let mut original_paths = FxHashMap::default();
    analysis.set_builtin_defs(builtins, rules);

    for path in &paths {
        let err = || anyhow!("Could not resolve the path {:?} as a Starlark file.", path);
        let resolved = PathBuf::from(path).canonicalize().map_err(|_| err())?;
        if interner.lookup_by_path_buf(&resolved).is_some() {
            continue;
        }

        let contents = fs::read_to_string(&resolved).map_err(|_| err())?;
        let (dialect, api_context) = match document::dialect_and_api_context_for_path(&resolved) {
            Some(res) => res,
            None => return Err(err()),
        };

        let file_id = interner.intern_path(resolved);
        original_paths.insert(file_id, path);
        change.create_file(file_id, dialect, api_context, contents);
        file_ids.push(file_id);
    }

    analysis.apply_change(change);

    let snap = analysis.snapshot();
    let mut rendered_diagnostics = String::new();
    let mut has_error = false;

    for file_id in file_ids.into_iter() {
        let line_index = snap.line_index(file_id).unwrap().unwrap();

        for diagnostic in snap.diagnostics(file_id)? {
            let start = line_index.line_col(diagnostic.range.range.start());
            writeln!(
                &mut rendered_diagnostics,
                "{}:{}:{} - {}: {}",
                original_paths.get(&file_id).unwrap(),
                start.line + 1,
                start.col + 1,
                match diagnostic.severity {
                    Severity::Warning => "warn",
                    Severity::Error => {
                        has_error = true;
                        "error"
                    }
                },
                diagnostic.message,
            )?;
        }
    }

    print!("{}", rendered_diagnostics);

    if has_error {
        process::exit(1);
    }

    Ok(())
}
