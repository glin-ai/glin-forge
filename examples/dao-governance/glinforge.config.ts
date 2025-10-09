import { defineConfig } from '@glin-forge/sdk';

export default defineConfig({
  networks: {
    local: {
      rpc: 'ws://localhost:9944',
      accounts: ['alice'],
    },
    testnet: {
      rpc: 'wss://testnet-rpc.glin.network',
      accounts: [],
    },
  },

  defaultNetwork: 'local',

  paths: {
    contracts: './contracts',
    artifacts: './artifacts',
    types: './types',
  },

  compiler: {
    optimize: true,
    debug: false,
  },

  // DAO-specific configuration
  vars: {
    daoName: 'GLIN DAO',
    votingPeriod: 100800, // 7 days in blocks (6s per block)
    executionDelay: 28800, // 2 days in blocks
    quorumPercentage: 20, // 20%
    minStake: '100000000000000000000', // 100 GLIN
    treasuryInitialFunds: '1000000000000000000000', // 1000 GLIN
  },
});
