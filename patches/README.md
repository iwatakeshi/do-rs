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
- **droplet_with_ssh_keys.rs**: Example file demonstrating SSH keys usage (copied to `examples/`)

### Automatic Fixes (sed-based)

The `apply-patches.sh` script also applies the following automatic fixes using `sed`:

- **credentials.rs serde_as fix**: Removes incorrectly generated `#[serde_as]` attribute and `use serde_with::serde_as;` import from `src/models/credentials.rs`. This fix is applied using `sed` rather than a patch file because the generated file structure varies, making context-dependent patches unreliable.

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
