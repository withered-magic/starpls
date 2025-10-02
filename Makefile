# Starpls Development Makefile

.PHONY: build test fmt clean run dev-build check

# Default target
all: build

# Build the project
build:
	bazel build //...

# Build starpls binary specifically
build-starpls:
	bazel build //crates/starpls:starpls

# Check compilation with clippy and rustfmt (fast feedback)
check:
	bazel build //...

# Run tests
test:
	bazel test //...

# Format code using hermetic rustfmt
fmt:
	bazel run @rules_rust//:rustfmt

# Sync and repin crate dependencies after adding new dependencies
# See: https://bazelbuild.github.io/rules_rust/crate_universe_bzlmod.html
sync-deps:
	CARGO_BAZEL_REPIN=1 bazel sync --only=crates --enable_workspace

# Run hermetic cargo
cargo:
	bazel run //:cargo

# Run hermetic cargo check (fast compilation check)
cargo-check:
	bazel run //:cargo -- check

# Clean build artifacts
clean:
	bazel clean

# Development build (optimized)
dev-build:
	bazel build -c opt //crates/starpls:starpls

# Quick development cycle - format and build
dev: fmt build-starpls

# Run starpls with common development flags
run:
	./bazel-bin/crates/starpls/starpls server

# Help target
help:
	@echo "Available targets:"
	@echo "  build         - Build all targets"
	@echo "  build-starpls - Build starpls binary only"
	@echo "  check         - Check with clippy and rustfmt"
	@echo "  test          - Run all tests"
	@echo "  fmt           - Format code with rustfmt"
	@echo "  sync-deps     - Sync and repin crate dependencies"
	@echo "  cargo         - Run hermetic cargo"
	@echo "  cargo-check   - Run hermetic cargo check"
	@echo "  clean         - Clean build artifacts"
	@echo "  dev-build     - Optimized build"
	@echo "  dev           - Format and build (development cycle)"
	@echo "  run           - Run starpls server"
	@echo "  help          - Show this help"