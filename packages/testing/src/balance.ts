/**
 * Balance testing utilities
 *
 * Provides helpers for checking and manipulating account balances,
 * useful for testing token transfers and financial operations.
 */

import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';

let api: ApiPromise | null = null;

/**
 * Initialize balance utilities with API instance
 */
export function initBalance(apiInstance: ApiPromise): void {
  api = apiInstance;
}

/**
 * Get the balance of an account
 */
export async function getBalance(account: string | KeyringPair): Promise<bigint> {
  if (!api) {
    throw new Error('Balance utilities not initialized. Call initBalance() first.');
  }

  const address = typeof account === 'string' ? account : account.address;
  const accountInfo: any = await api.query.system.account(address);
  return BigInt(accountInfo.data.free.toString());
}

/**
 * Get free balance (spendable balance)
 */
export async function getFreeBalance(account: string | KeyringPair): Promise<bigint> {
  return getBalance(account);
}

/**
 * Get reserved balance
 */
export async function getReservedBalance(
  account: string | KeyringPair
): Promise<bigint> {
  if (!api) {
    throw new Error('Balance utilities not initialized. Call initBalance() first.');
  }

  const address = typeof account === 'string' ? account : account.address;
  const accountInfo: any = await api.query.system.account(address);
  return BigInt(accountInfo.data.reserved.toString());
}

/**
 * Get total balance (free + reserved)
 */
export async function getTotalBalance(
  account: string | KeyringPair
): Promise<bigint> {
  const free = await getFreeBalance(account);
  const reserved = await getReservedBalance(account);
  return free + reserved;
}

/**
 * Format balance to human-readable string (with decimals)
 */
export function formatBalance(balance: bigint, decimals: number = 18): string {
  const divisor = BigInt(10 ** decimals);
  const whole = balance / divisor;
  const fraction = balance % divisor;

  if (fraction === 0n) {
    return whole.toString();
  }

  const fractionStr = fraction.toString().padStart(decimals, '0');
  const trimmed = fractionStr.replace(/0+$/, '');
  return `${whole}.${trimmed}`;
}

/**
 * Parse balance from human-readable string to bigint
 */
export function parseBalance(balanceStr: string, decimals: number = 18): bigint {
  const parts = balanceStr.split('.');
  const whole = BigInt(parts[0] || '0');
  const fraction = parts[1] || '0';

  const fractionPadded = fraction.padEnd(decimals, '0').slice(0, decimals);
  const fractionBigInt = BigInt(fractionPadded);

  return whole * BigInt(10 ** decimals) + fractionBigInt;
}

/**
 * Check if an account has at least the specified balance
 */
export async function hasBalance(
  account: string | KeyringPair,
  amount: bigint
): Promise<boolean> {
  const balance = await getBalance(account);
  return balance >= amount;
}

/**
 * Assert that an account has a specific balance
 */
export async function expectBalance(
  account: string | KeyringPair,
  expected: bigint,
  message?: string
): Promise<void> {
  const actual = await getBalance(account);
  if (actual !== expected) {
    const address = typeof account === 'string' ? account : account.address;
    throw new Error(
      message ||
        `Balance mismatch for ${address}. Expected: ${expected}, Got: ${actual}`
    );
  }
}

/**
 * Assert that an account has at least a certain balance
 */
export async function expectMinBalance(
  account: string | KeyringPair,
  minimum: bigint,
  message?: string
): Promise<void> {
  const actual = await getBalance(account);
  if (actual < minimum) {
    const address = typeof account === 'string' ? account : account.address;
    throw new Error(
      message ||
        `Balance too low for ${address}. Minimum: ${minimum}, Got: ${actual}`
    );
  }
}

/**
 * Track balance changes during a transaction
 */
export class BalanceTracker {
  private balances: Map<string, bigint> = new Map();

  /**
   * Record balance before transaction
   */
  async before(account: string | KeyringPair): Promise<void> {
    const address = typeof account === 'string' ? account : account.address;
    const balance = await getBalance(account);
    this.balances.set(address, balance);
  }

  /**
   * Get balance change after transaction
   */
  async after(account: string | KeyringPair): Promise<bigint> {
    const address = typeof account === 'string' ? account : account.address;
    const before = this.balances.get(address);

    if (before === undefined) {
      throw new Error(`No before balance recorded for ${address}`);
    }

    const current = await getBalance(account);
    return current - before;
  }

  /**
   * Assert balance change
   */
  async expectChange(
    account: string | KeyringPair,
    expected: bigint,
    message?: string
  ): Promise<void> {
    const actual = await this.after(account);
    if (actual !== expected) {
      const address = typeof account === 'string' ? account : account.address;
      throw new Error(
        message ||
          `Balance change mismatch for ${address}. Expected: ${expected}, Got: ${actual}`
      );
    }
  }

  /**
   * Assert balance increased
   */
  async expectIncrease(
    account: string | KeyringPair,
    message?: string
  ): Promise<void> {
    const change = await this.after(account);
    if (change <= 0n) {
      const address = typeof account === 'string' ? account : account.address;
      throw new Error(
        message || `Expected balance increase for ${address}, but got ${change}`
      );
    }
  }

  /**
   * Assert balance decreased
   */
  async expectDecrease(
    account: string | KeyringPair,
    message?: string
  ): Promise<void> {
    const change = await this.after(account);
    if (change >= 0n) {
      const address = typeof account === 'string' ? account : account.address;
      throw new Error(
        message || `Expected balance decrease for ${address}, but got ${change}`
      );
    }
  }

  /**
   * Clear tracked balances
   */
  clear(): void {
    this.balances.clear();
  }
}

/**
 * Create a balance tracker instance
 */
export function createBalanceTracker(): BalanceTracker {
  return new BalanceTracker();
}

/**
 * Helper to track balance changes for a single transaction
 */
export async function trackBalance<T>(
  account: string | KeyringPair,
  fn: () => Promise<T>
): Promise<{ result: T; change: bigint }> {
  const tracker = createBalanceTracker();
  await tracker.before(account);
  const result = await fn();
  const change = await tracker.after(account);
  return { result, change };
}

/**
 * Balance utilities namespace
 */
export const balance = {
  get: getBalance,
  getFree: getFreeBalance,
  getReserved: getReservedBalance,
  getTotal: getTotalBalance,
  format: formatBalance,
  parse: parseBalance,
  has: hasBalance,
  expect: expectBalance,
  expectMin: expectMinBalance,
  createTracker: createBalanceTracker,
  track: trackBalance,
};
