use std::marker::PhantomData;

use cosmwasm_std::Uint128;

use crate::Denom;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmountU128<T: Denom>(pub(crate) Uint128, PhantomData<T>);

impl<T: Denom> AmountU128<T> {
    pub fn new(amount: Uint128) -> Self {
        AmountU128(amount, PhantomData)
    }

    pub fn unwrap(self) -> Uint128 {
        self.0
    }

    pub fn u128(&self) -> u128 {
        self.0.u128()
    }
}

impl<T: Denom + Clone> Copy for AmountU128<T> {}
