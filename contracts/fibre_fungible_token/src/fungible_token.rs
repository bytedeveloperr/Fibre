use near_contract_standards::fungible_token::events;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use near_sdk::{log, near_bindgen, AccountId, Balance, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct Contract {
    pub token: FungibleToken,
    pub metadata: FungibleTokenMetadata,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String, symbol: String, icon: Option<String>, decimals: u8) -> Self {
        let token = FungibleToken::new("token".as_bytes());
        let metadata = FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name,
            symbol,
            icon,
            decimals,
            reference: None,
            reference_hash: None,
        };

        Self { token, metadata }
    }

    pub fn ft_mint(&mut self, account_id: AccountId, amount: U128) {
        if !self.token.accounts.contains_key(&account_id) {
            self.token.internal_register_account(&account_id);
        }

        self.token.internal_deposit(&account_id, amount.into());

        events::FtMint {
            amount: &amount.into(),
            owner_id: &account_id,
            memo: None,
        }
        .emit();
    }

    pub fn ft_burn(&mut self, account_id: AccountId, amount: U128) {
        if !self.token.accounts.contains_key(&account_id) {
            self.token.internal_register_account(&account_id);
        }

        self.token.internal_withdraw(&account_id, amount.into());

        events::FtBurn {
            amount: &amount.into(),
            owner_id: &account_id,
            memo: None,
        }
        .emit()
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);
