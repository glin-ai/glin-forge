# DAO Governance Example

A decentralized autonomous organization (DAO) with on-chain governance, proposals, and voting.

## Features

### Smart Contract
- **DAO Contract**: Complete governance system
  - Member management
  - Proposal creation
  - Voting mechanisms (for/against/abstain)
  - Proposal execution
  - Treasury management
  - Configurable voting periods and quorums

### Frontend
- Modern React governance dashboard
- Create and submit proposals
- Vote on active proposals
- View proposal history
- Track voting power
- Treasury overview

## Quick Start

```bash
# Install dependencies
npm install

# Deploy DAO
glin-forge run scripts/deploy.ts

# Run tests
glin-forge test

# Start frontend
cd frontend && npm run dev
```

## Contract API

### Membership

```rust
// Join DAO (with stake)
#[ink(message)]
#[ink(payable)]
pub fn join(&mut self) -> Result<()>

// Leave DAO (unstake)
#[ink(message)]
pub fn leave(&mut self) -> Result<()>

// Get member voting power
#[ink(message)]
pub fn get_voting_power(&self, member: AccountId) -> Balance
```

### Proposals

```rust
// Create a new proposal
#[ink(message)]
pub fn propose(&mut self, description: String, execution_data: Vec<u8>) -> Result<u32>

// Vote on a proposal
#[ink(message)]
pub fn vote(&mut self, proposal_id: u32, support: Vote) -> Result<()>

// Execute passed proposal
#[ink(message)]
pub fn execute(&mut self, proposal_id: u32) -> Result<()>
```

### Treasury

```rust
// Deposit to treasury
#[ink(message)]
#[ink(payable)]
pub fn deposit(&mut self) -> Result<()>

// Withdraw from treasury (requires proposal)
#[ink(message)]
pub fn withdraw(&mut self, to: AccountId, amount: Balance) -> Result<()>
```

## Architecture

### Proposal Lifecycle

```
1. Created → 2. Voting → 3. Passed/Rejected → 4. Executed/Expired
```

### Proposal States
- **Pending**: Waiting for voting period to start
- **Active**: Currently accepting votes
- **Passed**: Achieved quorum and majority
- **Rejected**: Failed to achieve quorum or majority
- **Executed**: Successfully executed
- **Expired**: Voting period ended without execution

### Voting Rules
- Quorum: Minimum 20% of total voting power must participate
- Majority: >50% of votes must be in favor
- Voting Period: 7 days by default
- Execution Delay: 2 days after passing

## Governance Parameters

```rust
pub struct GovernanceConfig {
    pub voting_period: u64,      // In blocks
    pub quorum_percentage: u8,   // 0-100
    pub execution_delay: u64,    // In blocks
    pub min_stake: Balance,      // Minimum to join
}
```

## Example Proposals

### Treasury Transfer
```typescript
await dao.propose(
  "Transfer 1000 GLIN to development team",
  encodeCall("treasury_transfer", [devTeamAddress, 1000])
);
```

### Parameter Change
```typescript
await dao.propose(
  "Increase voting period to 14 days",
  encodeCall("update_config", { voting_period: 14 * 24 * 60 * 10 })
);
```

### Add Member
```typescript
await dao.propose(
  "Grant membership to contributor",
  encodeCall("add_member", [contributorAddress])
);
```

## Testing

```bash
# Run all DAO tests
glin-forge test

# Test proposal flow
glin-forge test test/proposals.test.ts

# Test voting mechanisms
glin-forge test test/voting.test.ts
```

## Security Considerations

1. **Proposal Execution**: Ensure execution data is validated
2. **Voting Power**: Implement sybil resistance mechanisms
3. **Treasury Security**: Multi-sig or timelock for large withdrawals
4. **Flash Loan Protection**: Snapshot voting power at proposal creation
5. **Governance Attacks**: Monitor for whale concentration

## Best Practices

- Always review proposals before voting
- Participate in governance discussions off-chain
- Set reasonable quorum and majority thresholds
- Implement gradual parameter changes
- Use timelocks for critical operations

## Learn More

- [DAO Best Practices](https://docs.glin.network/dao)
- [Governance Patterns](https://docs.glin.network/governance)
- [Security Checklist](https://docs.glin.network/security)

## License

MIT
