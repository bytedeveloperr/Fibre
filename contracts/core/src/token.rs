use near_sdk::{AccountId, Balance};

pub struct Token {
    pub base_token_id: AccountId,
    pub amount: Balance,
}
