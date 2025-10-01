# Extensible Starlark Language Server - Detailed Roadmap & TODO

## Overview

This document contains a comprehensive, prioritized roadmap for completing the extensible Starlark language server vision. Items are organized by priority (P0 = Critical, P1 = High, P2 = Medium, P3 = Nice to have) and include detailed implementation notes, effort estimates, and dependencies.

**Current Status**: Foundation complete, JSON plugin system working, integration with language features pending.

---

## 🚨 P0 - CRITICAL PATH (Must Complete for MVP)

### P0.1 - File Detection Integration ⚡ **BLOCKING EVERYTHING**

**Problem**: Files are still detected using hardcoded Bazel patterns. Plugin dialects are loaded but never used for file detection.

**Current Code Location**: `crates/starpls/src/document.rs:dialect_and_api_context_for_workspace_path()`

**Implementation Plan**:

1. **Update DocumentManager to use DialectRegistry**
   - Modify `DocumentManager::new()` to accept a `DialectRegistry`
   - Replace hardcoded detection with `registry.detect(workspace, file_path)`
   - Ensure priority-based detection works correctly

2. **Integration Points**:
   ```rust
   // In document.rs
   pub(crate) fn dialect_and_api_context_for_workspace_path(
       registry: &DialectRegistry,  // NEW PARAMETER
       workspace: impl AsRef<Path>,
       path: impl AsRef<Path>,
   ) -> Option<(DialectId, Option<APIContext>)> {  // UPDATED RETURN TYPE
       registry.detect(workspace.as_ref(), path.as_ref())
           .map(|info| (info.dialect_id, info.api_context))
   }
   ```

3. **Backward Compatibility Strategy**:
   - Keep old `Dialect` enum working for existing code
   - Add conversion utilities between `DialectId` and `Dialect`
   - Gradual migration path for internal code

4. **Testing Requirements**:
   - Test that Tiltfiles are detected with plugin loaded
   - Test priority ordering (Tilt > Bazel > Standard)
   - Test fallback to standard dialect when no plugins match
   - Test multiple file patterns per dialect

**Effort**: 2-3 days
**Dependencies**: None
**Risk**: Medium (changes core file detection logic)

### P0.2 - Symbol System Integration 🎯 **USER-VISIBLE IMPACT**

**Problem**: Plugin symbols are parsed but don't appear in IDE completions, hover, or go-to-definition.

**Current State**: Symbols are loaded into `JsonBuiltinProvider` but not connected to the HIR/IDE layer.

**Implementation Plan**:

1. **Connect Plugin Registry to Analysis**:
   ```rust
   // In server.rs, modify Analysis creation
   analysis.register_dialect_registry(registry);

   // Analysis needs new method:
   impl Analysis {
       pub fn register_dialect_registry(&mut self, registry: DialectRegistry) {
           // Store registry and use for symbol resolution
       }
   }
   ```

2. **Update Builtin Resolution in HIR**:
   - Modify `builtin_globals()` function to query dialect registry
   - Update `builtin_types()` to include plugin-defined types
   - Ensure plugin symbols participate in name resolution

3. **Symbol Extension Application**:
   ```rust
   // Apply symbol extensions to existing dialects
   for extension in symbol_extensions {
       if let Some(provider) = registry.builtin_provider(&extension.dialect_id) {
           // Merge extension symbols into provider
           provider.extend_symbols(extension.symbols);
       }
   }
   ```

4. **Integration with Completion System**:
   - Update `completions.rs` to use plugin symbols
   - Ensure context-aware completions work (e.g., only show Tilt functions in Tiltfiles)
   - Add plugin symbol documentation to hover providers

**Effort**: 3-4 days
**Dependencies**: P0.1 (file detection)
**Risk**: High (complex integration across multiple layers)

### P0.3 - Database Layer Registry Integration 🔧 **ARCHITECTURAL CRITICAL**

