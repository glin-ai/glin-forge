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
  },

  // NFT-specific configuration
  vars: {
    collectionName: 'GLIN NFTs',
    collectionSymbol: 'GNFT',
    baseUri: 'ipfs://',
    marketplaceFee: 250, // 2.5% fee (in basis points)
  },
});
