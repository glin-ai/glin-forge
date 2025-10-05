---
title: Frontend Integration
description: Integrate smart contracts with React and TypeScript using GLIN Forge
---

# Frontend Integration

Learn how to integrate your smart contracts with React frontends using GLIN Forge's type generation features.

## Overview

GLIN Forge automatically generates:
- **TypeScript types** - Type-safe contract interfaces
- **React hooks** - Ready-to-use contract hooks
- **Type mapping** - Rust types to TypeScript types

This eliminates manual type definitions and reduces bugs in your dApp.

## Quick Start

### 1. Generate Types

After building your contract:

```bash
glin-forge build --release
glin-forge typegen --output ./frontend/src/contracts --hooks
```

### 2. Install Dependencies

In your frontend project:

```bash
npm install @glin-ai/sdk @glin-ai/sdk-react
```

### 3. Use Generated Hooks

```tsx
import { useMyToken } from './contracts/useMyToken'

function App() {
  const contractAddress = '5FHneW46...'
  const { contract, loading } = useMyToken(contractAddress)

  if (loading) return <div>Loading...</div>

  return <div>Contract loaded!</div>
}
```

## Complete Example

### Contract

```rust
#[ink::contract]
mod my_token {
    #[ink(storage)]
    pub struct MyToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(message)]
    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }

    #[ink(message)]
    pub fn balance_of(&self, account: AccountId) -> Balance {
        self.balances.get(account).unwrap_or(0)
    }

    #[ink(message)]
    pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        // Transfer logic...
    }
}
```

### Generate Types

```bash
glin-forge typegen --hooks --output ./src/contracts
```

### React Component

```tsx
import { useState, useEffect } from 'react'
import { useMyToken } from './contracts/useMyToken'
import { useAccount } from '@glin-ai/sdk-react'

function TokenDashboard() {
  const contractAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'
  const { account } = useAccount()
  const { contract, loading, error } = useMyToken(contractAddress, account?.signer)

  const [totalSupply, setTotalSupply] = useState<bigint>(0n)
  const [myBalance, setMyBalance] = useState<bigint>(0n)

  // Load total supply
  useEffect(() => {
    if (contract) {
      contract.query.totalSupply().then(setTotalSupply)
    }
  }, [contract])

  // Load user balance
  useEffect(() => {
    if (contract && account) {
      contract.query.balanceOf(account.address).then(setMyBalance)
    }
  }, [contract, account])

  // Transfer function
  const handleTransfer = async (to: string, amount: bigint) => {
    if (!contract) return

    try {
      const result = await contract.tx.transfer(to, amount)
      console.log('Transfer successful:', result.hash)

      // Refresh balance
      if (account) {
        const newBalance = await contract.query.balanceOf(account.address)
        setMyBalance(newBalance)
      }
    } catch (err) {
      console.error('Transfer failed:', err)
    }
  }

  if (loading) return <div>Loading contract...</div>
  if (error) return <div>Error: {error.message}</div>

  return (
    <div>
      <h1>My Token</h1>
      <p>Total Supply: {totalSupply.toString()}</p>
      <p>My Balance: {myBalance.toString()}</p>

      <button onClick={() => handleTransfer('5FrLwHLX...', 100n)}>
        Transfer 100 Tokens
      </button>
    </div>
  )
}

export default TokenDashboard
```

## Type Generation

### Generated Types

**contracts/MyToken.ts:**

```typescript
export interface MyTokenContractQuery {
  totalSupply: () => Promise<bigint>
  balanceOf: (account: string) => Promise<bigint>
}

export interface MyTokenContractTx {
  transfer: (to: string, amount: bigint) => Promise<TxResult>
}

export interface MyTokenContract {
  query: MyTokenContractQuery
  tx: MyTokenContractTx
  address: string
}
```

### Generated Hooks

**contracts/useMyToken.ts:**

```typescript
import { useContract } from '@glin-ai/sdk-react'
import type { MyTokenContract } from './MyToken'
import metadata from './metadata.json'

export function useMyToken(address: string, signer?: any) {
  const { contract, loading, error } = useContract<MyTokenContract>({
    address,
    abi: metadata,
    signer,
  })

  return { contract, loading, error }
}
```

## Query Patterns

### Basic Query

```tsx
const { contract } = useMyToken(contractAddress)

// Simple query
const totalSupply = await contract.query.totalSupply()
console.log('Total supply:', totalSupply)
```

### Query with Arguments

```tsx
// Query with parameters
const balance = await contract.query.balanceOf('5GrwvaEF...')
console.log('Balance:', balance)
```

### Reactive Queries

