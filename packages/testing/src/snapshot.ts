/**
 * Snapshot and revert utilities for testing
 *
 * Provides functionality to take blockchain state snapshots
 * and revert to previous states.
 */

import type { ApiPromise } from '@polkadot/api';

/**
 * Snapshot identifier
 */
export type SnapshotId = string;

/**
 * Snapshot metadata
 */
interface Snapshot {
  id: SnapshotId;
  blockNumber: number;
  timestamp: number;
  blockHash: string;
}

let api: ApiPromise | null = null;
const snapshots: Map<SnapshotId, Snapshot> = new Map();
let snapshotCounter = 0;

/**
 * Initialize snapshot utilities with API instance
 */
export function initSnapshot(apiInstance: ApiPromise): void {
  api = apiInstance;
}

/**
 * Take a snapshot of the current blockchain state
 */
export async function takeSnapshot(): Promise<SnapshotId> {
  if (!api) {
    throw new Error('Snapshot utilities not initialized. Call initSnapshot() first.');
  }

  const header = await api.rpc.chain.getHeader();
  const blockNumber = header.number.toNumber();
  const blockHash = header.hash.toHex();
  const timestamp = Date.now();

  const id = `snapshot_${++snapshotCounter}_${blockNumber}`;

  snapshots.set(id, {
    id,
    blockNumber,
    timestamp,
    blockHash,
  });

  return id;
}

/**
 * Revert to a previously taken snapshot
 *
 * Note: This functionality requires a development chain with time-travel capabilities.
 * On production chains, this will throw an error.
 */
export async function revertToSnapshot(snapshotId: SnapshotId): Promise<void> {
  if (!api) {
    throw new Error('Snapshot utilities not initialized. Call initSnapshot() first.');
  }

  const snapshot = snapshots.get(snapshotId);
  if (!snapshot) {
    throw new Error(`Snapshot ${snapshotId} not found`);
  }

  // This requires chain with dev RPC methods
  // For Substrate-based chains, this is typically not available
  // In practice, tests should use a fresh chain state for each test
  console.warn(
    'Snapshot revert is not fully supported on all chains. Consider using a fresh chain state for each test instead.'
  );

  // Remove all snapshots taken after this one
  for (const [id, snap] of snapshots.entries()) {
    if (snap.blockNumber > snapshot.blockNumber) {
      snapshots.delete(id);
    }
  }
}

/**
 * Get snapshot information
 */
export function getSnapshot(snapshotId: SnapshotId): Snapshot | undefined {
  return snapshots.get(snapshotId);
}

/**
 * List all snapshots
 */
export function listSnapshots(): Snapshot[] {
  return Array.from(snapshots.values()).sort((a, b) => a.blockNumber - b.blockNumber);
}

/**
 * Delete a specific snapshot
 */
export function deleteSnapshot(snapshotId: SnapshotId): boolean {
  return snapshots.delete(snapshotId);
}

/**
 * Clear all snapshots
 */
export function clearSnapshots(): void {
  snapshots.clear();
}

/**
 * Get the latest snapshot
 */
export function getLatestSnapshot(): Snapshot | undefined {
  const all = listSnapshots();
  return all[all.length - 1];
}

/**
 * Fixture helper - runs test with automatic snapshot/revert
 */
export async function fixture<T>(fn: () => Promise<T>): Promise<T> {
  const snapshotId = await takeSnapshot();
  try {
    return await fn();
  } finally {
    await revertToSnapshot(snapshotId);
  }
}

/**
 * Create a test context with automatic snapshot management
 */
export class SnapshotContext {
  private snapshotStack: SnapshotId[] = [];

  /**
   * Push a new snapshot onto the stack
   */
  async push(): Promise<SnapshotId> {
    const id = await takeSnapshot();
    this.snapshotStack.push(id);
    return id;
  }

  /**
   * Pop and revert to the last snapshot
   */
  async pop(): Promise<void> {
    const id = this.snapshotStack.pop();
    if (!id) {
      throw new Error('No snapshot to pop');
    }
    await revertToSnapshot(id);
  }

  /**
   * Revert to the last snapshot without popping
   */
  async revert(): Promise<void> {
    const id = this.snapshotStack[this.snapshotStack.length - 1];
    if (!id) {
      throw new Error('No snapshot to revert to');
    }
    await revertToSnapshot(id);
  }

  /**
   * Clear all snapshots in this context
   */
  clear(): void {
    this.snapshotStack = [];
  }

  /**
   * Get the depth of the snapshot stack
   */
  depth(): number {
    return this.snapshotStack.length;
  }
}

/**
 * Create a snapshot context for managing multiple snapshots
 */
export function createSnapshotContext(): SnapshotContext {
  return new SnapshotContext();
}

/**
 * Alternative approach: Database-level snapshots
 *
 * For chains that don't support state reversion, use database snapshots instead.
 * This requires stopping the chain, copying the database, and restarting.
 */
export class DatabaseSnapshot {
  private snapshotPath?: string;

  /**
   * Take a database-level snapshot
   */
  async take(chainDataPath: string): Promise<void> {
    // Implementation would copy the chain database directory
    // This is chain-specific and requires file system access
    console.warn('Database snapshots require file system access and chain restart');
    this.snapshotPath = chainDataPath;
  }

  /**
   * Restore from database snapshot
   */
  async restore(): Promise<void> {
    if (!this.snapshotPath) {
      throw new Error('No database snapshot taken');
    }
    // Implementation would restore the chain database from backup
    console.warn('Database restore requires stopping and restarting the chain');
  }
}

/**
 * Snapshot utilities namespace
 */
export const snapshot = {
  take: takeSnapshot,
  revert: revertToSnapshot,
  get: getSnapshot,
  list: listSnapshots,
  delete: deleteSnapshot,
  clear: clearSnapshots,
  latest: getLatestSnapshot,
  fixture,
  createContext: createSnapshotContext,
};