**Problem**: The new `DialectRegistry` exists but the database layer still uses the old `DashMap<Dialect, BuiltinDefs>` system.

**Implementation Plan**:

1. **Update Database Trait**:
   ```rust
   // Modify crates/starpls_hir/src/lib.rs
   pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {
       // Keep legacy methods for backward compatibility
       fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins, rules: Builtins);
       fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs;

       // Add new registry-based methods
       fn set_dialect_registry(&mut self, registry: DialectRegistry);
       fn get_builtin_defs_by_id(&self, dialect_id: &DialectId, context: Option<APIContext>) -> BuiltinDefs;
       fn resolve_symbol(&self, name: &str, dialect_id: &DialectId) -> Option<SymbolInfo>;
   }
   ```

2. **Database Implementation Updates**:
   - Update `TestDatabase` and main `Database` to store `DialectRegistry`
   - Implement registry-based symbol resolution
   - Ensure backward compatibility with legacy Bazel code

3. **Symbol Resolution Pipeline**:
   ```rust
   // New symbol resolution flow
   fn resolve_symbol(name: &str, file: File) -> Option<SymbolInfo> {
       let dialect_info = detect_dialect(file.path());
       let provider = registry.builtin_provider(&dialect_info.dialect_id)?;
       provider.resolve_symbol(name, dialect_info.api_context)
   }
   ```

**Effort**: 2-3 days
**Dependencies**: P0.1, P0.2
**Risk**: Medium (database layer changes)

### P0.4 - End-to-End Plugin Functionality Testing 🧪 **VALIDATION**

**Problem**: Need comprehensive testing to ensure plugins actually work end-to-end.

**Implementation Plan**:

1. **Integration Test Suite**:
   ```rust
   #[test]
   fn test_tilt_plugin_end_to_end() {
       let server = start_server_with_plugin("examples/plugins/tilt-dialect.json");

       // Test file detection
       let tiltfile = create_test_file("Tiltfile", "docker_build('my-image', '.')");
       assert_eq!(server.detect_dialect(&tiltfile), DialectId::new("tilt"));

       // Test completions
       let completions = server.get_completions(&tiltfile, position_after("docker_"));
       assert!(completions.contains("docker_build"));

       // Test hover
       let hover = server.get_hover(&tiltfile, position_on("docker_build"));
       assert!(hover.contains("Builds a Docker image"));

       // Test signature help
       let sig_help = server.get_signature_help(&tiltfile, position_after("docker_build("));
       assert_eq!(sig_help.active_parameter, 0);
       assert_eq!(sig_help.parameters[0].name, "ref");
   }
   ```

2. **Real-world Test Cases**:
   - Create actual Tiltfile examples and test full IDE experience
   - Test Bazel + company rules combination
   - Test error cases (malformed plugins, missing files)
   - Performance testing with multiple plugins

3. **User Experience Validation**:
   - Manual testing with VS Code extension
   - Verify all language features work (completions, hover, go-to-def, diagnostics)
   - Test plugin error handling and recovery

**Effort**: 2-3 days
**Dependencies**: P0.1, P0.2, P0.3
**Risk**: Low (testing only)

---

## 🔥 P1 - HIGH PRIORITY (Essential for Production)

### P1.1 - Symbol Extension Implementation 📚 **PLUGIN ECOSYSTEM**

**Problem**: `--load-symbols` files are parsed but symbols aren't actually added to existing dialects.

**Current Gap**: Symbol extensions are loaded but not applied to dialect providers.

**Implementation Plan**:

1. **Dynamic Symbol Merging**:
   ```rust
   pub trait BuiltinProvider {
       fn load_builtins(&self, context: Option<APIContext>) -> Result<Builtins>;
       fn extend_with_symbols(&mut self, symbols: Vec<SymbolDefinition>) -> Result<()>; // NEW
       fn merge_extension(&mut self, extension: &SymbolExtension) -> Result<()>; // NEW
   }
   ```

