/*!
Minimal extension system for adding symbols and virtual modules to Starlark dialects.

This module provides a simple way to extend existing dialects with:
1. Global symbols - Available without load() statements
2. Virtual modules - Can be loaded but don't exist on disk
3. Load prefix - Context-aware path resolution

Extensions are additive and applied based on file patterns.
*/

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Collection of extensions that can be applied to files.
#[derive(Debug, Clone, Default)]
pub struct Extensions {
    items: Vec<Extension>,
}

impl Extensions {
    /// Create a new empty Extensions collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an extension to the collection.
    pub fn add(&mut self, extension: Extension) {
        self.items.push(extension);
    }

    /// Get all extensions that match the given file path.
    pub fn matching(&self, file_path: &Path) -> Vec<&Extension> {
        self.items
            .iter()
            .filter(|ext| ext.matches(file_path))
            .collect()
    }

    /// Get all global symbols that apply to the given file path.
    pub fn globals_for_file(&self, file_path: &Path) -> Vec<&Symbol> {
        self.matching(file_path)
            .into_iter()
            .flat_map(|ext| &ext.globals)
            .collect()
    }

    /// Get virtual module symbols if they exist for the given module path and file context.
    pub fn virtual_module(&self, module_path: &str, file_path: &Path) -> Option<&[Symbol]> {
        for ext in self.matching(file_path) {
            if let Some(symbols) = ext.modules.get(module_path) {
                return Some(symbols);
            }
        }
        None
    }

    /// Get the load prefix for the given file path (first match wins).
    pub fn load_prefix_for_file(&self, file_path: &Path) -> Option<&str> {
        self.matching(file_path)
            .into_iter()
            .find_map(|ext| ext.configuration.load_prefix.as_deref())
    }

    /// Get all global symbols from extensions that apply to all files (no 'when' clause).
    pub fn global_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.items
            .iter()
            .filter(|ext| ext.when.is_none()) // Only global extensions
            .flat_map(|ext| &ext.globals)
    }

    /// Get all module symbols from all extensions (for type processing).
    pub fn all_module_symbols(&self) -> impl Iterator<Item = (&str, &Symbol)> {
        self.items
            .iter()
            .flat_map(|ext| &ext.modules)
            .flat_map(|(module_name, symbols)| {
                symbols.iter().map(move |symbol| (module_name.as_str(), symbol))
            })
    }
}

/// A single extension that can modify dialect behavior.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Extension {
    /// When this extension applies (optional, if None applies to all files).
    #[serde(default)]
    pub when: Option<FilePatterns>,

    /// Global symbols available without load().
    #[serde(default)]
    pub globals: Vec<Symbol>,

    /// Virtual modules that can be loaded.
    #[serde(default)]
    pub modules: HashMap<String, Vec<Symbol>>,

    /// Configuration for this extension.
    #[serde(default)]
    pub configuration: ExtensionConfig,

    // For backward compatibility with old schema
    #[serde(alias = "config")]
    #[serde(skip)]
    _config: Option<ExtensionConfig>,
}

impl Extension {
    /// Check if this extension applies to the given file path.
    pub fn matches(&self, file_path: &Path) -> bool {
        match &self.when {
            Some(patterns) => patterns.matches(file_path),
            None => true, // Apply to all files if no patterns specified
        }
    }
}

/// Configuration for an extension.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ExtensionConfig {
    /// Optional prefix to prepend to load paths.
    pub load_prefix: Option<String>,
}

/// File patterns for determining when an extension applies.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FilePatterns {
    pub file_patterns: Vec<String>,
}

impl FilePatterns {
    /// Check if any pattern matches the given file path.
    pub fn matches(&self, file_path: &Path) -> bool {
        let file_name = match file_path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return false,
        };

        self.file_patterns
            .iter()
            .any(|pattern| pattern_matches(pattern, file_name))
    }
}

