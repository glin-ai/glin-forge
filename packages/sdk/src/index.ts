import axios, { AxiosInstance } from 'axios';
import { EventWatcher, ContractEvent, type WatchOptions } from './events';
import { ArgumentEncoder, type ContractArg, encodeArgs, balance, decodeBalance, formatBalance } from './encoding';
import { NetworkHelper, type BalanceInfo, type FaucetResult, type GasEstimate } from './network';
import { Transaction, type TransactionResult, type TransactionReceipt } from './transaction';
import { TypedContract, TypedContractFactory, type TypedDeployOptions, type TypedContractOptions, type QueryMethodsOf, type TransactionMethodsOf, type EventNamesOf } from './typed-contract';
import { defineConfig, mergeConfig, loadConfig, getNetwork, validateConfig, ConfigBuilder, defaultConfig, type ForgeConfig, type NetworkConfig, type PathsConfig, type CompilerConfig, type TypeGenConfig, type TestConfig, type DeploymentConfig, type Config, type Network as NetworkType, type Paths, type Compiler, type TypeGen, type Test, type Deployment } from './config';

// ========================================
// Types
// ========================================

export interface DeployOptions {
  wasm: string;
  metadata: string;
  args?: ContractArg[];
  value?: number;
  network: Network | string;
  account: Signer | string;
  gasLimit?: number;
  salt?: string;
}

export interface DeployResult {
  success: boolean;
  address?: string;
  codeHash?: string;
  txHash?: string;
  blockHash?: string;
  error?: string;
}

export interface CallOptions {
  address: string;
  metadata: string;
  method: string;
  args?: ContractArg[];
  value?: number;
  network: Network | string;
  account: Signer | string;
  gasLimit?: number;
}

export interface CallResult {
  success: boolean;
  txHash?: string;
  blockHash?: string;
  error?: string;
}

export interface QueryOptions {
  address: string;
  metadata: string;
  method: string;
  args?: ContractArg[];
  network: Network | string;
}

export interface QueryResult {
  success: boolean;
  data?: any;
  error?: string;
}

// ========================================
// Enums
// ========================================

export enum Network {
  Testnet = 'testnet',
  Mainnet = 'mainnet',
  Local = 'local',
}

export enum Signer {
  Alice = 'alice',
  Bob = 'bob',
  Charlie = 'charlie',
  Dave = 'dave',
  Eve = 'eve',
  Ferdie = 'ferdie',
}

// ========================================
// RPC Client
// ========================================

class GlinForgeClient {
  private rpcUrl: string;
  private axios: AxiosInstance;
  private requestId: number = 0;

