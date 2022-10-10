use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

use crate::account::Account;
use crate::misc::TokenId;
use crate::token::Token;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Fibre {
    pub accounts: LookupMap<AccountId, Account>,
    pub tokens: LookupMap<TokenId, Token>,
}

impl Default for Fibre {
    fn default() -> Self {
        env::panic_str("Contract have not yet been initialized")
    }
}

#[near_bindgen]
impl Fibre {
    #[init]
    pub fn new() -> Self {
        Self {
            accounts: LookupMap::new("accounts".as_bytes()),
            tokens: LookupMap::new("tokens".as_bytes()),
        }
    }
}
