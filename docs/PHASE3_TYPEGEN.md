# Phase 3: Enhanced TypeGen - Type-Safe Contract Interactions

Phase 3 introduces comprehensive TypeScript type generation from ink! smart contract metadata, providing full type safety, IDE autocomplete, and compile-time error detection for contract interactions.

## Overview

The Enhanced TypeGen system generates TypeScript interfaces directly from your ink! contract metadata, giving you:

- **Full Type Safety**: All contract methods, parameters, and return types are fully typed
- **IDE Autocomplete**: Your editor shows all available methods with their signatures
- **Compile-Time Errors**: Catch mistakes before runtime
- **Self-Documenting**: Generated types serve as documentation
- **Seamless Integration**: Works with existing Phase 1 & 2 features

## Quick Start

### 1. Generate Types

```bash
# From contract metadata file
glin-forge typegen --abi ./target/ink/metadata.json --output ./types

# Or let it auto-detect from target/ink/
glin-forge typegen

# From deployed contract (fetches metadata from network)
glin-forge typegen --contract 5GrwvaEF... --network testnet
```

### 2. Import and Use

```typescript
import type { Flipper, FlipperQueries, FlipperTransactions } from './types/flipper';
import { deploy, getContract } from '@glin-forge/sdk';

// Deploy with type-safe arguments
const contract = await deploy({
  wasm: './target/ink/flipper.wasm',
  metadata: './target/ink/metadata.json',
  args: [true], // ✅ TypeScript knows this must be boolean
  network: Network.Testnet,
  account: Signer.Alice,
});

// Query with inferred return types
const value: boolean = await contract.query('get');

// Transaction with autocomplete
await contract.call('flip'); // IDE shows: 'flip' | 'get'
```

## Generated Types

The typegen command generates comprehensive TypeScript definitions:

### Contract Interface

```typescript
export interface Flipper {
  /** Contract address */
  address: string;

  /** Read-only query methods */
  query: FlipperQueries;

  /** State-changing transaction methods */
  tx: FlipperTransactions;

  /** Event types */
  events: FlipperEvents;

  /** Contract metadata */
  metadata: string;
}
```

### Query Methods

```typescript
export interface FlipperQueries {
  /**
   * Returns the current value of the Flipper's boolean.
   */
  get(): Promise<boolean>;
}
```

### Transaction Methods

```typescript
export interface FlipperTransactions {
  /**
   * Flips the current value of the Flipper's boolean.
   */
  flip(): Promise<Transaction>;
}
```

### Event Definitions

```typescript
/**
 * Emitted whenever the value is flipped.
 */
export interface FlippedEvent {
  old_value: boolean;
  new_value: boolean;
}

export interface FlipperEvents {
  Flipped: FlippedEvent;
}
```

### Constructor Interfaces

```typescript
export interface FlipperConstructors {
  /**
   * Constructor that initializes the bool value to the given value.
   */
  new(
    init_value: boolean,
    options?: {
      value?: number | bigint;
      gasLimit?: number | bigint;
      salt?: string;
    }
  ): Promise<Flipper>;

  /**
   * Constructor that initializes the bool value to false.
   */
  default(
    options?: { /* ... */ }
  ): Promise<Flipper>;
}
```

## Type Mappings

The TypeGen system intelligently maps all ink! types to TypeScript:

### Primitive Types

| ink! Type | TypeScript Type |
|-----------|----------------|
| `bool` | `boolean` |
| `u8`, `u16`, `u32`, `i8`, `i16`, `i32` | `number` |
| `u64`, `u128`, `u256`, `i64`, `i128` | `string \| number \| bigint` |
| `String`, `str` | `string` |

### Special Types

| ink! Type | TypeScript Type |
|-----------|----------------|
| `AccountId` | `string` |
| `Balance` | `string \| number \| bigint` |
| `Hash` | `string` |
| `[u8; 32]` | `Uint8Array \| string` |