2. **Extension Application Pipeline**:
   ```rust
   // In plugin loading
   for extension in symbol_extensions {
       if let Some(mut provider) = registry.get_mutable_provider(&extension.dialect_id) {
           provider.merge_extension(&extension)?;
       } else {
           warn!("Cannot extend unknown dialect: {}", extension.dialect_id);
       }
   }
   ```

3. **Conflict Resolution**:
   - Handle symbol name conflicts (last wins? error? merge?)
   - Support symbol overrides for customization
   - Validate that extensions don't break existing symbols

4. **Context-Aware Extensions**:
   - Support extensions that only apply to specific API contexts
   - Enable different symbols for BUILD vs .bzl files

**Effort**: 2-3 days
**Dependencies**: P0.2
**Risk**: Medium (complex merging logic)

### P1.2 - Error Handling and Recovery 🛡️ **PRODUCTION READY**

**Problem**: Current plugin system has basic error handling but lacks production-grade robustness.

**Implementation Plan**:

1. **Graceful Plugin Failure**:
   ```rust
   // Plugin loading should never crash the server
   match plugin::load_dialect_plugins(&mut registry, &config.args.dialect_files) {
       Ok(loaded_count) => info!("Loaded {} dialect plugins", loaded_count),
       Err(errors) => {
           for error in errors {
               error!("Plugin load failed: {}", error);
               // Send user notification through LSP
               server.send_error_notification(&format!("Plugin error: {}", error));
           }
           // Continue with built-in dialects only
       }
   }
   ```

2. **Plugin Validation Levels**:
   - **Syntax Level**: JSON schema validation (already implemented)
   - **Semantic Level**: Symbol name conflicts, circular dependencies
   - **Runtime Level**: Plugin symbols that cause type errors

3. **Error Recovery Strategies**:
   - Skip individual broken symbols while keeping rest of plugin
   - Fallback to built-in dialects when plugin fails
   - Partial plugin loading (load symbols that work, skip broken ones)

4. **User-Friendly Error Messages**:
   ```rust
   // Instead of: "Failed to parse JSON"
   // Provide: "Plugin 'tilt-dialect.json' line 15: Symbol 'docker_build' has invalid parameter 'ref': parameter names must be valid Starlark identifiers"
   ```

5. **Hot Plugin Reloading** (Future):
   - Watch plugin files for changes
   - Reload plugins without restarting server
   - Handle reload failures gracefully

**Effort**: 3-4 days
**Dependencies**: P0.1, P0.2
**Risk**: Low (error handling improvements)

### P1.3 - Type System Integration 🔍 **ADVANCED IDE FEATURES**

**Problem**: Plugin symbols don't participate in Starlark's type system, limiting type checking and inference.

**Implementation Plan**:

1. **Type Definition Support**:
   ```json
   // Enhanced JSON schema for types
   {
     "name": "TiltResource",
     "kind": "type",
     "type_definition": {
       "kind": "struct",
       "fields": [
         {"name": "name", "type": "string"},
         {"name": "image", "type": "string"},
         {"name": "port_forwards", "type": "list[int]"}
       ]
     }
   }
   ```

2. **Function Return Type Integration**:
   - Plugin functions return meaningful types (not just "None")
   - Support for generic types (list[T], dict[K,V])
   - Union types for functions with multiple return possibilities

3. **Cross-Dialect Type Checking**:
   ```python
   # In a Tiltfile that loads Bazel functions
   load("//build:rules.bzl", "my_bazel_function")

   # Type checker should understand both Tilt and Bazel symbols
   docker_build("app", ".")  # Tilt function
   my_bazel_function("arg")  # Bazel function from plugin
   ```

4. **Type Inference Enhancement**:
   - Plugin symbols participate in type inference
   - Better error messages for type mismatches
   - Support for plugin-defined type constraints

**Effort**: 4-5 days
**Dependencies**: P0.2, P1.1
**Risk**: High (complex type system integration)

### P1.4 - Performance Optimization 🚀 **SCALABILITY**

