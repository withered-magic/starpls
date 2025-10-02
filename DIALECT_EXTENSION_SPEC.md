# Starpls Extensible Dialect Support - Specification

## Executive Summary

This document specifies a minimal, elegant approach to extending Starlark dialects in starpls with custom symbols and modules via JSON configuration files. The current implementation adds ~2200 lines of complex code with architectural issues. This specification proposes a solution requiring only ~200-300 lines that properly integrates with the existing system.

## Problem Statement

Starpls needs to support various Starlark-based tools (Tilt, Buck2, Please, etc.) that extend the base language with:
1. **Global symbols** - Functions/variables available without `load()` statements
2. **Virtual modules** - Loadable modules that don't exist as files on disk
3. **Path resolution customization** - Prefixes for resolving relative imports
4. **Context-aware extensions** - Different symbols for different file types

## Current Implementation Analysis

### Issues with Current Approach

1. **Dual Dialect Systems** (~1500 lines of unnecessary abstraction)
   - Creates parallel `ExtensibleDialect` system alongside existing `Dialect` enum
   - Registry is populated but never actually used (see TODO comment in server.rs:211)
   - Creates unnecessary traits: `DialectDetector`, `BuiltinProvider`, etc.

2. **Broken load_prefix Implementation**
   - Only uses the FIRST prefix from all plugins
   - Applies globally to ALL Standard dialect loads instead of per-context
   - No association between prefix and the dialect that requested it

3. **Type System Violations**
   - Generic `starpls_common` crate depends on Bazel-specific types
   - Circular dependency already encountered (see server.rs:132 comment)
   - Converts all symbols to Bazel format, defeating extensibility

4. **Missing Core Functionality**
   - Loaded symbols are never provided to the analysis system
   - No support for virtual modules (all loads must resolve to disk files)
   - Pattern matching exists but isn't integrated with symbol resolution

5. **Over-Engineering**
   - `JsonDialectDetector`, `JsonBuiltinProvider` for simple config
   - Multiple validation passes with duplicated logic
   - Abstraction layers that add no value

## Proposed Solution

### Core Principle: Configuration Over Code

JSON files should **configure existing dialects**, not create new ones. All extensions are additive on top of the detected base dialect (Standard or Bazel).

### Three Distinct Features

1. **Global Symbols** - Added to the global scope without requiring `load()`
2. **Virtual Modules** - Can be loaded via `load()` but don't exist on disk
3. **Load Prefix** - Path prefix for resolving relative imports

### JSON Schema

```json
{
  "when": {
    "file_patterns": ["Tiltfile", "*.tilt"]  // Optional: when to apply
  },

  "configuration": {
    "load_prefix": "tilt_libs"  // Optional: prefix for relative imports
  },

  "globals": [  // Symbols available without load()
    {
      "name": "tilt_env",
      "type": "dict",
      "doc": "Tilt environment variables"
    }
  ],

  "modules": {  // Virtual modules that can be loaded
    "tilt/docker": {
      "symbols": [
        {
          "name": "docker_build",
          "callable": {
            "params": [
              {"name": "ref", "type": "string", "is_mandatory": true},
              {"name": "context", "type": "string", "default": "."}
            ],
            "return_type": "None"
          },
          "doc": "Build a Docker image"
        }
      ]
    },
    "tilt/k8s": {
      "symbols": [
        {
          "name": "k8s_yaml",
          "callable": {
            "params": [
              {"name": "yaml", "type": "string", "is_mandatory": true}
            ]
          }
        }
      ]
    }
  }
}
```

### Architecture

```
┌─────────────────┐
│ JSON Extensions │
└────────┬────────┘
         │ Loaded at startup
         ▼
┌─────────────────┐
│ ExtensionStore  │ ◄── Simple struct holding extensions
└────────┬────────┘
         │ Referenced by
         ▼
┌─────────────────┐
│ DefaultFileLoader│ ◄── Check virtual modules first
└────────┬────────┘
         │ Falls back to
         ▼
┌─────────────────┐
│ Disk Resolution │ ◄── Normal file loading
└─────────────────┘
```

## Technical Design

### 1. Core Data Structures (50 lines)

```rust
// crates/starpls_common/src/extensions.rs
pub struct Extensions {
    items: Vec<Extension>,
}

pub struct Extension {
    // When this extension applies
    when: Option<FilePatterns>,

    // Global symbols (no load needed)
    globals: Vec<Symbol>,

    // Virtual modules
    modules: HashMap<String, ModuleSymbols>,

    // Configuration
    config: ExtensionConfig,
}

pub struct ExtensionConfig {
    load_prefix: Option<String>,
}

pub struct FilePatterns {
    patterns: Vec<String>,
}

impl FilePatterns {
    pub fn matches(&self, path: &Path) -> bool {
        let file_name = path.file_name()?.to_str()?;
        self.patterns.iter().any(|p| pattern_matches(p, file_name))
    }
}
```

### 2. Integration Points

#### A. Virtual Module Resolution (30 lines)

In `DefaultFileLoader::load_file`, check virtual modules BEFORE disk resolution:

```rust
fn load_file(&self, path: &str, dialect: Dialect, from: FileId)
    -> Result<Option<LoadFileResult>> {

    // 1. Check virtual modules for current file context
    let from_path = self.interner.lookup_by_file_id(from);

    for ext in self.extensions.matching(from_path) {
        if let Some(module) = ext.modules.get(path) {
            // Return synthetic file with module's symbols
            return Ok(Some(LoadFileResult::Virtual {
                symbols: module.symbols.clone(),
            }));
        }
    }

    // 2. Apply load_prefix if configured
    let resolved_path = self.apply_load_prefix(path, from_path);

    // 3. Continue with normal file resolution
    // ... existing code ...
}
```

