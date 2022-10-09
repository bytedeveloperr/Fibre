use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId};
use std::collections::HashMap;

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
