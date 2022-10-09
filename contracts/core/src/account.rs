use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};
use std::collections::HashMap;

use crate::fibre::{Fibre, FibreExt};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    account_id: AccountId,
    tokens: HashMap<AccountId, Balance>,
}

impl Account {
    pub fn new(account_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(account_id.as_bytes()));

        Self {
            account_id,
            tokens: HashMap::new(),
        }
    }

    pub fn get_balance(&self, token_id: AccountId) -> Balance {
        *self.tokens.get(&token_id).unwrap_or(&0)
    }

    pub fn deposit(&mut self, token_id: AccountId, amount: Balance) {
        let balance = self.get_balance(token_id.clone());

        if let Some(new_balance) = balance.checked_add(amount) {
            self.tokens.insert(token_id, new_balance);
        } else {
            env::panic_str("Error: Deposit overflow")
        }
    }

    pub fn withdraw(&mut self, token_id: AccountId, amount: Balance) {
        let balance = self.get_balance(token_id.clone());

        if let Some(new_balance) = balance.checked_sub(amount) {
            self.tokens.insert(token_id, new_balance);
        } else {
            env::panic_str("Error: Insuffficient deposit balance")
        }
    }
}

impl Fibre {
    pub fn internal_save_account(&mut self, account_id: AccountId, account: Account) {
        assert!(env::is_valid_account_id(account_id.as_bytes()));

        self.accounts.insert(&account_id, &account);
    }

    pub fn internal_get_account(&self, account_id: AccountId) -> Account {
        self.accounts
            .get(&account_id)
            .unwrap_or(Account::new(account_id))
    }
}

#[near_bindgen]
impl Fibre {
    pub fn get_account(&self, account_id: AccountId) -> Account {
        self.internal_get_account(account_id)
    }
}
