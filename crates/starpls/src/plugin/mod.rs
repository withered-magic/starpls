/*!
JSON Plugin System for starpls

This module provides a simple JSON-based plugin system that allows users to:
1. Define new Starlark dialects (--load-dialect)
2. Extend existing dialects with additional symbols (--load-symbols)

## Usage

```bash
# Load a new dialect (e.g., Tilt support)
starpls server --load-dialect tilt-dialect.json

# Extend existing Bazel dialect with company-specific rules
starpls server --load-symbols company-bazel-rules.json

# Combine both
starpls server --load-dialect tilt.json --load-symbols bazel-extensions.json
```

## JSON Format

### Dialect Plugin (--load-dialect)
```json
{
  "dialect": {
    "id": "tilt",
    "name": "Tilt",
    "description": "Tilt development environment",
    "file_patterns": ["Tiltfile", "*.tiltfile"],
    "priority": 150
  },
  "symbols": [...]
}
```

### Symbol Extension (--load-symbols)
```json
{
  "dialect_id": "bazel",
  "context": "build",
  "symbols": [...]
}
```
*/

use std::path::Path;

use anyhow::Result;
use starpls_common::DialectRegistry;
use starpls_common::ExtensibleDialect;

pub mod loader;
pub mod schema;

use crate::plugin::loader::load_dialect_plugin;
use crate::plugin::loader::load_symbol_extension;
use crate::plugin::schema::DialectPlugin;
use crate::plugin::schema::SymbolExtension;

/// Load dialect plugins from the specified files and register them with the registry.
pub fn load_dialect_plugins(
    registry: &mut DialectRegistry,
    dialect_files: &[impl AsRef<Path>],
) -> Result<()> {
    if dialect_files.is_empty() {
        log::debug!("No dialect plugins specified");
        return Ok(());
    }

    log::info!("Loading {} dialect plugin(s)...", dialect_files.len());
    let mut loaded_count = 0;
    let mut failed_count = 0;

    for file_path in dialect_files {
        let file_path = file_path.as_ref();
        log::debug!("Processing dialect plugin: {}", file_path.display());

        match load_dialect_plugin(file_path) {
            Ok(plugin) => {
                let dialect_name = plugin.dialect.name.clone();
                let dialect_id = plugin.dialect.id.clone();
                let symbol_count = plugin.symbols.len();

                match create_dialect_from_plugin(plugin) {
                    Ok(dialect) => {
                        registry.register(dialect);
                        loaded_count += 1;
                        log::info!(
                            "✓ Loaded dialect '{}' (id: {}) with {} symbol(s) from {}",
                            dialect_name,
                            dialect_id,
                            symbol_count,
                            file_path.display()
                        );
                    }
                    Err(e) => {
                        failed_count += 1;
                        log::error!(
                            "✗ Failed to register dialect from plugin {}: {}",
                            file_path.display(),
                            e
                        );
                    }
                }
            }
            Err(e) => {
                failed_count += 1;
                log::error!(
                    "✗ Failed to load dialect plugin {}: {}",
                    file_path.display(),
                    e
                );
            }
        }
    }

    // Log summary
    if loaded_count > 0 || failed_count > 0 {
        if failed_count == 0 {
            log::info!(
                "Plugin loading complete: {} dialect(s) loaded successfully",
                loaded_count
            );
        } else {
            log::warn!(
                "Plugin loading complete: {} succeeded, {} failed. Use RUST_LOG=debug for detailed error information.",
                loaded_count, failed_count
            );
        }
    }

    Ok(())
}

/// Load symbol extensions from the specified files.
/// Returns a Vec of symbol extensions that can be applied to existing dialects.
pub fn load_symbol_extensions(symbol_files: &[impl AsRef<Path>]) -> Result<Vec<SymbolExtension>> {
    let mut extensions = Vec::new();

    if symbol_files.is_empty() {
        log::debug!("No symbol extensions specified");
        return Ok(extensions);
    }

    log::info!("Loading {} symbol extension(s)...", symbol_files.len());
    let mut loaded_count = 0;
    let mut failed_count = 0;

    for file_path in symbol_files {
        let file_path = file_path.as_ref();
        log::debug!("Processing symbol extension: {}", file_path.display());

        match load_symbol_extension(file_path) {
            Ok(extension) => {
                let dialect_id = extension.dialect_id.clone();
                let symbol_count = extension.symbols.len();
                let context = extension
                    .context
                    .clone()
                    .unwrap_or_else(|| "default".to_string());

                extensions.push(extension);
                loaded_count += 1;
                log::info!(
                    "✓ Loaded {} symbol(s) for dialect '{}' (context: {}) from {}",
                    symbol_count,
                    dialect_id,
                    context,
                    file_path.display()
                );
            }
            Err(e) => {
                failed_count += 1;
                log::error!(
                    "✗ Failed to load symbol extension {}: {}",
                    file_path.display(),
                    e
                );
            }
        }
    }

    // Log summary
    if loaded_count > 0 || failed_count > 0 {
        if failed_count == 0 {
            log::info!(
                "Symbol extension loading complete: {} extension(s) loaded successfully",
                loaded_count
            );
        } else {
            log::warn!(
                "Symbol extension loading complete: {} succeeded, {} failed. Use RUST_LOG=debug for detailed error information.",
                loaded_count, failed_count
            );
        }
    }

    Ok(extensions)
}