**Problem**: Current implementation loads all plugins eagerly and doesn't optimize for large numbers of plugins.

**Implementation Plan**:

1. **Lazy Plugin Loading**:
   ```rust
   struct LazyDialectProvider {
       path: PathBuf,
       loaded: Option<Box<dyn BuiltinProvider>>,
   }

   impl BuiltinProvider for LazyDialectProvider {
       fn load_builtins(&self, context: Option<APIContext>) -> Result<Builtins> {
           if self.loaded.is_none() {
               self.loaded = Some(load_plugin_from_file(&self.path)?);
           }
           self.loaded.as_ref().unwrap().load_builtins(context)
       }
   }
   ```

2. **Symbol Caching Strategy**:
   - Cache parsed plugin symbols in memory
   - Invalidate cache when plugin files change
   - Use Salsa for incremental recomputation

3. **File Detection Optimization**:
   - Short-circuit detection once match is found
   - Optimize pattern matching algorithms
   - Cache detection results per file path

4. **Memory Usage Optimization**:
   - Share common symbol definitions between plugins
   - Use string interning for repeated names/types
   - Compact representation for large plugin sets

**Effort**: 3-4 days
**Dependencies**: P0.1, P0.2
**Risk**: Medium (performance changes)

---

## 🎯 P2 - MEDIUM PRIORITY (Enhanced Experience)

### P2.1 - Configuration File Support 📄 **USER EXPERIENCE**

**Problem**: CLI flags for plugins are verbose and hard to manage for teams with many plugins.

**Implementation Plan**:

1. **starpls.toml Schema Design**:
   ```toml
   # Global settings
   [plugins]
   enabled = true
   discovery_dirs = ["~/.starpls/plugins", "./team-plugins", "/opt/company/starpls"]

   # Individual dialect plugins
   [[plugins.dialect]]
   name = "tilt"
   path = "tilt-dialect.json"
   enabled = true
   priority_override = 200  # Override plugin's built-in priority

   [[plugins.dialect]]
   name = "buck2"
   source = "https://github.com/facebook/buck2/starpls-plugin.json"
   version = "^2.0.0"
   enabled = false

   # Symbol extensions
   [[plugins.symbols]]
   dialect = "bazel"
   path = "company-rules.json"
   contexts = ["build", "bzl"]  # Only apply to these contexts

   [[plugins.symbols]]
   dialect = "bazel"
   path = "team-macros.json"
   enabled_for_projects = ["//src/backend/...", "//src/frontend/..."]
   ```

2. **Configuration Loading Pipeline**:
   ```rust
   // Priority order: CLI args > local starpls.toml > global ~/.starpls/config.toml > defaults
   fn load_plugin_config(args: &ServerCommand) -> PluginConfig {
       let mut config = PluginConfig::default();

       // 1. Load global config
       if let Ok(global) = load_config_file("~/.starpls/config.toml") {
           config.merge(global);
       }

       // 2. Load local config
       if let Ok(local) = load_config_file("./starpls.toml") {
           config.merge(local);
       }

       // 3. Apply CLI overrides
       config.apply_cli_args(args);

       config
   }
   ```

3. **Plugin Discovery**:
   - Scan discovery directories for .json files
   - Auto-detect plugin types (dialect vs symbols)
   - Support for plugin manifests and metadata

4. **Configuration Validation**:
   - Validate TOML syntax and schema
   - Check that referenced plugin files exist
   - Warn about conflicting configurations

**Effort**: 3-4 days
**Dependencies**: P0.2, P1.1
**Risk**: Low (additive feature)

### P2.2 - Advanced Plugin Schema Features 🔧 **PLUGIN ECOSYSTEM**

**Problem**: Current JSON schema is basic and doesn't support advanced Starlark features.

**Implementation Plan**:

