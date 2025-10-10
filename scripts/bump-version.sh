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

# Check if bump type is provided
BUMP_TYPE=${1:-patch}

if [[ ! "$BUMP_TYPE" =~ ^(major|minor|patch)$ ]]; then
    print_error "Invalid bump type: $BUMP_TYPE"
    echo "Usage: $0 [major|minor|patch]"
    exit 1
fi

# Read current version
CURRENT_VERSION=$(jq -r '.version' version.json)
print_info "Current version: $CURRENT_VERSION"

# Parse version
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

# Bump version based on type
case "$BUMP_TYPE" in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
print_info "New version: $NEW_VERSION"

# Update version.json
print_step "Updating version.json..."
jq ".version = \"$NEW_VERSION\"" version.json > version.json.tmp
mv version.json.tmp version.json

# Sync all packages
print_step "Syncing all packages..."
bash scripts/sync-version.sh

# Verify
VERIFY_VERSION=$(jq -r '.version' version.json)
if [ "$VERIFY_VERSION" = "$NEW_VERSION" ]; then
    print_info "✅ Version bumped successfully: $CURRENT_VERSION → $NEW_VERSION"
    echo ""
    print_warning "Next steps:"
    echo "  1. Review changes: git diff"
    echo "  2. Run tests: cargo test && npm test"
    echo "  3. Commit changes: git add -A && git commit -m \"Bump version to v$NEW_VERSION\""
    echo "  4. Create release: bash scripts/release.sh $NEW_VERSION"
else
    print_error "Version verification failed"
    exit 1
fi
