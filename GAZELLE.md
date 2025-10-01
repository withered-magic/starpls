# Gazelle Support for Rust in Starpls

## Current Status

This repository **does not currently have Gazelle setup** for Rust. All BUILD files are maintained manually.

### What is Gazelle?

[Gazelle](https://github.com/bazelbuild/bazel-gazelle) is a build file generator for Bazel projects. It can automatically generate and update BUILD files by analyzing source code and dependencies, reducing the manual maintenance burden of keeping BUILD files in sync with code changes.

### Gazelle for Rust

While Gazelle has official support for Go and limited support for other languages, Rust support is provided through a third-party plugin called [`gazelle_rust`](https://github.com/Calsign/gazelle_rust).

## Current BUILD File Structure

The repository currently uses manually maintained BUILD files:

```
crates/
├── starpls/BUILD.bazel              # Main binary
├── starpls_bazel/BUILD.bazel        # Bazel-specific functionality
├── starpls_common/BUILD.bazel       # Common utilities
├── starpls_hir/BUILD.bazel          # High-level IR
├── starpls_ide/BUILD.bazel          # IDE functionality
├── starpls_intern/BUILD.bazel       # String interning
├── starpls_lexer/BUILD.bazel        # Lexical analysis
├── starpls_parser/BUILD.bazel       # Parser
├── starpls_syntax/BUILD.bazel       # Syntax tree
└── starpls_test_util/BUILD.bazel    # Test utilities
```

Each BUILD file contains:
- `rust_library` or `rust_binary` rules
- Manual dependency specifications
- Compile-time data and configuration
- Test configurations

## Enabling Gazelle Support

### Prerequisites

- **Bazel version**: 7.0.0+ (current project uses Bazel 7.x)
- **rules_rust version**: 0.40.0+ (✅ current project uses 0.53.0)
- **Crate Universe setup**: ✅ Already configured in `MODULE.bazel`

### Step 1: Add Gazelle Dependencies

Add the following to your `MODULE.bazel`:

```python
# Gazelle core
bazel_dep(name = "gazelle", version = "0.35.0")

# Gazelle Rust plugin (no official release yet, use latest commit)
git_override(
    module_name = "gazelle_rust",
    remote = "https://github.com/Calsign/gazelle_rust.git",
    commit = "main",  # Use latest commit hash for stability
)
bazel_dep(name = "gazelle_rust", version = "0.0.0")
```

### Step 2: Create Root BUILD File Configuration

Create or update the root `BUILD.bazel` file:

```python
load("@gazelle//:def.bzl", "gazelle")
load("@gazelle_rust//:def.bzl", "rust_gazelle")

# Standard Gazelle for overall project structure
gazelle(
    name = "gazelle",
    gazelle = "@gazelle_rust//gazelle:gazelle_rust",
)

# Rust-specific Gazelle configuration
rust_gazelle(
    name = "gazelle_rust",
    cargo_toml = "//:Cargo.toml",  # If you have a workspace Cargo.toml
    # Alternative: specify individual crates
    # crates = [
    #     "//crates/starpls:Cargo.toml",
    #     "//crates/starpls_bazel:Cargo.toml",
    #     # ... other crates
    # ],
)
```

### Step 3: Configure Gazelle Directives

Add gazelle directives to guide BUILD file generation. Create or update `BUILD.bazel` files with:

```python
# gazelle:rust_crate_universe_resolver
# gazelle:rust_lockfile Cargo.Bazel.lock

# For each crate directory, add specific directives:
# gazelle:rust_crate_name starpls_bazel
# gazelle:rust_edition 2021
```

### Step 4: Workspace Configuration

If using a workspace-style Cargo.toml, create one at the root:

```toml
[workspace]
members = [
    "crates/starpls",
    "crates/starpls_bazel",
    "crates/starpls_common",
    "crates/starpls_hir",
    "crates/starpls_ide",
    "crates/starpls_intern",
    "crates/starpls_lexer",
    "crates/starpls_parser",
    "crates/starpls_syntax",
    "crates/starpls_test_util",
]

[workspace.dependencies]
# Common dependencies can be specified here
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
# ... other shared dependencies
```

### Step 5: Individual Crate Configuration

For each crate, ensure `Cargo.toml` files have proper metadata:

```toml
[package]
name = "starpls_bazel"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow.workspace = true
serde.workspace = true
# ... other dependencies

# For proto generation
[build-dependencies]
prost-build = "0.11"
tonic-build = "0.9"
```

### Step 6: Running Gazelle

After setup, generate BUILD files:

```bash
# Generate all BUILD files
bazel run //:gazelle

# Update specific directories
bazel run //:gazelle -- crates/starpls_bazel

# Fix imports and dependencies
bazel run //:gazelle -- -fix

# Preview changes without writing
bazel run //:gazelle -- -print_diff
```

## Advanced Configuration

### Custom Rules and Dependencies

For special cases like protocol buffers (used in `starpls_bazel`), add custom directives:

```python
# In crates/starpls_bazel/BUILD.bazel
# gazelle:rust_proto_library builtin_proto_rust
# gazelle:rust_proto_library build_proto_rust
```

### External Dependencies

Gazelle will automatically handle Cargo dependencies through Crate Universe, but for system dependencies or non-Rust dependencies:

```python
# gazelle:rust_sys_deps openssl-sys
# gazelle:rust_build_script_deps cc
```

### Exclusions

To prevent Gazelle from modifying certain files:

```python
# gazelle:exclude integration_tests/
# gazelle:rust_ignore_file some_file.rs
```

## Current Project Compatibility

### ✅ Compatible Features

- **Crate Universe**: Already using `crate_universe` for dependency management
- **Module System**: Using `MODULE.bazel` (bzlmod) which is supported
- **Rules Rust Version**: Using 0.53.0 which exceeds minimum requirement (0.40.0)
- **Standard Crate Structure**: All crates follow standard Rust conventions

### ⚠️ Potential Challenges

1. **Proto Generation**: The `starpls_bazel` crate uses custom proto compilation which may need manual configuration
2. **Complex Dependencies**: Some inter-crate dependencies might need fine-tuning
3. **Custom Flags**: Current `rustc_flags = ["--cfg=bazel"]` may need preservation
4. **Test Configuration**: Custom test setups might require manual directives

### 🔧 Required Adaptations

1. **Compile Data**: Current `compile_data` specifications need to be preserved
2. **Feature Flags**: Conditional compilation features need gazelle directives
3. **Binary vs Library**: Ensure gazelle correctly identifies binary crates vs libraries

## Migration Strategy

### Phase 1: Preparation
1. Create workspace `Cargo.toml` if desired
2. Standardize crate `Cargo.toml` files
3. Document current BUILD file customizations

### Phase 2: Setup
1. Add gazelle dependencies to `MODULE.bazel`
2. Create root BUILD configuration
3. Add initial gazelle directives

### Phase 3: Generation
1. Generate BUILD files in a separate branch
2. Compare with current manual BUILD files
3. Identify and preserve necessary customizations

### Phase 4: Validation
1. Ensure build continues to work: `bazel build //...`
2. Verify tests pass: `bazel test //...`
3. Check that all features work correctly

### Phase 5: Integration
1. Merge gazelle-generated BUILD files
2. Add gazelle to CI/CD pipeline
3. Document maintenance procedures

## Benefits of Adding Gazelle

### Development Efficiency
- **Automatic BUILD Updates**: Adding new dependencies or files automatically updates BUILD files
- **Reduced Manual Errors**: Less chance of forgetting to update BUILD files when refactoring
- **Consistent Structure**: Standardized BUILD file organization

### Maintenance
- **Dependency Tracking**: Automatic detection of new dependencies
- **Cleanup**: Removal of unused dependencies
- **Validation**: Ensures BUILD files stay in sync with source code

### Team Productivity
- **Reduced Review Overhead**: BUILD file changes become automated
- **Faster Iteration**: Less time spent on build configuration
- **Onboarding**: New team members don't need to learn BUILD file syntax

## Drawbacks and Considerations

### Learning Curve
- Team needs to learn gazelle directives and configuration
- Debugging gazelle-generated BUILD files can be complex

### Customization Limitations
- Some advanced BUILD patterns may not be supported
- Proto generation might need special handling
- Custom rules may require manual maintenance

### Tool Maturity
- `gazelle_rust` is not officially maintained by Google/Bazel team
- Fewer resources and community support compared to official languages
- May have bugs or missing features

## Maintenance with Gazelle

### Daily Workflow
```bash
# After adding new dependencies or files
bazel run //:gazelle

# Commit both source and BUILD file changes
git add .
git commit -m "Add new feature with auto-generated BUILD updates"
```

### CI Integration
```yaml
# .github/workflows/build.yml
- name: Check BUILD files are up to date
  run: |
    bazel run //:gazelle
    git diff --exit-code || (echo "BUILD files need updating. Run 'bazel run //:gazelle'" && exit 1)
```

### Periodic Maintenance
- Regularly update `gazelle_rust` to latest commit
- Review generated BUILD files for optimization opportunities
- Update gazelle directives as project evolves

## Conclusion

While this project doesn't currently use Gazelle, it is well-positioned to adopt `gazelle_rust` for automated BUILD file management. The existing Bazel setup with `rules_rust` 0.53.0 and Crate Universe provides a solid foundation.

The decision to adopt Gazelle should consider:
- **Team Size**: Larger teams benefit more from automation
- **Change Frequency**: Projects with frequent dependency changes see more benefit
- **Customization Needs**: Projects requiring heavily customized BUILD files may prefer manual control
- **Risk Tolerance**: Early adopters of `gazelle_rust` accept some tooling immaturity

For this project, a gradual migration approach in a feature branch would allow evaluation of benefits vs. complexity before full adoption.