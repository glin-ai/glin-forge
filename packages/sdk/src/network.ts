import axios, { AxiosInstance } from 'axios';
import { decodeBalance, formatBalance } from './encoding';

// ========================================
// Types
// ========================================

export interface BalanceInfo {
  /** Raw balance in smallest unit */
  raw: string;
  /** Formatted balance as number */
  value: number;
  /** Formatted balance as string with symbol */
  formatted: string;
}

export interface FaucetResult {
  success: boolean;
  amount?: string;
  txHash?: string;
  error?: string;
}

export interface GasEstimate {
  gasLimit: number;
  estimatedCost: string;
}

// ========================================
// NetworkHelper Class
// ========================================

/**
 * NetworkHelper provides network utility functions
 *
 * @example
 * ```typescript
 * const helper = new NetworkHelper(Network.Testnet);
 *
 * // Get balance
 * const balance = await helper.getBalance('5GrwvaEF...');
 * console.log(balance.formatted); // "10.5000 GLIN"
 *
 * // Request faucet
 * const result = await helper.requestFaucet('5GrwvaEF...');
 * console.log(`Received ${result.amount} GLIN`);
 *
 * // Estimate gas
 * const estimate = await helper.estimateGas({
 *   address: '5GrwvaEF...',
 *   method: 'transfer',
 *   args: ['5Account...', 1000]
 * });
 * console.log(`Estimated gas: ${estimate.gasLimit}`);
 * ```
 */
export class NetworkHelper {
  private rpcUrl: string;
  private axios: AxiosInstance;
  private requestId: number = 0;
  private network: string;

  constructor(network: string) {
    const port = process.env.GLIN_FORGE_RPC_PORT;
    if (!port) {
      throw new Error(
        'glin-forge RPC server not running. ' +
          'This SDK must be used with "glin-forge run" command.'
      );
    }

    this.network = network;
    this.rpcUrl = `http://127.0.0.1:${port}`;
    this.axios = axios.create({
      baseURL: this.rpcUrl,
      timeout: 60000,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Get account balance
   *
   * @param address - Account address
   * @param decimals - Token decimals (default: 18 for GLIN)
   * @param symbol - Token symbol for formatting (default: 'GLIN')
   * @returns Balance information
   *
   * @example
   * ```typescript
   * const balance = await helper.getBalance('5GrwvaEF...');
   * console.log(balance.formatted); // "10.5000 GLIN"
   * console.log(balance.value); // 10.5
   * console.log(balance.raw); // "10500000000000000000"
   * ```
   */
  async getBalance(
    address: string,
    decimals: number = 18,
    symbol: string = 'GLIN'
  ): Promise<BalanceInfo> {
    try {
      const result = await this.rpc('getBalance', {
        address,
        network: this.network,
      });

      if (!result.success) {
        throw new Error(result.error || 'Failed to get balance');
      }

      const raw = result.balance;
      const value = decodeBalance(raw, decimals);
      const formatted = formatBalance(raw, decimals, 4, symbol);

      return { raw, value, formatted };
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(`Failed to get balance: ${error.message}`);
      }
      throw error;
    }
  }

  /**
   * Request tokens from faucet (testnet only)
   *
   * @param address - Account address to receive tokens
   * @returns Faucet result
   *
   * @example
   * ```typescript
   * const result = await helper.requestFaucet('5GrwvaEF...');
   * if (result.success) {
   *   console.log(`Received ${result.amount} GLIN`);
   *   console.log(`Transaction: ${result.txHash}`);
   * }
   * ```
   */
  async requestFaucet(address: string): Promise<FaucetResult> {
    try {
      const result = await this.rpc('requestFaucet', {
        address,
        network: this.network,
      });

      return result;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(`Failed to request faucet: ${error.message}`);
      }
      throw error;
    }
  }

  /**
   * Estimate gas for a contract call
   *
   * @param options - Gas estimation options
   * @returns Gas estimate
   *
   * @example
   * ```typescript
   * const estimate = await helper.estimateGas({
   *   address: '5GrwvaEF...',
   *   method: 'transfer',
   *   args: ['5Account...', 1000],
   *   value: 0
   * });
   * console.log(`Estimated gas: ${estimate.gasLimit}`);
   * console.log(`Estimated cost: ${estimate.estimatedCost}`);
   * ```
   */
  async estimateGas(options: {
    address: string;
    method: string;
    args?: any[];
    value?: number;
    from?: string;
  }): Promise<GasEstimate> {
    try {
      const result = await this.rpc('estimateGas', {
        address: options.address,
        method: options.method,
        args: options.args || [],
        value: options.value || 0,
        from: options.from || 'alice',
        network: this.network,
      });

      if (!result.success) {
        throw new Error(result.error || 'Failed to estimate gas');
      }

      return {
        gasLimit: result.gasLimit,
        estimatedCost: result.estimatedCost,
      };
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(`Failed to estimate gas: ${error.message}`);
      }
      throw error;
    }
  }

  /**
   * Check if an address is valid
   *
   * @param address - Address to validate
   * @returns True if address is valid
   *
   * @example
   * ```typescript
   * const isValid = helper.isValidAddress('5GrwvaEF...');
   * console.log(isValid); // true or false
   * ```
   */
  isValidAddress(address: string): boolean {
    // Substrate addresses are base58-encoded and typically start with 5
    // They are 47-48 characters long
    return /^[1-9A-HJ-NP-Za-km-z]{47,48}$/.test(address);
  }

  /**
   * Get current block number
   *
   * @returns Current block number
   *
   * @example
   * ```typescript
   * const blockNumber = await helper.getBlockNumber();
   * console.log(`Current block: ${blockNumber}`);
   * ```
   */
  async getBlockNumber(): Promise<number> {
    try {
      const result = await this.rpc('getBlockNumber', {
        network: this.network,
      });

      if (!result.success) {
        throw new Error(result.error || 'Failed to get block number');
      }

      return result.blockNumber;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(`Failed to get block number: ${error.message}`);
      }
      throw error;
    }
  }

  /**
   * Get network information
   *
   * @returns Network information
   *
   * @example
   * ```typescript
   * const info = await helper.getNetworkInfo();
   * console.log(info);
   * // { name: 'testnet', chainId: 42, blockNumber: 123456 }
   * ```
   */
  async getNetworkInfo(): Promise<{
    name: string;
    rpc: string;
    blockNumber: number;
  }> {
    try {
      const result = await this.rpc('getNetworkInfo', {
        network: this.network,
      });

      if (!result.success) {
        throw new Error(result.error || 'Failed to get network info');
      }

      return {
        name: result.name,
        rpc: result.rpc,
        blockNumber: result.blockNumber,
      };
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new Error(`Failed to get network info: ${error.message}`);
      }
      throw error;
    }
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
}

// ========================================
// Convenience Functions
// ========================================

/**
 * Get balance (convenience function)
 */
export async function getBalance(
  address: string,
  network: string,
  decimals?: number,
  symbol?: string
): Promise<BalanceInfo> {
  const helper = new NetworkHelper(network);
  return helper.getBalance(address, decimals, symbol);
}

/**
 * Request faucet (convenience function)
 */
export async function requestFaucet(
  address: string,
  network: string
): Promise<FaucetResult> {
  const helper = new NetworkHelper(network);
  return helper.requestFaucet(address);
}

/**
 * Estimate gas (convenience function)
 */
export async function estimateGas(
  options: {
    address: string;
    method: string;
    args?: any[];
    value?: number;
    from?: string;
  },
  network: string
): Promise<GasEstimate> {
  const helper = new NetworkHelper(network);
  return helper.estimateGas(options);
}
