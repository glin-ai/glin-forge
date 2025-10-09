# Publishing v0.2.0 - Phase 4 Features

Quick guide to publish v0.2.0 with all Phase 4 features (npx support, SDK, testing utilities).

## 🎯 What's New in v0.2.0

- ✨ **npx support** - No Cargo installation needed!
- 📦 **@glin-forge/sdk** - TypeScript SDK for deployment scripts
- 🧪 **@glin-forge/testing** - Comprehensive testing utilities
- 🎨 **Interactive init** - Beautiful CLI prompts
- 📊 **Full-stack templates** - React, Next.js, Vue
- 🔍 **analyze command** - Security and gas analysis
- 💻 **console command** - Interactive REPL
- 🧹 **clean command** - Artifact cleanup
- 📝 **Config system** - Type-safe glinforge.config.ts
- 📚 **Examples** - Token dApp, NFT Marketplace, DAO

## 🚀 Publishing Steps

### Step 1: Update Version Numbers

```bash
cd /home/eralp/Projects/glin/glin-forge

# Update Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# Update npm packages
sed -i 's/"version": "0.1.0"/"version": "0.2.0"/' packages/sdk/package.json
sed -i 's/"version": "0.1.0"/"version": "0.2.0"/' packages/testing/package.json
sed -i 's/"version": "0.1.0"/"version": "0.2.0"/' packages/cli/package.json
```

### Step 2: Commit All Changes

```bash
# Stage all changes
git add .

# Commit
git commit -m "feat: Phase 4 - Add npx support, SDK, and testing utilities

Major additions:
- npm wrapper for npx support (no Cargo needed)
- @glin-forge/sdk TypeScript package
- @glin-forge/testing utilities package
- Interactive project initialization
- Full-stack templates (React, Next.js, Vue)
- analyze command for security/gas analysis
- console command (interactive REPL)
- clean command
- Type-safe configuration system
- Complete examples (token-dapp, nft-marketplace, dao-governance)

Breaking changes: None (backwards compatible)

Closes #1"

# Push to main
git push origin main
```

### Step 3: Build Release Binary

```bash
# Build optimized binary
cargo build --release

# Verify
./target/release/glin-forge --version
# Should show: glin-forge 0.2.0

# Create tarball
VERSION="0.2.0"
PLATFORM="linux-x86_64"  # Adjust for your platform
mkdir -p dist

cd target/release
tar -czf ../../dist/glin-forge-${VERSION}-${PLATFORM}.tar.gz glin-forge
cd ../..

echo "✓ Binary tarball created: dist/glin-forge-${VERSION}-${PLATFORM}.tar.gz"
ls -lh dist/
```

### Step 4: Create GitHub Release

```bash
VERSION="0.2.0"

# Create and push tag
git tag -a "v${VERSION}" -m "Release v${VERSION} - Phase 4: npx support and DX improvements"
git push origin "v${VERSION}"

# Create GitHub release
gh release create "v${VERSION}" \
  --title "glin-forge v${VERSION} - Phase 4: Hardhat-style DX" \
  --notes "## 🎉 Major Update: Hardhat-Style Developer Experience!

glin-forge now supports **npx** - no Rust/Cargo installation required!

\`\`\`bash
# Just run this - no installation needed!
npx glin-forge init my-contract
\`\`\`

## ✨ What's New

### npx Support
- **No Rust required** - JavaScript developers can start immediately
- **Automatic binary download** - Works on macOS, Linux, Windows
- **Same UX as Hardhat** - Familiar workflow for Ethereum developers

### New Packages
- **@glin-forge/sdk** - TypeScript SDK for deployment scripts
- **@glin-forge/testing** - Comprehensive testing utilities
  - Account management
  - Event assertions
  - Balance tracking
  - Time manipulation
  - Snapshot/revert

### New Commands
- \`glin-forge analyze\` - Security and gas optimization analysis
- \`glin-forge console\` - Interactive REPL with pre-loaded accounts
- \`glin-forge clean\` - Clean build artifacts
- \`glin-forge build --all\` - Build all contracts in workspace

### Interactive Init
- Beautiful CLI prompts (dialoguer)
- Project type selection (Basic, Full-stack, Library)
- Template selection (ERC20, ERC721, DAO, Flipper)
- Frontend framework selection (React, Next.js, Vue)

### Full-Stack Templates
- React + Vite with complete setup
- Next.js 13+ with App Router
- Vue 3 with Composition API
- All templates include SDK integration

### Configuration System
- Type-safe \`glinforge.config.ts\`
- \`defineConfig()\` helper
- Network, paths, compiler settings
- Environment variables

### Complete Examples
- **token-dapp** - ERC20-like token with tests
- **nft-marketplace** - PSP34 NFT marketplace
- **dao-governance** - DAO with proposals and voting

## 📦 Installation

### Option 1: npx (Recommended)
\`\`\`bash
npx glin-forge init my-project
\`\`\`

### Option 2: npm Global
\`\`\`bash
npm install -g glin-forge
glin-forge init my-project
\`\`\`

### Option 3: Cargo
\`\`\`bash
cargo install glin-forge
glin-forge init my-project
\`\`\`

## 🔧 Usage

\`\`\`bash
# Initialize with interactive prompts
npx glin-forge init my-contract

# Build contracts
npx glin-forge build

# Analyze security and gas
npx glin-forge analyze

# Interactive console
npx glin-forge console

# Deploy to testnet
npx glin-forge deploy --network testnet
\`\`\`

## 📚 Documentation

- Installation Guide: [INSTALLATION_GUIDE.md](https://github.com/glin-ai/glin-forge/blob/main/INSTALLATION_GUIDE.md)
- Phase 4 Summary: [PHASE4_COMPLETE.md](https://github.com/glin-ai/glin-forge/blob/main/PHASE4_COMPLETE.md)
- Publishing Guide: [PUBLISHING_GUIDE.md](https://github.com/glin-ai/glin-forge/blob/main/PUBLISHING_GUIDE.md)

## 🐛 Bug Fixes

- Enhanced build command for workspace support
- Improved error messages
- Better artifact handling

## 💔 Breaking Changes

None - v0.2.0 is fully backwards compatible with v0.1.0

## 📊 Statistics

- **50+ new files**
- **10,000+ lines of code**
- **3 new npm packages**
- **3 new CLI commands**
- **3 complete example projects**

---

**Full Changelog**: https://github.com/glin-ai/glin-forge/compare/v0.1.0...v0.2.0" \
  dist/glin-forge-${VERSION}-${PLATFORM}.tar.gz

echo ""
echo "✓ GitHub release created!"
echo "  View at: https://github.com/glin-ai/glin-forge/releases/tag/v${VERSION}"
```

