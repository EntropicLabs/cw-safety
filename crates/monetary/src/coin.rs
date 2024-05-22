use std::ops::{Add, Sub};

use cosmwasm_std::Coin;

use crate::{AmountU128, Denom, MonetaryError};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CheckedCoin<T> {
    pub denom: Denom<T>,
    pub amount: AmountU128<T>,
}

impl<T> CheckedCoin<T> {
    pub fn from_coin(coin: Coin, denom: Denom<T>) -> Result<Self, MonetaryError> {
        if coin.denom != denom.repr() {
            return Err(MonetaryError::DenomMismatch(coin.denom, denom.to_string()));
        }

        Ok(CheckedCoin {
            denom,
            amount: AmountU128::new(coin.amount),
        })
    }

    pub fn new(denom: Denom<T>, amount: AmountU128<T>) -> Self {
        CheckedCoin { denom, amount }
    }

    pub fn to_unchecked(self) -> Coin {
        Coin {
            denom: self.denom.into(),
            amount: self.amount.uint128(),
        }
    }
}

impl<T> Add for CheckedCoin<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CheckedCoin {
            denom: self.denom,
            amount: self.amount + rhs.amount,
        }
    }
}

impl<T> Add<AmountU128<T>> for CheckedCoin<T> {
    type Output = Self;

    fn add(self, rhs: AmountU128<T>) -> Self::Output {
        CheckedCoin {
            denom: self.denom,
            amount: self.amount + rhs,
        }
    }
}

impl<T> Sub for CheckedCoin<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        CheckedCoin {
            denom: self.denom,
            amount: self.amount - rhs.amount,
        }
    }
}

impl<T> Sub<AmountU128<T>> for CheckedCoin<T> {
    type Output = Self;

    fn sub(self, rhs: AmountU128<T>) -> Self::Output {
        CheckedCoin {
            denom: self.denom,
            amount: self.amount - rhs,
        }
    }
}

impl<T> From<CheckedCoin<T>> for Coin {
    fn from(val: CheckedCoin<T>) -> Self {
        Coin {
            denom: val.denom.to_string(),
            amount: val.amount.uint128(),
        }
    }
}