/// Convert a DialectPlugin into an ExtensibleDialect.
fn create_dialect_from_plugin(plugin: DialectPlugin) -> Result<ExtensibleDialect> {
    // This is a simplified implementation. In a full implementation,
    // we would create proper DialectDetector and BuiltinProvider implementations
    // that use the plugin data.

    // For now, we'll create a placeholder that shows the concept
    // TODO: Implement JsonDialectDetector and JsonBuiltinProvider

    let dialect_id = starpls_common::DialectId::new(&plugin.dialect.id);

    // Create a simple detector that matches the file patterns
    let detector = std::sync::Arc::new(JsonDialectDetector {
        id: dialect_id.clone(),
        patterns: plugin.dialect.file_patterns.clone(),
        priority: plugin.dialect.priority,
    });

    // Create a builtin provider that serves the symbols from the plugin
    let provider = std::sync::Arc::new(JsonBuiltinProvider {
        symbols: plugin.symbols,
    });

    Ok(ExtensibleDialect::new(
        dialect_id,
        plugin.dialect.name,
        plugin.dialect.description,
        detector,
        provider,
    ))
}

/// Simple dialect detector that matches file patterns from JSON config.
struct JsonDialectDetector {
    id: starpls_common::DialectId,
    patterns: Vec<String>,
    priority: u32,
}

impl starpls_common::DialectDetector for JsonDialectDetector {
    fn detect(
        &self,
        _workspace_path: &Path,
        file_path: &Path,
    ) -> Option<starpls_common::DialectInfo> {
        let file_name = file_path.file_name()?.to_str()?;

        for pattern in &self.patterns {
            if pattern_matches(pattern, file_name) {
                return Some(starpls_common::DialectInfo {
                    dialect_id: self.id.clone(),
                    api_context: None, // JSON plugins don't specify API context yet
                });
            }
        }

        None
    }

    fn priority(&self) -> u32 {
        self.priority
    }
}

/// Simple builtin provider that serves symbols from JSON config.
struct JsonBuiltinProvider {
    symbols: Vec<crate::plugin::schema::SymbolDefinition>,
}

impl starpls_common::BuiltinProvider for JsonBuiltinProvider {
    fn load_builtins(
        &self,
        _api_context: Option<starpls_bazel::APIContext>,
    ) -> anyhow::Result<starpls_bazel::Builtins> {
        // Convert JSON symbols to Bazel builtin format
        let mut builtins = starpls_bazel::Builtins::default();

        for symbol in &self.symbols {
            if let Ok(value) = convert_symbol_to_builtin_value(symbol) {
                builtins.global.push(value);
            }
        }

        Ok(builtins)
    }

    fn load_rules(
        &self,
        _api_context: Option<starpls_bazel::APIContext>,
    ) -> anyhow::Result<starpls_bazel::Builtins> {
        // JSON plugins don't provide rules yet
        Ok(starpls_bazel::Builtins::default())
    }

    fn supported_contexts(&self) -> Vec<starpls_bazel::APIContext> {
        // JSON plugins don't use contexts yet
        vec![]
    }
}

/// Simple pattern matching for file patterns.
/// Supports basic wildcards like "*.ext" and exact matches.
fn pattern_matches(pattern: &str, file_name: &str) -> bool {
    if pattern == file_name {
        return true; // Exact match
    }

    if let Some(extension) = pattern.strip_prefix("*.") {
        return file_name.ends_with(&format!(".{}", extension));
    }

    // Could add more sophisticated pattern matching here
    false
}

/// Convert a JSON symbol definition to a Bazel builtin Value.
fn convert_symbol_to_builtin_value(
    symbol: &crate::plugin::schema::SymbolDefinition,
) -> Result<starpls_bazel::builtin::Value> {
    use starpls_bazel::builtin::Callable;
    use starpls_bazel::builtin::Param;
    use starpls_bazel::builtin::Value;

    let callable = symbol.callable.as_ref().map(|callable_def| Callable {
        param: callable_def
            .params
            .iter()
            .map(|p| Param {
                name: p.name.clone(),
                r#type: p.param_type.clone(),
                doc: p.doc.clone(),
                default_value: p.default_value.clone().unwrap_or_default(),
                is_mandatory: p.is_mandatory,
                is_star_arg: false,      // JSON plugins don't support this yet
                is_star_star_arg: false, // JSON plugins don't support this yet
            })
            .collect(),
        return_type: callable_def.return_type.clone(),
    });

    Ok(Value {
        name: symbol.name.clone(),
        r#type: symbol.value_type.clone().unwrap_or_else(|| {
            if callable.is_some() {
                "function".to_string()
            } else {
                "unknown".to_string()
            }
        }),
        callable,
        doc: symbol.doc.clone(),
        api_context: Default::default(), // JSON plugins don't specify this yet
    })
}
