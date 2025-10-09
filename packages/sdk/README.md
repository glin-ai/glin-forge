# @glin-forge/sdk

TypeScript SDK for glin-forge - Hardhat-style smart contract deployment and interaction for GLIN Network.

## Installation

This SDK is designed to be used with deployment scripts executed via `glin-forge run`. You don't need to install it separately - just import it in your scripts.

```typescript
import { deploy, Contract, Network, Signer } from '@glin-forge/sdk';
```

## Quick Start

### 1. Write a Deployment Script

Create a TypeScript file (e.g., `scripts/deploy.ts`):

```typescript
import { deploy, Network, Signer } from '@glin-forge/sdk';

async function main() {
  const contract = await deploy({
    wasm: './target/ink/my_contract.wasm',
    metadata: './target/ink/metadata.json',
    args: ['arg1', 'arg2'],
    network: Network.Testnet,
    account: Signer.Alice,
  });

  console.log('Deployed at:', contract.address);
}

main().catch(console.error);
```

### 2. Run with glin-forge

```bash
glin-forge run scripts/deploy.ts
```

## API Reference

### `deploy(options)`

Deploy a new contract to the network.

**Parameters:**
- `wasm` (string): Path to WASM file
- `metadata` (string): Path to metadata JSON file
- `args` (string[]): Constructor arguments (optional)
- `value` (number): Value to send in GLIN (optional, default: 0)
- `network` (Network | string): Target network
- `account` (Signer | string): Deploying account
- `gasLimit` (number): Gas limit override (optional)
- `salt` (string): Deterministic deployment salt (optional)

**Returns:** `Promise<Contract>`

**Example:**

```typescript
const contract = await deploy({
  wasm: './target/ink/my_token.wasm',
  metadata: './target/ink/metadata.json',
  args: ['1000000', 'MyToken', 'MTK'],
  network: Network.Testnet,
  account: Signer.Alice,
  value: 1,
});
```

### `getContract(options)`

Get a Contract instance for an already deployed contract.

**Parameters:**
- `address` (string): Contract address
- `metadata` (string): Path to metadata JSON file
- `network` (Network | string): Network
- `account` (Signer | string): Account for transactions

**Returns:** `Contract`

**Example:**

```typescript
const contract = getContract({
  address: '5GrwvaEF...',
  metadata: './target/ink/metadata.json',
  network: Network.Testnet,
  account: Signer.Alice,
});
```

### `Contract` Class

Represents a deployed contract instance.

#### `contract.call(method, args, value)`

Call a contract method (transaction).

**Parameters:**
- `method` (string): Method name
- `args` (any[]): Method arguments (optional)
- `value` (number): Value to send (optional, default: 0)

**Returns:** `Promise<CallResult>`

**Example:**

```typescript
await contract.call('transfer', ['5Recipient...', '1000'], 0);
```

#### `contract.query(method, args)`

Query contract state (read-only).

**Parameters:**
- `method` (string): Method name
- `args` (any[]): Method arguments (optional)

**Returns:** `Promise<any>`

**Example:**

```typescript
const balance = await contract.query('balanceOf', ['5Account...']);
console.log('Balance:', balance);
```

### `Network` Enum

Pre-configured networks:

```typescript
enum Network {
  Testnet = 'testnet',
  Mainnet = 'mainnet',
  Local = 'local',
}
```

### `Signer` Enum

Development accounts:

```typescript
enum Signer {
  Alice = 'alice',
  Bob = 'bob',
  Charlie = 'charlie',
  Dave = 'dave',
  Eve = 'eve',
  Ferdie = 'ferdie',
}
```

## Examples

### Deploy Multiple Contracts

```typescript
import { deploy, Network, Signer } from '@glin-forge/sdk';

async function main() {
  // Deploy 3 contracts in parallel
  const contracts = await Promise.all([
    deploy({
      wasm: './target/ink/contract.wasm',
      metadata: './target/ink/metadata.json',
      args: ['Contract1'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
    deploy({
      wasm: './target/ink/contract.wasm',
      metadata: './target/ink/metadata.json',
      args: ['Contract2'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
    deploy({
      wasm: './target/ink/contract.wasm',
      metadata: './target/ink/metadata.json',
      args: ['Contract3'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
  ]);

  console.log('Deployed contracts:');
  contracts.forEach((c, i) => {
    console.log(`  Contract ${i + 1}: ${c.address}`);
  });
}

main().catch(console.error);
```

### Interact with Deployed Contract

```typescript
import { getContract, Network, Signer } from '@glin-forge/sdk';

async function main() {
  const contract = getContract({
    address: '5GrwvaEF...',
    metadata: './target/ink/metadata.json',
    network: Network.Testnet,
    account: Signer.Alice,
  });

  // Query state
  const totalSupply = await contract.query('totalSupply', []);
  console.log('Total supply:', totalSupply);

  // Call method
  await contract.call('transfer', ['5Recipient...', '1000']);
  console.log('Transfer successful!');
}

main().catch(console.error);
```

### Complete Deployment Flow

```typescript
import { deploy, Network, Signer } from '@glin-forge/sdk';

async function main() {
  console.log('Deploying ERC20 token...');

  const token = await deploy({
    wasm: './target/ink/erc20.wasm',
    metadata: './target/ink/metadata.json',
    args: ['1000000', 'MyToken', 'MTK'],
    network: Network.Testnet,
    account: Signer.Alice,
  });

  console.log('Token deployed at:', token.address);

  // Query initial state
  const totalSupply = await token.query('totalSupply', []);
  console.log('Total supply:', totalSupply);

  const aliceBalance = await token.query('balanceOf', [
    '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
  ]);
  console.log('Alice balance:', aliceBalance);

  // Transfer tokens
  console.log('Transferring tokens...');
  await token.call('transfer', [
    '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
    '100',
  ]);

  console.log('Transfer complete!');
}

main().catch(console.error);
```

## Error Handling

```typescript
import { deploy, Network, Signer } from '@glin-forge/sdk';

async function main() {
  try {
    const contract = await deploy({
      wasm: './target/ink/contract.wasm',
      metadata: './target/ink/metadata.json',
      args: [],
      network: Network.Testnet,
      account: Signer.Alice,
    });

    console.log('Success:', contract.address);
  } catch (error) {
    console.error('Deployment failed:', error.message);
    process.exit(1);
  }
}

main();
```

## Type Safety

The SDK is fully typed with TypeScript. Your IDE will provide autocomplete and type checking:

```typescript
import { deploy, Network, Signer } from '@glin-forge/sdk';

// TypeScript will catch errors:
await deploy({
  wasm: './contract.wasm',
  metadata: './metadata.json',
  network: 'invalid-network', // ❌ Error: not a valid Network
  account: Signer.Alice,
});

await deploy({
  wasm: './contract.wasm',
  metadata: './metadata.json',
  network: Network.Testnet, // ✅ OK
  account: Signer.Alice,
});
```

## Requirements

- Node.js 18+ or TypeScript runtime (tsx/ts-node)
- glin-forge CLI tool
- Compiled contract WASM and metadata files

## License

Apache-2.0

## Support

- **Documentation**: https://docs.glin.ai/forge
- **Discord**: https://discord.gg/glin
- **GitHub Issues**: https://github.com/glin-ai/glin-forge/issues
