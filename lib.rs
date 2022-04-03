use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env,
    log,
    ext_contract,
    near_bindgen,
    AccountId,
};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GrantMinterDelegate {}

// If the name is not provided, the namespace for generated methods in derived by applying snake
// case to the trait name, e.g. ext_status_message.
#[ext_contract]
pub trait ExtGrantMinter {
    fn grant_minter(&mut self, account_id: AccountId);
}

#[near_bindgen]
impl GrantMinterDelegate {
    #[payable]
    pub fn grant_minter(&mut self, mintbase_account_id: AccountId) {
        let account_id = env::signer_account_id();
        log!("account_id {}", account_id);
        ext_grant_minter::grant_minter(account_id, mintbase_account_id, 1, env::prepaid_gas() / 2);
    }
}