  constructor() {
    const port = process.env.GLIN_FORGE_RPC_PORT;
    if (!port) {
      throw new Error(
        'glin-forge RPC server not running. ' +
          'This SDK must be used with "glin-forge run" command.'
      );
    }

    this.rpcUrl = `http://127.0.0.1:${port}`;
    this.axios = axios.create({
      baseURL: this.rpcUrl,
      timeout: 120000, // 2 minutes
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  private async rpc(method: string, params: any): Promise<any> {
    try {
      const response = await this.axios.post('/', {
        jsonrpc: '2.0',
        id: ++this.requestId,
        method,
        params: [params],
      });

      if (response.data.error) {
        throw new Error(
          response.data.error.message || 'RPC call failed'
        );
      }

      return response.data.result;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(
          `RPC call failed: ${error.message}. ` +
            `Is glin-forge RPC server running on ${this.rpcUrl}?`
        );
      }
      throw error;
    }
  }

  async deploy(options: DeployOptions): Promise<DeployResult> {
    // Encode arguments if provided
    const encodedOptions = {
      ...options,
      args: options.args ? ArgumentEncoder.encodeAll(options.args) : [],
    };
    return this.rpc('deploy', encodedOptions);
  }

  async call(options: CallOptions): Promise<CallResult> {
    // Encode arguments if provided
    const encodedOptions = {
      ...options,
      args: options.args ? ArgumentEncoder.encodeAll(options.args) : [],
    };
    return this.rpc('call', encodedOptions);
  }

  async query(options: QueryOptions): Promise<QueryResult> {
    // Encode arguments if provided
    const encodedOptions = {
      ...options,
      args: options.args ? ArgumentEncoder.encodeAll(options.args) : [],
    };
    return this.rpc('query', encodedOptions);
  }
}

// ========================================
// Contract Class
// ========================================

export class Contract {
  constructor(
    public address: string,
    private metadata: string,
    private network: Network | string,
    private account: Signer | string,
    private client: GlinForgeClient
  ) {}

  /**
   * Call a contract method (transaction)
   *
   * Returns a Transaction object that can be used to wait for finalization
   *
   * @example
   * ```typescript
   * // Simple call
   * const tx = await contract.call('transfer', ['5GrwvaEF...', 1000]);
   * console.log('Transaction hash:', tx.hash());
   *
   * // Wait for finalization
   * const receipt = await tx.wait();
   * console.log('Finalized in block:', receipt.blockNumber);
   *
   * // With BigInt
   * await contract.call('transfer', ['5GrwvaEF...', 1000000000000000000n]);
   *
   * // With balance helper
   * await contract.call('transfer', ['5GrwvaEF...', balance(1, 18)]);
   * ```
   */
  async call(
    method: string,
    args: ContractArg[] = [],
    value: number = 0
  ): Promise<Transaction> {
    const result = await this.client.call({
      address: this.address,
      metadata: this.metadata,
      method,
      args,
      value,
      network: this.network,
      account: this.account,
    });

    // Convert CallResult to TransactionResult and return Transaction
    return new Transaction(
      {
        txHash: result.txHash || '',
        blockHash: result.blockHash,
        success: result.success,
        error: result.error,
      },
      this.network as string
    );
  }

  /**
   * Query contract state (read-only)
   *
   * @example
   * ```typescript
   * const balance = await contract.query('balanceOf', ['5GrwvaEF...']);
   * const totalSupply = await contract.query('totalSupply');
   * ```
   */
  async query(method: string, args: ContractArg[] = []): Promise<any> {
    const result = await this.client.query({
      address: this.address,
      metadata: this.metadata,
      method,
      args,
      network: this.network,
    });

    if (!result.success) {
      throw new Error(result.error || 'Query failed');
    }

    return result.data;
  }

  /**
   * Create an EventWatcher for this contract
   *
   * @example
   * ```typescript
   * const watcher = contract.watch({ event: 'Transfer' });
   * watcher.on('Transfer', (event) => {
   *   console.log('Transfer:', event.data);
   * });
   * await watcher.start();
   * ```
   */
  watch(options?: {
    event?: string;
    follow?: boolean;
    limit?: number;
    fromBlock?: number;
  }): EventWatcher {
    return new EventWatcher({
      address: this.address,
      network: this.network as string,
      event: options?.event,
      follow: options?.follow,
      limit: options?.limit,
      fromBlock: options?.fromBlock,
    });
  }

  /**
   * Listen for contract events (convenience method)
   *
   * @example
   * ```typescript
   * // Listen for specific event
   * await contract.on('Transfer', (event) => {
   *   console.log('Transfer:', event.data);
   * });
   *
   * // Listen for all events
   * await contract.on('*', (event) => {
   *   console.log('Event:', event.eventName, event.data);
   * });
   * ```
   */
  async on(
    eventName: string,
    callback: (event: ContractEvent) => void,
    options?: {
      follow?: boolean;
      limit?: number;
      fromBlock?: number;
    }
  ): Promise<void> {
    const watcher = this.watch({
      event: eventName === '*' ? undefined : eventName,
      follow: options?.follow,
      limit: options?.limit,
      fromBlock: options?.fromBlock,
    });

    watcher.on(eventName, callback);
    await watcher.start();
  }
}

// ========================================
// High-level API
// ========================================

/**
 * Deploy a contract to the network
 *
 * @example
 * ```typescript
 * const contract = await deploy({
 *   wasm: './target/ink/my_contract.wasm',
 *   metadata: './target/ink/metadata.json',
 *   args: ['arg1', 'arg2'],
 *   network: Network.Testnet,
 *   account: Signer.Alice
 * });
 *
 * console.log('Deployed at:', contract.address);
 * ```
 */
export async function deploy(options: DeployOptions): Promise<Contract> {
  const client = new GlinForgeClient();
  const result = await client.deploy(options);

  if (!result.success || !result.address) {
    throw new Error(result.error || 'Deployment failed');
  }

  return new Contract(
    result.address,
    options.metadata,
    options.network,
    options.account,
    client
  );
}

/**
 * Get a contract instance for an already deployed contract
 *
 * @example
 * ```typescript
 * const contract = getContract({
 *   address: '5GrwvaEF...',
 *   metadata: './target/ink/metadata.json',
 *   network: Network.Testnet,
 *   account: Signer.Alice
 * });
 *
 * const balance = await contract.query('balanceOf', ['5Account...']);
 * ```
 */
export function getContract(options: {
  address: string;
  metadata: string;
  network: Network | string;
  account: Signer | string;
}): Contract {
  const client = new GlinForgeClient();
  return new Contract(
    options.address,
    options.metadata,
    options.network,
    options.account,
    client
  );
}

// ========================================
// Exports
// ========================================

export { GlinForgeClient };
export { EventWatcher, ContractEvent };
export type { WatchOptions };
export { ArgumentEncoder, encodeArgs, balance, decodeBalance, formatBalance };
export type { ContractArg };
export { NetworkHelper };
export type { BalanceInfo, FaucetResult, GasEstimate };
export { Transaction };
export type { TransactionResult, TransactionReceipt };
export { TypedContract, TypedContractFactory };
export type { TypedDeployOptions, TypedContractOptions, QueryMethodsOf, TransactionMethodsOf, EventNamesOf };
export { defineConfig, mergeConfig, loadConfig, getNetwork, validateConfig, ConfigBuilder, defaultConfig };
export type {
  ForgeConfig,
  NetworkConfig,
  PathsConfig,
  CompilerConfig,
  TypeGenConfig,
  TestConfig,
  DeploymentConfig,
  Config,
  Network as NetworkType,
  Paths,
  Compiler,
  TypeGen,
  Test,
  Deployment,
} from './config';

// Default export for convenience
export default {
  deploy,
  getContract,
  Network,
  Signer,
  Contract,
  EventWatcher,
  ArgumentEncoder,
  NetworkHelper,
  Transaction,
  encodeArgs,
  balance,
  decodeBalance,
  formatBalance,
};
