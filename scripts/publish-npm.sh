#!/bin/bash
set -e

# Publish all npm packages
# Usage: ./scripts/publish-npm.sh [--dry-run]

DRY_RUN=""
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN="--dry-run"
    echo "🧪 Running in DRY-RUN mode (no actual publishing)"
fi

echo "📦 Publishing glin-forge npm packages..."
echo ""

# Function to publish a package
publish_package() {
    local PACKAGE_DIR=$1
    local PACKAGE_NAME=$2

    echo "Publishing ${PACKAGE_NAME}..."
    cd ${PACKAGE_DIR}

    # Build if needed
    if [ -f "tsconfig.json" ]; then
        echo "  Building TypeScript..."
        npm run build
    fi

    # Publish
    npm publish $DRY_RUN --access public

    echo "✓ ${PACKAGE_NAME} published"
    echo ""

    cd -
}

# Publish in order (SDK first, then testing, then CLI)
publish_package "packages/sdk" "@glin-forge/sdk"
publish_package "packages/testing" "@glin-forge/testing"
publish_package "packages/cli" "glin-forge"

echo "✅ All packages published successfully!"
echo ""
echo "Test with:"
echo "  npx glin-forge@latest init test-project"