### Container Types

| ink! Type | TypeScript Type |
|-----------|----------------|
| `Option<T>` | `T \| null` |
| `Result<T, E>` | `{ Ok: T } \| { Err: E }` |
| `Vec<T>` | `T[]` |
| `[T; N]` | `T[]` or `Uint8Array` |
| `(T1, T2)` | `[T1, T2]` |

### Custom Types

**ink! Struct:**
```rust
#[derive(scale::Encode, scale::Decode)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
```

**Generated TypeScript:**
```typescript
export interface TokenInfo {
  name: string;
  symbol: string;
  decimals: number;
}
```

**ink! Enum:**
```rust
#[derive(scale::Encode, scale::Decode)]
pub enum Status {
    Active,
    Paused,
    Terminated { reason: String },
}
```

**Generated TypeScript:**
```typescript
export type Status =
  | { type: 'Active' }
  | { type: 'Paused' }
  | { type: 'Terminated', reason: string };
```

## CLI Options

```bash
glin-forge typegen [OPTIONS]

Options:
  -a, --abi <PATH>        Path to contract metadata JSON file
  -c, --contract <ADDR>   Contract address to fetch ABI from
  -o, --output <DIR>      Output directory [default: ./types]
  -n, --network <NET>     Network for fetching [default: testnet]
      --hooks             Generate React hooks alongside types
      --legacy            Use legacy simple type generator
  -h, --help              Print help
```

### Examples

```bash
# Basic usage - auto-detect metadata
glin-forge typegen

# Specify metadata file and output
glin-forge typegen --abi ./contracts/erc20/metadata.json --output ./src/types

# Fetch from deployed contract
glin-forge typegen --contract 5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt --network testnet

# Generate with React hooks
glin-forge typegen --hooks

# Use legacy simple generator
glin-forge typegen --legacy
```

## Integration with TypedContract

The generated types work seamlessly with the `TypedContract` base class:

```typescript
import { TypedContract } from '@glin-forge/sdk';
import type { FlipperQueries, FlipperTransactions, FlipperEvents } from './types/flipper';

class FlipperContract extends TypedContract<
  FlipperQueries,
  FlipperTransactions,
  FlipperEvents
> {
  // Implement typed methods
  query = {
    get: async (): Promise<boolean> => {
      return this.queryRaw('get', []);
    }
  };

  tx = {
    flip: async (): Promise<Transaction> => {
      return this.callRaw('flip', []);
    }
  };
}
```

## Workflow Integration

### Development Workflow

1. **Write Contract**
   ```bash
   cd my-contract
   cargo contract build
   ```

2. **Generate Types**
   ```bash
   glin-forge typegen
   ```

3. **Use in Frontend**
   ```typescript
   import type { MyContract } from './types/MyContract';
   // Use with full type safety
   ```

4. **Regenerate on Changes**
   ```bash
   cargo contract build && glin-forge typegen
   ```

### CI/CD Integration

```yaml
# .github/workflows/build.yml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build Contract
        run: cargo contract build

      - name: Generate Types
        run: glin-forge typegen --output src/types

      - name: Type Check
        run: npm run typecheck

      - name: Run Tests
        run: npm test
```

### Package.json Scripts

```json
{
  "scripts": {
    "build:contract": "cargo contract build",
    "typegen": "glin-forge typegen --output src/types",
    "dev": "npm run typegen && vite",
    "build": "npm run typegen && vite build",
    "typecheck": "tsc --noEmit"
  }
}
```

## Type Safety Examples

### ✅ Valid Code

```typescript
// Correct method name, types, and parameters
const balance = await contract.query('balanceOf', [address]);

// Correct transaction
await contract.call('transfer', [recipient, amount]);

// Type-safe event listening
contract.on('Transfer', (event) => {
  console.log(event.data.from, event.data.to, event.data.value);
});
```

### ❌ Invalid Code (Won't Compile)

