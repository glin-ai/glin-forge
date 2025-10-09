import { defineConfig } from '@glin-forge/sdk';

/**
 * Minimal glin-forge configuration
 *
 * This is a minimal configuration file that only defines the essentials.
 * All other settings will use default values.
 */

export default defineConfig({
  networks: {
    testnet: {
      rpc: 'wss://testnet.glin.network',
      accounts: ['alice'],
    },
  },

  defaultNetwork: 'testnet',
});
