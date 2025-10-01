use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use starpls_bazel::{Builtins, APIContext};

/// A unique identifier for a Starlark dialect.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DialectId(pub String);

impl DialectId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Built-in dialect IDs for backward compatibility.
pub mod builtin_dialects {
    use super::DialectId;

    pub fn standard() -> DialectId {
        DialectId::new("standard")
    }

    pub fn bazel() -> DialectId {
        DialectId::new("bazel")
    }
}

/// Information about a detected dialect and its context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialectInfo {
    pub dialect_id: DialectId,
    pub api_context: Option<APIContext>,
}

/// Trait for detecting dialect from file paths.
pub trait DialectDetector: Send + Sync {
    /// Detect dialect and API context for a given file path.
    /// Returns None if this detector doesn't recognize the file.
    fn detect(&self, workspace_path: &Path, file_path: &Path) -> Option<DialectInfo>;

    /// Priority for this detector (higher = checked first).
    fn priority(&self) -> u32 {
        0
    }
}

/// Trait for providing builtin definitions for a dialect.
pub trait BuiltinProvider: Send + Sync {
    /// Load builtin definitions for the given API context.
    fn load_builtins(&self, api_context: Option<APIContext>) -> anyhow::Result<Builtins>;

    /// Load rule definitions for the given API context.
    fn load_rules(&self, api_context: Option<APIContext>) -> anyhow::Result<Builtins>;

    /// Get supported API contexts for this dialect.
    fn supported_contexts(&self) -> Vec<APIContext>;
}

/// A complete dialect definition.
#[derive(Clone)]
pub struct Dialect {
    pub id: DialectId,
    pub name: String,
    pub description: String,
    pub detector: Arc<dyn DialectDetector>,
    pub builtin_provider: Arc<dyn BuiltinProvider>,
}

impl Dialect {
    pub fn new(
        id: DialectId,
        name: String,
        description: String,
        detector: Arc<dyn DialectDetector>,
        builtin_provider: Arc<dyn BuiltinProvider>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            detector,
            builtin_provider,
        }
    }
}

/// Registry for managing multiple dialects.
pub struct DialectRegistry {
    dialects: HashMap<DialectId, Dialect>,
    detectors: Vec<Arc<dyn DialectDetector>>,
}

impl DialectRegistry {
    pub fn new() -> Self {
        Self {
            dialects: HashMap::new(),
            detectors: Vec::new(),
        }
    }

    /// Register a new dialect.
    pub fn register(&mut self, dialect: Dialect) {
        self.detectors.push(dialect.detector.clone());
        // Sort detectors by priority (highest first)
        self.detectors.sort_by_key(|d| std::cmp::Reverse(d.priority()));
        self.dialects.insert(dialect.id.clone(), dialect);
    }

    /// Get a dialect by ID.
    pub fn get(&self, id: &DialectId) -> Option<&Dialect> {
        self.dialects.get(id)
    }

    /// Detect dialect for a given file path.
    pub fn detect(&self, workspace_path: &Path, file_path: &Path) -> Option<DialectInfo> {
        for detector in &self.detectors {
            if let Some(info) = detector.detect(workspace_path, file_path) {
                // Verify the detected dialect is actually registered
                if self.dialects.contains_key(&info.dialect_id) {
                    return Some(info);
                }
            }
        }
        None
    }

    /// List all registered dialects.
    pub fn list(&self) -> impl Iterator<Item = &Dialect> {
        self.dialects.values()
    }

    /// Get builtin provider for a dialect.
    pub fn builtin_provider(&self, id: &DialectId) -> Option<&Arc<dyn BuiltinProvider>> {
        self.dialects.get(id).map(|d| &d.builtin_provider)
    }
}

impl Default for DialectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestDetector {
        extension: String,
        dialect_id: DialectId,
    }

    impl DialectDetector for TestDetector {
        fn detect(&self, _workspace: &Path, file_path: &Path) -> Option<DialectInfo> {
            if file_path.extension()?.to_str()? == self.extension {
                Some(DialectInfo {
                    dialect_id: self.dialect_id.clone(),
                    api_context: None,
                })
            } else {
                None
            }
        }
    }

    struct TestBuiltinProvider;

    impl BuiltinProvider for TestBuiltinProvider {
        fn load_builtins(&self, _context: Option<APIContext>) -> anyhow::Result<Builtins> {
            Ok(Builtins::default())
        }

        fn load_rules(&self, _context: Option<APIContext>) -> anyhow::Result<Builtins> {
            Ok(Builtins::default())
        }

        fn supported_contexts(&self) -> Vec<APIContext> {
            vec![]
        }
    }

    #[test]
    fn test_dialect_registry() {
        let mut registry = DialectRegistry::new();

        let dialect = Dialect::new(
            DialectId::new("test"),
            "Test Dialect".to_string(),
            "A test dialect".to_string(),
            Arc::new(TestDetector {
                extension: "test".to_string(),
                dialect_id: DialectId::new("test"),
            }),
            Arc::new(TestBuiltinProvider),
        );

        registry.register(dialect);

        // Test detection
        let workspace = PathBuf::from("/workspace");
        let test_file = PathBuf::from("/workspace/file.test");
        let info = registry.detect(&workspace, &test_file).unwrap();
        assert_eq!(info.dialect_id, DialectId::new("test"));

        // Test non-matching file
        let other_file = PathBuf::from("/workspace/file.other");
        assert!(registry.detect(&workspace, &other_file).is_none());
    }
}