/// A symbol definition (function, variable, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Symbol {
    /// Name of the symbol.
    pub name: String,

    /// Type of the symbol (e.g., "function", "string", "dict", "object", "type").
    #[serde(default = "default_symbol_type")]
    pub r#type: String,

    /// If this is a function, its signature.
    #[serde(default)]
    pub callable: Option<Callable>,

    /// Documentation string.
    #[serde(default)]
    pub doc: String,

    /// Properties for object-type symbols (recursive structure).
    #[serde(default)]
    pub properties: HashMap<String, Box<Symbol>>,

    /// Whether this symbol should be registered as both a type AND a global variable.
    ///
    /// When true, creates the self-referential pattern used by Bazel built-ins like `apple_common`.
    /// This is the key to making object-like extensions work properly in starpls.
    ///
    /// ## How the Self-Referential Pattern Works
    ///
    /// In Bazel's protobuf system, objects like `apple_common` are defined using TWO entries:
    ///
    /// 1. **Type Definition**: Defines what fields/methods the object has
    ///    ```protobuf
    ///    Type {
    ///      name: "apple_common"
    ///      field: [
    ///        Value { name: "apple_toolchain", type: "unknown", ... },
    ///        Value { name: "platform", type: "string", ... },
    ///        // ... other fields and methods
    ///      ]
    ///    }
    ///    ```
    ///
    /// 2. **Global Variable**: Makes it accessible as a global, pointing to itself
    ///    ```protobuf
    ///    Value {
    ///      name: "apple_common"
    ///      type: "apple_common"  // ← SELF-REFERENTIAL! Points to the type above
    ///    }
    ///    ```
    ///
    /// ## Why This Pattern is Essential
    ///
    /// This dual registration enables:
    /// - **Global Access**: You can write `apple_common.apple_toolchain` in any .bzl file
    /// - **Type Safety**: The LSP knows what fields/methods are available
    /// - **Proper Completion**: IDE shows `apple_toolchain`, `platform` etc. when typing `apple_common.`
    /// - **Hover Documentation**: Shows docs for both the object and its members
    /// - **Type Inference**: Knows that `apple_common.platform` returns a string
    ///
    /// ## Extension Example
    ///
    /// For an extension with `"name": "exec", "as_type": true`, this creates:
    /// 1. An `Exec` type with methods like `sh()`, `echo()`, etc.
    /// 2. A global variable `exec` of type `Exec`
    /// 3. Now `exec.sh("ls")` works with full LSP support
    ///
    /// Without this pattern, extensions generate `struct()` calls which create generic
    /// runtime objects that the type system can't understand, resulting in "Unknown" types
    /// and broken completions.
    #[serde(default)]
    pub as_type: bool,
}

/// Function signature definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Callable {
    /// Function parameters.
    #[serde(default)]
    pub params: Vec<Param>,

    /// Return type.
    #[serde(default = "default_return_type")]
    pub return_type: String,
}

/// Function parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Param {
    /// Parameter name.
    pub name: String,

    /// Parameter type.
    #[serde(alias = "param_type")]
    pub r#type: String,

    /// Documentation string.
    #[serde(default)]
    pub doc: String,

    /// Default value (as string).
    #[serde(default)]
    pub default_value: String,

    /// Whether this parameter is mandatory.
    #[serde(default)]
    pub is_mandatory: bool,

    /// Whether this is a *args parameter.
    #[serde(default)]
    pub is_star_arg: bool,

    /// Whether this is a **kwargs parameter.
    #[serde(default)]
    pub is_star_star_arg: bool,
}

/// Load extensions from JSON files.
pub fn load_extensions(paths: &[impl AsRef<Path>]) -> Result<Extensions> {
    let mut extensions = Extensions::new();

    for path in paths {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read extension file: {}", path.display()))?;

        let extension: Extension = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse extension JSON: {}", path.display()))?;

        // Basic validation
        validate_extension(&extension)
            .with_context(|| format!("Invalid extension: {}", path.display()))?;

        extensions.add(extension);
    }

    Ok(extensions)
}

/// Validate an extension for basic consistency.
fn validate_extension(extension: &Extension) -> Result<()> {
    // Validate global symbols
    for symbol in &extension.globals {
        validate_symbol(symbol)?;
    }

    // Validate module symbols
    for (module_name, symbols) in &extension.modules {
        if module_name.trim().is_empty() {
            anyhow::bail!("Module name cannot be empty");
        }

        for symbol in symbols {
            validate_symbol(symbol)?;
        }
    }

    Ok(())
}

