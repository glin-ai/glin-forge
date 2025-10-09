import axios, { AxiosInstance } from 'axios';

// ========================================
// Types
// ========================================

export interface ContractEvent {
  blockNumber: number;
  eventName: string;
  data: any;
}

export interface WatchOptions {
  /** Contract address to watch */
  address: string;
  /** Event name filter (optional, shows all if not specified) */
  event?: string;
  /** Network to watch on */
  network: string;
  /** Follow mode (keep watching for new events) */
  follow?: boolean;
  /** Maximum number of events to return */
  limit?: number;
  /** Show events from block number */
  fromBlock?: number;
}

interface WatchResult {
  success: boolean;
  events: ContractEvent[];
  error?: string;
}

type EventCallback = (event: ContractEvent) => void;

// ========================================
// EventWatcher Class
// ========================================

/**
 * EventWatcher provides real-time and historical contract event monitoring
 *
 * @example
 * ```typescript
 * // Historical events (last 100 blocks)
 * const watcher = new EventWatcher({
 *   address: '5GrwvaEF...',
 *   network: Network.Testnet
 * });
 *
 * watcher.on('Transfer', (event) => {
 *   console.log('Transfer event:', event.data);
 * });
 *
 * await watcher.start();
 *
 * // Real-time events (follow mode)
 * const liveWatcher = new EventWatcher({
 *   address: '5GrwvaEF...',
 *   network: Network.Testnet,
 *   follow: true
 * });
 *
 * liveWatcher.on('*', (event) => {
 *   console.log('Any event:', event.eventName, event.data);
 * });
 *
 * await liveWatcher.start();
 *
 * // Stop watching
 * liveWatcher.stop();
 * ```
 */
export class EventWatcher {
  private rpcUrl: string;
  private axios: AxiosInstance;
  private requestId: number = 0;
  private listeners: Map<string, EventCallback[]> = new Map();
  private running: boolean = false;
  private stopRequested: boolean = false;

  constructor(private options: WatchOptions) {
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
      timeout: 300000, // 5 minutes for long-running watches
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Register an event listener
   *
   * @param eventName - Event name to listen for, or '*' for all events
   * @param callback - Function to call when event occurs
   */
  on(eventName: string, callback: EventCallback): void {
    if (!this.listeners.has(eventName)) {
      this.listeners.set(eventName, []);
    }
    this.listeners.get(eventName)!.push(callback);
  }

  /**
   * Remove an event listener
   *
   * @param eventName - Event name
   * @param callback - Callback to remove (if not provided, removes all callbacks for this event)
   */
  off(eventName: string, callback?: EventCallback): void {
    if (!this.listeners.has(eventName)) {
      return;
    }

    if (!callback) {
      this.listeners.delete(eventName);
      return;
    }

    const callbacks = this.listeners.get(eventName)!;
    const index = callbacks.indexOf(callback);
    if (index > -1) {
      callbacks.splice(index, 1);
    }

    if (callbacks.length === 0) {
      this.listeners.delete(eventName);
    }
  }

  /**
   * Remove all event listeners
   */
  removeAllListeners(): void {
    this.listeners.clear();
  }

  /**
   * Start watching for events
   *
   * In follow mode, this will continue until stop() is called.
   * In historical mode, this will return after fetching events.
   */
  async start(): Promise<void> {
    if (this.running) {
      throw new Error('EventWatcher is already running');
    }

    this.running = true;
    this.stopRequested = false;

    if (this.options.follow) {
      // Follow mode: keep watching until stopped
      await this.watchContinuously();
    } else {
      // Historical mode: fetch once and return
      await this.fetchHistoricalEvents();
    }
  }

  /**
   * Stop watching for events (only applicable in follow mode)
   */
  stop(): void {
    this.stopRequested = true;
    this.running = false;
  }

  /**
   * Check if the watcher is currently running
   */
  isRunning(): boolean {
    return this.running;
  }

  private async fetchHistoricalEvents(): Promise<void> {
    try {
      const result = await this.rpc('watch', {
        address: this.options.address,
        event: this.options.event,
        network: this.options.network,
        follow: false,
        limit: this.options.limit,
        fromBlock: this.options.fromBlock,
      });

      if (!result.success) {
        throw new Error(result.error || 'Failed to fetch events');
      }

      // Emit all events to listeners
      for (const event of result.events) {
        this.emitEvent(event);
      }
    } finally {
      this.running = false;
    }
  }

  private async watchContinuously(): Promise<void> {
    while (!this.stopRequested) {
      try {
        const result = await this.rpc('watch', {
          address: this.options.address,
          event: this.options.event,
          network: this.options.network,
          follow: true,
          limit: this.options.limit || 10,
          fromBlock: this.options.fromBlock,
        });

        if (!result.success) {
          throw new Error(result.error || 'Failed to watch events');
        }

        // Emit all events to listeners
        for (const event of result.events) {
          this.emitEvent(event);
        }

        // If we got events, update fromBlock to avoid duplicates
        if (result.events.length > 0) {
          const lastBlock = result.events[result.events.length - 1].blockNumber;
          this.options.fromBlock = lastBlock + 1;
        }

        // Small delay before next poll (if not stopped)
        if (!this.stopRequested) {
          await new Promise((resolve) => setTimeout(resolve, 1000));
        }
      } catch (error) {
        if (!this.stopRequested) {
          console.error('Error watching events:', error);
          // Wait before retrying
          await new Promise((resolve) => setTimeout(resolve, 5000));
        }
      }
    }

    this.running = false;
  }

  private emitEvent(event: ContractEvent): void {
    // Emit to specific event listeners
    const specificListeners = this.listeners.get(event.eventName);
    if (specificListeners) {
      for (const callback of specificListeners) {
        try {
          callback(event);
        } catch (error) {
          console.error('Error in event listener:', error);
        }
      }
    }

    // Emit to wildcard listeners
    const wildcardListeners = this.listeners.get('*');
    if (wildcardListeners) {
      for (const callback of wildcardListeners) {
        try {
          callback(event);
        } catch (error) {
          console.error('Error in wildcard event listener:', error);
        }
      }
    }
  }

  private async rpc(method: string, params: any): Promise<WatchResult> {
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
 * Watch contract events (convenience function)
 *
 * @example
 * ```typescript
 * await watchEvents({
 *   address: '5GrwvaEF...',
 *   network: Network.Testnet,
 *   onEvent: (event) => console.log(event)
 * });
 * ```
 */
export async function watchEvents(options: {
  address: string;
  network: string;
  event?: string;
  follow?: boolean;
  limit?: number;
  fromBlock?: number;
  onEvent: EventCallback;
}): Promise<void> {
  const watcher = new EventWatcher({
    address: options.address,
    network: options.network,
    event: options.event,
    follow: options.follow,
    limit: options.limit,
    fromBlock: options.fromBlock,
  });

  watcher.on('*', options.onEvent);
  await watcher.start();
}
