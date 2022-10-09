use crate::fibre::{Fibre, FibreExt};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub token_id: AccountId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintTokenInput {
    collateral_amount: U128,
    mint_token_id: AccountId,
    collateral_token_id: AccountId,
}

impl Token {
    pub fn new(token_id: AccountId) -> Self {
        Self {
            token_id,
            amount: 0,
        }
    }

    pub fn mint(&mut self) {}
}

#[near_bindgen]
impl Fibre {
    pub fn get_token(&self, token_id: AccountId) -> Token {
        self.tokens.get(&token_id).unwrap_or_else(|| {
            env::panic_str(format!("Error: The token {} was not found", token_id).as_str())
        })
    }

    pub fn mint_token(&mut self, input: MintTokenInput) {
        let minter_id = env::predecessor_account_id();
        let mut token = self.get_token(input.mint_token_id.clone());

        let mut account = self.internal_get_account(minter_id);

        account.deposit(input.mint_token_id, 100);
        token.mint();
    }
}
