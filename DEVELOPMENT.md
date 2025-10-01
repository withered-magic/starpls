# Development Guide for Starpls

## Code Formatting

### Automatic Formatting with Bazel

This project uses **hermetic rustfmt** through Bazel to ensure consistent code formatting across all environments, including CI/CD.

#### Running Code Formatting

To format all Rust source files in the project:

```bash
bazel run @rules_rust//:rustfmt
```

This command:
- ✅ Uses the **same rustfmt version** as CI/CD (hermetic/reproducible)
- ✅ Applies formatting to **all Rust files** in the workspace
- ✅ Follows the project's `rustfmt.toml` configuration
- ✅ Ensures **CI compliance** and prevents formatting-related build failures

#### Why Use Bazel's Hermetic Rustfmt?

**Problem with Local Rustfmt:**
- Different developers may have different rustfmt versions
- Local rustfmt might format differently than CI
- Formatting issues cause CI failures that are hard to debug

**Solution with Bazel's Hermetic Rustfmt:**
- **Consistent**: Same rustfmt version for all developers and CI
- **Reproducible**: Deterministic formatting across all environments
- **Automatic**: Handles all files in the workspace automatically

#### Development Workflow

**Before Committing Code:**
```bash
# Format all code
bazel run @rules_rust//:rustfmt

# Build and test
bazel build //...
bazel test //...

# Commit changes
git add .
git commit -m "your commit message"
```

**When CI Fails with Formatting Issues:**
```bash
# Apply the same formatting as CI
bazel run @rules_rust//:rustfmt

# Commit the formatting changes
git add crates/
git commit -m "fix: apply hermetic rustfmt formatting"
git push
```

#### Manual Rustfmt (Not Recommended)

If you need to format individual files manually, use the project's configuration:

```bash
rustfmt --config-path rustfmt.toml --edition 2021 path/to/file.rs
```

**However, prefer using Bazel's hermetic rustfmt for consistency.**

---

## Build System

### Building the Project

Build all targets:
```bash
bazel build //...
```

Build specific binary:
```bash
bazel build //crates/starpls:starpls
```

### Running Tests

Run all tests:
```bash
bazel test //...
```

Run tests for specific crate:
```bash
bazel test //crates/starpls_common/...
```

### Build Configuration

The project uses:
- **Bazel 8.x** with bzlmod (MODULE.bazel)
- **rules_rust 0.53.0** for Rust compilation
- **Crate Universe** for external dependency management
- **Hermetic toolchain** for reproducible builds

---

## Project Structure

```
crates/
├── starpls/              # Main binary and language server
├── starpls_bazel/        # Bazel-specific functionality
├── starpls_common/       # Common utilities and dialect system
├── starpls_hir/          # High-level intermediate representation
├── starpls_ide/          # IDE features (completion, hover, etc.)
├── starpls_intern/       # String interning
├── starpls_lexer/        # Lexical analysis
├── starpls_parser/       # Starlark parser
├── starpls_syntax/       # Syntax tree definitions
└── starpls_test_util/    # Test utilities
```

Each crate has its own `BUILD.bazel` file with dependencies and configuration.

---

## Code Quality

### Linting and Formatting

The project enforces code quality through:

1. **Rustfmt**: Automatic code formatting
   ```bash
   bazel run @rules_rust//:rustfmt
   ```

2. **Clippy**: Rust linting (runs during build)
   - Configured with `-Dwarnings` (treats warnings as errors)
   - Automatic fixes applied where possible

3. **Build Validation**: All code must compile without warnings

### Testing

- **Unit Tests**: Each crate includes comprehensive unit tests
- **Integration Tests**: End-to-end testing of language server functionality
- **CI Testing**: Automated testing on Linux and macOS

---

## Contribution Guidelines

### Before Submitting a Pull Request

1. **Format code**:
   ```bash
   bazel run @rules_rust//:rustfmt
   ```

2. **Build successfully**:
   ```bash
   bazel build //...
   ```

3. **Pass all tests**:
   ```bash
   bazel test //...
   ```

4. **Check for lint warnings**:
   - Clippy warnings will cause build failures
   - Fix any warnings that appear

### Commit Message Format

Use conventional commit format:
```
type: description

- Detailed explanation of changes
- Additional context if needed
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### Feature Development

For new features:
1. Create a feature branch: `feature/your-feature-name`
2. Implement changes with tests
3. Run formatting and validation
4. Submit pull request to `main`

---

## Debugging and Development Tools

### Language Server Development

Test the language server locally:
```bash
# Build the binary
bazel build //crates/starpls:starpls

# Run language server
./bazel-bin/crates/starpls/starpls server

# Run with debug logging
STARPLS_LOG=debug ./bazel-bin/crates/starpls/starpls server
```

### Bazel Development

Useful Bazel commands:
```bash
# Query all targets
bazel query //...

# Build with optimizations
bazel build -c opt //...

# Clean build cache
bazel clean

# Show build dependencies
bazel query 'deps(//crates/starpls:starpls)'
```

### IDE Setup

For VS Code development:
1. Install Rust Analyzer extension
2. Configure to use Bazel-built rust-analyzer if available
3. Set rustfmt path to use project configuration

---

## Continuous Integration

### GitHub Actions Workflows

The project includes several CI workflows:

1. **`build.yml`**: Main branch validation (Linux + Windows)
2. **`release.yml`**: Release builds for all platforms
3. **`feature-branch-build.yml`**: Feature branch validation

### CI Environment

- **Bazel**: Uses remote caching via BuildBuddy
- **Rustfmt**: Hermetic formatting validation
- **Testing**: Comprehensive test suite execution
- **Artifact Generation**: Platform-specific binaries

### Troubleshooting CI Failures

**Formatting Issues:**
```bash
bazel run @rules_rust//:rustfmt
git commit -am "fix: apply hermetic rustfmt formatting"
```

**Build Issues:**
- Check for clippy warnings in CI logs
- Verify all dependencies are properly declared in BUILD files
- Test locally with `bazel build //...`

**Test Failures:**
- Run tests locally: `bazel test //...`
- Check for environment-specific issues
- Review test logs in CI output

---

## Performance and Optimization

### Build Performance

- Use `bazel build -c opt` for optimized builds
- Remote caching reduces build times in CI
- Incremental builds handle most development changes efficiently

### Runtime Performance

- Profile with standard Rust tools (`cargo flamegraph`, `perf`)
- Use `bazel build -c opt` for performance testing
- Monitor memory usage with language server clients

---

## Release Process

### Version Management

Versions are managed through:
- Git tags for releases (`v0.1.x`)
- Automatic binary generation via GitHub Actions
- Multi-platform release artifacts

### Creating a Release

1. Update version in relevant files
2. Create git tag: `git tag v0.1.x`
3. Push tag: `git push origin v0.1.x`
4. GitHub Actions automatically creates release with binaries

---

## Additional Resources

- **Bazel**: https://bazel.build/
- **rules_rust**: https://github.com/bazelbuild/rules_rust
- **Starlark Language**: https://github.com/bazelbuild/starlark
- **Language Server Protocol**: https://microsoft.github.io/language-server-protocol/

For questions or issues, please check existing GitHub issues or create a new one.