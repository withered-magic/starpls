# Starpls JSON Plugin Examples

This directory contains example JSON plugin files that demonstrate how to extend starpls with custom Starlark dialects and symbol definitions.

## Plugin Types

### Dialect Plugins (`--load-dialect`)

Complete dialect definitions that add support for new types of Starlark files.

**Example: Tilt Support**
```bash
starpls server --load-dialect examples/plugins/tilt-dialect.json
```

This adds support for Tiltfiles with functions like:
- `docker_build()` - Build Docker images
- `k8s_yaml()` - Deploy Kubernetes YAML
- `k8s_resource()` - Configure Kubernetes resources
- `local_resource()` - Run local commands

### Symbol Extensions (`--load-symbols`)

Additional symbols that extend existing dialects with new functions, variables, or types.

**Example: Company Bazel Rules**
```bash
starpls server --load-symbols examples/plugins/bazel-company-rules.json
```

This adds company-specific Bazel rules to BUILD files:
- `company_library()` - Standard library with metrics
- `company_service()` - Service with monitoring
- `company_test()` - Test with coverage

## Usage Examples

### Load a single plugin
```bash
# Add Tilt support
starpls server --load-dialect examples/plugins/tilt-dialect.json

# Add company rules to Bazel
starpls server --load-symbols examples/plugins/bazel-company-rules.json
```

### Load multiple plugins
```bash
# Support both Tilt and custom Bazel rules
starpls server \
  --load-dialect examples/plugins/tilt-dialect.json \
  --load-symbols examples/plugins/bazel-company-rules.json
```

### Use with other flags
```bash
# Combine with existing starpls options
starpls server \
  --bazel_path /usr/local/bin/bazel \
  --experimental_enable_label_completions \
  --load-dialect examples/plugins/tilt-dialect.json \
  --load-symbols examples/plugins/bazel-company-rules.json
```

## JSON Schema

### Dialect Plugin Format

```json
{
  "dialect": {
    "id": "my-dialect",
    "name": "My Custom Dialect",
    "description": "A custom Starlark dialect",
    "file_patterns": ["Myfile", "*.my"],
    "priority": 100
  },
  "symbols": [
    {
      "name": "my_function",
      "kind": "function",
      "callable": {
        "params": [
          {
            "name": "param_name",
            "type": "string",
            "doc": "Parameter description",
            "is_mandatory": true
          }
        ],
        "return_type": "None"
      },
      "doc": "Function description"
    }
  ]
}
```

### Symbol Extension Format

```json
{
  "dialect_id": "bazel",
  "context": "build",
  "symbols": [
    {
      "name": "my_rule",
      "kind": "function",
      "callable": {
        "params": [
          {
            "name": "name",
            "type": "string",
            "is_mandatory": true
          }
        ],
        "return_type": "None"
      },
      "doc": "My custom rule"
    }
  ]
}
```

## Field Reference

### Dialect Definition
- `id`: Unique identifier (letters, numbers, hyphens, underscores)
- `name`: Human-readable name
- `description`: Description of the dialect
- `file_patterns`: Array of file patterns (supports `*.ext` wildcards)
- `priority`: Detection priority (higher = checked first, default: 50)

### Symbol Definition
- `name`: Symbol name (must be valid Starlark identifier)
- `kind`: `"function"`, `"variable"`, `"type"`, or `"module"`
- `callable`: Function signature (only for functions)
- `value_type`: Type of the value (for variables/constants)
- `doc`: Documentation string

### Parameter Definition
- `name`: Parameter name (must be valid Starlark identifier)
- `type`: Parameter type (free-form string)
- `doc`: Parameter documentation
- `default_value`: Default value (optional)
- `is_mandatory`: Whether parameter is required

## Validation

Plugins are validated when loaded:
- ✅ Valid JSON syntax
- ✅ Required fields present
- ✅ Valid identifiers (no Starlark keywords)
- ✅ No duplicate parameter names
- ✅ File patterns not empty

## Tips

1. **File Patterns**: Use exact matches (`"Tiltfile"`) or simple wildcards (`"*.tiltfile"`)
2. **Priority**: Higher values are checked first (Tilt: 150, Bazel: 100, Standard: 0)
3. **Documentation**: Include comprehensive `doc` strings for better IDE experience
4. **Types**: Use descriptive type names (`"list of labels"` vs just `"list"`)
5. **Validation**: Test your JSON files before deployment

## Future Features

Coming soon:
- Configuration file support (`starpls.toml`)
- Symbol inheritance and composition
- Advanced file pattern matching
- Plugin dependency system