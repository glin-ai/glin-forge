#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if version is provided
VERSION=${1}

if [ -z "$VERSION" ]; then
    CURRENT_VERSION=$(jq -r '.version' version.json)
    print_error "No version specified"
    echo "Usage: $0 <version>"
    echo "Current version: $CURRENT_VERSION"
    exit 1
fi

# Remove 'v' prefix if present
VERSION=${VERSION#v}

print_info "Preparing release for version: $VERSION"
echo ""

# Step 1: Validate everything
print_step "1/6 Validating release readiness..."
if ! bash scripts/validate-release.sh; then
    print_error "Validation failed. Fix errors and try again."
    exit 1
fi

echo ""

# Step 2: Confirm with user
print_warning "You are about to release version $VERSION"
print_warning "This will:"
echo "  • Create and push git tag v$VERSION"
echo "  • Trigger CI/CD workflow"
echo "  • Build binaries for all platforms"
echo "  • Publish to crates.io"
echo "  • Publish to npm (@glin-ai/forge-sdk, @glin-ai/forge-testing, glin-forge)"
echo "  • Create GitHub release with binaries"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Release cancelled"
    exit 0
fi

# Step 3: Ensure everything is committed
print_step "2/6 Checking git status..."
if ! git diff-index --quiet HEAD --; then
    print_error "Working directory has uncommitted changes. Commit or stash them first."
    exit 1
fi
print_info "✓ Working directory is clean"

# Step 4: Create and push tag
print_step "3/6 Creating git tag..."
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    print_error "Tag v$VERSION already exists"
    exit 1
fi

git tag -a "v$VERSION" -m "Release v$VERSION"
print_info "✓ Tag v$VERSION created"

print_step "4/6 Pushing tag to remote..."
git push origin "v$VERSION"
print_info "✓ Tag pushed to remote"

# Step 5: Monitor CI/CD
echo ""
print_step "5/6 CI/CD workflow triggered!"
print_info "Monitor the release at:"
REPO_URL=$(git config --get remote.origin.url | sed 's/\.git$//' | sed 's/git@github\.com:/https:\/\/github.com\//')
echo "  $REPO_URL/actions"
echo ""

# Step 6: Done
print_step "6/6 Release initiated!"
echo ""
print_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_info "🎉 Release v$VERSION is in progress!"
print_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "The CI/CD workflow will:"
echo "  ✓ Build binaries for all platforms (Linux, macOS, Windows)"
echo "  ✓ Run tests"
echo "  ✓ Publish to crates.io"
echo "  ✓ Publish to npm"
echo "  ✓ Create GitHub release with binaries"
echo ""
echo "Check the Actions tab for progress:"
echo "  $REPO_URL/actions"
echo ""
print_warning "Note: Make sure you have configured these GitHub secrets:"
echo "  • CARGO_TOKEN - for publishing to crates.io"
echo "  • NPM_TOKEN - for publishing to npm"
