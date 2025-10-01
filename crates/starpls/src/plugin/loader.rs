/*!
JSON Plugin File Loading

This module handles loading and parsing JSON plugin files from disk.
It provides functions to load both dialect plugins and symbol extensions.
*/

use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::plugin::schema::DialectPlugin;
use crate::plugin::schema::SymbolExtension;

/// Load a dialect plugin from a JSON file.
///
/// # Arguments
/// * `path` - Path to the JSON file containing a dialect plugin
///
/// # Returns
/// * `Ok(DialectPlugin)` - Successfully loaded and parsed plugin
/// * `Err` - File I/O error or JSON parsing error
///
/// # Example
/// ```rust
/// use crate::plugin::loader::load_dialect_plugin;
///
/// let plugin = load_dialect_plugin("tilt-dialect.json")?;
/// println!("Loaded dialect: {}", plugin.dialect.name);
/// ```
pub fn load_dialect_plugin(path: impl AsRef<Path>) -> Result<DialectPlugin> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read dialect plugin file: {}", path.display()))?;

    let plugin: DialectPlugin = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse dialect plugin JSON: {}", path.display()))?;

    // Basic validation
    validate_dialect_plugin(&plugin)
        .with_context(|| format!("Invalid dialect plugin: {}", path.display()))?;

    Ok(plugin)
}

/// Load a symbol extension from a JSON file.
///
/// # Arguments
/// * `path` - Path to the JSON file containing a symbol extension
///
/// # Returns
/// * `Ok(SymbolExtension)` - Successfully loaded and parsed extension
/// * `Err` - File I/O error or JSON parsing error
///
/// # Example
/// ```rust
/// use crate::plugin::loader::load_symbol_extension;
///
/// let extension = load_symbol_extension("bazel-extensions.json")?;
/// println!("Extending dialect: {}", extension.dialect_id);
/// ```
pub fn load_symbol_extension(path: impl AsRef<Path>) -> Result<SymbolExtension> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read symbol extension file: {}", path.display()))?;

    let extension: SymbolExtension = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse symbol extension JSON: {}", path.display()))?;

    // Basic validation
    validate_symbol_extension(&extension)
        .with_context(|| format!("Invalid symbol extension: {}", path.display()))?;

    Ok(extension)
}

/// Validate a dialect plugin for basic consistency.
fn validate_dialect_plugin(plugin: &DialectPlugin) -> Result<()> {
    let dialect = &plugin.dialect;

    // Check that ID is not empty and contains only valid characters
    if dialect.id.is_empty() {
        anyhow::bail!("Dialect ID cannot be empty");
    }

    if !is_valid_identifier(&dialect.id) {
        anyhow::bail!(
            "Dialect ID '{}' contains invalid characters. Use only letters, numbers, and hyphens.",
            dialect.id
        );
    }

    // Check that name is not empty
    if dialect.name.trim().is_empty() {
        anyhow::bail!("Dialect name cannot be empty");
    }

    // Check that we have at least one file pattern
    if dialect.file_patterns.is_empty() {
        anyhow::bail!("Dialect must specify at least one file pattern");
    }

    // Validate file patterns
    for pattern in &dialect.file_patterns {
        if pattern.trim().is_empty() {
            anyhow::bail!("File patterns cannot be empty");
        }
    }

    // Validate symbols
    for symbol in &plugin.symbols {
        validate_symbol_definition(symbol)?;
    }

    Ok(())
}

/// Validate a symbol extension for basic consistency.
fn validate_symbol_extension(extension: &SymbolExtension) -> Result<()> {
    // Check that dialect_id is not empty and valid
    if extension.dialect_id.is_empty() {
        anyhow::bail!("Dialect ID cannot be empty");
    }

    if !is_valid_identifier(&extension.dialect_id) {
        anyhow::bail!(
            "Dialect ID '{}' contains invalid characters",
            extension.dialect_id
        );
    }

    // Check that we have at least one symbol
    if extension.symbols.is_empty() {
        anyhow::bail!("Symbol extension must define at least one symbol");
    }

    // Validate each symbol
    for symbol in &extension.symbols {
        validate_symbol_definition(symbol)?;
    }

    Ok(())
}

