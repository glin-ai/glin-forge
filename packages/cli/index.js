/**
 * glin-forge npm wrapper
 *
 * This package provides an npm-based installation method for glin-forge,
 * enabling users to run it via npx without installing Rust/Cargo.
 */

module.exports = {
  // Future: Could export programmatic API here
  version: require('./package.json').version,
};
