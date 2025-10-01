# Extensible Dialect System for starpls

This proposal implements a comprehensive extensible dialect system that allows starpls to support multiple Starlark dialects beyond just Bazel, including Tiltfiles and other custom Starlark implementations.

## Overview

The current starpls architecture is tightly coupled to Bazel with hardcoded dialect detection and builtin definitions. This proposal introduces a pluggable system that:

1. **Maintains backward compatibility** with existing code
2. **Enables easy addition** of new Starlark dialects
3. **Separates concerns** between dialect detection, builtin loading, and type system integration
4. **Supports dynamic registration** of dialects at runtime

## Architecture Changes

### Core Abstractions

#### 1. `DialectId` - Unique Dialect Identification
```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DialectId(pub String);
```
Replaces hardcoded enum values with extensible string-based identifiers.

#### 2. `DialectDetector` - Pluggable File Detection
```rust
pub trait DialectDetector: Send + Sync {
    fn detect(&self, workspace_path: &Path, file_path: &Path) -> Option<DialectInfo>;
    fn priority(&self) -> u32 { 0 }
}
```
Allows each dialect to define its own file detection logic with configurable priority.

#### 3. `BuiltinProvider` - Pluggable Builtin Loading
```rust
pub trait BuiltinProvider: Send + Sync {
    fn load_builtins(&self, api_context: Option<APIContext>) -> anyhow::Result<Builtins>;
    fn load_rules(&self, api_context: Option<APIContext>) -> anyhow::Result<Builtins>;
    fn supported_contexts(&self) -> Vec<APIContext>;
}
```
Separates builtin definition loading from core language server logic.

#### 4. `DialectRegistry` - Central Management
```rust
pub struct DialectRegistry {
    dialects: HashMap<DialectId, Dialect>,
    detectors: Vec<Arc<dyn DialectDetector>>,
}
```
Manages all registered dialects and provides unified detection and lookup.

### Database Layer Integration

Extended the HIR database trait with new methods while maintaining backward compatibility:

```rust
pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {
    // Legacy methods (preserved)
    fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins, rules: Builtins);
    fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs;

    // New extensible methods
    fn get_dialect_registry(&self) -> &DialectRegistry;
    fn register_dialect(&mut self, dialect: ExtensibleDialect);
    fn get_builtin_defs_by_id(&self, dialect_id: &DialectId, api_context: Option<APIContext>) -> BuiltinDefs;
}
```

## Implemented Dialects

### 1. Bazel Dialect (Refactored)
- **Files**: `BUILD`, `BUILD.bazel`, `*.bzl`, `WORKSPACE`, `MODULE.bazel`, etc.
- **Priority**: 100
- **Features**: Full Bazel API context support, existing builtin definitions
- **Location**: `crates/starpls_bazel/src/dialect.rs`

### 2. Standard Starlark Dialect
- **Files**: `*.star`, `*.starlark`
- **Priority**: 0 (fallback)
- **Features**: Minimal core Starlark builtins
- **Location**: `crates/starpls_common/src/standard_dialect.rs`

### 3. Tilt Dialect (Example Implementation)
- **Files**: `Tiltfile`, `Tiltfile.*`, `*.tiltfile`
- **Priority**: 150
- **Features**: Tilt-specific functions (`docker_build`, `k8s_yaml`, `local_resource`)
- **Location**: `crates/starpls_common/src/tilt_dialect.rs`

## Usage Examples

### Basic Setup
```rust
let mut registry = DialectRegistry::new();

// Register built-in dialects
registry.register(create_standard_dialect());
registry.register(create_bazel_dialect());
registry.register(create_tilt_dialect());

// Detect dialect for any file
let info = registry.detect(workspace_path, file_path);
```

### Custom Dialect Creation
```rust
// Implement the traits
struct MyDialectDetector;
impl DialectDetector for MyDialectDetector { /* ... */ }

struct MyBuiltinProvider;
impl BuiltinProvider for MyBuiltinProvider { /* ... */ }

// Create and register
let dialect = ExtensibleDialect::new(
    DialectId::new("my-dialect"),
    "My Custom Dialect".to_string(),
    "Description".to_string(),
    Arc::new(MyDialectDetector),
    Arc::new(MyBuiltinProvider),
);

registry.register(dialect);
```

### Migration Path
```rust
// Old code continues to work
let old_dialect = Dialect::Bazel;
let new_id = old_dialect.to_dialect_id();

// New code can use either system
let builtin_defs = db.get_builtin_defs_by_id(&new_id, Some(APIContext::Build));
```

## Benefits

### 1. **True Extensibility**
- No need to modify core starpls code to add new dialects
- Each dialect is self-contained with its own detection and builtin logic
- Plugin-like architecture for third-party dialect implementations

### 2. **Clean Separation of Concerns**
- File detection logic is dialect-specific
- Builtin definitions are loaded on-demand
- Type system integration remains centralized

### 3. **Backward Compatibility**
- Existing Bazel-specific code continues to work unchanged
- Gradual migration path from old to new system
- No breaking changes to public APIs

### 4. **Performance**
- Priority-based detection for optimal file matching
- Lazy loading of builtin definitions
- Efficient caching through Salsa

### 5. **Maintainability**
- Each dialect is independently testable
- Clear interfaces between components
- Reduced coupling between dialect-specific and core logic

## Implementation Details

### File Structure
```
crates/
├── starpls_common/
│   ├── src/
│   │   ├── dialect.rs              # Core abstractions
│   │   ├── standard_dialect.rs     # Standard Starlark
│   │   ├── tilt_dialect.rs         # Tilt example
│   │   └── examples.rs             # Usage examples
├── starpls_bazel/
│   └── src/
│       └── dialect.rs              # Bazel implementation
└── starpls_hir/
    └── src/
        ├── lib.rs                  # Extended Db trait
        └── test_database.rs        # Updated implementation
```

### Testing
- Comprehensive unit tests for each dialect
- Integration tests for the full system
- Backward compatibility tests
- Example usage validation

## Future Enhancements

### 1. Configuration-Based Registration
```toml
[dialects.custom]
name = "Custom Dialect"
detector = "path/to/detector.so"
builtin_data = "path/to/builtins/"
```

### 2. Dynamic Plugin Loading
- Runtime loading of dialect implementations
- Plugin discovery mechanisms
- Hot-reloading of dialect definitions

### 3. Dialect-Specific Features
- Custom syntax highlighting rules
- Dialect-specific completions
- Context-aware documentation

## Migration Strategy

### Phase 1: Core Infrastructure ✅
- Implement core abstractions
- Create dialect registry
- Add database layer support

### Phase 2: Dialect Implementations ✅
- Refactor Bazel to use new system
- Implement standard Starlark dialect
- Create Tilt example

### Phase 3: Integration
- Update language server to use registry
- Modify file detection in document management
- Add configuration support

### Phase 4: Advanced Features
- Plugin loading mechanism
- External dialect packages
- Documentation and tooling

## Conclusion

This extensible dialect system transforms starpls from a Bazel-specific language server into a truly extensible platform for Starlark development. The implementation maintains full backward compatibility while enabling easy addition of new dialects like Tiltfiles, Buck2, or any other Starlark-based configuration language.

The modular architecture ensures that adding support for new dialects requires no changes to the core starpls codebase, making it suitable for community contributions and third-party extensions.