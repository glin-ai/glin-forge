/**
 * Configuration system for glin-forge projects
 *
 * Provides type-safe configuration for networks, paths, compiler settings, and more.
 * Similar to hardhat.config.ts for Hardhat projects.
 *
 * @example
 * ```typescript
 * // glinforge.config.ts
 * import { defineConfig } from '@glin-forge/sdk';
 *
 * export default defineConfig({
 *   networks: {
 *     testnet: {
 *       rpc: 'wss://testnet.glin.network',
 *       accounts: ['alice'],
 *     }
 *   }
 * });
 * ```
 */

/**
 * Network configuration
 */
export interface NetworkConfig {
  /** RPC endpoint (WebSocket or HTTP) */
  rpc: string;

  /** Block explorer URL (optional) */
  explorer?: string;

  /** Default accounts for this network */
  accounts?: string[];

  /** Gas price (optional, for gas estimation) */
  gasPrice?: number | bigint;

  /** Gas limit override (optional) */
  gasLimit?: number | bigint;

  /** Network timeout in ms */
  timeout?: number;
}

/**
 * Path configuration
 */
export interface PathsConfig {
  /** Source contracts directory */
  contracts?: string;

  /** Build artifacts output directory */
  artifacts?: string;

  /** Generated TypeScript types directory */
  types?: string;

  /** Deployment scripts directory */
  scripts?: string;

  /** Test files directory */
  tests?: string;

  /** Cache directory for metadata and temp files */
  cache?: string;
}

/**
 * Compiler configuration
 */
export interface CompilerConfig {
  /** Optimization level (true = --release) */
  optimize?: boolean;

  /** Cargo features to enable */
  features?: string[];

  /** Target triple (for cross-compilation) */
  target?: string;

  /** Additional cargo flags */
  cargoFlags?: string[];

  /** Workspace mode (for multi-contract projects) */
  workspace?: boolean;
}

/**
 * TypeScript generation configuration
 */
export interface TypeGenConfig {
  /** Auto-generate types after build */
  autoGenerate?: boolean;

  /** Output directory for generated types */
  outDir?: string;

  /** Generate React hooks */
  hooks?: boolean;

  /** Use legacy type generator */
  legacy?: boolean;

  /** Type generation style */
  style?: 'interface' | 'type' | 'class';
}

/**
 * Testing configuration
 */
export interface TestConfig {
  /** Test framework */
  framework?: 'mocha' | 'jest';

  /** Test file pattern */
  pattern?: string;

  /** Test timeout in ms */
  timeout?: number;

  /** Parallel test execution */
  parallel?: boolean;

  /** Coverage reporting */
  coverage?: boolean;
}

/**
 * Deployment configuration per network
 */
export interface DeploymentConfig {
  /** Deployer account */
  from?: string;

  /** Constructor arguments */
  args?: any[];

  /** Value to send with deployment */
  value?: number | bigint;

  /** Salt for deterministic deployment */
  salt?: string;

  /** Verify contract after deployment */
  verify?: boolean;

  /** Wait for finalization */
  waitForFinalization?: boolean;
}

/**
 * Complete glin-forge configuration
 */
export interface ForgeConfig {
  /** Network configurations */
  networks?: {
    [networkName: string]: NetworkConfig;
  };

  /** Default network to use */
  defaultNetwork?: string;

  /** Path configurations */
  paths?: PathsConfig;

  /** Compiler settings */
  compiler?: CompilerConfig;

  /** TypeScript generation settings */
  typegen?: TypeGenConfig;

  /** Testing configuration */
  test?: TestConfig;

  /** Deployment configurations per contract per network */
  deployments?: {
    [contractName: string]: {
      [networkName: string]: DeploymentConfig;
    };
  };

  /** Custom configuration variables */
  vars?: {
    [key: string]: any;
  };
}

/**
 * Default configuration values
 */
export const defaultConfig: ForgeConfig = {
  networks: {
    local: {
      rpc: 'ws://localhost:9944',
      accounts: ['alice', 'bob'],
    },
    testnet: {
      rpc: 'wss://testnet.glin.network',
      explorer: 'https://explorer-testnet.glin.network',
      accounts: ['alice'],
    },
    mainnet: {
      rpc: 'wss://rpc.glin.network',
      explorer: 'https://explorer.glin.network',
    },
  },
  defaultNetwork: 'testnet',
  paths: {
    contracts: './contracts',
    artifacts: './artifacts',
    types: './types',
    scripts: './scripts',
    tests: './test',
    cache: './.cache',
  },
  compiler: {
    optimize: true,
    features: [],
    workspace: false,
  },
  typegen: {
    autoGenerate: true,
    outDir: './types',
    hooks: false,
    legacy: false,
    style: 'interface',
  },
  test: {
    framework: 'mocha',
    pattern: 'test/**/*.test.ts',
    timeout: 30000,
    parallel: false,
    coverage: false,
  },
};

