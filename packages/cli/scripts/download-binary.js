#!/usr/bin/env node

/**
 * Post-install script to download the appropriate glin-forge binary
 *
 * Downloads pre-built binaries from GitHub releases based on the user's platform.
 * This eliminates the need for Rust/Cargo to be installed.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const { createGunzip } = require('zlib');
const { pipeline } = require('stream');
const { promisify } = require('util');
const tar = require('tar');

const pipelineAsync = promisify(pipeline);

// Package version from package.json
const pkg = require('../package.json');
const VERSION = pkg.version;

// GitHub release configuration
const GITHUB_ORG = 'glin-ai';
const GITHUB_REPO = 'glin-forge';
const RELEASE_TAG = `v${VERSION}`;

/**
 * Determine the download URL based on platform
 */
function getDownloadInfo() {
  const platform = process.platform;
  const arch = process.arch;

  // Platform mapping
  const platformMap = {
    darwin: 'macos',
    linux: 'linux',
    win32: 'windows',
  };

  // Architecture mapping
  const archMap = {
    x64: 'x86_64',
    arm64: 'aarch64',
  };

  const platformName = platformMap[platform];
  const archName = archMap[arch];

  if (!platformName || !archName) {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  // Binary filename pattern: glin-forge-{version}-{platform}-{arch}.tar.gz
  const filename = `glin-forge-${VERSION}-${platformName}-${archName}.tar.gz`;
  const url = `https://github.com/${GITHUB_ORG}/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${filename}`;

  return {
    url,
    filename,
    platform: platformName,
    arch: archName,
  };
}

/**
 * Download file from URL
 */
async function downloadFile(url, destPath) {
  return new Promise((resolve, reject) => {
    console.log(`📦 Downloading glin-forge binary from: ${url}`);

    https.get(url, (response) => {
      // Follow redirects
      if (response.statusCode === 302 || response.statusCode === 301) {
        return downloadFile(response.headers.location, destPath)
          .then(resolve)
          .catch(reject);
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Download failed with status: ${response.statusCode}`));
        return;
      }

      const fileStream = fs.createWriteStream(destPath);
      response.pipe(fileStream);

      fileStream.on('finish', () => {
        fileStream.close();
        console.log('✓ Download complete');
        resolve();
      });

      fileStream.on('error', (err) => {
        fs.unlinkSync(destPath);
        reject(err);
      });
    }).on('error', reject);
  });
}

/**
 * Extract tarball
 */
async function extractTarball(tarballPath, destDir) {
  console.log('📂 Extracting binary...');

  await tar.extract({
    file: tarballPath,
    cwd: destDir,
  });

  console.log('✓ Extraction complete');
}

/**
 * Make binary executable (Unix only)
 */
function makeExecutable(binaryPath) {
  if (process.platform !== 'win32') {
    fs.chmodSync(binaryPath, 0o755);
    console.log('✓ Binary made executable');
  }
}

/**
 * Fallback: Compile from source using Cargo
 */
function compileFromSource() {
  console.log(`
⚠️  Pre-built binary not available for your platform.

Attempting to compile from source using Cargo...
This requires Rust to be installed: https://rustup.rs
`);

  const { execSync } = require('child_process');

  try {
    // Check if cargo is installed
    execSync('cargo --version', { stdio: 'ignore' });

    console.log('Building glin-forge from source...');

    // Build the binary
    const projectRoot = path.join(__dirname, '../../..');
    execSync('cargo build --release', {
      cwd: projectRoot,
      stdio: 'inherit',
    });

    // Copy binary to bin/
    const binDir = path.join(__dirname, '..', 'bin');
    const sourceBinary = path.join(projectRoot, 'target/release/glin-forge');
    const destBinary = path.join(binDir, process.platform === 'win32' ? 'glin-forge.exe' : 'glin-forge');

    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }

    fs.copyFileSync(sourceBinary, destBinary);
    makeExecutable(destBinary);

    console.log('✓ Successfully built from source');
    return true;
  } catch (err) {
    console.error(`
❌ Failed to compile from source: ${err.message}

Please install Rust from https://rustup.rs and try again, or install directly:

  cargo install glin-forge

Then use the cargo-installed version instead of npm.
`);
    return false;
  }
}

/**
 * Main installation logic
 */
async function install() {
  try {
    console.log('🔧 Installing glin-forge...');

    const downloadInfo = getDownloadInfo();
    const binDir = path.join(__dirname, '..', 'bin');
    const tarballPath = path.join(binDir, downloadInfo.filename);
    const binaryName = process.platform === 'win32' ? 'glin-forge.exe' : 'glin-forge';
    const binaryPath = path.join(binDir, binaryName);

    // Create bin directory
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }

    // Skip download if binary already exists
    if (fs.existsSync(binaryPath)) {
      console.log('✓ Binary already installed');
      return;
    }

    try {
      // Download binary
      await downloadFile(downloadInfo.url, tarballPath);

      // Extract tarball
      await extractTarball(tarballPath, binDir);

      // Clean up tarball
      fs.unlinkSync(tarballPath);

      // Make executable
      makeExecutable(binaryPath);

      console.log(`
✅ glin-forge installed successfully!

Platform: ${downloadInfo.platform}-${downloadInfo.arch}
Binary: ${binaryPath}

Try it out:
  npx glin-forge --version
  npx glin-forge init my-project
`);
    } catch (downloadErr) {
      console.error(`
⚠️  Failed to download pre-built binary: ${downloadErr.message}

This might be because:
  1. Release ${RELEASE_TAG} doesn't exist yet
  2. Binary for ${downloadInfo.platform}-${downloadInfo.arch} not available
  3. Network connectivity issues
`);

      // Fallback to compiling from source
      const compiled = compileFromSource();
      if (!compiled) {
        process.exit(1);
      }
    }
  } catch (err) {
    console.error('❌ Installation failed:', err.message);
    console.error(`
Alternative installation methods:

1. Install via Cargo (requires Rust):
   cargo install glin-forge

2. Download binary manually from:
   https://github.com/${GITHUB_ORG}/${GITHUB_REPO}/releases

3. Build from source:
   git clone https://github.com/${GITHUB_ORG}/${GITHUB_REPO}
   cd glin-forge
   cargo build --release
`);
    process.exit(1);
  }
}

// Run installation
install();
