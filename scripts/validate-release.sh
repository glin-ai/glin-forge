#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0

# Function to print colored output
print_info() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    ERRORS=$((ERRORS + 1))
}

echo "🔍 Validating release readiness..."
echo ""

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Check if version.json exists
if [ ! -f "version.json" ]; then
    print_error "version.json not found"
else
    print_info "version.json exists"
fi

# Read version from version.json
VERSION=$(jq -r '.version' version.json 2>/dev/null || echo "")
if [ -z "$VERSION" ] || [ "$VERSION" = "null" ]; then
    print_error "Failed to read version from version.json"
else
    print_info "Version: $VERSION"
fi

# Check Cargo.toml version
CARGO_VERSION=$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    print_error "Cargo.toml version ($CARGO_VERSION) doesn't match version.json ($VERSION)"
else
    print_info "Cargo.toml version matches"
fi

# Check SDK package.json version
SDK_VERSION=$(jq -r '.version' packages/sdk/package.json)
if [ "$SDK_VERSION" != "$VERSION" ]; then
    print_error "SDK package.json version ($SDK_VERSION) doesn't match version.json ($VERSION)"
else
    print_info "SDK package.json version matches"
fi

# Check testing package.json version
TESTING_VERSION=$(jq -r '.version' packages/testing/package.json)
if [ "$TESTING_VERSION" != "$VERSION" ]; then
    print_error "Testing package.json version ($TESTING_VERSION) doesn't match version.json ($VERSION)"
else
    print_info "Testing package.json version matches"
fi

# Check CLI package.json version
CLI_VERSION=$(jq -r '.version' packages/cli/package.json)
if [ "$CLI_VERSION" != "$VERSION" ]; then
    print_error "CLI package.json version ($CLI_VERSION) doesn't match version.json ($VERSION)"
else
    print_info "CLI package.json version matches"
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD -- 2>/dev/null; then
    print_warning "Working directory has uncommitted changes"
else
    print_info "Working directory is clean"
fi

# Check if on main branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_warning "Not on main branch (current: $CURRENT_BRANCH)"
else
    print_info "On main branch"
fi

# Check if tests pass
echo ""
echo "🧪 Running tests..."
if cargo test --quiet 2>&1 | tail -5; then
    print_info "Cargo tests passed"
else
    print_error "Cargo tests failed"
fi

# Check if build succeeds
echo ""
echo "🔨 Checking build..."
if cargo build --release --quiet 2>&1 | tail -5; then
    print_info "Release build successful"
else
    print_error "Release build failed"
fi

# Check if TypeScript packages build
echo ""
echo "📦 Building TypeScript packages..."
if (cd packages/sdk && npm run build > /dev/null 2>&1); then
    print_info "SDK builds successfully"
else
    print_error "SDK build failed"
fi

if (cd packages/testing && npm run build > /dev/null 2>&1); then
    print_info "Testing package builds successfully"
else
    print_error "Testing package build failed"
fi

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed! Ready to release v$VERSION${NC}"
    echo ""
    echo "To create a release, run:"
    echo "  bash scripts/release.sh $VERSION"
    exit 0
else
    echo -e "${RED}❌ $ERRORS error(s) found. Please fix before releasing.${NC}"
    exit 1
fi
