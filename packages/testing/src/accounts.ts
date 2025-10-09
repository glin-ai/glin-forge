/**
 * Account testing utilities
 *
 * Provides helper functions for managing test accounts, generating signers,
 * and mocking account behavior.
 */

import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import { cryptoWaitReady } from '@polkadot/util-crypto';

let keyring: Keyring | null = null;
let accountCache: Map<string, KeyringPair> = new Map();

/**
 * Initialize the keyring (must be called before using accounts)
 */
export async function initAccounts(): Promise<void> {
  if (!keyring) {
    await cryptoWaitReady();
    keyring = new Keyring({ type: 'sr25519' });
  }
}

/**
 * Get a development account by name (alice, bob, charlie, etc.)
 */
export async function getAccount(name: string): Promise<KeyringPair> {
  await initAccounts();

  const cached = accountCache.get(name);
  if (cached) return cached;

  const account = keyring!.addFromUri(`//${name.charAt(0).toUpperCase()}${name.slice(1)}`);
  accountCache.set(name, account);
  return account;
}

/**
 * Get multiple accounts at once
 */
export async function getAccounts(names: string[]): Promise<KeyringPair[]> {
  return Promise.all(names.map(name => getAccount(name)));
}

/**
 * Get common test accounts (Alice, Bob, Charlie, Dave, Eve, Ferdie)
 */
export async function getTestAccounts(): Promise<{
  alice: KeyringPair;
  bob: KeyringPair;
  charlie: KeyringPair;
  dave: KeyringPair;
  eve: KeyringPair;
  ferdie: KeyringPair;
}> {
  const [alice, bob, charlie, dave, eve, ferdie] = await getAccounts([
    'alice',
    'bob',
    'charlie',
    'dave',
    'eve',
    'ferdie',
  ]);

  return { alice, bob, charlie, dave, eve, ferdie };
}

/**
 * Create a new random account
 */
export async function createRandomAccount(): Promise<KeyringPair> {
  await initAccounts();
  return keyring!.addFromUri(`//${Math.random().toString(36).substring(7)}`);
}

/**
 * Create multiple random accounts
 */
export async function createRandomAccounts(count: number): Promise<KeyringPair[]> {
  const accounts: KeyringPair[] = [];
  for (let i = 0; i < count; i++) {
    accounts.push(await createRandomAccount());
  }
  return accounts;
}

/**
 * Create an account from a seed phrase
 */
export async function createAccountFromSeed(seed: string): Promise<KeyringPair> {
  await initAccounts();
  return keyring!.addFromUri(seed);
}

/**
 * Create an account from a mnemonic
 */
export async function createAccountFromMnemonic(mnemonic: string): Promise<KeyringPair> {
  await initAccounts();
  return keyring!.addFromMnemonic(mnemonic);
}

/**
 * Get account address as string
 */
export function getAddress(account: KeyringPair): string {
  return account.address;
}

/**
 * Get account public key as hex
 */
export function getPublicKey(account: KeyringPair): string {
  return `0x${Buffer.from(account.publicKey).toString('hex')}`;
}

/**
 * Clear account cache (useful for test cleanup)
 */
export function clearAccountCache(): void {
  accountCache.clear();
}

/**
 * Account impersonation for testing
 */
export class AccountImpersonator {
  private originalSigner: KeyringPair | null = null;

  /**
   * Start impersonating an account
   */
  async impersonate(account: KeyringPair | string): Promise<KeyringPair> {
    if (typeof account === 'string') {
      return getAccount(account);
    }
    return account;
  }

  /**
   * Stop impersonating and return to original signer
   */
  stopImpersonating(): KeyringPair | null {
    const original = this.originalSigner;
    this.originalSigner = null;
    return original;
  }
}

/**
 * Create an impersonator instance
 */
export function createImpersonator(): AccountImpersonator {
  return new AccountImpersonator();
}