```tsx
function TokenBalance({ userAddress }: { userAddress: string }) {
  const { contract } = useMyToken(contractAddress)
  const [balance, setBalance] = useState<bigint>(0n)

  useEffect(() => {
    if (contract) {
      contract.query.balanceOf(userAddress).then(setBalance)
    }
  }, [contract, userAddress])

  return <div>Balance: {balance.toString()}</div>
}
```

### Polling for Updates

```tsx
useEffect(() => {
  if (!contract) return

  const interval = setInterval(async () => {
    const supply = await contract.query.totalSupply()
    setTotalSupply(supply)
  }, 5000) // Poll every 5 seconds

  return () => clearInterval(interval)
}, [contract])
```

## Transaction Patterns

### Basic Transaction

```tsx
const { contract } = useMyToken(contractAddress, signer)

const transfer = async () => {
  const result = await contract.tx.transfer('5FrLwHLX...', 1000n)
  console.log('Transaction hash:', result.hash)
}
```

### Transaction with Loading State

```tsx
function TransferButton() {
  const { contract } = useMyToken(contractAddress, signer)
  const [loading, setLoading] = useState(false)

  const handleTransfer = async () => {
    setLoading(true)

    try {
      const result = await contract.tx.transfer('5FrLwHLX...', 1000n)
      alert('Transfer successful: ' + result.hash)
    } catch (error) {
      alert('Transfer failed: ' + error.message)
    } finally {
      setLoading(false)
    }
  }

  return (
    <button onClick={handleTransfer} disabled={loading}>
      {loading ? 'Transferring...' : 'Transfer'}
    </button>
  )
}
```

### Transaction with Confirmation

```tsx
const handleTransfer = async (to: string, amount: bigint) => {
  if (!confirm(`Transfer ${amount} tokens to ${to}?`)) {
    return
  }

  try {
    const result = await contract.tx.transfer(to, amount)

    // Wait for confirmation
    await result.wait()

    alert('Transfer confirmed!')
  } catch (error) {
    alert('Transfer failed: ' + error.message)
  }
}
```

## Account Management

### Connect Wallet

```tsx
import { useAccount, useConnect } from '@glin-ai/sdk-react'

function WalletConnect() {
  const { account } = useAccount()
  const { connect, disconnect } = useConnect()

  if (account) {
    return (
      <div>
        <p>Connected: {account.address}</p>
        <button onClick={disconnect}>Disconnect</button>
      </div>
    )
  }

  return <button onClick={connect}>Connect Wallet</button>
}
```

### Use Account in Contract

```tsx
function TokenTransfer() {
  const { account } = useAccount()
  const { contract } = useMyToken(contractAddress, account?.signer)

  if (!account) {
    return <div>Please connect your wallet</div>
  }

  return (
    <button onClick={() => contract.tx.transfer('5FrLwHLX...', 100n)}>
      Transfer
    </button>
  )
}
```

## Event Handling

### Listen for Events

```tsx
import { useContractEvents } from '@glin-ai/sdk-react'

function TokenEvents() {
  const { events } = useContractEvents(contractAddress, 'Transfer')

  return (
    <div>
      <h2>Recent Transfers</h2>
      {events.map((event, i) => (
        <div key={i}>
          From: {event.from} → To: {event.to} = {event.value.toString()}
        </div>
      ))}
    </div>
  )
}
```

### Real-time Event Updates

```tsx
useEffect(() => {
  if (!contract) return

  const unsubscribe = contract.events.Transfer.subscribe((event) => {
    console.log('Transfer event:', event)
    // Update UI
  })

  return () => unsubscribe()
}, [contract])
```

## Full Stack Example

### Project Structure

```
my-dapp/
├── contract/              # Smart contract
│   ├── lib.rs
│   └── Cargo.toml
├── frontend/              # React app
│   ├── src/
│   │   ├── contracts/     # Generated types
│   │   │   ├── MyToken.ts
│   │   │   └── useMyToken.ts
│   │   ├── components/
│   │   │   ├── TokenBalance.tsx
│   │   │   ├── TransferForm.tsx
│   │   │   └── WalletConnect.tsx
│   │   ├── App.tsx
│   │   └── main.tsx
│   └── package.json
└── scripts/
    └── deploy.sh
```

### Build Script

```bash
#!/bin/bash
# scripts/build-all.sh

set -e

echo "Building contract..."
cd contract
glin-forge build --release

echo "Generating types..."
glin-forge typegen \
  --output ../frontend/src/contracts \
  --hooks

echo "Building frontend..."
cd ../frontend
npm run build

echo "Done!"
```

### Deploy Script