1. **Enhanced Parameter Types**:
   ```json
   {
     "name": "my_function",
     "callable": {
       "params": [
         {
           "name": "files",
           "type": "list[string] | string",  // Union types
           "doc": "File(s) to process",
           "is_mandatory": true
         },
         {
           "name": "config",
           "type": "dict[string, any]",      // Generic types
           "default_value": "{}",
           "schema": {                       // Nested validation
             "required": ["name"],
             "optional": ["timeout", "retries"]
           }
         }
       ],
       "varargs": {                          // *args support
         "name": "extra_files",
         "type": "string"
       },
       "kwargs": {                           // **kwargs support
         "name": "options",
         "type": "any"
       }
     }
   }
   ```

2. **Module and Namespace Support**:
   ```json
   {
     "name": "config",
     "kind": "module",
     "members": [
       {
         "name": "parse",
         "kind": "function",
         "callable": { ... }
       },
       {
         "name": "VERSION",
         "kind": "variable",
         "value_type": "string",
         "value": "\"1.0.0\""
       }
     ]
   }
   ```

3. **Advanced Type Definitions**:
   ```json
   {
     "name": "Resource",
     "kind": "type",
     "type_definition": {
       "kind": "class",
       "constructor": {
         "params": [{"name": "name", "type": "string"}]
       },
       "methods": [
         {
           "name": "deploy",
           "callable": {
             "params": [{"name": "namespace", "type": "string"}],
             "return_type": "bool"
           }
         }
       ],
       "properties": [
         {"name": "name", "type": "string", "readonly": true},
         {"name": "status", "type": "string", "readonly": false}
       ]
     }
   }
   ```

4. **Plugin Metadata and Versioning**:
   ```json
   {
     "plugin_metadata": {
       "name": "Tilt Dialect",
       "version": "1.2.3",
       "author": "Tilt Team",
       "description": "Official Tilt language support",
       "homepage": "https://tilt.dev",
       "starpls_version": ">=0.2.0",
       "dependencies": ["bazel-base >= 1.0.0"]
     },
     "dialect": { ... },
     "symbols": [ ... ]
   }
   ```

**Effort**: 4-5 days
**Dependencies**: P1.1, P1.3
**Risk**: Medium (complex schema changes)

### P2.3 - Plugin Development Tools 🛠️ **DEVELOPER EXPERIENCE**

**Problem**: Creating and debugging plugins is currently manual and error-prone.

**Implementation Plan**:

1. **Plugin Validation CLI Tool**:
   ```bash
   # Validate plugin before deployment
   starpls validate-plugin my-dialect.json

   # Output:
   ✅ JSON syntax valid
   ✅ Schema validation passed
   ✅ All symbols have valid identifiers
   ⚠️  Warning: Symbol 'docker_build' conflicts with existing Bazel symbol
   ❌ Error: Parameter 'ref' in 'docker_build' has invalid default value
   ```

2. **Plugin Generation Templates**:
   ```bash
   # Interactive plugin creation
   starpls create-plugin

   # Prompts:
   # Plugin type: (dialect/symbols) > dialect
   # Dialect name: > MyTool
   # Dialect ID: > mytool
   # File patterns: > Mytoolfile,*.mytool
   # Generated: mytool-dialect.json
   ```

3. **Documentation Generation**:
   ```bash
   # Generate markdown docs from plugin
   starpls generate-docs tilt-dialect.json > tilt-reference.md

   # Output includes:
   # - Function signatures
   # - Parameter descriptions
   # - Usage examples
   # - Type definitions
   ```

4. **Plugin Testing Framework**:
   ```json
   // plugin-tests.json
   {
     "plugin": "tilt-dialect.json",
     "tests": [
       {
         "name": "docker_build completion",
         "file_content": "docker_",
         "cursor_position": 7,
         "expect_completions": ["docker_build"]
       },
       {
         "name": "docker_build signature help",
         "file_content": "docker_build(",
         "cursor_position": 13,
         "expect_signature": {
           "active_parameter": 0,
           "parameter_name": "ref"
         }
       }
     ]
   }
   ```

