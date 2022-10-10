use crate::fibre::{Fibre, FibreExt};
use crate::misc::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Promise, PromiseOrValue};
use std::collections::HashMap;

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_total_supply(&self) -> U128;

    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

#[ext_contract(ext_fibre_fungible_token)]
pub trait FibreFungibleToken: FungibleToken {
    fn ft_mint(&mut self, account_id: AccountId, amount: U128);

    fn ft_burn(&mut self, account_id: AccountId, amount: U128);
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub token_id: AccountId,
    pub amount: Balance,
    shares: HashMap<AccountId, Balance>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintTokenInput {
    collateral_amount: U128,
    mint_token_id: TokenId,
    collateral_token_id: AccountId,
}

impl Token {
    pub fn new(token_id: AccountId) -> Self {
        Self {
            token_id,
            amount: 0,
            shares: HashMap::new(),
        }
    }

    pub fn get_total_shares(&self, account_id: AccountId) -> Balance {
        *self.shares.get(&account_id).unwrap_or(&0)
    }

    pub fn internal_increment_shares(&mut self, account_id: AccountId, amount: Balance) {
        let balance = self.get_total_shares(account_id.clone());
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_else(|| env::panic_str("message"));

        self.shares.insert(account_id, new_balance);
    }

    pub fn mint(&mut self, account_id: AccountId, amount: Balance) {
        self.internal_increment_shares(account_id, amount)
    }
}

impl Fibre {
    pub fn internal_ft_mint(
        &self,
        token_id: TokenId,
        account_id: AccountId,
        amount: U128,
    ) -> Promise {
        ext_fibre_fungible_token::ext(token_id)
            .with_static_gas(100_000_000_000.into())
            .ft_mint(account_id, amount)
    }
}

#[near_bindgen]
impl Fibre {
    pub fn get_token(&self, token_id: TokenId) -> Token {
        self.tokens.get(&token_id).unwrap_or_else(|| {
            env::panic_str(format!("Error: The token {} was not found", token_id).as_str())
        })
    }

    pub fn mint_token(&mut self, input: MintTokenInput) {
        let minter_id = env::predecessor_account_id();

        let mint = self.internal_ft_mint(
            input.mint_token_id.clone(),
            minter_id.clone(),
            input.collateral_amount.clone(),
        );

        mint.then(
            Self::ext(env::current_account_id())
                .with_static_gas(100_000_000.into())
                .on_mint_token(minter_id, input.mint_token_id),
        );
    }

    pub fn on_mint_token(
        &mut self,
        #[callback_result] call_result: Result<String, Promise>,
        account_id: AccountId,
        token_id: TokenId,
    ) {
        if call_result.is_err() {
            env::panic_str("Error")
        }

        let mut account = self.internal_get_account(account_id.clone().clone());
        let mut token = self.get_token(token_id.clone());
        account.deposit(token_id.clone(), 100);
        token.mint(account_id.clone(), 0);
    }
}
