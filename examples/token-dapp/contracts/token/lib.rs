#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod token {
    use ink::storage::Mapping;

    /// ERC20-like token contract
    #[ink(storage)]
    pub struct Token {
        /// Total supply of tokens
        total_supply: Balance,
        /// Mapping from account to balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping from (owner, spender) to allowance
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Contract owner (can mint tokens)
        owner: AccountId,
    }

    /// Event emitted when tokens are transferred
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval is made
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// Token errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Insufficient balance for transfer
        InsufficientBalance,
        /// Insufficient allowance for transfer
        InsufficientAllowance,
        /// Only owner can perform this action
        OnlyOwner,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Token {
        /// Constructor that initializes the token with a total supply
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
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
                owner: caller,
            }
        }

        /// Returns the total token supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of the given account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        /// Returns the allowance
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        /// Transfers tokens from caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Approves spender to spend tokens on behalf of caller
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        /// Transfers tokens from one account to another using allowance
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(&from, &to, value)?;
            self.allowances
                .insert((from, caller), &(allowance - value));

            Ok(())
        }

        /// Mints new tokens to the given account (owner only)
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::OnlyOwner);
            }

            let balance = self.balance_of(to);
            self.balances.insert(to, &(balance + value));
            self.total_supply += value;

            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value,
            });

            Ok(())
        }

        /// Burns tokens from the caller's account
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let balance = self.balance_of(caller);

            if balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(caller, &(balance - value));
            self.total_supply -= value;

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: None,
                value,
            });

            Ok(())
        }

        /// Internal transfer function
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(token.total_supply(), 1000);
            assert_eq!(token.balance_of(accounts.alice), 1000);
        }

        #[ink::test]
        fn transfer_works() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(token.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(token.balance_of(accounts.alice), 900);
            assert_eq!(token.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            let mut token = Token::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(
                token.transfer(accounts.bob, 200),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn approve_works() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(token.approve(accounts.bob, 100), Ok(()));
            assert_eq!(token.allowance(accounts.alice, accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob to spend 100 tokens
            assert_eq!(token.approve(accounts.bob, 100), Ok(()));

            // Change caller to Bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob transfers 100 from Alice to Charlie
            assert_eq!(
                token.transfer_from(accounts.alice, accounts.charlie, 100),
                Ok(())
            );

            assert_eq!(token.balance_of(accounts.alice), 900);
            assert_eq!(token.balance_of(accounts.charlie), 100);
            assert_eq!(token.allowance(accounts.alice, accounts.bob), 0);
        }

        #[ink::test]
        fn mint_works() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(token.mint(accounts.bob, 500), Ok(()));
            assert_eq!(token.total_supply(), 1500);
            assert_eq!(token.balance_of(accounts.bob), 500);
        }

        #[ink::test]
        fn mint_fails_non_owner() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            assert_eq!(token.mint(accounts.charlie, 500), Err(Error::OnlyOwner));
        }

        #[ink::test]
        fn burn_works() {
            let mut token = Token::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(token.burn(100), Ok(()));
            assert_eq!(token.total_supply(), 900);
            assert_eq!(token.balance_of(accounts.alice), 900);
        }
    }
}