/**
 * Helper function to define configuration with type safety
 *
 * @example
 * ```typescript
 * export default defineConfig({
 *   networks: {
 *     testnet: {
 *       rpc: 'wss://testnet.glin.network',
 *     }
 *   }
 * });
 * ```
 */
export function defineConfig(config: ForgeConfig): ForgeConfig {
  return config;
}

/**
 * Merge user config with default config
 */
export function mergeConfig(userConfig: Partial<ForgeConfig>): ForgeConfig {
  return {
    ...defaultConfig,
    ...userConfig,
    networks: {
      ...defaultConfig.networks,
      ...userConfig.networks,
    },
    paths: {
      ...defaultConfig.paths,
      ...userConfig.paths,
    },
    compiler: {
      ...defaultConfig.compiler,
      ...userConfig.compiler,
    },
    typegen: {
      ...defaultConfig.typegen,
      ...userConfig.typegen,
    },
    test: {
      ...defaultConfig.test,
      ...userConfig.test,
    },
    deployments: {
      ...defaultConfig.deployments,
      ...userConfig.deployments,
    },
    vars: {
      ...defaultConfig.vars,
      ...userConfig.vars,
    },
  };
}

/**
 * Load and parse config file
 *
 * @internal - Used by glin-forge CLI
 */
export async function loadConfig(configPath?: string): Promise<ForgeConfig> {
  const path = configPath || findConfigFile();

  if (!path) {
    return defaultConfig;
  }

  try {
    // Dynamic import for TypeScript config
    const userConfig = await import(path);
    const config = userConfig.default || userConfig;
    return mergeConfig(config);
  } catch (error) {
    console.warn(`Failed to load config from ${path}:`, error);
    return defaultConfig;
  }
}

/**
 * Find config file in project root
 *
 * Searches for:
 * - glinforge.config.ts
 * - glinforge.config.js
 * - glinforge.config.json
 */
function findConfigFile(): string | null {
  const fs = require('fs');
  const path = require('path');

  const configFiles = [
    'glinforge.config.ts',
    'glinforge.config.js',
    'glinforge.config.json',
    'glin-forge.config.ts',
    'glin-forge.config.js',
    'glin-forge.config.json',
  ];

  for (const file of configFiles) {
    const filePath = path.join(process.cwd(), file);
    if (fs.existsSync(filePath)) {
      return filePath;
    }
  }

  return null;
}

/**
 * Get network configuration by name
 */
export function getNetwork(config: ForgeConfig, networkName?: string): NetworkConfig | undefined {
  const name = networkName || config.defaultNetwork || 'testnet';
  return config.networks?.[name];
}

/**
 * Validate configuration
 *
 * Throws error if configuration is invalid
 */
export function validateConfig(config: ForgeConfig): void {
  // Validate default network exists
  if (config.defaultNetwork && !config.networks?.[config.defaultNetwork]) {
    throw new Error(`Default network '${config.defaultNetwork}' not found in networks configuration`);
  }

  // Validate network configurations
  if (config.networks) {
    for (const [name, network] of Object.entries(config.networks)) {
      if (!network.rpc) {
        throw new Error(`Network '${name}' is missing 'rpc' configuration`);
      }

      // Validate RPC format
      if (!network.rpc.startsWith('ws://') && !network.rpc.startsWith('wss://') &&
          !network.rpc.startsWith('http://') && !network.rpc.startsWith('https://')) {
        throw new Error(`Network '${name}' has invalid RPC URL: ${network.rpc}`);
      }
    }
  }

  // Validate paths
  if (config.paths) {
    // Paths should be relative or absolute
    // No validation needed for now, just ensure they're strings
  }

  // Validate compiler config
  if (config.compiler) {
    if (config.compiler.features && !Array.isArray(config.compiler.features)) {
      throw new Error('Compiler features must be an array of strings');
    }
  }
}

/**
 * Configuration builder for programmatic usage
 */
export class ConfigBuilder {
  private config: Partial<ForgeConfig> = {};

  network(name: string, config: NetworkConfig): this {
    if (!this.config.networks) {
      this.config.networks = {};
    }
    this.config.networks[name] = config;
    return this;
  }

  defaultNetwork(name: string): this {
    this.config.defaultNetwork = name;
    return this;
  }

  paths(paths: PathsConfig): this {
    this.config.paths = paths;
    return this;
  }

  compiler(compiler: CompilerConfig): this {
    this.config.compiler = compiler;
    return this;
  }

  build(): ForgeConfig {
    return mergeConfig(this.config);
  }
}

/**
 * Export config types for use in glinforge.config.ts
 */
export type {
  ForgeConfig as Config,
  NetworkConfig as Network,
  PathsConfig as Paths,
  CompilerConfig as Compiler,
  TypeGenConfig as TypeGen,
  TestConfig as Test,
  DeploymentConfig as Deployment,
};
