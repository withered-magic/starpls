# Object Methods Support for Starpls Extensions

## Executive Summary

The current starpls extension system cannot express object-with-methods patterns like `exec.sh()` after `load("command/execution.star", "exec")`. This is a fundamental limitation since this pattern is extensively used in both Starlark/Bazel core (`ctx.actions.run()`, `repo_ctx.download()`) and real-world implementations.

## Problem Statement

### Current Limitation
The extension system's `Symbol` struct only supports flat exports:
```json
{
  "modules": {
    "command/execution.star": [
      {"name": "exec_sh", "type": "function"},  // Forces exec_sh() instead of exec.sh()
      {"name": "exec_echo", "type": "function"} // Forces exec_echo() instead of exec.echo()
    ]
  }
}
```

### Desired Pattern
Users expect to write:
```python
load("command/execution.star", "exec")
exec.sh("ls -la")        # Natural, idiomatic
exec.echo("hello world")  # Follows Starlark conventions
```

### Why This Matters
1. **Core Starlark Pattern**: Built-in objects use this extensively
   - `ctx.actions.run()`, `ctx.actions.declare_file()`
   - `ctx.attr.username`, `ctx.file.src`
   - `repo_ctx.download_and_extract()`
   - `json.decode()`, `proto.encode_text()`

2. **starpls Already Handles This**: The LSP internally supports nested properties via `BuiltinType` with `fields` and `methods`

3. **Real Implementations Use This**: Go-based Starlark modules naturally create this pattern

## Research Findings

### How starpls Handles Built-in Nested Properties

From `crates/starpls_hir/src/typeck/builtins.rs`:
```rust
#[salsa::tracked]
pub(crate) struct BuiltinType {
    pub(crate) name: Name,
    pub(crate) fields: Vec<BuiltinField>,    // Properties
    pub(crate) methods: Vec<BuiltinFunction>, // Methods
    pub(crate) doc: String,
}
```

The LSP provides completions after dots using `.fields(db)`, proving this is core functionality.

### Reference Implementation (Go Starlark)

A typical Go implementation creating the `exec` module:
```go
package exec

import (
    "sync"
    "go.starlark.net/starlark"
    "go.starlark.net/starlarkstruct"
    "custom.com/utils/cmd"
)

const ModuleName = "command/execution.star"

var (
    once       sync.Once
    fileModule starlark.StringDict
)

func LoadModule() (starlark.StringDict, error) {
    once.Do(func() {
        fileModule = starlark.StringDict{
            "exec": &starlarkstruct.Module{
                Name: "exec.star",
                Members: starlark.StringDict{
                    "sh": starlark.NewBuiltin("sh", func(
                        thread *starlark.Thread,
                        b *starlark.Builtin,
                        args starlark.Tuple,
                        kwargs []starlark.Tuple,
                    ) (starlark.Value, error) {
                        var command string
                        if err := starlark.UnpackArgs("sh", args, kwargs,
                            "command", &command); err != nil {
                            return nil, err
                        }
                        result := cmd.ExecuteShell(command)
                        return starlark.String(result), nil
                    }),
                    "echo": starlark.NewBuiltin("echo", func(
                        thread *starlark.Thread,
                        b *starlark.Builtin,
                        args starlark.Tuple,
                        kwargs []starlark.Tuple,
                    ) (starlark.Value, error) {
                        var message string
                        if err := starlark.UnpackArgs("echo", args, kwargs,
                            "message", &message); err != nil {
                            return nil, err
                        }
                        cmd.PrintOutput(message)
                        return starlark.None, nil
                    }),
                    "checkout": starlark.NewBuiltin("checkout", func(
                        thread *starlark.Thread,
                        b *starlark.Builtin,
                        args starlark.Tuple,
                        kwargs []starlark.Tuple,
                    ) (starlark.Value, error) {
                        var branch string
                        if err := starlark.UnpackArgs("checkout", args, kwargs,
                            "branch", &branch); err != nil {
                            return nil, err
                        }
                        result := cmd.GitCheckout(branch)
                        return starlark.String(result), nil
                    }),
                },
            },
        }
    })
    return fileModule, nil
}
```

