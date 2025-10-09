# Quick Start: Publishing glin-forge

**TL;DR**: How to publish glin-forge packages right now.

## 📦 Current Status

✅ **CLI** - Builds successfully (18MB binary)
✅ **@glin-forge/sdk** - Builds successfully
✅ **@glin-forge/testing** - Builds successfully
✅ **glin-forge (npm wrapper)** - Ready to publish

## 🚀 Publish Now (Manual Process)

### Step 1: Prepare

```bash
cd /home/eralp/Projects/glin/glin-forge

# Ensure everything builds
cargo build --release
cd packages/sdk && npm run build && cd ../..
cd packages/testing && npm install --legacy-peer-deps && npm run build && cd ../..
```

### Step 2: Build Binary for Your Platform

```bash
# Current platform (Linux x86_64)
cargo build --release

# Create tarball
VERSION="0.1.0"
PLATFORM="linux-x86_64"
mkdir -p dist
cd target/release
tar -czf ../../dist/glin-forge-${VERSION}-${PLATFORM}.tar.gz glin-forge
cd ../..
```

### Step 3: Create GitHub Repository

```bash
# Initialize git if not already done
git init
git add .
git commit -m "Initial commit: glin-forge v0.1.0"

# Create GitHub repo and push
# Via GitHub CLI:
gh repo create glin-ai/glin-forge --public --source=. --push

# Or manually:
git remote add origin https://github.com/glin-ai/glin-forge.git
git branch -M main
git push -u origin main
```

### Step 4: Create GitHub Release

```bash
VERSION="0.1.0"
git tag "v${VERSION}"
git push origin "v${VERSION}"

# Create release with binary
gh release create "v${VERSION}" \
  --title "glin-forge v${VERSION}" \
  --notes "Initial release of glin-forge - Hardhat-style development environment for GLIN Network smart contracts" \
  dist/glin-forge-${VERSION}-linux-x86_64.tar.gz
```

### Step 5: Publish to npm

```bash
# Login to npm
npm login

# Publish SDK
cd packages/sdk
npm publish --access public

# Publish Testing
cd ../testing
npm publish --access public

# Publish CLI wrapper
cd ../cli
npm publish --access public

cd ../..
```

### Step 6: Publish to crates.io

```bash
# Login to crates.io
cargo login

# Publish
cargo publish
```

### Step 7: Test!

```bash
# Test npx
cd /tmp
npx glin-forge@latest init test-project

# Should work! (Will download the binary from GitHub release)
```

## ⚡ Quick Commands

```bash
# Build everything
cargo build --release && \
cd packages/sdk && npm run build && cd ../.. && \
cd packages/testing && npm install --legacy-peer-deps && npm run build && cd ../..

# Create release
VERSION="0.1.0"
mkdir -p dist
cd target/release && tar -czf ../../dist/glin-forge-${VERSION}-linux-x86_64.tar.gz glin-forge && cd ../..
git tag "v${VERSION}"
git push origin "v${VERSION}"
gh release create "v${VERSION}" --title "glin-forge v${VERSION}" dist/*.tar.gz

# Publish npm
cd packages/sdk && npm publish --access public && cd ../..
cd packages/testing && npm publish --access public && cd ../..
cd packages/cli && npm publish --access public && cd ../..

# Publish cargo
cargo publish
```

## 🎯 What Users Will Do

After you publish, users can:

```bash
# Option 1: npx (no installation)
npx glin-forge init my-contract

# Option 2: Global install
npm install -g glin-forge
glin-forge init my-contract

# Option 3: Cargo
cargo install glin-forge
glin-forge init my-contract
```

## ⚠️ Important Notes

1. **GitHub Release First**: The npm CLI wrapper downloads binaries from GitHub releases, so you MUST create the GitHub release with binaries BEFORE publishing the npm package.

2. **Binary Naming**: Binaries must follow this pattern:
   - `glin-forge-{version}-{platform}-{arch}.tar.gz`
   - Example: `glin-forge-0.1.0-linux-x86_64.tar.gz`

3. **Version Consistency**: Use the same version across:
   - Cargo.toml
   - packages/sdk/package.json
   - packages/testing/package.json
   - packages/cli/package.json
   - Git tag

4. **First Publish**: For the first publish:
   - Create GitHub repo
   - Push code
   - Create release with binary
   - Publish npm packages
   - Publish to crates.io

5. **Subsequent Publishes**: Use GitHub Actions (already set up in `.github/workflows/release.yml`)

## 📋 Verification Checklist

After publishing:

- [ ] GitHub repo exists and is public
- [ ] GitHub release v0.1.0 exists with binary
- [ ] npm package `glin-forge` published
- [ ] npm package `@glin-forge/sdk` published
- [ ] npm package `@glin-forge/testing` published
- [ ] crates.io package published
- [ ] `npx glin-forge@latest init test` works

## 🐛 If Something Goes Wrong

```bash
# Unpublish from npm (within 72 hours)
npm unpublish glin-forge@0.1.0 --force
npm unpublish @glin-forge/sdk@0.1.0 --force
npm unpublish @glin-forge/testing@0.1.0 --force

# Yank from crates.io
cargo yank --vers 0.1.0 glin-forge

# Delete GitHub release
gh release delete v0.1.0 --yes

# Fix and try again with v0.1.1
```

## 📞 Need Help?

Check:
- `PUBLISHING_GUIDE.md` - Comprehensive guide
- `PRE_PUBLISH_CHECKLIST.md` - Detailed checklist
- `.github/workflows/release.yml` - Automated process

Ready to publish? Follow the steps above! 🚀
