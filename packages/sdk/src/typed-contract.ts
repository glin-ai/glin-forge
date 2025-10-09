import { Contract, Network, Signer } from './index';
import { EventWatcher, ContractEvent } from './events';
import { Transaction } from './transaction';
import type { ContractArg } from './encoding';

/**
 * Base class for type-safe contract interactions
 *
 * This class is extended by generated contract-specific classes that provide
 * full type safety for all contract methods, events, and constructors.
 *
 * Generated classes will have:
 * - Typed query methods (read-only)
 * - Typed transaction methods (state-changing)
 * - Typed event interfaces
 * - Typed constructor parameters
 *
 * @example
 * ```typescript
 * // Generated code example:
 * class Flipper extends TypedContract<FlipperQueries, FlipperTransactions, FlipperEvents> {
 *   query = {
 *     get: async (): Promise<boolean> => {
 *       return this.queryRaw('get', []);
 *     }
 *   };
 *
 *   tx = {
 *     flip: async (): Promise<Transaction> => {
 *       return this.callRaw('flip', []);
 *     }
 *   };
 * }
 * ```
 */
export abstract class TypedContract<
  TQueries = Record<string, (...args: any[]) => Promise<any>>,
  TTransactions = Record<string, (...args: any[]) => Promise<Transaction>>,
  TEvents = Record<string, any>
> {
  /** The underlying untyped Contract instance */
  protected readonly contract: Contract;

  /** Contract address */
  public readonly address: string;

  /** Path to contract metadata */
  public readonly metadata: string;

  /** Network the contract is deployed on */
  public readonly network: Network | string;

  /** Account to use for transactions */
  public readonly account: Signer | string;

  /**
   * Type-safe query methods (read-only)
   * Implemented by generated contract classes
   */
  public abstract readonly query: TQueries;

  /**
   * Type-safe transaction methods (state-changing)
   * Implemented by generated contract classes
   */
  public abstract readonly tx: TTransactions;

  /**
   * Event type definitions
   * Implemented by generated contract classes
   */
  public abstract readonly events: TEvents;

  constructor(
    address: string,
    metadata: string,
    network: Network | string,
    account: Signer | string,
    contract: Contract
  ) {
    this.address = address;
    this.metadata = metadata;
    this.network = network;
    this.account = account;
    this.contract = contract;
  }

  /**
   * Call a query method (read-only)
   *
   * This is used internally by generated query methods.
   * Use the typed `query` property instead.
   *
   * @internal
   */
  protected async queryRaw(method: string, args: ContractArg[] = []): Promise<any> {
    return this.contract.query(method, args);
  }

  /**
   * Call a transaction method (state-changing)
   *
   * This is used internally by generated transaction methods.
   * Use the typed `tx` property instead.
   *
   * @internal
   */
  protected async callRaw(
    method: string,
    args: ContractArg[] = [],
    value: number = 0
  ): Promise<Transaction> {
    return this.contract.call(method, args, value);
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
  public watch(options?: {
    event?: string;
    follow?: boolean;
    limit?: number;
    fromBlock?: number;
  }): EventWatcher {
    return this.contract.watch(options);
  }

  /**
   * Listen for contract events
   *
   * @example
   * ```typescript
   * await contract.on('Transfer', (event) => {
   *   console.log('Transfer:', event.data);
   * });
   * ```
   */
  public async on(
    eventName: string,
    callback: (event: ContractEvent) => void,
    options?: {
      follow?: boolean;
      limit?: number;
      fromBlock?: number;
    }
  ): Promise<void> {
    return this.contract.on(eventName, callback, options);
  }
}

/**
 * Options for deploying a typed contract
 */
export interface TypedDeployOptions<TArgs extends any[] = any[]> {
  /** Path to compiled WASM file */
  wasm: string;
  /** Path to contract metadata JSON */
  metadata: string;
  /** Constructor arguments (type-safe) */
  args?: TArgs;
  /** Value to transfer (in smallest units) */
  value?: number | bigint;
  /** Network to deploy to */
  network: Network | string;
  /** Account to deploy from */
  account: Signer | string;
  /** Gas limit */
  gasLimit?: number | bigint;
  /** Salt for create2-style deployment */
  salt?: string;
}

/**
 * Options for getting an existing typed contract instance
 */
export interface TypedContractOptions {
  /** Contract address */
  address: string;
  /** Path to contract metadata JSON */
  metadata: string;
  /** Network the contract is on */
  network: Network | string;
  /** Account to use for transactions */
  account: Signer | string;
}

/**
 * Base factory for typed contract deployment and instantiation
 *
 * Generated contract classes will provide static methods that return
 * properly typed instances.
 *
 * @example
 * ```typescript
 * // Generated code example:
 * class Flipper extends TypedContract<...> {
 *   static async deploy(
 *     initValue: boolean,
 *     options: Omit<TypedDeployOptions, 'args'>
 *   ): Promise<Flipper> {
 *     const deployOpts = { ...options, args: [initValue] };
 *     const contract = await TypedContractFactory.deployContract(deployOpts);
 *     return new Flipper(
 *       contract.address,
 *       options.metadata,
 *       options.network,
 *       options.account,
 *       contract
 *     );
 *   }
 *
 *   static at(options: TypedContractOptions): Flipper {
 *     const contract = TypedContractFactory.getContract(options);
 *     return new Flipper(
 *       options.address,
 *       options.metadata,
 *       options.network,
 *       options.account,
 *       contract
 *     );
 *   }
 * }
 * ```
 */
export class TypedContractFactory {
  /**
   * Deploy a typed contract
   *
   * @internal - Use the generated static deploy() method on contract classes
   */
  static async deployContract(options: TypedDeployOptions): Promise<Contract> {
    const { deploy } = await import('./index');
    return deploy({
      wasm: options.wasm,
      metadata: options.metadata,
      args: options.args || [],
      value: options.value ? Number(options.value) : undefined,
      network: options.network,
      account: options.account,
      gasLimit: options.gasLimit ? Number(options.gasLimit) : undefined,
      salt: options.salt,
    });
  }

  /**
   * Get an existing typed contract instance
   *
   * @internal - Use the generated static at() method on contract classes
   */
  static getContract(options: TypedContractOptions): Contract {
    const { getContract } = require('./index');
    return getContract({
      address: options.address,
      metadata: options.metadata,
      network: options.network,
      account: options.account,
    });
  }
}

/**
 * Helper type for extracting query method names from a typed contract
 */
export type QueryMethodsOf<T> = T extends TypedContract<infer Q, any, any> ? keyof Q : never;

/**
 * Helper type for extracting transaction method names from a typed contract
 */
export type TransactionMethodsOf<T> = T extends TypedContract<any, infer TX, any> ? keyof TX : never;

/**
 * Helper type for extracting event names from a typed contract
 */
export type EventNamesOf<T> = T extends TypedContract<any, any, infer E> ? keyof E : never;
