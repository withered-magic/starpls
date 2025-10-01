use std::path::Path;
use std::sync::Arc;

use starpls_bazel::{APIContext, Builtins};

use crate::{BuiltinProvider, DialectDetector, DialectId, DialectInfo, ExtensibleDialect};

/// Detector for standard Starlark files (non-Bazel).
pub struct StandardDialectDetector;

impl DialectDetector for StandardDialectDetector {
    fn detect(&self, _workspace_path: &Path, file_path: &Path) -> Option<DialectInfo> {
        // Standard Starlark files are detected by .star extension
        // or any file that doesn't match other dialect patterns
        match file_path.extension()?.to_str()? {
            "star" | "starlark" => Some(DialectInfo {
                dialect_id: crate::dialect::builtin_dialects::standard(),
                api_context: None,
            }),
            _ => None, // Let other detectors handle it first
        }
    }

    fn priority(&self) -> u32 {
        0 // Lowest priority - fallback for unrecognized files
    }
}

/// Provider for standard Starlark builtin definitions.
pub struct StandardBuiltinProvider;

impl BuiltinProvider for StandardBuiltinProvider {
    fn load_builtins(&self, _api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        // Standard Starlark has minimal builtins
        // This could be populated with core Starlark functions
        Ok(Builtins::default())
    }

    fn load_rules(&self, _api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        // Standard Starlark doesn't have rules
        Ok(Builtins::default())
    }

    fn supported_contexts(&self) -> Vec<APIContext> {
        // Standard Starlark doesn't use API contexts
        vec![]
    }
}

/// Create a complete standard Starlark dialect definition.
pub fn create_standard_dialect() -> ExtensibleDialect {
    ExtensibleDialect::new(
        crate::dialect::builtin_dialects::standard(),
        "Standard Starlark".to_string(),
        "Standard Starlark configuration language".to_string(),
        Arc::new(StandardDialectDetector),
        Arc::new(StandardBuiltinProvider),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_standard_detector() {
        let detector = StandardDialectDetector;
        let workspace = PathBuf::from("/workspace");

        // Test .star file
        let star_file = PathBuf::from("/workspace/config.star");
        let info = detector.detect(&workspace, &star_file).unwrap();
        assert_eq!(info.dialect_id, crate::dialect::builtin_dialects::standard());
        assert_eq!(info.api_context, None);

        // Test .starlark file
        let starlark_file = PathBuf::from("/workspace/config.starlark");
        let info = detector.detect(&workspace, &starlark_file).unwrap();
        assert_eq!(info.dialect_id, crate::dialect::builtin_dialects::standard());

        // Test non-standard file
        let other_file = PathBuf::from("/workspace/BUILD");
        assert!(detector.detect(&workspace, &other_file).is_none());
    }

    #[test]
    fn test_standard_builtin_provider() {
        let provider = StandardBuiltinProvider;

        // Test loading builtins
        let builtins = provider.load_builtins(None).unwrap();
        // Standard dialect starts with empty builtins
        assert!(builtins.global.is_empty());

        // Test supported contexts
        let contexts = provider.supported_contexts();
        assert!(contexts.is_empty());
    }
}