/// Validate a symbol definition.
fn validate_symbol(symbol: &Symbol) -> Result<()> {
    if symbol.name.trim().is_empty() {
        anyhow::bail!("Symbol name cannot be empty");
    }

    if !is_valid_identifier(&symbol.name) {
        anyhow::bail!("Symbol name '{}' is not a valid identifier", symbol.name);
    }

    // Validate properties (recursive)
    for (prop_name, prop_symbol) in &symbol.properties {
        if !is_valid_identifier(prop_name) {
            anyhow::bail!(
                "Property name '{}' in symbol '{}' is not a valid identifier",
                prop_name,
                symbol.name
            );
        }
        validate_symbol(prop_symbol)
            .with_context(|| format!("validating property '{}.{}'", symbol.name, prop_name))?;
    }

    if let Some(callable) = &symbol.callable {
        for param in &callable.params {
            if param.name.trim().is_empty() {
                anyhow::bail!(
                    "Parameter name cannot be empty for symbol '{}'",
                    symbol.name
                );
            }

            if !is_valid_identifier(&param.name) {
                anyhow::bail!("Parameter name '{}' is not a valid identifier", param.name);
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

/// Check if a string is a valid Starlark identifier.
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && (s.chars().next().unwrap().is_alphabetic() || s.starts_with('_'))
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !is_starlark_keyword(s)
}

/// Check if a string is a reserved Starlark keyword.
fn is_starlark_keyword(s: &str) -> bool {
    matches!(
        s,
        "and"
            | "as"
            | "assert"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
            | "load"
            | "True"
            | "False"
            | "None"
    )
}

/// Simple pattern matching for file patterns.
fn pattern_matches(pattern: &str, file_name: &str) -> bool {
    if pattern == file_name {
        return true; // Exact match
    }

    if let Some(extension) = pattern.strip_prefix("*.") {
        return file_name.ends_with(&format!(".{}", extension));
    }

    // Could add more sophisticated pattern matching here (glob, regex, etc.)
    false
}

impl Symbol {
    /// Check if this symbol defines a type (either explicitly or implicitly).
    pub fn is_type_definition(&self) -> bool {
        self.r#type == "type" || (!self.properties.is_empty() && self.r#type == "object")
    }

    /// Check if this symbol should be registered as both type and global.
    pub fn should_register_as_type(&self) -> bool {
        self.as_type && self.is_type_definition()
    }
}

fn default_symbol_type() -> String {
    "unknown".to_string()
}

fn default_return_type() -> String {
    "None".to_string()
}

/// Generate JSON schema for extensions.
pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Extension);
    serde_json::to_string_pretty(&schema).unwrap()
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_pattern_matching() {
        assert!(pattern_matches("Tiltfile", "Tiltfile"));
        assert!(pattern_matches("*.star", "test.star"));
        assert!(pattern_matches("*.bzl", "defs.bzl"));
        assert!(!pattern_matches("*.py", "test.star"));
        assert!(!pattern_matches("BUILD", "Tiltfile"));
    }

    #[test]
    fn test_file_patterns_matching() {
        let patterns = FilePatterns {
            file_patterns: vec!["Tiltfile".to_string(), "*.tilt".to_string()],
        };

        assert!(patterns.matches(Path::new("/workspace/Tiltfile")));
        assert!(patterns.matches(Path::new("/workspace/test.tilt")));
        assert!(!patterns.matches(Path::new("/workspace/BUILD")));
    }

    #[test]
    fn test_extension_matching() {
        let extension = Extension {
            when: Some(FilePatterns {
                file_patterns: vec!["Tiltfile".to_string()],
            }),
            globals: vec![],
            modules: HashMap::new(),
            configuration: ExtensionConfig::default(),
            _config: None,
        };

        assert!(extension.matches(Path::new("/workspace/Tiltfile")));
        assert!(!extension.matches(Path::new("/workspace/BUILD")));
    }

    #[test]
    fn test_load_extension_from_json() {
        let json_content = r#"
        {
            "when": {
                "file_patterns": ["Tiltfile"]
            },
            "globals": [
                {
                    "name": "tilt_env",
                    "type": "dict",
                    "doc": "Tilt environment variables"
                }
            ],
            "modules": {
                "tilt/docker": [
                    {
                        "name": "docker_build",
                        "type": "function",
                        "callable": {
                            "params": [
                                {
                                    "name": "ref",
                                    "type": "string",
                                    "is_mandatory": true
                                }
                            ],
                            "return_type": "None"
                        },
                        "doc": "Build a Docker image"
                    }
                ]
            },
            "configuration": {
                "load_prefix": "tilt_libs"
            }
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let extensions = load_extensions(&[temp_file.path()]).unwrap();
        assert_eq!(extensions.items.len(), 1);

        let ext = &extensions.items[0];
        assert_eq!(ext.globals.len(), 1);
        assert_eq!(ext.globals[0].name, "tilt_env");
        assert_eq!(ext.modules.len(), 1);
        assert!(ext.modules.contains_key("tilt/docker"));
        assert_eq!(ext.configuration.load_prefix, Some("tilt_libs".to_string()));
    }

    #[test]
    fn test_virtual_module_lookup() {
        let mut extensions = Extensions::new();
        let mut modules = HashMap::new();
        modules.insert(
            "tilt/docker".to_string(),
            vec![Symbol {
                name: "docker_build".to_string(),
                r#type: "function".to_string(),
                callable: None,
                doc: "Build Docker image".to_string(),
                properties: HashMap::new(),
                as_type: false,
            }],
        );

        extensions.add(Extension {
            when: Some(FilePatterns {
                file_patterns: vec!["Tiltfile".to_string()],
            }),
            globals: vec![],
            modules,
            configuration: ExtensionConfig::default(),
            _config: None,
        });

        // Should find module for Tiltfile
        let tiltfile = Path::new("/workspace/Tiltfile");
        let symbols = extensions.virtual_module("tilt/docker", tiltfile);
        assert!(symbols.is_some());
        assert_eq!(symbols.unwrap().len(), 1);

        // Should not find module for BUILD file
        let build_file = Path::new("/workspace/BUILD");
        let symbols = extensions.virtual_module("tilt/docker", build_file);
        assert!(symbols.is_none());
    }

    #[test]
    fn test_identifier_validation() {
        assert!(is_valid_identifier("my_func"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("test123"));
        assert!(!is_valid_identifier("123test")); // can't start with number
        assert!(!is_valid_identifier("def")); // keyword
        assert!(!is_valid_identifier("load")); // Starlark keyword
        assert!(!is_valid_identifier("")); // empty
    }

    #[test]
    fn test_object_with_methods() {
        let json_content = r#"
        {
            "modules": {
                "command/execution.star": [{
                    "name": "exec",
                    "type": "object",
                    "doc": "Command execution utilities",
                    "properties": {
                        "sh": {
                            "name": "sh",
                            "type": "function",
                            "callable": {
                                "params": [
                                    {
                                        "name": "command",
                                        "type": "string",
                                        "is_mandatory": true
                                    }
                                ],
                                "return_type": "string"
                            },
                            "doc": "Execute shell command",
                            "properties": {}
                        },
                        "echo": {
                            "name": "echo",
                            "type": "function",
                            "callable": {
                                "params": [
                                    {
                                        "name": "message",
                                        "type": "string",
                                        "is_mandatory": true
                                    }
                                ]
                            },
                            "properties": {}
                        }
                    }
                }]
            }
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let extensions = load_extensions(&[temp_file.path()]).unwrap();
        assert_eq!(extensions.items.len(), 1);

        let ext = &extensions.items[0];
        assert_eq!(ext.modules.len(), 1);

        let exec_symbols = ext.modules.get("command/execution.star").unwrap();
        assert_eq!(exec_symbols.len(), 1);

        let exec_symbol = &exec_symbols[0];
        assert_eq!(exec_symbol.name, "exec");
        assert_eq!(exec_symbol.r#type, "object");
        assert_eq!(exec_symbol.properties.len(), 2);

        // Test that both methods exist
        assert!(exec_symbol.properties.contains_key("sh"));
        assert!(exec_symbol.properties.contains_key("echo"));

        // Test sh method details
        let sh_method = &exec_symbol.properties["sh"];
        assert_eq!(sh_method.name, "sh");
        assert_eq!(sh_method.r#type, "function");
        assert!(sh_method.callable.is_some());
        assert_eq!(sh_method.callable.as_ref().unwrap().params.len(), 1);
        assert_eq!(
            sh_method.callable.as_ref().unwrap().params[0].name,
            "command"
        );

        // Test that virtual module lookup works
        let from_path = Path::new("/workspace/test.star");
        let symbols = extensions.virtual_module("command/execution.star", from_path);
        assert!(symbols.is_some());

        let found_symbols = symbols.unwrap();
        assert_eq!(found_symbols.len(), 1);
        assert_eq!(found_symbols[0].name, "exec");
        assert!(!found_symbols[0].properties.is_empty());
    }

    #[test]
    fn test_nested_object_validation() {
        let exec_symbol = Symbol {
            name: "exec".to_string(),
            r#type: "object".to_string(),
            callable: None,
            doc: "Command execution".to_string(),
            properties: HashMap::from([(
                "sh".to_string(),
                Box::new(Symbol {
                    name: "sh".to_string(),
                    r#type: "function".to_string(),
                    callable: Some(Callable {
                        params: vec![Param {
                            name: "cmd".to_string(),
                            r#type: "string".to_string(),
                            doc: String::new(),
                            default_value: String::new(),
                            is_mandatory: true,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        }],
                        return_type: "string".to_string(),
                    }),
                    doc: "Execute shell command".to_string(),
                    properties: HashMap::new(),
                    as_type: false,
                }),
            )]),
            as_type: false,
        };

        // Should validate successfully
        assert!(validate_symbol(&exec_symbol).is_ok());

        // Test invalid property name
        let invalid_symbol = Symbol {
            name: "test".to_string(),
            r#type: "object".to_string(),
            callable: None,
            doc: String::new(),
            properties: HashMap::from([(
                "123invalid".to_string(),
                Box::new(Symbol {
                    name: "invalid".to_string(),
                    r#type: "function".to_string(),
                    callable: None,
                    doc: String::new(),
                    properties: HashMap::new(),
                    as_type: false,
                }),
            )]),
            as_type: false,
        };

        // Should fail validation due to invalid property name
        assert!(validate_symbol(&invalid_symbol).is_err());
    }
}
