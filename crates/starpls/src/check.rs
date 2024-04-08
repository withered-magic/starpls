use crate::{
    document::{DefaultFileLoader, PathInterner},
    server::{load_bazel_build_language, load_bazel_builtins},
};
use anyhow::anyhow;
use rustc_hash::FxHashMap;
use starpls_bazel::client::{BazelCLI, BazelClient};
use starpls_common::{Dialect, Severity};
use starpls_ide::{Analysis, Change};
use std::{fmt::Write, fs, path::PathBuf, sync::Arc};

pub(crate) fn run_check(paths: Vec<String>, output_base: Option<String>) -> anyhow::Result<()> {
    let bazel_client = BazelCLI::default();
    let output_base = match output_base {
        Some(output_base) => PathBuf::from(output_base),
        None => bazel_client
            .output_base()
            .map_err(|_| anyhow!("Failed to determine Bazel output base."))?,
    };

    let builtins = load_bazel_builtins()?;
    let rules = load_bazel_build_language(&bazel_client)?;
    let interner = Arc::new(PathInterner::default());
    let loader = DefaultFileLoader::new(interner.clone(), PathBuf::from(output_base));
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
        let dialect = match resolved.extension().and_then(|ext| ext.to_str()) {
            Some("bzl" | "bazel") => Dialect::Bazel,
            Some("sky" | "star") => Dialect::Standard,
            None if matches!(
                resolved.file_name().and_then(|name| name.to_str()),
                Some("WORKSPACE | BUILD")
            ) =>
            {
                Dialect::Bazel
            }
            _ => return Err(err()),
        };

        let file_id = interner.intern_path(resolved);
        original_paths.insert(file_id, path);
        change.create_file(file_id, dialect, contents);
        file_ids.push(file_id);
    }

    analysis.apply_change(change);

    let mut rendered_diagnostics = String::new();
    let snap = analysis.snapshot();
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
                    Severity::Error => "error",
                },
                diagnostic.message,
            )?;
        }
    }

    print!("{}", rendered_diagnostics);

    Ok(())
}
