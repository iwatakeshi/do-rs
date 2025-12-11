# Patches Directory

This directory contains patches that are applied to the auto-generated OpenAPI client code after generation.

## Why Patches?

The DigitalOcean Rust client is auto-generated from the [DigitalOcean OpenAPI specification](https://github.com/digitalocean/openapi) using [OpenAPI Generator](https://openapi-generator.tech/). However, some aspects of the API cannot be accurately represented in the OpenAPI spec or require custom implementations that differ from the generator's output.

Patches allow us to:
1. Fix union types that the OpenAPI spec doesn't model correctly
2. Add custom convenience methods and trait implementations
3. Improve documentation with Rust-specific examples
4. Preserve these customizations across code regeneration cycles

## Patch Files

Patches are applied in numerical order:

- **001-droplet-ssh-keys-union-type.patch**: Converts `DropletCreateSshKeysInner` from an empty struct to an enum supporting both integer IDs and string fingerprints
- **002-droplet-ssh-keys-tests.patch**: Adds comprehensive tests for the SSH keys union type
- **003-size-networking-throughput.patch**: Adds the missing `networking_throughput` field to the `Size` struct. This field is returned by the DigitalOcean API but not defined in the OpenAPI spec, causing deserialization failures for `DropletsCreate202Response`.
- **004-action-link-id-i64.patch**: Changes `ActionLink.id` from `i32` to `i64` to handle action IDs that exceed `i32::MAX`.
- **005-image-regions-string.patch**: Changes `Image.regions` from `Vec<RegionSlug>` to `Vec<String>` for forward compatibility with new regions like `atl1` not yet in the enum.
- **006-pagination-module.patch**: Adds `pub mod pagination;` export to `lib.rs` for the pagination helper module.
- **droplet_with_ssh_keys.rs**: Example file demonstrating SSH keys usage (copied to `examples/`)

### File Replacements

The `apply-patches.sh` script also replaces certain generated files with corrected versions:

- **Cargo.toml**: Replaces the generated `Cargo.toml` with our version that includes dev-dependencies (`tokio`, `wiremock`) and custom package metadata. The version number is updated automatically when the script is called with a version argument.
- **credentials.rs**: Replaces `src/models/credentials.rs` with a corrected version that doesn't include the broken `#[serde_as]` attributes. The OpenAPI Generator incorrectly generates `serde_as` attributes with `Vec<u8>` fields that cause compilation errors. The corrected version uses `Option<String>` fields instead.
- **pagination.rs**: Adds `src/pagination.rs` - a custom pagination helper module with utilities for navigating paginated API responses, including `PaginatedResponse` trait, `PageRequest` builder, and URL parsing helpers.

## How Patches Work

### Automatic Application

Patches are automatically applied during the quarterly release workflow in `.github/workflows/quarterly-release.yml`. After the OpenAPI Generator creates fresh code, the `apply-patches.sh` script runs to apply all customizations.

### Manual Application

To apply patches manually after regenerating the code:

```bash
./apply-patches.sh
```

The script will:
1. Apply all `.patch` files from this directory using `patch -p1`
2. Copy any standalone files (like examples) to their destination
3. Skip patches that are already applied

## Creating New Patches

When you need to add a new customization:

1. **Make your changes** to the generated code
2. **Create a unified diff patch**:
   ```bash
   git diff src/path/to/file.rs > patches/00X-description.patch
   ```
3. **Test the patch** by reverting your changes and applying the patch:
   ```bash
   git checkout src/path/to/file.rs
   patch -p1 < patches/00X-description.patch
   ```
4. **Update this README** with information about the new patch
5. **Commit the patch file** to version control

## Patch Naming Convention

- Use three-digit prefixes (001, 002, 003, etc.) for ordering
- Use descriptive names: `00X-feature-description.patch`
- For standalone files, use the target filename without a prefix

## Maintenance

When the OpenAPI specification changes, patches might need to be updated:

1. Regenerate the code
2. Try applying patches with `./apply-patches.sh`
3. If patches fail, manually resolve conflicts and recreate the patch files
4. Run tests to ensure everything works: `cargo test`

## Testing Patches

After applying patches, always run:

```bash
# Check that code compiles
cargo check

# Run all tests
cargo test

# Build documentation
cargo doc --no-deps
```

## Related Issues

- #5: Implement SSH keys union type for Droplet creation - Original implementation of the SSH keys enum
