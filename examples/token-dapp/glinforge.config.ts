import { defineConfig } from '@glin-forge/sdk';

export default defineConfig({
  networks: {
    local: {
      rpc: 'ws://localhost:9944',
      accounts: ['alice'],
    },
    testnet: {
      rpc: 'wss://testnet-rpc.glin.network',
      accounts: [], // Add your accounts here
    },
    mainnet: {
      rpc: 'wss://rpc.glin.network',
      accounts: [],
    },
  },

  defaultNetwork: 'local',

  paths: {
    contracts: './contracts',
    artifacts: './artifacts',
    types: './types',
    scripts: './scripts',
    tests: './test',
  },

  compiler: {
    optimize: true,
    debug: false,
  },

  typegen: {
    enabled: true,
    outDir: './types',
  },

  test: {
    timeout: 60000,
    coverage: true,
  },

  // Token-specific configuration
  vars: {
    tokenName: 'GLIN Token',
    tokenSymbol: 'GLIN',
    initialSupply: '1000000000000000000000000', // 1 million tokens with 18 decimals
  },
});
