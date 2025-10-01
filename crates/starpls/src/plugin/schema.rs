/*!
JSON Schema definitions for starpls plugins.

This module defines the structures that represent JSON plugin files.
All structures use serde for serialization/deserialization.
*/

use serde::Deserialize;
use serde::Serialize;

/// A complete dialect plugin that defines a new Starlark dialect.
///
/// Used with `--load-dialect` flag.
///
/// # Example JSON
/// ```json
/// {
///   "dialect": {
///     "id": "tilt",
///     "name": "Tilt",
///     "description": "Tilt development environment",
///     "file_patterns": ["Tiltfile", "*.tiltfile"],
///     "priority": 150
///   },
///   "symbols": [
///     {
///       "name": "docker_build",
///       "kind": "function",
///       "callable": {
///         "params": [
///           {
///             "name": "ref",
///             "type": "string",
///             "doc": "Image reference",
///             "is_mandatory": true
///           }
///         ],
///         "return_type": "None"
///       },
///       "doc": "Builds a Docker image"
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialectPlugin {
    /// The dialect definition
    pub dialect: DialectDefinition,
    /// Symbols (functions, variables, types) provided by this dialect
    #[serde(default)]
    pub symbols: Vec<SymbolDefinition>,
}

/// Definition of a new Starlark dialect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialectDefinition {
    /// Unique identifier for the dialect (e.g., "tilt", "buck2")
    pub id: String,
    /// Human-readable name (e.g., "Tilt", "Buck2")
    pub name: String,
    /// Description of the dialect
    pub description: String,
    /// File patterns that indicate this dialect (e.g., ["Tiltfile", "*.tiltfile"])
    pub file_patterns: Vec<String>,
    /// Detection priority (higher = checked first)
    #[serde(default = "default_priority")]
    pub priority: u32,
}

/// Symbol extension that adds symbols to an existing dialect.
///
/// Used with `--load-symbols` flag.
///
/// # Example JSON
/// ```json
/// {
///   "dialect_id": "bazel",
///   "context": "build",
///   "symbols": [
///     {
///       "name": "company_library",
///       "kind": "function",
///       "callable": {
///         "params": [
///           {
///             "name": "name",
///             "type": "string",
///             "is_mandatory": true
///           }
///         ],
///         "return_type": "None"
///       },
///       "doc": "Company-specific library rule"
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolExtension {
    /// ID of the dialect to extend (e.g., "bazel", "standard")
    pub dialect_id: String,
    /// Optional context within the dialect (e.g., "build", "bzl")
    pub context: Option<String>,
    /// Symbols to add to the dialect
    pub symbols: Vec<SymbolDefinition>,
}

/// Definition of a symbol (function, variable, type, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolDefinition {
    /// Name of the symbol
    pub name: String,
    /// Kind of symbol ("function", "variable", "type", etc.)
    pub kind: SymbolKind,
    /// If this is a callable (function), its signature
    pub callable: Option<CallableDefinition>,
    /// If this is a value, its type (optional)
    pub value_type: Option<String>,
    /// Documentation string
    #[serde(default)]
    pub doc: String,
}

/// Kind of symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    /// A function or method
    Function,
    /// A variable or constant
    Variable,
    /// A type or class
    Type,
    /// A module or namespace
    Module,
}

/// Definition of a callable symbol (function signature).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallableDefinition {
    /// Function parameters
    #[serde(default)]
    pub params: Vec<ParamDefinition>,
    /// Return type
    #[serde(default = "default_return_type")]
    pub return_type: String,
}

/// Definition of a function parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDefinition {
    /// Parameter name
    pub name: String,
    /// Parameter type
    #[serde(rename = "type")]
    pub param_type: String,
    /// Parameter documentation
    #[serde(default)]
    pub doc: String,
    /// Default value (if any)
    pub default_value: Option<String>,
    /// Whether this parameter is mandatory
    #[serde(default)]
    pub is_mandatory: bool,
    /// Whether this is a *args parameter (not yet supported)
    #[serde(default)]
    pub is_star_arg: bool,
    /// Whether this is a **kwargs parameter (not yet supported)
    #[serde(default)]
    pub is_star_star_arg: bool,
}

/// Default priority for dialect detection.
fn default_priority() -> u32 {
    50
}

/// Default return type for functions.
fn default_return_type() -> String {
    "None".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialect_plugin_serialization() {
        let plugin = DialectPlugin {
            dialect: DialectDefinition {
                id: "test".to_string(),
                name: "Test Dialect".to_string(),
                description: "A test dialect".to_string(),
                file_patterns: vec!["Testfile".to_string(), "*.test".to_string()],
                priority: 100,
            },
            symbols: vec![SymbolDefinition {
                name: "test_function".to_string(),
                kind: SymbolKind::Function,
                callable: Some(CallableDefinition {
                    params: vec![ParamDefinition {
                        name: "name".to_string(),
                        param_type: "string".to_string(),
                        doc: "The name parameter".to_string(),
                        default_value: None,
                        is_mandatory: true,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    }],
                    return_type: "None".to_string(),
                }),
                value_type: None,
                doc: "A test function".to_string(),
            }],
        };

        // Test serialization
        let json = serde_json::to_string_pretty(&plugin).unwrap();
        println!("Serialized plugin:\n{}", json);

        // Test deserialization
        let deserialized: DialectPlugin = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.dialect.id, "test");
        assert_eq!(deserialized.symbols.len(), 1);
        assert_eq!(deserialized.symbols[0].name, "test_function");
    }

    #[test]
    fn test_symbol_extension_serialization() {
        let extension = SymbolExtension {
            dialect_id: "bazel".to_string(),
            context: Some("build".to_string()),
            symbols: vec![SymbolDefinition {
                name: "my_rule".to_string(),
                kind: SymbolKind::Function,
                callable: Some(CallableDefinition {
                    params: vec![],
                    return_type: "None".to_string(),
                }),
                value_type: None,
                doc: "My custom rule".to_string(),
            }],
        };

        // Test serialization
        let json = serde_json::to_string_pretty(&extension).unwrap();
        println!("Serialized extension:\n{}", json);

        // Test deserialization
        let deserialized: SymbolExtension = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.dialect_id, "bazel");
        assert_eq!(deserialized.context, Some("build".to_string()));
        assert_eq!(deserialized.symbols.len(), 1);
    }

    #[test]
    fn test_minimal_dialect_plugin() {
        let json = r#"
        {
            "dialect": {
                "id": "minimal",
                "name": "Minimal",
                "description": "A minimal dialect",
                "file_patterns": ["Minimal"]
            }
        }
        "#;

        let plugin: DialectPlugin = serde_json::from_str(json).unwrap();
        assert_eq!(plugin.dialect.id, "minimal");
        assert_eq!(plugin.dialect.priority, 50); // default
        assert!(plugin.symbols.is_empty()); // default
    }

    #[test]
    fn test_minimal_symbol_extension() {
        let json = r#"
        {
            "dialect_id": "test",
            "symbols": [
                {
                    "name": "simple_var",
                    "kind": "variable",
                    "doc": "A simple variable"
                }
            ]
        }
        "#;

        let extension: SymbolExtension = serde_json::from_str(json).unwrap();
        assert_eq!(extension.dialect_id, "test");
        assert_eq!(extension.context, None);
        assert_eq!(extension.symbols.len(), 1);
        assert_eq!(extension.symbols[0].name, "simple_var");
    }
}
