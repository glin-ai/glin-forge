# Release Process

This document describes the production-grade release process for glin-forge.

## Overview

glin-forge uses a synchronized versioning system across all packages:
- **Cargo package**: `glin-forge` on crates.io
- **npm packages**: `@glin-ai/forge-sdk`, `@glin-ai/forge-testing`, `glin-forge` on npmjs.com
- **GitHub releases**: Pre-built binaries for all platforms

All packages share the same version number, managed from a single source of truth: `version.json`.

## Prerequisites

Before releasing, ensure you have:

1. **GitHub Secrets Configured**:
   - `CARGO_TOKEN` - Token from crates.io (https://crates.io/me)
   - `NPM_TOKEN` - Token from npmjs.com (https://www.npmjs.com/settings/YOUR_USERNAME/tokens)

2. **Local Tools**:
   - Git (with push access to the repository)
   - jq (for JSON manipulation)
   - Bash 4.0+

3. **Permissions**:
   - Maintainer access to GitHub repository
   - Owner/member of `@glin-ai` org on npm
   - Owner of `glin-forge` crate on crates.io

## Quick Release Guide

### 1. Bump Version

Choose the appropriate version bump type:

```bash
# Patch release (0.2.0 → 0.2.1) - Bug fixes
./scripts/bump-version.sh patch

# Minor release (0.2.1 → 0.3.0) - New features (backwards compatible)
./scripts/bump-version.sh minor

# Major release (0.3.0 → 1.0.0) - Breaking changes
./scripts/bump-version.sh major
```

This will:
- Update `version.json`
- Sync all package.json files
- Sync Cargo.toml
- Show you what changed

### 2. Review Changes

```bash
git diff
```

Make sure:
- All version numbers are correct
- No unexpected changes

### 3. Run Tests

```bash
# Rust tests
cargo test --all-features

# TypeScript builds
cd packages/sdk && npm run build && cd ../..
cd packages/testing && npm run build && cd ../..

# Or use validation script
./scripts/validate-release.sh
```

### 4. Commit Version Bump

```bash
git add -A
git commit -m "Bump version to vX.Y.Z"
git push
```

### 5. Create Release

```bash
# Replace X.Y.Z with your version
./scripts/release.sh X.Y.Z
```

This will:
- Run validation checks
- Ask for confirmation
- Create and push a git tag
- Trigger the CI/CD workflow

### 6. Monitor Release

The GitHub Actions workflow will automatically:

1. ✅ **Validate** - Check version consistency and run tests
2. 🔨 **Build** - Build binaries for all platforms (Linux, macOS, Windows)
3. 📦 **Publish to crates.io** - Publish Rust package
4. 📦 **Publish to npm** - Publish all TypeScript packages
5. 🚀 **Create GitHub Release** - Upload binaries and generate release notes
6. ✔️ **Validate** - Verify all packages are published correctly

Monitor progress at: `https://github.com/glin-ai/glin-forge/actions`

## CI/CD Workflow

The release workflow is triggered on tag push matching `v*.*.*`:

```
┌─────────────┐
│   Tag Push  │
│   v0.2.1    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Validate   │  ← Check version sync, run tests, clippy, fmt
└──────┬──────┘
       │
       ├──────────────┬──────────────┬──────────────┐
       ▼              ▼              ▼              ▼
   ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐
   │ Linux  │    │ macOS  │    │Windows │    │  Test  │
   │ x86/ARM│    │x86/ARM │    │  x86   │    │   TS   │
   └────┬───┘    └────┬───┘    └────┬───┘    └────┬───┘
        │             │             │             │
        └─────────────┴─────────────┴─────────────┘
                      │
                      ▼
              ┌──────────────┐
              │Publish Cargo │  ← crates.io
              └──────┬───────┘
                     │
                     ▼
              ┌──────────────┐
              │ Publish npm  │  ← npmjs.com (3 packages)
              └──────┬───────┘
                     │
                     ▼
              ┌──────────────┐
              │GitHub Release│  ← Attach binaries
              └──────┬───────┘
                     │
                     ▼
              ┌──────────────┐
              │   Validate   │  ← Verify published
              └──────────────┘
```

## Manual Operations

### Sync Versions Only

If you need to sync versions without bumping:

```bash
./scripts/sync-version.sh
```

### Validate Before Release

Run pre-flight checks without releasing:

```bash
./scripts/validate-release.sh
```

### Dry Run Release

Test the CI/CD workflow without publishing:

1. Go to GitHub Actions
2. Click "Release" workflow
3. Click "Run workflow"
4. Check "Dry run" option
5. Enter version number
6. Run

This will build everything but skip publishing steps.

## Troubleshooting

### Version Mismatch Error

If CI fails with version mismatch:

```bash
# Re-sync all versions
./scripts/sync-version.sh

# Verify
./scripts/validate-release.sh

# Commit and push
git add -A
git commit -m "Fix version sync"
git push
```

### Failed npm Publish

If npm publish fails:

1. Check you're logged in: `npm whoami`
2. Check you're a member of `@glin-ai` org
3. Check NPM_TOKEN is set in GitHub secrets
4. Manually publish:
   ```bash
   cd packages/sdk && npm publish --access public
   cd ../testing && npm publish --access public
   cd ../cli && npm publish --access public
   ```

### Failed Cargo Publish

If cargo publish fails:

1. Check CARGO_TOKEN is set in GitHub secrets
2. Verify you own the crate on crates.io
3. Manually publish: `cargo publish`

### Binary Build Failures

If platform-specific builds fail:

1. Check the Actions log for the specific platform
2. Common issues:
   - Cross-compilation dependencies (ARM Linux)
   - macOS code signing
   - Windows line endings
3. Test locally with: `cargo build --release --target <target-triple>`

## Version Strategy

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes, incompatible API changes
- **MINOR** (0.x.0): New features, backwards compatible
- **PATCH** (0.0.x): Bug fixes, backwards compatible

Pre-1.0.0 versions may introduce breaking changes in minor releases.

## Post-Release

After a successful release:

1. **Verify Installation**:
   ```bash
   # Via npm
   npx glin-forge@X.Y.Z --version

   # Via cargo
   cargo install glin-forge@X.Y.Z
   glin-forge --version
   ```

2. **Update Documentation**: If needed, update docs to reference the new version

3. **Announce**: Share the release on:
   - GitHub Discussions
   - Twitter/X
   - Discord/Community channels

## Scripts Reference

| Script | Purpose | Usage |
|--------|---------|-------|
| `bump-version.sh` | Increment version | `./scripts/bump-version.sh [major\|minor\|patch]` |
| `sync-version.sh` | Sync all package versions | `./scripts/sync-version.sh` |
| `validate-release.sh` | Pre-flight validation | `./scripts/validate-release.sh` |
| `release.sh` | Create and push release tag | `./scripts/release.sh X.Y.Z` |

## Files Reference

| File | Purpose |
|------|---------|
| `version.json` | Single source of truth for version |
| `.github/workflows/release.yml` | CI/CD release workflow |
| `Cargo.toml` | Rust package version |
| `packages/*/package.json` | npm package versions |

## Emergency Rollback

If a release has critical issues:

1. **Yank from crates.io**:
   ```bash
   cargo yank --vers X.Y.Z glin-forge
   ```

2. **Deprecate npm packages**:
   ```bash
   npm deprecate @glin-ai/forge-sdk@X.Y.Z "Critical bug, use X.Y.Z+1 instead"
   npm deprecate @glin-ai/forge-testing@X.Y.Z "Critical bug, use X.Y.Z+1 instead"
   npm deprecate glin-forge@X.Y.Z "Critical bug, use X.Y.Z+1 instead"
   ```

3. **Mark GitHub release as draft**

4. **Create hotfix release** with fixed version

## Support

For issues with the release process:
- Check GitHub Actions logs
- Review this documentation
- Open an issue on GitHub
