# GLIN SDK Architecture

This document explains the multi-language SDK strategy for the GLIN Network ecosystem.

## Overview

The GLIN SDK ecosystem consists of multiple language-specific SDKs that share a common core feature set while providing language-specific extensions. This approach follows best practices from major blockchain ecosystems like Ethereum and Polkadot.

```
GLIN Network
    │
    ├── TypeScript SDK (glin-sdk)
    │   ├── Core: Network, Contracts, Queries, Events
    │   └── Extensions: React Hooks, Next.js helpers
    │
    ├── Rust SDK (glin-sdk-rust)
    │   ├── Core: Network, Contracts, Queries, Events
    │   └── Extensions: CLI tools, Indexing utilities
    │
    └── Python SDK (glin-sdk-python) [Future]
        ├── Core: Network, Contracts, Queries, Events
        └── Extensions: Pandas integration, Data analysis
```

## 1. TypeScript SDK (`glin-sdk`)

### Location
- Repository: `/home/eralp/Projects/glin/glin-sdk`
- Package: `@glin/sdk` on npm

### Use Cases
- **Frontend**: React applications, browser extensions (glin-extension)
- **Backend**: Node.js services, Next.js API routes, backend automation

### Core Features
```typescript
// Network connection
import { GlinClient } from '@glin/sdk';
const client = await GlinClient.connect('wss://testnet.glin.ai');

// Contract interaction
import { Contract } from '@glin/sdk';
const contract = new Contract(client, address, metadata);
const result = await contract.query.getMessage();
await contract.tx.setMessage({ value: 'Hello' });

// Events subscription
client.events.subscribe('Contracts.ContractEmitted', (event) => {
  console.log(event);
});
```

### React Extensions
```typescript
// React hooks for frontend
import { useContract, useContractQuery } from '@glin/sdk/react';

function MyComponent() {
  const contract = useContract(address, metadata);
  const { data, loading } = useContractQuery(contract, 'getMessage');

  return <div>{data}</div>;
}
```

### Backend Usage (Node.js)
```typescript
// Works in Node.js backend
import { GlinClient } from '@glin/sdk';

async function processContracts() {
  const client = await GlinClient.connect(process.env.RPC_URL);
  const contracts = await client.contracts.list();
  // Process with full TypeScript type safety
}
```

## 2. Rust SDK (`glin-sdk-rust`)

### Workspace Structure
```
glin-sdk-rust/
├── glin-client/       # Core blockchain client
├── glin-contracts/    # Contract utilities
├── glin-types/        # Shared types (Address, Hash, etc.)
├── glin-indexer/      # Blockchain indexing utilities
└── examples/          # Usage examples
```

### Use Cases
- **CLI Tools**: glin-forge (developer tools)
- **Backend Services**: glinscan API (explorer backend)
- **Indexers**: Transaction and event indexing
- **High Performance**: Low-latency, resource-intensive tasks

### Core Features
```rust
// Network connection
use glin_client::GlinClient;
let client = GlinClient::connect("wss://testnet.glin.ai").await?;

// Contract interaction
use glin_contracts::Contract;
let contract = Contract::new(&client, address, metadata)?;
let result = contract.query("getMessage", vec![]).await?;
contract.tx("setMessage", vec!["Hello".into()]).await?;

// Events subscription
use glin_client::events;
events::subscribe(&client, "Contracts.ContractEmitted", |event| {
    println!("{:?}", event);
}).await?;
```

### Integration with glin-forge
```rust
// glin-forge uses glin-sdk-rust internally
[dependencies]
glin-client = { path = "../glin-sdk-rust/glin-client" }
glin-contracts = { path = "../glin-sdk-rust/glin-contracts" }
glin-types = { path = "../glin-sdk-rust/glin-types" }
```

### Integration with glinscan
```rust
// glinscan API uses glin-indexer
[dependencies]
glin-indexer = { path = "../glin-sdk-rust/glin-indexer" }
glin-client = { path = "../glin-sdk-rust/glin-client" }
```

## 3. Python SDK (`glin-sdk-python`) [Future]

### Use Cases
- **Data Science**: Analytics, ML model training
- **Research**: Academic research on blockchain data
- **Automation**: Backend scripts, cron jobs

### Planned Features
```python
# Network connection
from glin_sdk import GlinClient
client = await GlinClient.connect("wss://testnet.glin.ai")

# Contract interaction
from glin_sdk import Contract
contract = Contract(client, address, metadata)
result = await contract.query.get_message()
await contract.tx.set_message(value="Hello")

# Data analysis with Pandas
import pandas as pd
from glin_sdk.analytics import get_contract_calls

df = await get_contract_calls(contract_address, start_block=1000)
df.groupby('method').count()  # Analyze method calls
```

## Multi-Language Strategy

### Common Core Features (All SDKs)

These features must exist in every language SDK:

1. **Network Connection**
   - Connect to RPC endpoints
   - WebSocket subscriptions
   - Network configuration (mainnet, testnet, local)

2. **Contract Interaction**
   - Load contract metadata
   - Query contract state (read-only)
   - Submit transactions (write)
   - Estimate gas fees

