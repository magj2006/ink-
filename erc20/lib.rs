#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/// Module of ERC-20
#[ink::contract]
mod erc20 {
    use ink_storage::collections::HashMap;
    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: HashMap<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfered {
        /// from's account id
        #[ink(topic)]
        from: Option<AccountId>,
        /// to's account id
        #[ink(topic)]
        to: Option<AccountId>,
        /// amount which be transfered
        #[ink(topic)]
        amount: Balance,
    }

    /// Implement of ERC-20
    impl Erc20 {
        /// Instantiate a new ERC-20 contract
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = HashMap::new();
            balances.insert(caller, initial_supply);

            Self::env().emit_event(Transfered {
                from: None,
                to: Some(caller),
                amount: initial_supply,
            });
            Self {
                total_supply: initial_supply,
                balances,
            }
        }

        /// Reture the total supply for the ERC-20 contract
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Reture the balance of user for the ERC-20 contract
        #[ink(message)]
        pub fn balance_of(&self, user: AccountId) -> Balance {
            self.balances.get(&user).copied().unwrap_or(0)
        }

        /// Transfer amount to other account id, return true if successful, otherwise false.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);

            if from_balance < amount {
                return false;
            }

            let to_balance = self.balance_of(to);
            self.balances.insert(from, from_balance - amount);
            self.balances.insert(to, to_balance + amount);

            self.env().emit_event(Transfered {
                from: Some(from),
                to: Some(to),
                amount,
            });

            true
        }
    }
}
