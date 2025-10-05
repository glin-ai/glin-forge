---
title: ERC20 Template
description: Fungible token template for GLIN Forge
---

# ERC20 Token Template

The ERC20 template provides a complete implementation of a fungible token contract, compatible with the ERC20 standard.

## Overview

Create an ERC20 token contract:

```bash
glin-forge new my-token --template erc20
```

This generates a production-ready token contract with all standard ERC20 methods.

## Features

- **Total Supply Tracking** - Monitor total token circulation
- **Balance Management** - Track balances for each account
- **Transfer** - Send tokens between accounts
- **Approve/TransferFrom** - Delegate spending to other accounts
- **Events** - Transfer and Approval events
- **Error Handling** - Comprehensive error types

## Contract Structure

### Storage

```rust
#[ink(storage)]
pub struct MyToken {
    /// Total token supply
    total_supply: Balance,
    /// Mapping from account to balance
    balances: Mapping<AccountId, Balance>,
    /// Mapping from (owner, spender) to allowance
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
```

### Constructor

```rust
#[ink(constructor)]
pub fn new(
    total_supply: Balance,
    name: String,
    symbol: String,
) -> Self {
    let caller = Self::env().caller();

    let mut balances = Mapping::default();
    balances.insert(caller, &total_supply);

    Self::env().emit_event(Transfer {
        from: None,
        to: Some(caller),
        value: total_supply,
    });

    Self {
        total_supply,
        balances,
        allowances: Mapping::default(),
    }
}
```

## Methods

### Query Methods (Read-only)

#### total_supply

Get the total token supply.

```rust
#[ink(message)]
pub fn total_supply(&self) -> Balance {
    self.total_supply
}
```

**Usage:**
```bash
glin-forge query 5Contract... totalSupply
```

#### balance_of

Get balance of an account.

```rust
#[ink(message)]
pub fn balance_of(&self, account: AccountId) -> Balance {
    self.balances.get(account).unwrap_or(0)
}
```

**Usage:**
```bash
glin-forge query 5Contract... balanceOf 5Account...
```

#### allowance

Get approved amount for spender.

```rust
#[ink(message)]
pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
    self.allowances.get((owner, spender)).unwrap_or(0)
}
```

**Usage:**
```bash
glin-forge query 5Contract... allowance 5Owner... 5Spender...
```

### Transaction Methods (State-changing)

#### transfer

Transfer tokens to another account.

```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
    let from = self.env().caller();
    self.transfer_from_to(from, to, amount)?;
    Ok(())
}
```

**Usage:**
```bash
glin-forge call 5Contract... transfer 5Recipient... 1000 \
  --account alice \
  --wait
```

#### approve

Approve spender to spend tokens on your behalf.

```rust
#[ink(message)]
pub fn approve(&mut self, spender: AccountId, amount: Balance) -> Result<()> {
    let owner = self.env().caller();
    self.allowances.insert((owner, spender), &amount);

    self.env().emit_event(Approval {
        owner,
        spender,
        value: amount,
    });

    Ok(())
}
```

**Usage:**
```bash
glin-forge call 5Contract... approve 5Spender... 5000 \
  --account alice \
  --wait
```

#### transfer_from

Transfer tokens from one account to another (requires approval).

```rust
#[ink(message)]
pub fn transfer_from(
    &mut self,
    from: AccountId,
    to: AccountId,
    amount: Balance,
) -> Result<()> {
    let caller = self.env().caller();

    let allowance = self.allowance(from, caller);
    if allowance < amount {
        return Err(Error::InsufficientAllowance);
    }

    self.transfer_from_to(from, to, amount)?;

    self.allowances.insert((from, caller), &(allowance - amount));

    Ok(())
}
```

**Usage:**
```bash
glin-forge call 5Contract... transferFrom \
  5From... 5To... 500 \
  --account spender \
  --wait
```

## Events

### Transfer

Emitted when tokens are transferred.

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,
}
```

**Fields:**
- `from` - Source account (None for minting)
- `to` - Destination account (None for burning)
- `value` - Amount transferred

### Approval

Emitted when allowance is set.

```rust
#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: Balance,
}
```

**Fields:**
- `owner` - Token owner
- `spender` - Approved spender
- `value` - Approved amount

## Error Types

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Insufficient balance for transfer
    InsufficientBalance,
    /// Insufficient allowance for transfer_from
    InsufficientAllowance,
}
```

## Usage Examples

### Deploy Token

```bash
# Create project
glin-forge new my-token --template erc20
cd my-token

# Build
glin-forge build --release

# Deploy with 1 million tokens
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"
```

### Query Balance

```bash
# Check total supply
glin-forge query $CONTRACT totalSupply

# Check Alice's balance
glin-forge query $CONTRACT balanceOf 5GrwvaEF...

# Check allowance
glin-forge query $CONTRACT allowance 5Owner... 5Spender...
```

### Transfer Tokens

