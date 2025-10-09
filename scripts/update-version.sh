#!/bin/bash
# Update version across all packages

NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: ./scripts/update-version.sh <version>"
    echo "Example: ./scripts/update-version.sh 0.2.0"
    exit 1
fi

echo "Updating version to ${NEW_VERSION}..."

# Update Cargo.toml
sed -i "0,/version = \".*\"/s//version = \"${NEW_VERSION}\"/" Cargo.toml
echo "✓ Updated Cargo.toml"

# Update npm packages
sed -i "s/\"version\": \".*\"/\"version\": \"${NEW_VERSION}\"/" packages/sdk/package.json
echo "✓ Updated packages/sdk/package.json"

sed -i "s/\"version\": \".*\"/\"version\": \"${NEW_VERSION}\"/" packages/testing/package.json
echo "✓ Updated packages/testing/package.json"

sed -i "s/\"version\": \".*\"/\"version\": \"${NEW_VERSION}\"/" packages/cli/package.json
echo "✓ Updated packages/cli/package.json"

echo ""
echo "✅ Version updated to ${NEW_VERSION} in all files"
echo ""
echo "Verify changes:"
echo "  git diff"
