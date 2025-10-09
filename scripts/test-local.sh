#!/bin/bash
set -e

# Test npm packages locally before publishing
# This creates a test project using local packages via npm link

echo "🧪 Testing glin-forge packages locally..."
echo ""

# Build CLI binary
echo "1. Building CLI binary..."
cargo build --release
echo "✓ CLI built"
echo ""

# Build TypeScript packages
echo "2. Building TypeScript packages..."
cd packages/sdk
npm run build
cd ../..
echo "✓ SDK built"

cd packages/testing
npm run build
cd ../..
echo "✓ Testing package built"
echo ""

# Create test directory
TEST_DIR="/tmp/glin-forge-test-$$"
echo "3. Creating test project in ${TEST_DIR}..."
mkdir -p ${TEST_DIR}

# Copy CLI binary to test location
mkdir -p ${TEST_DIR}/bin
cp target/release/glin-forge ${TEST_DIR}/bin/

# Use local CLI to initialize project
echo "4. Initializing test project..."
${TEST_DIR}/bin/glin-forge init ${TEST_DIR}/test-project --yes --template erc20

# Link local packages
echo "5. Linking local npm packages..."
cd packages/sdk
npm link
cd ../..

cd packages/testing
npm link
cd ../..

cd ${TEST_DIR}/test-project
npm link @glin-forge/sdk
npm link @glin-forge/testing

echo ""
echo "✅ Test setup complete!"
echo ""
echo "Test project location: ${TEST_DIR}/test-project"
echo ""
echo "Try these commands:"
echo "  cd ${TEST_DIR}/test-project"
echo "  ${TEST_DIR}/bin/glin-forge build"
echo "  ${TEST_DIR}/bin/glin-forge --help"
echo ""
echo "When done testing:"
echo "  npm unlink @glin-forge/sdk --global"
echo "  npm unlink @glin-forge/testing --global"
echo "  rm -rf ${TEST_DIR}"
