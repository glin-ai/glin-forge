# Pre-Publish Checklist ✅

Complete this checklist before publishing to npm and crates.io.

## 🔧 Build Verification

- [x] **CLI builds successfully**
  ```bash
  cargo build --release
  # Binary: target/release/glin-forge (18MB)
  # Version: 0.1.0
  ```

- [x] **SDK builds successfully**
  ```bash
  cd packages/sdk
  npm install
  npm run build
  # Output: dist/ directory with .js and .d.ts files
  ```

- [x] **Testing package builds successfully**
  ```bash
  cd packages/testing
  npm install
  npm run build
  # Output: dist/ directory with .js and .d.ts files
  ```

- [ ] **CLI wrapper package is ready**
  ```bash
  cd packages/cli
  # Check package.json, bin/glin-forge.js, scripts/download-binary.js
  ```

## 📝 Version Numbers

Update version in all files to match (e.g., `0.1.0`):

- [ ] `Cargo.toml` (root)
- [ ] `packages/sdk/package.json`
- [ ] `packages/testing/package.json`
- [ ] `packages/cli/package.json`

## 📚 Documentation

- [ ] `README.md` updated with current features
- [ ] `CHANGELOG.md` created/updated with changes
- [ ] `PUBLISHING_GUIDE.md` reviewed
- [ ] Example projects updated
- [ ] Phase 4 documentation complete

## 🧪 Local Testing

### Test CLI Binary

```bash
./target/release/glin-forge --version
./target/release/glin-forge --help
./target/release/glin-forge init /tmp/test-cli --yes
cd /tmp/test-cli
../../../target/release/glin-forge build  # Should work or show helpful error
```

- [ ] CLI version shows correctly
- [ ] Help text displays
- [ ] Init creates project structure
- [ ] Basic commands work

### Test npm Packages with npm link

```bash
# Run comprehensive test
./scripts/test-local.sh

# Or manually test
cd packages/sdk && npm link && cd ../..
cd packages/testing && npm link && cd ../..

mkdir -p /tmp/test-npm && cd /tmp/test-npm
npm init -y
npm link @glin-forge/sdk
npm link @glin-forge/testing

# Create test file
cat > test.js << 'EOF'
const sdk = require('@glin-forge/sdk');
const testing = require('@glin-forge/testing');
console.log('SDK version:', sdk.version);
console.log('SDK exports:', Object.keys(sdk));
console.log('Testing exports:', Object.keys(testing));
EOF

node test.js
```

- [ ] SDK can be imported
- [ ] Testing can be imported
- [ ] No import errors
- [ ] Types are available

## 🔐 Credentials Setup

### npm Authentication

```bash
# Login to npm
npm login

# Verify
npm whoami
# Should show your username

# Check access to @glin-forge scope
npm access ls-packages
```

- [ ] Logged into npm
- [ ] Access to @glin-forge organization (or public scope)

### crates.io Authentication

```bash
# Get token from https://crates.io/me
cargo login

# Verify Cargo.toml is ready
cargo publish --dry-run
```

- [ ] Logged into crates.io
- [ ] Dry run succeeds

### GitHub Secrets (for Actions)

In GitHub repository settings > Secrets and variables > Actions:

- [ ] `NPM_TOKEN` - npm access token
- [ ] `CARGO_TOKEN` - crates.io token
- [ ] Tokens have publish permissions

## 🏷️ Git Preparation

```bash
VERSION="0.1.0"

# Check status
git status

# Commit all changes
git add .
git commit -m "Release v${VERSION}"

# Create and push tag
git tag "v${VERSION}"
git push origin main
git push origin "v${VERSION}"
```

- [ ] All changes committed
- [ ] Tag created
- [ ] Tag pushed (triggers GitHub Actions)

## 🚀 Release Binaries

### Option A: Manual Build

```bash
VERSION="0.1.0"

# Build for current platform
cargo build --release

# Create tarball
cd target/release
tar -czf glin-forge-${VERSION}-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz glin-forge
```