3. **Transaction Management**
   - Build transactions
   - Sign transactions
   - Submit and track transactions
   - Wait for finalization

4. **Events & Subscriptions**
   - Subscribe to new blocks
   - Subscribe to contract events
   - Filter and parse events

5. **Account Management**
   - Create accounts/keypairs
   - Sign messages
   - Verify signatures

### Language-Specific Extensions

Each SDK provides unique features based on the language ecosystem:

| Feature | TypeScript | Rust | Python |
|---------|-----------|------|--------|
| **Core Features** | ✅ | ✅ | ✅ |
| React Hooks | ✅ | ❌ | ❌ |
| Next.js Helpers | ✅ | ❌ | ❌ |
| Browser Extension Support | ✅ | ❌ | ❌ |
| CLI Tools | ❌ | ✅ | ❌ |
| Blockchain Indexing | ❌ | ✅ | ❌ |
| High-Performance Sync | ❌ | ✅ | ❌ |
| Pandas Integration | ❌ | ❌ | ✅ |
| Data Analysis | ❌ | ❌ | ✅ |
| Jupyter Notebooks | ❌ | ❌ | ✅ |

## Integration with GLIN Ecosystem

### glin-forge (CLI)
- **SDK**: Rust SDK (glin-sdk-rust)
- **Modules**: glin-client, glin-contracts
- **Purpose**: Developer tools (build, deploy, verify, typegen)

### glinscan (Explorer API)
- **SDK**: Rust SDK (glin-sdk-rust)
- **Modules**: glin-indexer, glin-client, glin-types
- **Purpose**: Blockchain data indexing, contract verification

### glin-extension (Browser Wallet)
- **SDK**: TypeScript SDK (glin-sdk)
- **Modules**: Core + Browser extensions
- **Purpose**: User wallet, transaction signing

### glin-backend (AI Training)
- **SDK**: None (separate purpose)
- **Purpose**: AI training coordination, testnet points
- **Note**: Should NOT use blockchain SDKs (different domain)

## Best Practices from Other Ecosystems

### Ethereum Ecosystem
- **ethers.js**: TypeScript for frontend + backend
- **web3.py**: Python for data science
- **ethers-rs**: Rust for high-performance services

**Lesson**: Each language SDK has same core but different extensions

### Polkadot Ecosystem
- **polkadot.js**: TypeScript for frontend + backend
- **subxt**: Rust for substrate nodes and indexers
- **py-substrate-interface**: Python for analytics

**Lesson**: Rust SDK powers infrastructure, TypeScript powers apps

### GLIN Strategy (Following Best Practices)

1. **TypeScript SDK**: Powers user-facing apps
   - glin-extension (wallet)
   - DApp frontends
   - Backend automation

2. **Rust SDK**: Powers infrastructure
   - glin-forge (dev tools)
   - glinscan (explorer)
   - Indexers and validators

3. **Python SDK**: Powers analytics (future)
   - Research
   - Data science
   - ML model training

## Development Workflow

### Creating a New Feature

When adding a new feature (e.g., batch transactions):

1. **Design the API**: Same interface across languages
   ```typescript
   // TypeScript
   const results = await client.batch([tx1, tx2, tx3]);
   ```
   ```rust
   // Rust
   let results = client.batch(vec![tx1, tx2, tx3]).await?;
   ```
   ```python
   # Python
   results = await client.batch([tx1, tx2, tx3])
   ```

2. **Implement in Rust first**: Test in glin-forge

3. **Port to TypeScript**: Add to glin-sdk

4. **Document**: Update this file and SDK docs

### Avoiding Code Duplication

**❌ Wrong Approach**: Copy-paste blockchain logic between glin-forge and glinscan

**✅ Right Approach**: Create glin-sdk-rust and share:
```rust
// Both glin-forge and glinscan use same code
use glin_client::GlinClient;
use glin_contracts::metadata_fetcher;

let metadata = metadata_fetcher::fetch_contract_metadata(
    &client,
    address,
    options
).await?;
```

## Migration Plan

### Phase 1: Create glin-sdk-rust (Current)
- Extract common code from glin-forge
- Create workspace structure
- Migrate glin-forge to use it

### Phase 2: Build glinscan with glin-sdk-rust
- Use glin-indexer for blockchain data
- Use glin-client for RPC calls
- Share types with glin-forge

### Phase 3: Enhance TypeScript SDK
- Match Rust SDK feature parity
- Add React hooks (already done via typegen)
- Publish to npm

### Phase 4: Python SDK (Future)
- Core features for analytics
- Pandas integration
- Research tools

## Conclusion

The multi-language SDK strategy allows GLIN to:
1. **Serve different audiences**: Developers, researchers, enterprises
2. **Avoid duplication**: Shared core, unique extensions
3. **Follow industry standards**: Learn from Ethereum and Polkadot
4. **Enable innovation**: Each language excels at different tasks

By keeping common core features synchronized while allowing language-specific extensions, we provide the best tools for each use case without sacrificing consistency or maintainability.

---

**Last Updated**: 2025-10-04
**Author**: GLIN AI Development Team