This creates a module where users can:
```python
load("command/execution.star", "exec")
output = exec.sh("ls -la")
exec.echo("Build complete")
exec.checkout("main")
```

## Proposed Solution

### 1. Extend Symbol Structure

Add a `properties` field to support nested structures:

```rust
// In crates/starpls_common/src/extensions.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,

    /// Type of the symbol (e.g., "function", "string", "dict", "object")
    #[serde(default = "default_symbol_type")]
    pub r#type: String,

    /// If this is a function, its signature
    #[serde(default)]
    pub callable: Option<Callable>,

    /// Documentation string
    #[serde(default)]
    pub doc: String,

    /// Properties for object-type symbols (NEW)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Symbol>>,
}
```

### 2. JSON Configuration Format

The new format supports nested object definitions:

```json
{
  "modules": {
    "command/execution.star": [
      {
        "name": "exec",
        "type": "object",
        "doc": "Command execution utilities",
        "properties": {
          "sh": {
            "type": "function",
            "callable": {
              "params": [
                {
                  "name": "command",
                  "type": "string",
                  "is_mandatory": true,
                  "doc": "Shell command to execute"
                }
              ],
              "return_type": "string"
            },
            "doc": "Execute a shell command and return its output"
          },
          "echo": {
            "type": "function",
            "callable": {
              "params": [
                {
                  "name": "message",
                  "type": "string",
                  "is_mandatory": true
                }
              ],
              "return_type": "None"
            },
            "doc": "Print a message to stdout"
          },
          "checkout": {
            "type": "function",
            "callable": {
              "params": [
                {
                  "name": "branch",
                  "type": "string",
                  "is_mandatory": true
                }
              ],
              "return_type": "string"
            },
            "doc": "Checkout a git branch"
          }
        }
      }
    ]
  }
}
```

### 3. Virtual Module Generation

Update the virtual file content generator to handle objects with properties:

```rust
fn create_virtual_file_content(symbols: &[Symbol]) -> String {
    let mut content = String::new();
    content.push_str("# Virtual module - generated from extension\n\n");

    for symbol in symbols {
        match symbol.r#type.as_str() {
            "object" if symbol.properties.is_some() => {
                // Generate a struct with methods as a namespace object
                generate_object_with_methods(&mut content, symbol);
            }
            "function" => {
                generate_function(&mut content, symbol);
            }
            _ => {
                generate_variable(&mut content, symbol);
            }
        }
    }

    content
}

fn generate_object_with_methods(content: &mut String, symbol: &Symbol) {
    // Generate docstring
    if !symbol.doc.is_empty() {
        content.push_str(&format!("# {}\n", symbol.doc));
    }

    // Create struct with method stubs
    content.push_str(&format!("{} = struct(\n", symbol.name));

    if let Some(properties) = &symbol.properties {
        for (prop_name, prop_symbol) in properties {
            if prop_symbol.r#type == "function" {
                // Generate a lambda stub for each method
                content.push_str(&format!("    {} = lambda *args, **kwargs: None,\n", prop_name));
            } else {
                // Generate property with appropriate default value
                let default_value = match prop_symbol.r#type.as_str() {
                    "string" => "\"\"",
                    "int" => "0",
                    "bool" => "False",
                    "dict" => "{}",
                    "list" => "[]",
                    _ => "None",
                };
                content.push_str(&format!("    {} = {},\n", prop_name, default_value));
            }
        }
    }

    content.push_str(")\n\n");
}
```

### 4. Validation Updates

Extend validation to handle nested properties:

