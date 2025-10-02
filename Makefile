# Starpls Development Makefile

.PHONY: build test fmt clean run dev-build

# Default target
all: build

# Build the project
build:
	bazel build //...

# Build starpls binary specifically
build-starpls:
	bazel build //crates/starpls:starpls

# Run tests
test:
	bazel test //...

# Format code using hermetic rustfmt
fmt:
	bazel run @rules_rust//:rustfmt

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
	@echo "  test          - Run all tests"
	@echo "  fmt           - Format code with rustfmt"
	@echo "  clean         - Clean build artifacts"
	@echo "  dev-build     - Optimized build"
	@echo "  dev           - Format and build (development cycle)"
	@echo "  run           - Run starpls server"
	@echo "  help          - Show this help"