```bash
# Alice transfers 100 tokens to Bob
glin-forge call $CONTRACT transfer 5FrLwHLX... 100 \
  --account alice \
  --wait

# Verify transfer
glin-forge query $CONTRACT balanceOf 5FrLwHLX...
```

### Approve and TransferFrom

```bash
# Alice approves Bob to spend 500 tokens
glin-forge call $CONTRACT approve 5FrLwHLX... 500 \
  --account alice \
  --wait

# Bob transfers 100 from Alice to Charlie
glin-forge call $CONTRACT transferFrom \
  5GrwvaEF... 5DAAnrj7... 100 \
  --account bob \
  --wait

# Check remaining allowance
glin-forge query $CONTRACT allowance 5GrwvaEF... 5FrLwHLX...
# Result: 400
```

## Customization

### Add Token Metadata

```rust
#[ink(storage)]
pub struct MyToken {
    // ... existing fields ...
    name: String,
    symbol: String,
    decimals: u8,
}

impl MyToken {
    #[ink(message)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[ink(message)]
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    #[ink(message)]
    pub fn decimals(&self) -> u8 {
        self.decimals
    }
}
```

### Add Minting

```rust
#[ink(storage)]
pub struct MyToken {
    // ... existing fields ...
    owner: AccountId,
}

impl MyToken {
    #[ink(message)]
    pub fn mint(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        let caller = self.env().caller();
        if caller != self.owner {
            return Err(Error::NotOwner);
        }

        let new_balance = self.balance_of(to) + amount;
        self.balances.insert(to, &new_balance);
        self.total_supply += amount;

        self.env().emit_event(Transfer {
            from: None,
            to: Some(to),
            value: amount,
        });

        Ok(())
    }
}
```

### Add Burning

```rust
impl MyToken {
    #[ink(message)]
    pub fn burn(&mut self, amount: Balance) -> Result<()> {
        let caller = self.env().caller();
        let balance = self.balance_of(caller);

        if balance < amount {
            return Err(Error::InsufficientBalance);
        }

        self.balances.insert(caller, &(balance - amount));
        self.total_supply -= amount;

        self.env().emit_event(Transfer {
            from: Some(caller),
            to: None,
            value: amount,
        });

        Ok(())
    }
}
```

### Add Pausable

```rust
#[ink(storage)]
pub struct MyToken {
    // ... existing fields ...
    paused: bool,
}

impl MyToken {
    fn ensure_not_paused(&self) -> Result<()> {
        if self.paused {
            return Err(Error::Paused);
        }
        Ok(())
    }

    #[ink(message)]
    pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        self.ensure_not_paused()?;
        // ... rest of transfer logic
    }

    #[ink(message)]
    pub fn pause(&mut self) -> Result<()> {
        self.only_owner()?;
        self.paused = true;
        Ok(())
    }

    #[ink(message)]
    pub fn unpause(&mut self) -> Result<()> {
        self.only_owner()?;
        self.paused = false;
        Ok(())
    }
}
```

## Testing

The template includes comprehensive tests:

```bash
glin-forge test
```

### Test Coverage

- ✅ Constructor initialization
- ✅ Balance queries
- ✅ Transfers
- ✅ Approvals
- ✅ TransferFrom
- ✅ Error conditions
- ✅ Events

### Example Tests

```rust
#[ink::test]
fn transfer_works() {
    let mut contract = MyToken::new(1000, "Test".into(), "TST".into());
    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

    assert!(contract.transfer(accounts.bob, 100).is_ok());
    assert_eq!(contract.balance_of(accounts.bob), 100);
    assert_eq!(contract.balance_of(accounts.alice), 900);
}

#[ink::test]
fn transfer_fails_insufficient_balance() {
    let mut contract = MyToken::new(1000, "Test".into(), "TST".into());
    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

    assert_eq!(
        contract.transfer(accounts.bob, 2000),
        Err(Error::InsufficientBalance)
    );
}
```

## Frontend Integration

Generate TypeScript types:

```bash
glin-forge typegen --output ./frontend/src/contracts --hooks
```

Use in React:

```tsx
import { useMyToken } from './contracts/useMyToken'

function TokenBalance() {
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

## Best Practices

1. **Initialize with appropriate supply** - Consider use case
2. **Use decimals** - Standard is 18 for compatibility
3. **Test thoroughly** - Run all tests before deployment
4. **Document** - Add clear comments
5. **Verify** - Submit to block explorer after deployment

## Security Considerations

- ✅ Overflow protection (Rust handles this)
- ✅ Zero address checks
- ✅ Reentrancy protection (ink! handles this)
- ✅ Access control (add if needed)

## Related Templates

- [ERC721 Template](./erc721) - NFT implementation
- [DAO Template](./dao) - Governance with tokens

## Resources

- [ERC20 Standard](https://eips.ethereum.org/EIPS/eip-20)
- [ink! Documentation](https://use.ink/)
- [OpenBrush ERC20](https://openbrush.io/)
