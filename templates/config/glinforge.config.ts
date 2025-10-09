import { defineConfig } from '@glin-forge/sdk';

/**
 * glin-forge configuration
 *
 * This file defines your project's networks, paths, compiler settings, and more.
 * Similar to hardhat.config.ts for Hardhat projects.
 *
 * Documentation: https://docs.glin.ai/forge/configuration
 */

export default defineConfig({
  /**
   * Network configurations
   *
   * Define RPC endpoints, accounts, and explorer URLs for each network.
   */
  networks: {
    // Local development network
    local: {
      rpc: 'ws://localhost:9944',
      accounts: ['alice', 'bob', 'charlie'],
    },

    // GLIN testnet
    testnet: {
      rpc: 'wss://testnet.glin.network',
      explorer: 'https://explorer-testnet.glin.network',
      accounts: ['alice'], // Replace with your accounts
      timeout: 60000, // 60 seconds
    },

    // GLIN mainnet
    mainnet: {
      rpc: 'wss://rpc.glin.network',
      explorer: 'https://explorer.glin.network',
      // accounts: ['deployer'], // Uncomment and configure for mainnet
      gasPrice: 100000000, // Optional: gas price override
    },
  },

  /**
   * Default network to use when not specified
   */
  defaultNetwork: 'testnet',

  /**
   * Project path configurations
   */
  paths: {
    contracts: './contracts',      // ink! contract source files
    artifacts: './artifacts',      // Compiled contract outputs
    types: './types',              // Generated TypeScript types
    scripts: './scripts',          // Deployment and interaction scripts
    tests: './test',               // Test files
    cache: './.cache',            // Cache for metadata and temp files
  },

  /**
   * Compiler settings
   */
  compiler: {
    optimize: true,                // Enable release mode (--release)
    features: [],                  // Cargo features to enable (e.g., ['std'])
    // target: 'wasm32-unknown-unknown', // Custom target (optional)
    // cargoFlags: [],              // Additional cargo flags
    workspace: false,              // Workspace mode for multi-contract projects
  },

  /**
   * TypeScript generation settings
   */
  typegen: {
    autoGenerate: true,            // Auto-generate types after build
    outDir: './types',             // Output directory for generated types
    hooks: false,                  // Generate React hooks
    legacy: false,                 // Use legacy simple type generator
    style: 'interface',            // Type generation style: 'interface' | 'type' | 'class'
  },

  /**
   * Testing configuration
   */
  test: {
    framework: 'mocha',            // Test framework: 'mocha' | 'jest'
    pattern: 'test/**/*.test.ts',  // Test file pattern
    timeout: 30000,                // Test timeout in ms
    parallel: false,               // Parallel test execution
    coverage: false,               // Coverage reporting
  },

  /**
   * Deployment configurations per contract per network
   *
   * Define default deployment parameters for each contract on each network.
   */
  deployments: {
    // Example: ERC20 token deployment
    erc20: {
      testnet: {
        from: 'alice',
        args: [1000000, 'MyToken', 'MTK', 18],
        value: 0,
        verify: true,
        waitForFinalization: true,
      },
      mainnet: {
        // from: 'deployer',
        // args: [10000000, 'MainnetToken', 'MTK', 18],
        // verify: true,
      },
    },

    // Add more contracts here...
  },

  /**
   * Custom configuration variables
   *
   * Add any custom variables you need for your project.
   */
  vars: {
    // Example custom variables
    // PROJECT_NAME: 'my-dapp',
    // ENABLE_LOGGING: true,
  },
});
