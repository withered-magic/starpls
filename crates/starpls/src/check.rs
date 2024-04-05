use crate::document::{DefaultFileLoader, PathInterner};
use starpls_common::{Dialect, Severity};
use starpls_ide::{Analysis, Change};
use std::{fmt::Write, fs, path::PathBuf, sync::Arc};

pub(crate) fn run_check(paths: &[String], output_base: &str) -> anyhow::Result<()> {
    let interner = Arc::new(PathInterner::default());
    let loader = DefaultFileLoader::new(interner.clone(), PathBuf::from(output_base));
    let mut analysis = Analysis::new(Arc::new(loader));
    let mut change = Change::default();
    let mut file_ids = Vec::new();

    for path in paths {
        let path = PathBuf::from(path);
        if interner.lookup_by_path_buf(&path).is_some() {
            continue;
        }
        let contents = fs::read_to_string(&path)?;
        let dialect = match path.extension().and_then(|ext| ext.to_str()) {
            _ => Dialect::Standard,
        };
        let file_id = interner.intern_path(path);
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
                interner.lookup_by_file_id(file_id).display(),
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
