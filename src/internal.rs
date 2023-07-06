use near_sdk::{ext_contract, json_types::U128, PromiseError};

use crate::*;

// TODO: ask @TM about return value for this fuction
#[ext_contract(ext_opn_contract)]
pub trait OpnContract {
    fn mt_mint(&mut self, supply_payout: U128) -> PromiseOrValue<()>;
}

pub trait CallOpnContract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128) -> Result<U128, &'static str>;
    fn mint(&mut self, amount: U128) -> Result<PromiseOrValue<()>, &'static str>;
}

#[near_bindgen]
impl CallOpnContract for Contract {
    #[payable]
    #[handle_result]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128) -> Result<U128, &'static str> {
        self.assert_contract_running();
        assert_one_yocto();
        if amount.0 == 0 {
            return Err("Empty amount");
        }
        let mut sum = u128::from(amount);
        let acc = self
            .accounts
            .get(&receiver_id)
            .ok_or("Can't find account")?;
        if self.by_half {
            sum = sum.checked_div(2u128).ok_or("Amount overflowed")?;
        }
        let result = acc.balance.checked_sub(sum).ok_or("Balance overflowed")?;
        Ok(U128(result))
    }

    #[payable]
    #[handle_result]
    fn mint(&mut self, amount: U128) -> Result<PromiseOrValue<()>, &'static str> {
        if amount.0 == 0 {
            return Err("Empty amount");
        }
        self.assert_contract_running();
        assert_one_yocto();

        Ok(ext_opn_contract::ext(self.opn_address())
            .with_attached_deposit(ONE_YOCTO)
            .mt_mint(amount)
            .then(Self::ext(env::current_account_id()).mint_resolver(amount))
            .into())
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    #[handle_result]
    pub fn mint_resolver(
        &mut self,
        _withdrawn_amount: U128,
        #[callback_result] result: Result<(), PromiseError>,
    ) -> Result<PromiseOrValue<()>, &'static str> {
        match result {
            Ok(value) => Ok(PromiseOrValue::Value(value)),
            Err(e) => Err("Promise failed"),
        }
    }
}
