import axios, { AxiosInstance } from 'axios';

// ========================================
// Types
// ========================================

export interface TransactionResult {
  txHash: string;
  blockHash?: string;
  success: boolean;
  error?: string;
}

export interface TransactionReceipt {
  txHash: string;
  blockHash: string;
  blockNumber: number;
  success: boolean;
  events?: any[];
}

// ========================================
// Transaction Class
// ========================================

/**
 * Transaction represents a submitted transaction and provides methods to track its status
 *
 * @example
 * ```typescript
 * const tx = await contract.call('transfer', ['5GrwvaEF...', 1000]);
 * console.log('Transaction submitted:', tx.hash());
 *
 * // Wait for finalization
 * const receipt = await tx.wait();
 * console.log('Transaction finalized in block:', receipt.blockNumber);
 * ```
 */
export class Transaction {
  private rpcUrl: string;
  private axios: AxiosInstance;
  private requestId: number = 0;
  private network: string;

  constructor(
    private result: TransactionResult,
    network: string
  ) {
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
      timeout: 300000, // 5 minutes for transaction confirmation
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Get the transaction hash
   */
  hash(): string {
    return this.result.txHash;
  }

  /**
   * Get the block hash (if already included in a block)
   */
  blockHash(): string | undefined {
    return this.result.blockHash;
  }

  /**
   * Check if transaction was successful
   */
  isSuccess(): boolean {
    return this.result.success;
  }

  /**
   * Get error message if transaction failed
   */
  error(): string | undefined {
    return this.result.error;
  }

  /**
   * Wait for transaction to be finalized
   *
   * @param timeout - Timeout in milliseconds (default: 60000ms = 1 minute)
   * @returns Transaction receipt
   *
   * @example
   * ```typescript
   * const tx = await contract.call('transfer', ['5GrwvaEF...', 1000]);
   * const receipt = await tx.wait();
   * console.log('Finalized in block:', receipt.blockNumber);
   * ```
   */
  async wait(timeout: number = 60000): Promise<TransactionReceipt> {
    if (!this.result.success) {
      throw new Error(this.result.error || 'Transaction failed');
    }

    const startTime = Date.now();
    const pollInterval = 2000; // Poll every 2 seconds

    while (Date.now() - startTime < timeout) {
      try {
        // Query transaction status
        const status = await this.getTransactionStatus(this.result.txHash);

        if (status.finalized) {
          return {
            txHash: this.result.txHash,
            blockHash: status.blockHash,
            blockNumber: status.blockNumber,
            success: status.success,
            events: status.events,
          };
        }

        // Wait before next poll
        await new Promise((resolve) => setTimeout(resolve, pollInterval));
      } catch (error) {
        // Continue polling on errors
        await new Promise((resolve) => setTimeout(resolve, pollInterval));
      }
    }

    throw new Error(`Transaction wait timeout after ${timeout}ms`);
  }

  /**
   * Wait for transaction with progress callback
   *
   * @param onProgress - Callback called on each status check
   * @param timeout - Timeout in milliseconds
   * @returns Transaction receipt
   *
   * @example
   * ```typescript
   * const receipt = await tx.waitWithProgress(
   *   (elapsed) => console.log(`Waiting... ${elapsed}ms`),
   *   60000
   * );
   * ```
   */
  async waitWithProgress(
    onProgress: (elapsedMs: number) => void,
    timeout: number = 60000
  ): Promise<TransactionReceipt> {
    const startTime = Date.now();
    const pollInterval = 2000;

    while (Date.now() - startTime < timeout) {
      const elapsed = Date.now() - startTime;
      onProgress(elapsed);

      try {
        const status = await this.getTransactionStatus(this.result.txHash);

        if (status.finalized) {
          return {
            txHash: this.result.txHash,
            blockHash: status.blockHash,
            blockNumber: status.blockNumber,
            success: status.success,
            events: status.events,
          };
        }

        await new Promise((resolve) => setTimeout(resolve, pollInterval));
      } catch (error) {
        await new Promise((resolve) => setTimeout(resolve, pollInterval));
      }
    }

    throw new Error(`Transaction wait timeout after ${timeout}ms`);
  }

  /**
   * Get the current status of the transaction
   *
   * @returns Transaction status
   */
  private async getTransactionStatus(txHash: string): Promise<{
    finalized: boolean;
    blockHash: string;
    blockNumber: number;
    success: boolean;
    events?: any[];
  }> {
    // For now, since we don't have block finality in the result,
    // we'll assume the transaction is immediately finalized if it has a block hash
    // In a real implementation, this would query the chain for transaction status

    if (this.result.blockHash) {
      // Transaction is already in a block, get block number
      const blockNumber = await this.getBlockNumber();

      return {
        finalized: true,
        blockHash: this.result.blockHash,
        blockNumber,
        success: this.result.success,
        events: [],
      };
    }

    return {
      finalized: false,
      blockHash: '',
      blockNumber: 0,
      success: false,
    };
  }

  private async getBlockNumber(): Promise<number> {
    try {
      const result = await this.rpc('getBlockNumber', {
        network: this.network,
      });

      if (!result.success) {
        return 0;
      }

      return result.blockNumber || 0;
    } catch {
      return 0;
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
// Helper Functions
// ========================================

/**
 * Create a Transaction instance from a transaction result
 */
export function createTransaction(
  result: TransactionResult,
  network: string
): Transaction {
  return new Transaction(result, network);
}
