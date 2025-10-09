import { defineConfig } from '@glin-forge/sdk';

/**
 * Full-stack dApp configuration
 *
 * This configuration is optimized for full-stack dApp development
 * with frontend integration (React/Next.js).
 */

export default defineConfig({
  networks: {
    local: {
      rpc: 'ws://localhost:9944',
      accounts: ['alice', 'bob', 'charlie'],
    },
    testnet: {
      rpc: 'wss://testnet.glin.network',
      explorer: 'https://explorer-testnet.glin.network',
      accounts: ['alice'],
    },
  },

  defaultNetwork: 'local', // Use local for development

  paths: {
    contracts: './contracts',
    artifacts: './artifacts',
    types: './frontend/src/contracts', // Generate types in frontend
    scripts: './scripts',
    tests: './test',
    cache: './.cache',
  },

  compiler: {
    optimize: true,
    workspace: true, // Enable workspace mode for multiple contracts
  },

  typegen: {
    autoGenerate: true,
    outDir: './frontend/src/contracts', // Frontend integration
    hooks: true, // Generate React hooks
    style: 'interface',
  },

  test: {
    framework: 'mocha',
    pattern: 'test/**/*.test.ts',
    timeout: 60000, // Longer timeout for integration tests
    parallel: false,
    coverage: true, // Enable coverage reporting
  },

  deployments: {
    token: {
      local: {
        from: 'alice',
        args: [1000000, 'DevToken', 'DTOKEN', 18],
        verify: false, // No verification on local
      },
      testnet: {
        from: 'alice',
        args: [10000000, 'TestToken', 'TTOKEN', 18],
        verify: true,
        waitForFinalization: true,
      },
    },
    marketplace: {
      local: {
        from: 'alice',
        verify: false,
      },
      testnet: {
        from: 'alice',
        verify: true,
        waitForFinalization: true,
      },
    },
  },

  vars: {
    FRONTEND_URL: 'http://localhost:3000',
    API_ENDPOINT: 'http://localhost:8000',
    ENABLE_ANALYTICS: false,
  },
});
