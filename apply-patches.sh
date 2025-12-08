#!/bin/bash
# apply-patches.sh
# This script applies custom patches to the generated OpenAPI client code.
# It should be run after code generation to preserve necessary modifications
# that are not possible through OpenAPI spec customization.
#
# Usage: ./apply-patches.sh [VERSION]
#   VERSION: Optional version number to set in Cargo.toml (e.g., "0.1.2")

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PATCHES_DIR="${SCRIPT_DIR}/patches"
VERSION="${1:-}"

echo "Applying custom patches to generated code..."

# Replace Cargo.toml with our version that includes dev-dependencies
# The OpenAPI Generator overwrites Cargo.toml with a minimal version
CARGO_FILE="${SCRIPT_DIR}/Cargo.toml"
CARGO_PATCH="${PATCHES_DIR}/Cargo.toml"
if [ -f "$CARGO_PATCH" ]; then
    echo "Replacing Cargo.toml with patched version..."
    cp "$CARGO_PATCH" "$CARGO_FILE"
    # Update version if provided as argument
    if [ -n "$VERSION" ]; then
        echo "  Updating version to $VERSION..."
        sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$CARGO_FILE"
    fi
    echo "  ✓ Replaced Cargo.toml (preserves dev-dependencies and custom metadata)"
fi

# Fix serde_as compilation errors in credentials.rs
# The OpenAPI Generator incorrectly generates serde_as attributes that cause compilation errors.
# We replace the entire generated file with a corrected version stored in patches/.
CREDENTIALS_FILE="${SCRIPT_DIR}/src/models/credentials.rs"
CREDENTIALS_PATCH="${PATCHES_DIR}/credentials.rs"
if [ -f "$CREDENTIALS_PATCH" ]; then
    echo "Replacing credentials.rs with corrected version..."
    cp "$CREDENTIALS_PATCH" "$CREDENTIALS_FILE"
    echo "  ✓ Replaced credentials.rs with corrected version (fixes serde_as issues)"
fi

# Apply patch files
if [ -d "$PATCHES_DIR" ]; then
    # Apply .patch files
    for patch_file in "$PATCHES_DIR"/*.patch; do
        if [ -f "$patch_file" ]; then
            echo "Applying patch: $(basename "$patch_file")"
            # Check if patch can be applied (ignoring "already applied" warnings)
            if patch -p1 -N --dry-run --silent < "$patch_file" 2>&1 | grep -q "Reversed (or previously applied) patch detected"; then
                echo "  ⚠ Patch $(basename "$patch_file") already applied, skipping"
            elif patch -p1 -N --dry-run --silent < "$patch_file" 2>&1; then
                # Apply the patch for real
                patch -p1 -N < "$patch_file"
                echo "  ✓ Successfully applied $(basename "$patch_file")"
            else
                echo "  ✗ Failed to apply $(basename "$patch_file") - conflicts detected"
                echo "  Please review and resolve conflicts manually"
                exit 1
            fi
        fi
    done

    # Copy example files that don't exist in generated code
    if [ -f "$PATCHES_DIR/droplet_with_ssh_keys.rs" ]; then
        echo "Copying example file: droplet_with_ssh_keys.rs"
        mkdir -p examples
        cp "$PATCHES_DIR/droplet_with_ssh_keys.rs" examples/
        echo "  ✓ Successfully copied droplet_with_ssh_keys.rs to examples/"
    fi
else
    echo "No patches directory found at $PATCHES_DIR"
    exit 1
fi

echo "Patch application completed successfully!"