```bash
#!/bin/bash
# scripts/deploy.sh

set -e

echo "Deploying contract..."
cd contract
glin-forge deploy \
  --network testnet \
  --account alice \
  --yes \
  > ../deploy-output.txt

# Extract address
CONTRACT_ADDR=$(grep "Address:" ../deploy-output.txt | awk '{print $2}')

echo "Contract deployed: $CONTRACT_ADDR"

# Update frontend config
echo "export const CONTRACT_ADDRESS = '$CONTRACT_ADDR'" > ../frontend/src/config.ts

echo "Done! Contract address: $CONTRACT_ADDR"
```

## Framework Integration

### Next.js

```tsx
// pages/index.tsx
import type { NextPage } from 'next'
import { useMyToken } from '../contracts/useMyToken'
import { CONTRACT_ADDRESS } from '../config'

const Home: NextPage = () => {
  const { contract, loading } = useMyToken(CONTRACT_ADDRESS)

  if (loading) return <div>Loading...</div>

  return (
    <div>
      <h1>My Token DApp</h1>
      {/* Your UI here */}
    </div>
  )
}

export default Home
```

### Vite + React

```tsx
// src/App.tsx
import { useMyToken } from './contracts/useMyToken'

function App() {
  const contractAddress = import.meta.env.VITE_CONTRACT_ADDRESS
  const { contract } = useMyToken(contractAddress)

  return <div>My DApp</div>
}

export default App
```

### Create React App

```tsx
// src/App.tsx
import { useMyToken } from './contracts/useMyToken'

function App() {
  const contractAddress = process.env.REACT_APP_CONTRACT_ADDRESS
  const { contract } = useMyToken(contractAddress)

  return <div>My DApp</div>
}

export default App
```

## Best Practices

### 1. Error Handling

```tsx
const handleTransaction = async () => {
  try {
    const result = await contract.tx.transfer(to, amount)
    toast.success('Transaction successful!')
  } catch (error) {
    if (error.message.includes('InsufficientBalance')) {
      toast.error('Insufficient balance')
    } else {
      toast.error('Transaction failed')
    }
    console.error(error)
  }
}
```

### 2. Loading States

```tsx
function TokenInfo() {
  const { contract, loading } = useMyToken(contractAddress)
  const [data, setData] = useState(null)
  const [loadingData, setLoadingData] = useState(true)

  useEffect(() => {
    if (contract) {
      Promise.all([
        contract.query.totalSupply(),
        contract.query.name(),
      ])
      .then(([supply, name]) => {
        setData({ supply, name })
        setLoadingData(false)
      })
    }
  }, [contract])

  if (loading || loadingData) {
    return <div>Loading...</div>
  }

  return <div>{data.name}: {data.supply.toString()}</div>
}
```

### 3. Optimistic Updates

```tsx
const handleTransfer = async (to: string, amount: bigint) => {
  // Optimistically update UI
  const newBalance = myBalance - amount
  setMyBalance(newBalance)

  try {
    await contract.tx.transfer(to, amount)
  } catch (error) {
    // Revert on error
    setMyBalance(myBalance)
    alert('Transfer failed')
  }
}
```

### 4. Type Safety

```tsx
// Use generated types
import type { MyTokenContract } from './contracts/MyToken'

const contract: MyTokenContract = useMyToken(address).contract

// TypeScript will catch errors
contract.query.totalSupply()      // ✓ OK
contract.query.totalSuppply()     // ✗ Type error
contract.query.totalSupply(123)   // ✗ Type error (no args expected)
```

## Troubleshooting

### Types Not Found

**Problem:**
```
Cannot find module './contracts/MyToken'
```

**Solution:**
Generate types:
```bash
glin-forge typegen --output ./src/contracts --hooks
```

### Contract Not Loading

**Problem:**
Contract hook returns `undefined`.

**Solution:**
1. Check contract address is correct
2. Verify network connection
3. Ensure wallet is connected (for transactions)

### Transaction Fails

**Problem:**
Transaction reverts with error.

**Solution:**
1. Check account has sufficient balance
2. Verify arguments are correct types
3. Test contract method with `query` first
4. Check contract logic

## Next Steps

- [TypeScript Types](../code-generation/typescript-types) - Deep dive into type generation
- [React Hooks](../code-generation/react-hooks) - Advanced hook patterns
- [Frontend Example](../examples/frontend-dapp) - Complete dApp example
- [Event Handling](./event-handling) - Work with contract events

## Resources

- [@glin-ai/sdk Documentation](https://docs.glin.ai/sdk)
- [@glin-ai/sdk-react Documentation](https://docs.glin.ai/sdk-react)
- [React Documentation](https://react.dev/)
- [TypeScript Documentation](https://www.typescriptlang.org/)
