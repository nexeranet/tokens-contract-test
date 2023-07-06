use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::UnorderedMap;
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BlockHeight, BorshStorageKey, PanicOnDefault,
    Promise,
};

/// Contract state.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum RunningState {
    // Contract is active.
    Active,
    // Contract is paused.
    Paused,
}

/// Contract storage keys used for NEAR collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Accounts,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub(crate) balance: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    /// The account ID of the owner who's running the staking validator node.
    pub owner_id: AccountId,
    pub(crate) running_state: RunningState,
    pub opn_contract: Option<AccountId>,
    pub accounts: UnorderedMap<AccountId, Account>,
    pub by_half: bool,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            running_state: RunningState::Active,
            opn_contract: None,
            accounts: UnorderedMap::new(StorageKeys::Accounts),
            by_half: false,
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Init contract func
    #[init]
    pub fn init(owner_id: AccountId, opn_contract: AccountId) -> Self {
        Self {
            owner_id,
            running_state: RunningState::Active,
            opn_contract: Some(opn_contract),
            accounts: UnorderedMap::new(StorageKeys::Accounts),
            by_half: false,
        }
    }

    pub(crate) fn opn_address(&self) -> AccountId {
        self.opn_contract.as_ref().unwrap().clone()
    }

    pub(crate) fn assert_contract_running(&self) {
        require!(
            self.running_state == RunningState::Active,
            "Contract is not running."
        );
    }

    /// Asserts that the method was called by the owner.
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }
    pub fn set_settings(&mut self, by_half: bool) {
        self.by_half = by_half;
    }

    #[handle_result]
    pub fn get_balance(&self, acc_id: AccountId) -> Result<U128, &'static str> {
        let acc = self.accounts.get(&acc_id).ok_or("Can't find account")?;
        Ok(U128(acc.balance))
    }
}
