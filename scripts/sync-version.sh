#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Read version from version.json
VERSION=$(jq -r '.version' version.json)

if [ -z "$VERSION" ] || [ "$VERSION" = "null" ]; then
    print_error "Failed to read version from version.json"
    exit 1
fi

print_info "Syncing all packages to version: $VERSION"

# Update Cargo.toml
print_info "Updating Cargo.toml..."
sed -i "0,/version = \".*\"/s//version = \"${VERSION}\"/" Cargo.toml

# Update npm packages
print_info "Updating @glin-ai/forge-sdk..."
jq ".version = \"$VERSION\"" packages/sdk/package.json > packages/sdk/package.json.tmp
mv packages/sdk/package.json.tmp packages/sdk/package.json

print_info "Updating @glin-ai/forge-testing..."
jq ".version = \"$VERSION\" | .peerDependencies[\"@glin-ai/forge-sdk\"] = \"^$VERSION\"" packages/testing/package.json > packages/testing/package.json.tmp
mv packages/testing/package.json.tmp packages/testing/package.json

print_info "Updating glin-forge CLI wrapper..."
jq ".version = \"$VERSION\"" packages/cli/package.json > packages/cli/package.json.tmp
mv packages/cli/package.json.tmp packages/cli/package.json

# Update version.json package list
print_info "Updating version.json..."
jq ".packages[\"glin-forge\"] = \"$VERSION\" | .packages[\"forge-sdk\"] = \"$VERSION\" | .packages[\"forge-testing\"] = \"$VERSION\"" version.json > version.json.tmp
mv version.json.tmp version.json

print_info "✅ All packages synced to version $VERSION"
