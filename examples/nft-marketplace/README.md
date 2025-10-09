# NFT Marketplace Example

A complete NFT marketplace with minting, listing, and trading capabilities.

## Features

### Smart Contracts
- **NFT Contract**: PSP34-compatible non-fungible token
  - Minting with metadata
  - Ownership tracking
  - Approval mechanisms
  - Enumeration support

- **Marketplace Contract**: Trading platform
  - List NFTs for sale
  - Buy listed NFTs
  - Cancel listings
  - Royalty support

### Frontend
- Modern React app with NFT gallery
- Mint new NFTs with image upload to IPFS
- Browse and search marketplace
- Buy/sell NFTs
- View transaction history

## Quick Start

```bash
# Install dependencies
npm install

# Deploy contracts
glin-forge run scripts/deploy.ts

# Run tests
glin-forge test

# Start frontend
cd frontend && npm run dev
```

## Contract APIs

### NFT Contract

```rust
// Mint a new NFT
#[ink(message)]
pub fn mint(&mut self, to: AccountId, token_id: u32, metadata: String) -> Result<()>

// Get NFT owner
#[ink(message)]
pub fn owner_of(&self, token_id: u32) -> Option<AccountId>

// Transfer NFT
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, token_id: u32) -> Result<()>
```

### Marketplace Contract

```rust
// List NFT for sale
#[ink(message)]
pub fn list(&mut self, nft_contract: AccountId, token_id: u32, price: Balance) -> Result<()>

// Buy listed NFT
#[ink(message)]
#[ink(payable)]
pub fn buy(&mut self, listing_id: u32) -> Result<()>

// Cancel listing
#[ink(message)]
pub fn cancel(&mut self, listing_id: u32) -> Result<()>
```

## Architecture

```
User Wallet
     |
     v
React Frontend <---> glin-forge SDK
                          |
                          v
                    Marketplace Contract
                          |
                          v
                     NFT Contract
```

## Key Concepts

### NFT Metadata
Stored on IPFS with the following structure:
```json
{
  "name": "My NFT #1",
  "description": "A unique digital asset",
  "image": "ipfs://...",
  "attributes": [
    { "trait_type": "Rarity", "value": "Rare" }
  ]
}
```

### Marketplace Listings
```rust
pub struct Listing {
    pub seller: AccountId,
    pub nft_contract: AccountId,
    pub token_id: u32,
    pub price: Balance,
    pub active: bool,
}
```

## Testing

```bash
# Run all tests
glin-forge test

# Test specific contract
glin-forge test test/nft.test.ts

# Test with coverage
glin-forge test --coverage
```

## Learn More

- [PSP34 Standard](https://github.com/w3f/PSPs/blob/master/PSPs/psp-34.md)
- [IPFS Documentation](https://docs.ipfs.io/)
- [NFT Best Practices](https://docs.glin.network/nfts)

## License

MIT
