# Publishing Guide

Complete guide to testing and publishing glin-forge packages.

## 📋 Pre-Publishing Checklist

- [ ] All code compiles without errors
- [ ] Version numbers updated in all `package.json` and `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] Tests pass
- [ ] Local testing complete
- [ ] Git tag created

## 🧪 Testing Locally

### Step 1: Build Everything

```bash
# Build Rust CLI
cargo build --release

# Verify CLI works
./target/release/glin-forge --version
./target/release/glin-forge --help

# Build TypeScript packages
cd packages/sdk
npm install
npm run build
cd ../..

cd packages/testing
npm install
npm run build
cd ../..
```

### Step 2: Test with npm link

```bash
# Run the test script
./scripts/test-local.sh

# Or manually:
cd packages/sdk
npm link
cd ../..

cd packages/testing
npm link
cd ../..

# Create test project
mkdir -p /tmp/test-project
cd /tmp/test-project
/path/to/glin-forge/target/release/glin-forge init my-test

# Link packages
cd my-test
npm link @glin-forge/sdk
npm link @glin-forge/testing

# Test commands
/path/to/glin-forge/target/release/glin-forge build
```

### Step 3: Clean up

```bash
npm unlink @glin-forge/sdk --global
npm unlink @glin-forge/testing --global
rm -rf /tmp/test-project
```

## 📦 Publishing Process

### Option A: Manual Publishing (First Time)

#### 1. Prepare Version

```bash
# Update version in all files
VERSION="0.1.0"

# Cargo.toml
# packages/sdk/package.json
# packages/testing/package.json
# packages/cli/package.json

git add .
git commit -m "Release v${VERSION}"
git tag "v${VERSION}"
git push origin main
git push origin "v${VERSION}"
```

#### 2. Build Release Binaries

Build for each platform:

```bash
# Linux x86_64 (native or with cross)
cargo build --release --target x86_64-unknown-linux-gnu

# macOS x86_64 (on macOS)
cargo build --release --target x86_64-apple-darwin

# macOS ARM64 (on macOS)
cargo build --release --target aarch64-apple-darwin

# Windows x86_64 (with cross)
cargo build --release --target x86_64-pc-windows-gnu

# Create tarballs
mkdir -p dist
cd target/x86_64-unknown-linux-gnu/release
tar -czf ../../../dist/glin-forge-${VERSION}-linux-x86_64.tar.gz glin-forge
# ... repeat for each platform
```

Or use the build script (requires `cross`):

```bash
./scripts/build-release.sh ${VERSION}
```

#### 3. Create GitHub Release

```bash
# Via GitHub CLI
gh release create "v${VERSION}" \
  --title "glin-forge v${VERSION}" \
  --notes "Release notes here" \
  dist/*.tar.gz

# Or manually:
# 1. Go to https://github.com/glin-ai/glin-forge/releases/new
# 2. Tag: v0.1.0
# 3. Title: glin-forge v0.1.0
# 4. Upload all tarballs from dist/
# 5. Publish release
```

#### 4. Publish to npm

```bash
# Login to npm (first time only)
npm login

# Publish SDK
cd packages/sdk
npm publish --access public

# Publish testing
cd ../testing
npm publish --access public

# Publish CLI wrapper
cd ../cli
npm publish --access public

cd ../..
```

Or use the publish script:

```bash
# Dry run first
./scripts/publish-npm.sh --dry-run

# Real publish
./scripts/publish-npm.sh
```

#### 5. Publish to crates.io

```bash
# Login to crates.io (first time only)
cargo login

# Publish
cargo publish
```

### Option B: Automated Publishing (GitHub Actions)

Once GitHub Actions is set up:

```bash
# 1. Update version numbers
VERSION="0.1.0"
# ... update all package.json and Cargo.toml

# 2. Commit and tag
git add .
git commit -m "Release v${VERSION}"
git tag "v${VERSION}"

# 3. Push
git push origin main
git push origin "v${VERSION}"

# 4. GitHub Actions will automatically:
#    - Build binaries for all platforms
#    - Create GitHub release
#    - Publish to npm
#    - Publish to crates.io
```

## 🔐 Required Secrets

For GitHub Actions to work, add these secrets to your repository:

1. **NPM_TOKEN**
   ```bash
   # Create token at https://www.npmjs.com/settings/YOUR_USERNAME/tokens
   # Settings → Secrets → New repository secret
   # Name: NPM_TOKEN
   # Value: npm_xxxxxxxxxxxxx
   ```

2. **CARGO_TOKEN**
   ```bash
   # Get token from https://crates.io/me
   # Settings → Secrets → New repository secret
   # Name: CARGO_TOKEN
   # Value: your_cargo_token
   ```

## ✅ Post-Publishing Verification

### Test npm Installation

```bash
# Test npx (fresh install)
cd /tmp
npx glin-forge@latest init test-npx
cd test-npx
npx glin-forge@latest build

# Test global install
npm install -g glin-forge@latest
glin-forge init test-global
cd test-global
glin-forge build

# Clean up
npm uninstall -g glin-forge
cd /tmp
rm -rf test-npx test-global
```

### Test Cargo Installation

```bash
# Test cargo install
cargo install glin-forge
glin-forge --version
glin-forge init test-cargo

# Clean up
cargo uninstall glin-forge
rm -rf test-cargo
```

### Verify Package Pages

- npm: https://www.npmjs.com/package/glin-forge
- npm: https://www.npmjs.com/package/@glin-forge/sdk
- npm: https://www.npmjs.com/package/@glin-forge/testing
- crates.io: https://crates.io/crates/glin-forge

## 🐛 Troubleshooting

### Binary download fails for users

If users report binary download failures:

1. Check GitHub release has all platform binaries
2. Verify tarball names match pattern in `download-binary.js`
3. Check release is not marked as draft/pre-release

### npm publish fails

```bash
# Check if logged in
npm whoami

# Check package name availability
npm view glin-forge
npm view @glin-forge/sdk

# Fix permissions
npm publish --access public
```

### Cargo publish fails

```bash
# Check if logged in
cargo login

# Verify Cargo.toml
cargo package --list

# Check for issues
cargo publish --dry-run
```

## 📊 Version Management

Follow semantic versioning (semver):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backwards compatible
- **Patch** (0.0.1): Bug fixes

Update versions in:
1. `Cargo.toml` (workspace and bin)
2. `packages/sdk/package.json`
3. `packages/testing/package.json`
4. `packages/cli/package.json`

## 🎯 Quick Reference

```bash
# Build and test locally
cargo build --release
./scripts/test-local.sh

# Publish manually
./scripts/build-release.sh 0.1.0
gh release create v0.1.0 dist/*.tar.gz
./scripts/publish-npm.sh
cargo publish

# Publish via GitHub Actions
git tag v0.1.0
git push origin v0.1.0
# Wait for Actions to complete

# Test installation
npx glin-forge@latest init test
```

## 📞 Support

If you encounter issues:
- Check GitHub Actions logs
- Verify secrets are set correctly
- Test locally first
- Check npm and crates.io status pages