For all platforms (requires cross-compilation setup):
```bash
./scripts/build-release.sh ${VERSION}
```

- [ ] Binaries built for all platforms:
  - [ ] linux-x86_64
  - [ ] linux-aarch64
  - [ ] macos-x86_64
  - [ ] macos-aarch64
  - [ ] windows-x86_64

### Option B: GitHub Actions (Recommended)

- [ ] Tag pushed (triggers workflow)
- [ ] Workflow runs successfully
- [ ] Binaries appear in GitHub Release

## 📦 Publishing

### npm Packages

```bash
# Dry run first
./scripts/publish-npm.sh --dry-run

# Real publish
./scripts/publish-npm.sh
```

Or manually:
```bash
cd packages/sdk && npm publish --access public
cd ../testing && npm publish --access public
cd ../cli && npm publish --access public
```

- [ ] @glin-forge/sdk published
- [ ] @glin-forge/testing published
- [ ] glin-forge (CLI wrapper) published

### crates.io

```bash
cargo publish

# If workspace, publish from root
cargo publish -p glin-forge
```

- [ ] glin-forge published to crates.io

### GitHub Release

```bash
# Via gh CLI
gh release create "v${VERSION}" \
  --title "glin-forge v${VERSION}" \
  --notes "See CHANGELOG.md for details" \
  dist/*.tar.gz
```

Or via GitHub web interface:
- [ ] Release created with tag v0.1.0
- [ ] All platform binaries attached
- [ ] Release notes added
- [ ] Published (not draft)

## ✅ Post-Publish Verification

### Test npx Installation

```bash
# Fresh test (no prior installation)
cd /tmp/test-npx-$$
npx glin-forge@latest init test-project

# Should:
# 1. Download glin-forge npm package
# 2. Download platform binary
# 3. Create project
# 4. Install @glin-forge/sdk and @glin-forge/testing

cd test-project
npx glin-forge@latest --version
npx glin-forge@latest build  # May fail if no contract, but CLI should run
```

- [ ] npx works without prior installation
- [ ] Binary downloads automatically
- [ ] Project created successfully
- [ ] SDK packages installed

### Test Global Install

```bash
npm install -g glin-forge@latest
glin-forge --version
glin-forge init /tmp/test-global
cd /tmp/test-global
glin-forge build
```

- [ ] Global install works
- [ ] Binary works globally
- [ ] Commands execute correctly

### Test Cargo Install

```bash
cargo install glin-forge
glin-forge --version
```

- [ ] Cargo install works
- [ ] Version matches

### Verify Package Pages

- [ ] https://www.npmjs.com/package/glin-forge
- [ ] https://www.npmjs.com/package/@glin-forge/sdk
- [ ] https://www.npmjs.com/package/@glin-forge/testing
- [ ] https://crates.io/crates/glin-forge
- [ ] https://github.com/glin-ai/glin-forge/releases

## 📢 Announcement

- [ ] Tweet/announce on social media
- [ ] Update documentation website
- [ ] Notify community on Discord/Telegram
- [ ] Write blog post (optional)

## 🎉 Success!

All checks passed! glin-forge is now available via:

```bash
# npx (recommended)
npx glin-forge init my-project

# npm global
npm install -g glin-forge

# cargo
cargo install glin-forge
```

## 🐛 Rollback Plan

If critical issues are found after publishing:

```bash
# Unpublish from npm (within 72 hours)
npm unpublish glin-forge@0.1.0
npm unpublish @glin-forge/sdk@0.1.0
npm unpublish @glin-forge/testing@0.1.0

# Yank from crates.io (doesn't remove, marks as bad)
cargo yank --vers 0.1.0 glin-forge

# Delete GitHub release
gh release delete v0.1.0

# Delete git tag
git tag -d v0.1.0
git push origin :refs/tags/v0.1.0
```

Then fix issues and republish as v0.1.1.
