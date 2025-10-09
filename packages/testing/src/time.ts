/**
 * Time manipulation utilities for testing
 *
 * Provides functions to manipulate blockchain time, mine blocks,
 * and control block production.
 */

import type { ApiPromise } from '@polkadot/api';

/**
 * Configuration for time utilities
 */
export interface TimeConfig {
  api?: ApiPromise;
  autoMine?: boolean;
  blockTime?: number; // in milliseconds
}

let config: TimeConfig = {
  autoMine: true,
  blockTime: 6000, // 6 seconds default
};

/**
 * Initialize time utilities with API instance
 */
export function initTime(api: ApiPromise, options: Partial<TimeConfig> = {}): void {
  config = {
    ...config,
    ...options,
    api,
  };
}

/**
 * Get current block number
 */
export async function getCurrentBlock(): Promise<number> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }
  const header = await config.api.rpc.chain.getHeader();
  return header.number.toNumber();
}

/**
 * Get current block timestamp
 */
export async function getCurrentTimestamp(): Promise<number> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }
  const now = await config.api.query.timestamp.now();
  return now.toNumber();
}

/**
 * Mine a single block
 */
export async function mineBlock(): Promise<number> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }

  // Wait for next block
  return new Promise((resolve) => {
    const unsubscribe = config.api!.rpc.chain.subscribeNewHeads((header) => {
      unsubscribe.then((unsub: any) => unsub());
      resolve(header.number.toNumber());
    });
  });
}

/**
 * Mine multiple blocks
 */
export async function mineBlocks(count: number): Promise<number> {
  let currentBlock = await getCurrentBlock();

  for (let i = 0; i < count; i++) {
    currentBlock = await mineBlock();
  }

  return currentBlock;
}

/**
 * Increase blockchain time by the specified number of seconds
 */
export async function increaseTime(seconds: number): Promise<number> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }

  const blocksToMine = Math.ceil((seconds * 1000) / config.blockTime!);
  return mineBlocks(blocksToMine);
}

/**
 * Set the next block timestamp
 */
export async function setNextBlockTimestamp(timestamp: number): Promise<void> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }

  // This is a dev RPC method - may not be available on all chains
  // For local development chains only
  console.warn('setNextBlockTimestamp is only available on development chains');
}

/**
 * Fast forward time to a specific timestamp
 */
export async function fastForwardTo(timestamp: number): Promise<void> {
  const currentTimestamp = await getCurrentTimestamp();
  if (timestamp <= currentTimestamp) {
    throw new Error('Target timestamp must be in the future');
  }

  const secondsToIncrease = Math.ceil((timestamp - currentTimestamp) / 1000);
  await increaseTime(secondsToIncrease);
}

/**
 * Wait for a specific number of blocks
 */
export async function waitForBlocks(count: number): Promise<void> {
  await mineBlocks(count);
}

/**
 * Get latest block with full details
 */
export async function getLatestBlock(): Promise<{
  number: number;
  hash: string;
  timestamp: number;
}> {
  if (!config.api) {
    throw new Error('Time utilities not initialized. Call initTime() first.');
  }

  const header = await config.api.rpc.chain.getHeader();
  const hash = header.hash.toHex();
  const block = await config.api.rpc.chain.getBlock(hash);
  const timestamp = await getCurrentTimestamp();

  return {
    number: header.number.toNumber(),
    hash,
    timestamp,
  };
}

/**
 * Enable/disable auto-mining
 */
export function setAutoMine(enabled: boolean): void {
  config.autoMine = enabled;
}

/**
 * Set block time interval (in milliseconds)
 */
export function setBlockTime(ms: number): void {
  config.blockTime = ms;
}

/**
 * Time travel utilities for testing time-dependent contracts
 */
export const time = {
  getCurrentBlock,
  getCurrentTimestamp,
  mineBlock,
  mineBlocks,
  increaseTime,
  setNextBlockTimestamp,
  fastForwardTo,
  waitForBlocks,
  getLatestBlock,
  setAutoMine,
  setBlockTime,
};
