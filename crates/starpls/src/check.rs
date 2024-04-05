use crate::document::{DefaultFileLoader, PathInterner};
use anyhow::anyhow;
use rustc_hash::FxHashMap;
use starpls_common::{Dialect, Severity};
use starpls_ide::{Analysis, Change};
use std::{fmt::Write, fs, path::PathBuf, sync::Arc};

pub(crate) fn run_check(paths: &[String], output_base: &str) -> anyhow::Result<()> {
    let interner = Arc::new(PathInterner::default());
    let loader = DefaultFileLoader::new(interner.clone(), PathBuf::from(output_base));
    let mut analysis = Analysis::new(Arc::new(loader));
    let mut change = Change::default();
    let mut file_ids = Vec::new();
    let mut original_paths = FxHashMap::default();

    for path in paths {
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