**Effort**: 3-4 days
**Dependencies**: P0.2, P1.1
**Risk**: Low (tooling additions)

### P2.4 - Cross-Dialect Features 🌐 **ADVANCED INTEGRATION**

**Problem**: Current system treats each dialect in isolation, but real projects often mix multiple Starlark dialects.

**Implementation Plan**:

1. **Cross-Dialect Symbol Resolution**:
   ```python
   # In a Tiltfile that loads Bazel functions
   load("//build_tools:helpers.bzl", "get_version")  # Bazel function

   # Tiltfile should understand both Tilt and loaded Bazel symbols
   version = get_version()  # Type checker knows this returns string
   docker_build(f"app:{version}", ".")  # Tilt function with Bazel input
   ```

2. **Multi-Dialect Projects**:
   - Support for projects with both Tiltfiles and BUILD files
   - Shared symbol namespace across dialect boundaries
   - Context-aware completions based on loaded symbols

3. **Dialect Inheritance and Composition**:
   ```json
   {
     "dialect": {
       "id": "custom-bazel",
       "name": "Custom Bazel",
       "extends": "bazel",  // Inherit from base Bazel
       "file_patterns": ["BUILD.custom"],
       "priority": 120
     },
     "symbol_overrides": [
       {
         "name": "cc_library",
         "override_params": {
           "company_config": {
             "type": "string",
             "default_value": "\"production\""
           }
         }
       }
     ]
   }
   ```

4. **Plugin Dependencies**:
   - Plugins can declare dependencies on other plugins
   - Automatic loading of required base plugins
   - Version compatibility checking

**Effort**: 5-6 days
**Dependencies**: P1.3, P2.2
**Risk**: High (complex cross-cutting concerns)

---

## 🚀 P3 - NICE TO HAVE (Future Vision)

### P3.1 - Plugin Registry and Distribution 📦 **ECOSYSTEM**

**Problem**: No centralized way to discover, install, and manage community plugins.

**Implementation Plan**:

1. **Plugin Registry Service**:
   ```bash
   # Plugin discovery and installation
   starpls search tilt          # Find Tilt-related plugins
   starpls install tilt-official # Install from registry
   starpls list                 # Show installed plugins
   starpls update               # Update all plugins
   starpls uninstall tilt-official
   ```

2. **Plugin Manifest Standard**:
   ```json
   // plugin-manifest.json
   {
     "name": "tilt-dialect",
     "version": "1.2.3",
     "author": "Tilt Team",
     "license": "Apache-2.0",
     "description": "Official Tilt language support for starpls",
     "keywords": ["tilt", "kubernetes", "docker"],
     "homepage": "https://tilt.dev",
     "repository": "https://github.com/tilt-dev/starpls-plugin",
     "starpls_version": ">=0.2.0",
     "dependencies": {
       "bazel-base": "^1.0.0"
     },
     "files": {
       "dialect": "tilt-dialect.json",
       "documentation": "README.md",
       "examples": "examples/"
     }
   }
   ```

3. **Registry API**:
   ```http
   GET /api/v1/plugins?q=tilt
   GET /api/v1/plugins/tilt-dialect/versions
   GET /api/v1/plugins/tilt-dialect/1.2.3/download
   POST /api/v1/plugins (publish new plugin)
   ```

4. **Plugin Publishing Workflow**:
   ```bash
   # For plugin authors
   starpls login
   starpls publish                    # Publish current directory
   starpls publish --tag latest       # Tag as latest version
   starpls unpublish tilt-dialect@1.0.0  # Remove version
   ```

**Effort**: 8-10 days (requires backend service)
**Dependencies**: P2.1, P2.2
**Risk**: High (infrastructure requirements)

### P3.2 - Hot Plugin Reloading 🔄 **DEVELOPMENT EXPERIENCE**

**Problem**: Plugin developers must restart starpls to test changes, slowing development iteration.

**Implementation Plan**:

