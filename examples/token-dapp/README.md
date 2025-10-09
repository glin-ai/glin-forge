# Token dApp Example

A complete full-stack decentralized application featuring an ERC20-like token with a React frontend.

## Features

- **Smart Contract**: PSP22-compatible fungible token
  - Minting and burning
  - Transfer and approval mechanisms
  - Balance queries
  - Total supply tracking

- **Frontend**: Modern React app with TypeScript
  - Connect wallet
  - View token balance
  - Transfer tokens
  - Mint tokens (owner only)
  - Real-time event listening

- **Testing**: Comprehensive test suite
  - Unit tests for contract logic
  - Integration tests with frontend
  - Event assertions

## Project Structure

```
token-dapp/
├── contracts/
│   └── token/           # Token contract (ink!)
├── frontend/            # React frontend
│   ├── src/
│   │   ├── components/  # React components
│   │   ├── hooks/       # Custom hooks
│   │   └── App.tsx      # Main app
│   └── package.json
├── scripts/             # Deployment scripts
│   └── deploy.ts
├── test/                # Test files
│   └── token.test.ts
└── glinforge.config.ts  # Project configuration
```

## Quick Start

### 1. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies
npm install
```

### 2. Start Local Node

```bash
glin-forge run
```

This will start a local development node at `ws://localhost:9944`.

### 3. Deploy Contract

```bash
glin-forge deploy --network local
```

Or use the deployment script:

```bash
glin-forge run scripts/deploy.ts
```

### 4. Run Tests

```bash
glin-forge test
```

### 5. Start Frontend

```bash
cd frontend
npm run dev
```

The frontend will be available at `http://localhost:3000`.

## Contract API

### Constructor

```rust
#[ink(constructor)]
pub fn new(total_supply: Balance) -> Self
```

Creates a new token with the specified total supply, minted to the caller.

### Messages

#### Transfer

```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>
```

Transfer tokens to another account.

#### Approve

```rust
#[ink(message)]
pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>
```

Approve another account to spend tokens on your behalf.

#### Transfer From

```rust
#[ink(message)]
pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>
```

Transfer tokens on behalf of another account (requires approval).

#### Mint

```rust
#[ink(message)]
pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()>
```

Mint new tokens to an account (owner only).

#### Burn

```rust
#[ink(message)]
pub fn burn(&mut self, value: Balance) -> Result<()>
```

Burn tokens from the caller's balance.

### Queries

#### Balance Of

```rust
#[ink(message)]
pub fn balance_of(&self, owner: AccountId) -> Balance
```

Get the token balance of an account.

#### Total Supply

```rust
#[ink(message)]
pub fn total_supply(&self) -> Balance
```

Get the total token supply.

#### Allowance

```rust
#[ink(message)]
pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance
```

Get the amount of tokens that a spender can spend on behalf of an owner.

## Frontend Usage

### Connect Wallet

The app automatically detects and connects to available wallets (Polkadot.js extension).

### View Balance

```typescript
const { balance } = useTokenBalance(contractAddress, account);
```

### Transfer Tokens

```typescript
const { transfer, isLoading } = useTokenTransfer(contract);

await transfer(recipientAddress, amount);
```

### Mint Tokens

```typescript
const { mint } = useTokenMint(contract);

await mint(recipientAddress, amount);
```

## Testing

### Unit Tests

```bash
# Run all tests
glin-forge test

# Run specific test
glin-forge test test/token.test.ts
```

### Test Coverage

```bash
glin-forge test --coverage
```

## Deployment

### Local Network

```bash
glin-forge deploy --network local
```

### Testnet

```bash
glin-forge deploy --network testnet
```

### Mainnet

```bash
glin-forge deploy --network mainnet --verify
```

## Configuration

Edit `glinforge.config.ts` to customize:

- Network endpoints
- Deployment accounts
- Gas limits
- Compiler options

## Learn More

- [glin-forge Documentation](https://docs.glin.network/glin-forge)
- [ink! Documentation](https://use.ink)
- [PSP22 Standard](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md)

## License

MIT
