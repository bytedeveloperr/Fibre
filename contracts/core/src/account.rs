use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

use crate::fibre::{Fibre, FibreExt};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    account_id: AccountId,
    tokens: HashMap<AccountId, AccountId>,
}

impl Account {
    pub fn new(account_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(account_id.as_bytes()));

        Self {
            account_id,
            tokens: HashMap::new(),
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
