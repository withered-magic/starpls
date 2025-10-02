#!/bin/bash
# Hermetic cargo wrapper script

set -euo pipefail

# Find the cargo binary from the Rust toolchain
RUNFILES_DIR="${RUNFILES_DIR:-${0}.runfiles}"
WORKSPACE_NAME="${TEST_WORKSPACE:-_main}"

# Try to find cargo in the Rust toolchain
CARGO_PATH=""
for toolchain_dir in "${RUNFILES_DIR}"/"${WORKSPACE_NAME}"/external/rules_rust*rust*darwin*nightly*/rust_toolchain/bin; do
    if [[ -f "${toolchain_dir}/cargo" ]]; then
        CARGO_PATH="${toolchain_dir}/cargo"
        break
    fi
done

# Alternative: look in the external directory
if [[ -z "${CARGO_PATH}" ]]; then
    for cargo_bin in "${RUNFILES_DIR}"/"${WORKSPACE_NAME}"/external/rules_rust*rust*/bin/cargo; do
        if [[ -f "${cargo_bin}" ]]; then
            CARGO_PATH="${cargo_bin}"
            break
        fi
    done
fi

# Alternative: use system PATH to find cargo (fallback)
if [[ -z "${CARGO_PATH}" ]]; then
    CARGO_PATH="$(which cargo 2>/dev/null || echo "")"
fi

if [[ -z "${CARGO_PATH}" || ! -f "${CARGO_PATH}" ]]; then
    echo "Error: Could not find cargo binary" >&2
    echo "Searched in toolchain directories but cargo was not found" >&2
    exit 1
fi

# Execute cargo with all arguments
exec "${CARGO_PATH}" "$@"