/*!
# Example: Setting up an extensible dialect system

This module demonstrates how to use the new extensible dialect system to support multiple Starlark dialects including Bazel, Tilt, and standard Starlark.

## Usage Example

```rust
use starpls_common::{DialectRegistry, create_standard_dialect, create_tilt_dialect};
use starpls_bazel::create_bazel_dialect;
use std::path::Path;

// Create a dialect registry
let mut registry = DialectRegistry::new();

// Register built-in dialects
registry.register(create_standard_dialect());
registry.register(create_bazel_dialect());
registry.register(create_tilt_dialect());

// Detect dialect for different files
let workspace = Path::new("/workspace");

// Bazel files
let build_file = Path::new("/workspace/BUILD");
if let Some(info) = registry.detect(&workspace, &build_file) {
    println!("Detected dialect: {:?}", info.dialect_id);
    println!("API context: {:?}", info.api_context);
}

// Tilt files
let tiltfile = Path::new("/workspace/Tiltfile");
if let Some(info) = registry.detect(&workspace, &tiltfile) {
    println!("Detected dialect: {:?}", info.dialect_id);
}

// Standard Starlark files
let star_file = Path::new("/workspace/config.star");
if let Some(info) = registry.detect(&workspace, &star_file) {
    println!("Detected dialect: {:?}", info.dialect_id);
}
```

## Creating Custom Dialects

To create a new dialect, implement the `DialectDetector` and `BuiltinProvider` traits:

```rust
use starpls_common::{DialectDetector, BuiltinProvider, DialectId, DialectInfo, ExtensibleDialect};
use std::path::Path;
use std::sync::Arc;

struct MyCustomDetector;

impl DialectDetector for MyCustomDetector {
    fn detect(&self, _workspace: &Path, file_path: &Path) -> Option<DialectInfo> {
        if file_path.extension()?.to_str()? == "mystarlark" {
            Some(DialectInfo {
                dialect_id: DialectId::new("my-custom"),
                api_context: None,
            })
        } else {
            None
        }
    }

    fn priority(&self) -> u32 { 50 }
}

struct MyCustomProvider;

impl BuiltinProvider for MyCustomProvider {
    fn load_builtins(&self, _context: Option<starpls_bazel::APIContext>) -> anyhow::Result<starpls_bazel::Builtins> {
        // Load your custom builtin definitions here
        Ok(starpls_bazel::Builtins::default())
    }

    fn load_rules(&self, _context: Option<starpls_bazel::APIContext>) -> anyhow::Result<starpls_bazel::Builtins> {
        Ok(starpls_bazel::Builtins::default())
    }

    fn supported_contexts(&self) -> Vec<starpls_bazel::APIContext> {
        vec![]
    }
}

// Create and register the custom dialect
fn create_my_custom_dialect() -> ExtensibleDialect {
    ExtensibleDialect::new(
        DialectId::new("my-custom"),
        "My Custom Starlark".to_string(),
        "A custom Starlark dialect for my specific use case".to_string(),
        Arc::new(MyCustomDetector),
        Arc::new(MyCustomProvider),
    )
}

// Register it with the system
let mut registry = DialectRegistry::new();
registry.register(create_my_custom_dialect());
```

## Migration from Legacy System

The new system maintains backward compatibility with the old `Dialect` enum:

```rust
use starpls_common::{Dialect, DialectId};

// Convert between old and new systems
let old_dialect = Dialect::Bazel;
let new_id: DialectId = old_dialect.to_dialect_id();

// Convert back (if it's a known dialect)
if let Some(old_dialect) = Dialect::from_dialect_id(&new_id) {
    println!("Converted back to: {:?}", old_dialect);
}
```

## Database Integration

The database layer supports both old and new dialect systems:

```rust
use starpls_hir::Db;
use starpls_common::DialectId;

// Legacy method (still works)
// db.set_builtin_defs(Dialect::Bazel, builtins, rules);

// New method with extensible dialects
let dialect_id = DialectId::new("tilt");
let builtin_defs = db.get_builtin_defs_by_id(&dialect_id, None);
```
*/

#[cfg(test)]
mod integration_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::create_standard_dialect;
    use crate::create_tilt_dialect;
    use crate::DialectRegistry;

    #[test]
    fn test_full_integration() {
        let mut registry = DialectRegistry::new();

        // Register all built-in dialects
        registry.register(create_standard_dialect());
        registry.register(starpls_bazel::create_bazel_dialect());
        registry.register(create_tilt_dialect());

        let workspace = PathBuf::from("/workspace");

        // Test Bazel detection
        let build_file = PathBuf::from("/workspace/BUILD");
        let info = registry.detect(&workspace, &build_file).unwrap();
        assert_eq!(info.dialect_id.as_str(), "bazel");
        assert_eq!(info.api_context, Some(starpls_bazel::APIContext::Build));

        // Test Tilt detection
        let tiltfile = PathBuf::from("/workspace/Tiltfile");
        let info = registry.detect(&workspace, &tiltfile).unwrap();
        assert_eq!(info.dialect_id.as_str(), "tilt");
        assert_eq!(info.api_context, None);

        // Test standard Starlark detection
        let star_file = PathBuf::from("/workspace/config.star");
        let info = registry.detect(&workspace, &star_file).unwrap();
        assert_eq!(info.dialect_id.as_str(), "standard");
        assert_eq!(info.api_context, None);

        // Test builtin providers
        let tilt_provider = registry
            .builtin_provider(&crate::DialectId::new("tilt"))
            .unwrap();
        let builtins = tilt_provider.load_builtins(None).unwrap();
        assert!(!builtins.global.is_empty());
    }

    #[test]
    fn test_priority_ordering() {
        let mut registry = DialectRegistry::new();

        // Register dialects in reverse priority order to test sorting
        registry.register(create_standard_dialect()); // Priority 0
        registry.register(starpls_bazel::create_bazel_dialect()); // Priority 100
        registry.register(create_tilt_dialect()); // Priority 150

        let workspace = PathBuf::from("/workspace");

        // Test that Tiltfile is detected as Tilt, not falling through to standard
        let tiltfile = PathBuf::from("/workspace/Tiltfile");
        let info = registry.detect(&workspace, &tiltfile).unwrap();
        assert_eq!(info.dialect_id.as_str(), "tilt");
    }

    #[test]
    fn test_backward_compatibility() {
        use crate::Dialect;
        use crate::DialectId;

        // Test conversion from old to new
        let old_dialect = Dialect::Bazel;
        let new_id = old_dialect.to_dialect_id();
        assert_eq!(new_id.as_str(), "bazel");

        // Test conversion from new to old
        let tilt_id = DialectId::new("tilt");
        assert_eq!(Dialect::from_dialect_id(&tilt_id), None); // Tilt is not a legacy dialect

        let bazel_id = DialectId::new("bazel");
        assert_eq!(Dialect::from_dialect_id(&bazel_id), Some(Dialect::Bazel));
    }
}