### Step 5: Publish npm Packages

**IMPORTANT:** Release must exist first (CLI wrapper downloads from it)

```bash
# Make sure you're logged in
npm whoami
# If not logged in: npm login

# Publish SDK
cd packages/sdk
npm run build  # Make sure it's built
npm publish --access public
echo "✓ Published @glin-forge/sdk"
cd ../..

# Publish Testing
cd packages/testing
npm install --legacy-peer-deps  # Install deps
npm run build
npm publish --access public
echo "✓ Published @glin-forge/testing"
cd ../..

# Publish CLI Wrapper (THIS IS THE KEY ONE!)
cd packages/cli
npm publish --access public
echo "✓ Published glin-forge (npm wrapper)"
cd ../..

echo ""
echo "✅ All npm packages published!"
echo "  - https://www.npmjs.com/package/@glin-forge/sdk"
echo "  - https://www.npmjs.com/package/@glin-forge/testing"
echo "  - https://www.npmjs.com/package/glin-forge"
```

### Step 6: Publish to crates.io (Optional)

```bash
# This is optional if you want to update crates.io
cargo publish

echo "✓ Published to crates.io"
echo "  https://crates.io/crates/glin-forge"
```

### Step 7: Test Everything!

```bash
# Test npx (the most important!)
cd /tmp/test-npx-$$

# This should download binary from GitHub and work!
npx glin-forge@latest init test-project

# Check it worked
cd test-project
ls -la

# Try a command
npx glin-forge@latest --version
# Should show: glin-forge 0.2.0

npx glin-forge@latest --help
# Should show all commands including new ones

echo ""
echo "✅ npx works!"
```

## 📋 Quick Checklist

- [ ] Versions updated to 0.2.0 in all files
- [ ] Changes committed and pushed
- [ ] Tag v0.2.0 created and pushed
- [ ] Binary built and tarball created
- [ ] GitHub release created with binary
- [ ] @glin-forge/sdk published to npm
- [ ] @glin-forge/testing published to npm
- [ ] glin-forge published to npm
- [ ] npx test passes
- [ ] Announce on social media

## 🎉 After Publishing

Users can now run:

```bash
# No installation needed!
npx glin-forge init my-contract
```

This will:
1. Download glin-forge npm package
2. Download Linux binary from GitHub release
3. Create project with SDK and testing packages
4. Ready to build and deploy!

## 📣 Announcement Template

```markdown
🎉 glin-forge v0.2.0 is out!

Now with npx support - no Rust/Cargo needed!

npm install -g glin-forge
or just run:
npx glin-forge init my-contract

New features:
✨ npx support (Hardhat-style)
📦 TypeScript SDK (@glin-forge/sdk)
🧪 Testing utilities (@glin-forge/testing)
🎨 Interactive project init
📊 Full-stack templates (React, Next.js, Vue)
🔍 Security analyzer
💻 Interactive REPL

GitHub: https://github.com/glin-ai/glin-forge
Docs: [link]
```

---

Ready to publish? Follow the steps above! 🚀