```typescript
// ❌ Wrong method name
await contract.query('getBalance', [addr]);
// Error: Property 'getBalance' does not exist on type 'Queries'

// ❌ Wrong parameter type
await contract.call('transfer', ['Bob', '100']);
// Error: Expected [string, bigint], got [string, string]

// ❌ Missing parameter
await contract.query('balanceOf');
// Error: Expected 1 argument, got 0

// ❌ Wrong event name
contract.on('TransferToken', (event) => { ... });
// Error: Event 'TransferToken' does not exist
```

## Advanced Features

### Complex Types

The type generator handles complex nested types:

```typescript
// Option<Result<Vec<(AccountId, Balance)>, Error>>
type ComplexType =
  | null
  | { Ok: Array<[string, bigint]> }
  | { Err: Error };
```

### Multi-Constructor Support

```typescript
const constructors: FlipperConstructors = {
  new: async (initValue: boolean, options?) => { ... },
  default: async (options?) => { ... },
};
```

### Event Indexing

```typescript
export interface TransferEvent {
  from: string;      // indexed
  to: string;        // indexed
  value: bigint;     // not indexed
}
```

## Comparison: Before and After

### Before Phase 3 (Untyped)

```typescript
// No type safety - any mistakes caught at runtime
const value = await contract.query('get');  // value: any
await contract.call('tranfer', [addr, amt]); // Typo! Runtime error
contract.on('Transfr', (event) => { ... });  // Wrong event name, runtime error
```

### After Phase 3 (Typed)

```typescript
// Full type safety - mistakes caught at compile time
const value: boolean = await contract.query('get');
await contract.call('transfer', [addr, amt]); // ✅ Correct
//              ^^^^^^^^ Autocomplete works!
contract.on('Transfer', (event: TransferEvent) => {
  console.log(event.data.value); // ✅ Type-safe event data
});
```

## Legacy Mode

For simpler projects or backward compatibility, use `--legacy` flag:

```bash
glin-forge typegen --legacy
```

This generates basic interfaces without full type safety, similar to Phase 1.

## Examples

See the example scripts:

- `scripts/phase3-typegen.ts` - Type generation demo
- `scripts/phase3-complete-example.ts` - End-to-end type-safe workflow

Run examples:

```bash
npm run example:phase3-typegen
npm run example:phase3-complete
```

## Troubleshooting

### Types Not Generated

- Ensure contract metadata exists: `target/ink/metadata.json`
- Check metadata is valid JSON: `cat target/ink/metadata.json | jq`
- Verify contract compiled: `cargo contract build`

### TypeScript Errors After Generation

- Regenerate types: `glin-forge typegen`
- Check TypeScript version: `npm list typescript` (requires >=4.5)
- Clear node_modules and reinstall: `npm ci`

### IDE Not Showing Autocomplete

- Restart TypeScript server in VSCode: `Cmd+Shift+P` → "Restart TS Server"
- Check types are imported: `import type { ... } from './types/...'`
- Verify tsconfig.json includes types directory

## Next Steps

- Explore Phase 1: [JSON-RPC Bridge](./PHASE1_RPC_BRIDGE.md)
- Review Phase 2: [Core Features](./PHASE2_FEATURES.md)
- See [Examples](../scripts/) for complete demos
- Check [API Reference](./API.md) for detailed documentation

## Summary

Phase 3 brings professional-grade type safety to smart contract development:

✅ **Full Type Safety** - Catch errors at compile time
✅ **IDE Autocomplete** - Know what methods are available
✅ **Self-Documenting** - Types serve as documentation
✅ **Refactoring Safe** - TypeScript tracks all usages
✅ **Developer Experience** - Like Hardhat for Ethereum
✅ **Zero Runtime Overhead** - Types are compile-time only

With Phase 3, developing on GLIN Network feels as smooth as developing on Ethereum with Hardhat + Ethers.js + TypeChain!