```rust
fn validate_symbol(symbol: &Symbol, path: Vec<String>) -> Result<()> {
    // Existing validation...
    if symbol.name.trim().is_empty() {
        bail!("Symbol name cannot be empty at path: {}", path.join("."));
    }

    if !is_valid_identifier(&symbol.name) {
        bail!("Symbol name '{}' is not a valid identifier", symbol.name);
    }

    // NEW: Validate properties for object types
    if symbol.r#type == "object" {
        if let Some(properties) = &symbol.properties {
            for (prop_name, prop_symbol) in properties {
                let mut new_path = path.clone();
                new_path.push(symbol.name.clone());
                new_path.push(prop_name.clone());

                // Recursive validation
                validate_symbol(prop_symbol, new_path)?;
            }
        }
    }

    // Validate callable if present...

    Ok(())
}
```

### 5. JSON Schema Generation

Add schema generation support with schemars:

```toml
# Cargo.toml
[dependencies]
schemars = { version = "0.8", features = ["derive"], optional = true }

[features]
schema = ["schemars"]
```

```rust
// Command to generate schema
pub fn generate_extension_schema() -> String {
    #[cfg(feature = "schema")]
    {
        let schema = schemars::schema_for!(Extension);
        serde_json::to_string_pretty(&schema).unwrap()
    }
    #[cfg(not(feature = "schema"))]
    {
        "Schema generation not enabled. Build with --features schema".to_string()
    }
}
```

## Test Cases

### Basic Object with Methods
```json
{
  "modules": {
    "test/utils.star": [{
      "name": "utils",
      "type": "object",
      "properties": {
        "format": {
          "type": "function",
          "callable": {
            "params": [{"name": "text", "type": "string"}],
            "return_type": "string"
          }
        },
        "validate": {
          "type": "function",
          "callable": {
            "params": [{"name": "data", "type": "any"}],
            "return_type": "bool"
          }
        }
      }
    }]
  }
}
```

Usage:
```python
load("test/utils.star", "utils")
formatted = utils.format("hello")
is_valid = utils.validate({"key": "value"})
```

### Nested Objects (Deep Nesting)
```json
{
  "modules": {
    "cloud/aws.star": [{
      "name": "aws",
      "type": "object",
      "properties": {
        "s3": {
          "type": "object",
          "properties": {
            "upload": {
              "type": "function",
              "callable": {
                "params": [
                  {"name": "bucket", "type": "string"},
                  {"name": "key", "type": "string"},
                  {"name": "data", "type": "string"}
                ]
              }
            },
            "download": {
              "type": "function",
              "callable": {
                "params": [
                  {"name": "bucket", "type": "string"},
                  {"name": "key", "type": "string"}
                ],
                "return_type": "string"
              }
            }
          }
        },
        "ec2": {
          "type": "object",
          "properties": {
            "list_instances": {
              "type": "function",
              "callable": {
                "return_type": "list"
              }
            }
          }
        }
      }
    }]
  }
}
```

Usage:
```python
load("cloud/aws.star", "aws")
aws.s3.upload("my-bucket", "file.txt", content)
data = aws.s3.download("my-bucket", "file.txt")
instances = aws.ec2.list_instances()
```

## Migration Path

1. **Backward Compatible**: Existing flat symbols continue to work
2. **Opt-in**: Only symbols with `type: "object"` and `properties` use the new feature
3. **Gradual Adoption**: Extensions can be updated incrementally

## Implementation Checklist

- [ ] Add `properties` field to `Symbol` struct
- [ ] Update `Deserialize` to handle nested properties
- [ ] Implement recursive validation for properties
- [ ] Update `create_virtual_file_content` for object generation
- [ ] Add JSON schema generation with nested support
- [ ] Create comprehensive test suite
- [ ] Update documentation
- [ ] Add example extension files

## Benefits

1. **Natural API**: Matches user expectations from core Starlark/Bazel
2. **Consistency**: Aligns with how starpls handles built-in types
3. **Expressiveness**: Enables proper module organization
4. **Type Safety**: LSP can provide accurate completions for nested properties
5. **Documentation**: Each method can have its own documentation

## Conclusion

Adding nested property support to the extension system is essential for expressing idiomatic Starlark patterns. This proposal provides a minimal, backward-compatible solution that aligns with both user expectations and starpls's existing architecture for handling built-in types with properties and methods.