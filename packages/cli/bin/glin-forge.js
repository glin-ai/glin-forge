#!/usr/bin/env node

/**
 * glin-forge CLI wrapper
 *
 * This wrapper downloads and executes the native Rust binary
 * for the user's platform, enabling npx usage without Cargo.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the binary path based on platform
function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  let binaryName = 'glin-forge';
  if (platform === 'win32') {
    binaryName += '.exe';
  }

  const binaryPath = path.join(__dirname, '..', 'bin', binaryName);

  if (!fs.existsSync(binaryPath)) {
    console.error(`
❌ glin-forge binary not found at: ${binaryPath}

This usually means the binary download failed during installation.

Please try:
  1. npm install glin-forge --force
  2. Or install via Cargo: cargo install glin-forge

Supported platforms:
  - macOS (x64, arm64)
  - Linux (x64, arm64)
  - Windows (x64)

Your platform: ${platform}-${arch}
`);
    process.exit(1);
  }

  return binaryPath;
}

// Execute the binary with all arguments
function run() {
  const binary = getBinaryPath();
  const args = process.argv.slice(2);

  const child = spawn(binary, args, {
    stdio: 'inherit',
    shell: false,
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });

  child.on('error', (err) => {
    console.error('Failed to execute glin-forge:', err.message);
    process.exit(1);
  });
}

// Run the CLI
run();