#### B. Global Symbol Injection (20 lines)

When loading builtins for a file:

```rust
fn load_builtins_for_file(file_path: &Path) -> Builtins {
    // Start with base dialect builtins
    let mut builtins = base_dialect.load_builtins();

    // Add global symbols from matching extensions
    for ext in extensions.matching(file_path) {
        for symbol in &ext.globals {
            builtins.global.push(symbol.to_builtin());
        }
    }

    builtins
}
```

#### C. Load Prefix Application (15 lines)

```rust
fn apply_load_prefix(&self, path: &str, from_path: &Path) -> PathBuf {
    // Find first matching extension with load_prefix
    for ext in self.extensions.matching(from_path) {
        if let Some(prefix) = &ext.config.load_prefix {
            return from_path.parent().join(prefix).join(path);
        }
    }

    // No prefix, use normal resolution
    from_path.parent().join(path)
}
```

### 3. JSON Loading (50 lines)

```rust
pub fn load_extensions(paths: &[PathBuf]) -> Result<Extensions> {
    let mut extensions = Extensions::default();

    for path in paths {
        let content = fs::read_to_string(path)?;
        let ext: Extension = serde_json::from_str(&content)
            .context("Invalid extension JSON")?;

        // Basic validation
        validate_extension(&ext)?;
        extensions.items.push(ext);
    }

    Ok(extensions)
}

fn validate_extension(ext: &Extension) -> Result<()> {
    // Validate symbol names are valid identifiers
    for symbol in &ext.globals {
        validate_identifier(&symbol.name)?;
    }

    for (module_name, module) in &ext.modules {
        // Module names can have slashes
        validate_module_path(module_name)?;

        for symbol in &module.symbols {
            validate_identifier(&symbol.name)?;
        }
    }

    Ok(())
}
```

### 4. Server Integration (10 lines)

```rust
// In server.rs startup
let extensions = if !config.extension_files.is_empty() {
    load_extensions(&config.extension_files)?
} else {
    Extensions::default()
};

let loader = DefaultFileLoader::new(...)
    .with_extensions(Arc::new(extensions));
```

## Implementation Plan

### Phase 1: Remove Unused Code (Day 1)
1. Delete entire `plugin/` module (mod.rs, loader.rs, schema.rs) - ~1100 lines
2. Remove `ExtensibleDialect` and related traits from starpls_common - ~400 lines
3. Remove unused dialect_registry field from DefaultFileLoader
4. Clean up server.rs plugin loading code

### Phase 2: Implement Core Extensions (Day 2)
1. Create `extensions.rs` in starpls_common (~100 lines)
2. Add Extension, FilePatterns, and validation logic
3. Add JSON deserialization with serde

### Phase 3: Integrate Virtual Modules (Day 3)
1. Modify DefaultFileLoader::load_file to check virtual modules
2. Add LoadFileResult::Virtual variant
3. Test with example Tilt module

### Phase 4: Add Global Symbols (Day 4)
1. Modify builtin loading to inject global symbols
2. Ensure symbols are available in completion/hover
3. Test with Tiltfile

### Phase 5: Implement Load Prefix (Day 5)
1. Add load_prefix support to DefaultFileLoader
2. Ensure it only applies to matching file contexts
3. Test with nested imports

## Testing Strategy

### Unit Tests
1. Pattern matching for file patterns
2. Virtual module resolution
3. Load prefix path resolution
4. Symbol validation

### Integration Tests
1. Tiltfile with Tilt extensions
2. BUILD file should NOT get Tilt symbols
3. Virtual module loading
4. Nested imports with load_prefix

### Example Test Case
```python
# Tiltfile
# Should have access to globals
print(tilt_env["USER"])

# Should load virtual module
load("tilt/docker", "docker_build")
docker_build("myapp", ".")

# Should apply load_prefix
load("helpers", "my_func")  # Resolves to tilt_libs/helpers
```

## Benefits Over Current Implementation

| Aspect | Current | Proposed |
|--------|---------|----------|
| Lines of Code | ~2200 | ~200-300 |
| Architecture | Dual dialect systems | Single, unified |
| Virtual Modules | Not supported | First-class support |
| Load Prefix | Broken (global, first-only) | Context-aware |
| Integration | Never used | Direct integration |
| Circular Deps | Yes | No |
| Type Safety | Violated (generic→Bazel) | Clean separation |

## Edge Cases and Considerations

1. **Multiple Extensions**: Extensions are additive and applied in order
2. **Conflicting Symbols**: Last one wins (like CSS)
3. **Virtual vs Real**: Virtual modules checked first, then disk
4. **Load Cycles**: Virtual modules can't load each other (no impl)
5. **Performance**: Extension matching cached per file

## Migration Path

1. The new system is completely backward compatible
2. No changes needed to existing Bazel support
3. Command-line flags can remain the same: `--experimental_load_dialects`
4. JSON schema is simplified but conceptually similar

## Success Criteria

1. ✅ Tiltfile gets Tilt symbols, BUILD files don't
2. ✅ Virtual modules can be loaded
3. ✅ Load prefix works per-context
4. ✅ Less than 300 lines of new code
5. ✅ No circular dependencies
6. ✅ Symbols appear in completion/hover
7. ✅ Works with existing analysis system

## Conclusion

This specification reduces the extensible dialect feature from an over-engineered 2200-line implementation to a focused 200-300 line solution that:
- Actually works (current one doesn't integrate)
- Supports virtual modules (current one doesn't)
- Has correct load_prefix behavior (current one is broken)
- Maintains clean architecture (no circular deps)
- Is easy to understand and maintain

The key insight is that we don't need to create new dialects - we need to configure existing ones. This "configuration over code" approach dramatically simplifies the implementation while providing more functionality.