/// Validate a symbol definition.
fn validate_symbol_definition(symbol: &crate::plugin::schema::SymbolDefinition) -> Result<()> {
    // Check that name is not empty and is a valid identifier
    if symbol.name.is_empty() {
        anyhow::bail!("Symbol name cannot be empty");
    }

    if !is_valid_starlark_identifier(&symbol.name) {
        anyhow::bail!(
            "Symbol name '{}' is not a valid Starlark identifier",
            symbol.name
        );
    }

    // If it's a function, validate the callable definition
    if let Some(callable) = &symbol.callable {
        // Validate parameters
        for param in &callable.params {
            if param.name.is_empty() {
                anyhow::bail!(
                    "Parameter name cannot be empty for symbol '{}'",
                    symbol.name
                );
            }

            if !is_valid_starlark_identifier(&param.name) {
                anyhow::bail!(
                    "Parameter name '{}' is not a valid Starlark identifier",
                    param.name
                );
            }

            if param.param_type.trim().is_empty() {
                anyhow::bail!(
                    "Parameter type cannot be empty for parameter '{}' of symbol '{}'",
                    param.name,
                    symbol.name
                );
            }
        }

        // Check for duplicate parameter names
        let mut param_names = std::collections::HashSet::new();
        for param in &callable.params {
            if !param_names.insert(&param.name) {
                anyhow::bail!(
                    "Duplicate parameter name '{}' in symbol '{}'",
                    param.name,
                    symbol.name
                );
            }
        }
    }

    Ok(())
}

/// Check if a string is a valid identifier (for dialect IDs).
/// Allows letters, numbers, hyphens, and underscores.
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Check if a string is a valid Starlark identifier.
/// More restrictive than general identifiers - follows Python identifier rules.
fn is_valid_starlark_identifier(s: &str) -> bool {
    !s.is_empty()
        && (s.chars().next().unwrap().is_alphabetic() || s.starts_with('_'))
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !is_starlark_keyword(s)
}

/// Check if a string is a reserved Starlark keyword.
fn is_starlark_keyword(s: &str) -> bool {
    matches!(
        s,
        "and" | "as" | "assert" | "break" | "class" | "continue" | "def" | "del" |
        "elif" | "else" | "except" | "finally" | "for" | "from" | "global" | "if" |
        "import" | "in" | "is" | "lambda" | "not" | "or" | "pass" | "raise" |
        "return" | "try" | "while" | "with" | "yield" |
        // Additional Starlark-specific keywords
        "load" | "True" | "False" | "None"
    )
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_load_valid_dialect_plugin() {
        let json_content = r#"
        {
            "dialect": {
                "id": "test-dialect",
                "name": "Test Dialect",
                "description": "A test dialect",
                "file_patterns": ["Testfile", "*.test"],
                "priority": 100
            },
            "symbols": [
                {
                    "name": "test_func",
                    "kind": "function",
                    "callable": {
                        "params": [
                            {
                                "name": "param1",
                                "type": "string",
                                "is_mandatory": true
                            }
                        ],
                        "return_type": "None"
                    },
                    "doc": "A test function"
                }
            ]
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let plugin = load_dialect_plugin(temp_file.path()).unwrap();
        assert_eq!(plugin.dialect.id, "test-dialect");
        assert_eq!(plugin.dialect.name, "Test Dialect");
        assert_eq!(plugin.symbols.len(), 1);
        assert_eq!(plugin.symbols[0].name, "test_func");
    }

    #[test]
    fn test_load_valid_symbol_extension() {
        let json_content = r#"
        {
            "dialect_id": "bazel",
            "context": "build",
            "symbols": [
                {
                    "name": "my_rule",
                    "kind": "function",
                    "doc": "My custom rule"
                }
            ]
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let extension = load_symbol_extension(temp_file.path()).unwrap();
        assert_eq!(extension.dialect_id, "bazel");
        assert_eq!(extension.context, Some("build".to_string()));
        assert_eq!(extension.symbols.len(), 1);
        assert_eq!(extension.symbols[0].name, "my_rule");
    }

    #[test]
    fn test_invalid_dialect_id() {
        let json_content = r#"
        {
            "dialect": {
                "id": "",
                "name": "Test",
                "description": "Test",
                "file_patterns": ["test"]
            }
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let result = load_dialect_plugin(temp_file.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("ID cannot be empty"));
    }

    #[test]
    fn test_invalid_symbol_name() {
        let json_content = r#"
        {
            "dialect_id": "test",
            "symbols": [
                {
                    "name": "123invalid",
                    "kind": "function"
                }
            ]
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let result = load_symbol_extension(temp_file.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a valid Starlark identifier"));
    }

    #[test]
    fn test_starlark_keyword_validation() {
        assert!(is_starlark_keyword("def"));
        assert!(is_starlark_keyword("load"));
        assert!(is_starlark_keyword("True"));
        assert!(!is_starlark_keyword("my_function"));
    }

    #[test]
    fn test_identifier_validation() {
        assert!(is_valid_identifier("test-dialect"));
        assert!(is_valid_identifier("test_dialect"));
        assert!(is_valid_identifier("test123"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("test dialect")); // spaces not allowed
        assert!(!is_valid_identifier("test.dialect")); // dots not allowed

        assert!(is_valid_starlark_identifier("test_func"));
        assert!(is_valid_starlark_identifier("_private"));
        assert!(!is_valid_starlark_identifier("123test")); // can't start with number
        assert!(!is_valid_starlark_identifier("def")); // keyword
    }
}