1. **File System Watching**:
   ```rust
   use notify::{Watcher, RecommendedWatcher, RecursiveMode};

   struct PluginWatcher {
       watcher: RecommendedWatcher,
       registry: Arc<Mutex<DialectRegistry>>,
   }

   impl PluginWatcher {
       fn watch_plugin_files(&mut self, paths: Vec<PathBuf>) {
           for path in paths {
               self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
           }
       }

       fn handle_file_change(&mut self, path: &Path) {
           info!("Plugin file changed: {}", path.display());
           if let Err(e) = self.reload_plugin(path) {
               error!("Failed to reload plugin: {}", e);
               // Send LSP notification to client
           }
       }
   }
   ```

2. **Incremental Plugin Reloading**:
   - Reload only changed plugins, keep others intact
   - Preserve client state (open files, completions cache)
   - Handle reload failures gracefully (keep old version)

3. **Client Notifications**:
   ```typescript
   // LSP client notification
   interface PluginReloadedNotification {
       method: 'starpls/pluginReloaded';
       params: {
           pluginPath: string;
           success: boolean;
           error?: string;
       };
   }
   ```

4. **Development Mode**:
   ```bash
   # Enable hot reloading for development
   starpls server --dev-mode --watch-plugins
   ```

**Effort**: 3-4 days
**Dependencies**: P0.2, P1.2
**Risk**: Medium (file watching complexity)

### P3.3 - Visual Plugin Builder 🎨 **USER INTERFACE**

**Problem**: JSON editing for plugins is technical and intimidating for non-programmers.

**Implementation Plan**:

1. **Web-Based Plugin Builder**:
   ```typescript
   // React/Vue component for visual plugin creation
   interface PluginBuilderProps {
       onSave: (plugin: DialectPlugin) => void;
       initialPlugin?: DialectPlugin;
   }

   // Features:
   // - Drag-and-drop function creation
   // - Visual parameter editing
   // - Real-time JSON preview
   // - Built-in validation and error display
   // - Plugin testing interface
   ```

2. **Plugin Templates**:
   - Pre-built templates for common patterns
   - One-click generation for popular tools
   - Community-contributed templates

3. **Plugin Documentation Generator**:
   - Automatic API documentation from plugin definitions
   - Interactive examples and demos
   - Integration with plugin registry

4. **VS Code Extension Integration**:
   ```json
   // VS Code command palette
   "commands": [
     {
       "command": "starpls.createPlugin",
       "title": "Starpls: Create New Plugin"
     },
     {
       "command": "starpls.editPlugin",
       "title": "Starpls: Edit Plugin"
     }
   ]
   ```

**Effort**: 10+ days (requires frontend development)
**Dependencies**: P2.2, P2.3
**Risk**: Medium (UI/UX complexity)

### P3.4 - Advanced Type System Features 🧠 **LANGUAGE THEORY**

**Problem**: Starlark's type system is basic, but plugins could enable more sophisticated typing.

**Implementation Plan**:

1. **Generic Type Support**:
   ```json
   {
     "name": "map_files",
     "callable": {
       "type_parameters": ["T", "U"],
       "params": [
         {
           "name": "files",
           "type": "list[T]"
         },
         {
           "name": "transform",
           "type": "function(T) -> U"
         }
       ],
       "return_type": "list[U]"
     }
   }
   ```

2. **Protocol/Interface Support**:
   ```json
   {
     "name": "Buildable",
     "kind": "protocol",
     "methods": [
       {
         "name": "build",
         "callable": {
           "params": [],
           "return_type": "BuildResult"
         }
       }
     ]
   }
   ```

3. **Type Guards and Narrowing**:
   ```python
   # Plugin could define type guards
   if isinstance(value, Resource):
       # Type checker knows value is Resource here
       value.deploy()  # Method available
   ```

4. **Macro and Code Generation Support**:
   - Plugins can define macro-like functions
   - Static analysis of generated code
   - Template-based code expansion

