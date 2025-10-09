/**
 * ArgumentEncoder handles type marshaling between JavaScript types and contract arguments
 *
 * Supports:
 * - Numbers (including BigInt)
 * - Strings
 * - Booleans
 * - Arrays
 * - Objects (struct-like data)
 * - AccountIds (addresses)
 * - Optionals (Option types)
 */

// ========================================
// Types
// ========================================

export type ContractArg =
  | string
  | number
  | bigint
  | boolean
  | null
  | undefined
  | ContractArg[]
  | { [key: string]: ContractArg };

// ========================================
// ArgumentEncoder Class
// ========================================

export class ArgumentEncoder {
  /**
   * Encode a single argument to a contract-compatible string
   */
  static encode(arg: ContractArg): string {
    if (arg === null || arg === undefined) {
      return 'null';
    }

    if (typeof arg === 'string') {
      return this.encodeString(arg);
    }

    if (typeof arg === 'number') {
      return arg.toString();
    }

    if (typeof arg === 'bigint') {
      return arg.toString();
    }

    if (typeof arg === 'boolean') {
      return arg.toString();
    }

    if (Array.isArray(arg)) {
      return this.encodeArray(arg);
    }

    if (typeof arg === 'object') {
      return this.encodeObject(arg);
    }

    throw new Error(`Unsupported argument type: ${typeof arg}`);
  }

  /**
   * Encode multiple arguments to an array of contract-compatible strings
   */
  static encodeAll(args: ContractArg[]): string[] {
    return args.map((arg) => this.encode(arg));
  }

  /**
   * Encode a string value
   * Handles special cases like addresses (AccountId)
   */
  private static encodeString(value: string): string {
    // Check if it's an address (starts with 5, looks like substrate address)
    if (this.isAddress(value)) {
      return value;
    }

    // Regular string - return as-is for now
    // The backend will handle proper string encoding
    return value;
  }

  /**
   * Encode an array
   */
  private static encodeArray(arr: ContractArg[]): string {
    const encoded = arr.map((item) => this.encode(item));
    return `[${encoded.join(',')}]`;
  }

  /**
   * Encode an object (struct-like data)
   */
  private static encodeObject(obj: { [key: string]: ContractArg }): string {
    const entries = Object.entries(obj).map(([key, value]) => {
      const encodedValue = this.encode(value);
      return `"${key}":${encodedValue}`;
    });
    return `{${entries.join(',')}}`;
  }

  /**
   * Check if a string looks like a Substrate address
   */
  private static isAddress(value: string): boolean {
    // Substrate addresses are base58-encoded and typically start with 5
    // They are 48-58 characters long
    return /^[1-9A-HJ-NP-Za-km-z]{47,48}$/.test(value) && value.startsWith('5');
  }

  /**
   * Create an Option type (Some/None)
   */
  static option(value: ContractArg | null | undefined): string {
    if (value === null || value === undefined) {
      return 'null';
    }
    return this.encode(value);
  }

  /**
   * Encode a balance value (handles different units)
   *
   * @param value - The value to encode
   * @param decimals - Token decimals (default: 18 for GLIN)
   * @returns Encoded balance string
   *
   * @example
   * ```typescript
   * // 1 GLIN (with 18 decimals)
   * ArgumentEncoder.balance(1, 18) // "1000000000000000000"
   *
   * // 0.5 GLIN
   * ArgumentEncoder.balance(0.5, 18) // "500000000000000000"
   *
   * // Already in smallest unit
   * ArgumentEncoder.balance(1000000000000000000n) // "1000000000000000000"
   * ```
   */
  static balance(value: number | bigint | string, decimals: number = 18): string {
    if (typeof value === 'bigint') {
      return value.toString();
    }

    if (typeof value === 'string') {
      // Try to parse as BigInt if already in smallest unit
      try {
        return BigInt(value).toString();
      } catch {
        // If parsing fails, treat as decimal number
        value = parseFloat(value);
      }
    }

    // Convert from token units to smallest unit
    const multiplier = BigInt(10) ** BigInt(decimals);
    const valueInSmallestUnit = BigInt(Math.floor(value * Number(multiplier)));
    return valueInSmallestUnit.toString();
  }

  /**
   * Decode a balance from smallest unit to token units
   *
   * @param value - Balance in smallest unit
   * @param decimals - Token decimals (default: 18 for GLIN)
   * @returns Balance as a number
   *
   * @example
   * ```typescript
   * ArgumentEncoder.decodeBalance("1000000000000000000", 18) // 1.0
   * ArgumentEncoder.decodeBalance("500000000000000000", 18) // 0.5
   * ```
   */
  static decodeBalance(value: string | bigint, decimals: number = 18): number {
    const bigIntValue = typeof value === 'string' ? BigInt(value) : value;
    const divisor = BigInt(10) ** BigInt(decimals);
    return Number(bigIntValue) / Number(divisor);
  }

  /**
   * Format a balance for display (adds commas, limits decimals)
   *
   * @param value - Balance in smallest unit
   * @param decimals - Token decimals (default: 18)
   * @param displayDecimals - Number of decimals to show (default: 4)
   * @returns Formatted balance string
   *
   * @example
   * ```typescript
   * ArgumentEncoder.formatBalance("1234567890000000000", 18, 2) // "1.23 GLIN"
   * ArgumentEncoder.formatBalance("1000000000000000000", 18) // "1.0000 GLIN"
   * ```
   */
  static formatBalance(
    value: string | bigint,
    decimals: number = 18,
    displayDecimals: number = 4,
    symbol: string = 'GLIN'
  ): string {
    const balance = this.decodeBalance(value, decimals);
    const formatted = balance.toFixed(displayDecimals);
    return `${formatted} ${symbol}`;
  }
}

// ========================================
// Convenience Functions
// ========================================

/**
 * Encode arguments (convenience function)
 */
export function encodeArgs(...args: ContractArg[]): string[] {
  return ArgumentEncoder.encodeAll(args);
}

/**
 * Create an Option type (convenience function)
 */
export function option(value: ContractArg | null | undefined): string {
  return ArgumentEncoder.option(value);
}

/**
 * Encode a balance (convenience function)
 */
export function balance(value: number | bigint | string, decimals: number = 18): string {
  return ArgumentEncoder.balance(value, decimals);
}

/**
 * Decode a balance (convenience function)
 */
export function decodeBalance(value: string | bigint, decimals: number = 18): number {
  return ArgumentEncoder.decodeBalance(value, decimals);
}

/**
 * Format a balance for display (convenience function)
 */
export function formatBalance(
  value: string | bigint,
  decimals: number = 18,
  displayDecimals: number = 4,
  symbol: string = 'GLIN'
): string {
  return ArgumentEncoder.formatBalance(value, decimals, displayDecimals, symbol);
}

// ========================================
// Type Guards
// ========================================

/**
 * Check if a value is a valid contract argument
 */
export function isValidArg(value: any): value is ContractArg {
  if (value === null || value === undefined) {
    return true;
  }

  const type = typeof value;
  if (
    type === 'string' ||
    type === 'number' ||
    type === 'bigint' ||
    type === 'boolean'
  ) {
    return true;
  }

  if (Array.isArray(value)) {
    return value.every((item) => isValidArg(item));
  }

  if (type === 'object') {
    return Object.values(value).every((v) => isValidArg(v));
  }

  return false;
}
