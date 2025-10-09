#!/bin/bash
set -e

# Build release binaries for all platforms
# This script should be run in GitHub Actions or locally with cross-compilation setup

VERSION=${1:-"0.1.0"}
PLATFORMS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin" "x86_64-pc-windows-gnu")

echo "🔨 Building glin-forge v${VERSION} for all platforms..."
echo ""

# Create dist directory
mkdir -p dist

for PLATFORM in "${PLATFORMS[@]}"; do
    echo "Building for ${PLATFORM}..."

    # Determine the platform name for the tarball
    case $PLATFORM in
        x86_64-unknown-linux-gnu)
            PLATFORM_NAME="linux-x86_64"
            ;;
        aarch64-unknown-linux-gnu)
            PLATFORM_NAME="linux-aarch64"
            ;;
        x86_64-apple-darwin)
            PLATFORM_NAME="macos-x86_64"
            ;;
        aarch64-apple-darwin)
            PLATFORM_NAME="macos-aarch64"
            ;;
        x86_64-pc-windows-gnu)
            PLATFORM_NAME="windows-x86_64"
            ;;
    esac

    # Build (requires cross if not native platform)
    if command -v cross &> /dev/null; then
        cross build --release --target $PLATFORM
    else
        cargo build --release --target $PLATFORM
    fi

    # Create tarball
    BINARY_NAME="glin-forge"
    if [[ $PLATFORM == *"windows"* ]]; then
        BINARY_NAME="glin-forge.exe"
    fi

    TARBALL_NAME="glin-forge-${VERSION}-${PLATFORM_NAME}.tar.gz"

    cd target/${PLATFORM}/release
    tar -czf ../../../dist/${TARBALL_NAME} ${BINARY_NAME}
    cd ../../..

    echo "✓ Created dist/${TARBALL_NAME}"
    echo ""
done

echo "✅ All binaries built successfully!"
echo ""
echo "📦 Tarballs created in dist/:"
ls -lh dist/