**Effort**: 8-10 days
**Dependencies**: P1.3, P2.2
**Risk**: Very High (research-level type system work)

---

## 🔧 TECHNICAL DEBT & INFRASTRUCTURE

### TD.1 - Comprehensive Testing Suite 🧪

**Current Gap**: Limited testing of plugin system integration.

**Implementation Plan**:

1. **Unit Tests**:
   - Plugin loading and validation
   - JSON schema compliance
   - Error handling edge cases

2. **Integration Tests**:
   - End-to-end plugin functionality
   - Multiple plugins interaction
   - File detection with plugins

3. **Performance Tests**:
   - Plugin loading time with many plugins
   - Memory usage with large plugin sets
   - Response time impact on LSP operations

4. **Regression Tests**:
   - Backward compatibility with existing Bazel code
   - Plugin API stability across versions

**Effort**: 4-5 days
**Risk**: Low

### TD.2 - Documentation Overhaul 📚

**Current Gap**: Plugin system lacks comprehensive documentation.

**Implementation Plan**:

1. **User Documentation**:
   - Plugin creation tutorial
   - JSON schema reference
   - CLI usage guide
   - Troubleshooting guide

2. **Developer Documentation**:
   - Architecture overview
   - Contributing guidelines
   - API reference
   - Extension points guide

3. **Community Documentation**:
   - Plugin registry guidelines
   - Best practices
   - Example gallery
   - FAQ and common issues

**Effort**: 3-4 days
**Risk**: Low

### TD.3 - Performance Profiling and Optimization 📊

**Current Gap**: No performance baselines or optimization.

**Implementation Plan**:

1. **Baseline Measurements**:
   - LSP operation latency without plugins
   - Memory usage baseline
   - Plugin loading time benchmarks

2. **Performance Monitoring**:
   - Built-in performance metrics
   - Optional telemetry collection
   - Performance regression detection

3. **Optimization Targets**:
   - Sub-100ms plugin loading
   - <1MB memory overhead per plugin
   - No measurable LSP latency impact

**Effort**: 2-3 days
**Risk**: Low

---

## 📋 EXECUTION STRATEGY

### Phase Sequencing

1. **Week 1-2**: Complete P0 items (Critical Path)
   - Establish working end-to-end plugin functionality
   - Validate architecture with real usage

2. **Week 3-4**: Implement P1 items (Production Ready)
   - Robust error handling and performance
   - Production-grade reliability

3. **Week 5-8**: Selected P2 items based on user feedback
   - Configuration file support (high user value)
   - Advanced plugin features (ecosystem growth)

4. **Beyond**: P3 items as long-term vision
   - Plugin registry and ecosystem
   - Advanced tooling and UI

### Success Metrics

**Technical Metrics**:
- Plugin loading success rate >95%
- LSP operation latency increase <10%
- Memory overhead <5MB per plugin
- Zero crashes from plugin failures

**User Experience Metrics**:
- Plugin creation time <30 minutes for simple cases
- Community plugin contributions >10 within 6 months
- User satisfaction score >4.5/5

**Ecosystem Metrics**:
- Support for 5+ Starlark dialects
- 100+ community-contributed symbols
- Integration with 3+ popular development tools

---

## 🎯 IMMEDIATE NEXT STEPS

1. **Start with P0.1** - File detection integration
   - This unblocks all other features
   - Relatively contained change
   - High visibility impact

2. **Validate with real usage** - Create comprehensive test cases
   - Test with actual Tiltfiles and custom Bazel rules
   - Ensure plugin system works as intended
   - Get early feedback from potential users

3. **Document as you go** - Update documentation with each change
   - Keep architecture decisions recorded
   - Provide examples for each new feature
   - Enable community contributions

The roadmap is aggressive but achievable. The foundation is solid, and each piece builds naturally on the previous work. The key is maintaining backward compatibility while enabling the plugin ecosystem to flourish.

**Ready to transform starpls into the universal Starlark language server! 🚀**