use std::ops::{Add, Sub};

use cosmwasm_std::{Coin as CwCoin, Uint128};

use crate::{
    Currency, Imprecise, Precise, Precision, PrecisionSelector, PrecisionType,
    PrecisionTypeWrapper, Unverified,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coin<P: Precision> {
    pub(crate) amount: Uint128,
    pub(crate) denom: PrecisionTypeWrapper<P>,
}

impl<P: Precision> Coin<P> {
    pub fn amount(&self) -> Uint128 {
        self.amount
    }

    pub fn currency(&self) -> &P::C {
        self.denom.inner().currency()
    }

    pub fn denom(&self) -> &P {
        match &self.denom.inner() {
            PrecisionType::Precise(p) => {
                // This is safe because we know that the precision type is precise
                unsafe { &*(p as *const Precise<P::C> as *const P) }
            }
            PrecisionType::Imprecise(p) => {
                // This is safe because we know that the precision type is imprecise
                unsafe { &*(p as *const Imprecise<P::C> as *const P) }
            }
            PrecisionType::Unverified(p) => {
                // This is safe because we know that the precision type is unverified
                unsafe { &*(p as *const Unverified<P::C> as *const P) }
            }
        }
    }
}

impl<T: Currency> Coin<Imprecise<T>> {
    pub fn imprecise(coin: &CwCoin, denom: &T) -> Result<Self, String> {
        if coin.denom != denom.denom() {
            return Err(format!("Invalid denomination: {}", coin.denom));
        }
        Ok(Coin {
            amount: coin.amount,
            denom: Imprecise {
                currency: denom.clone(),
            }
            .select(),
        })
    }

    pub fn with_precision(&self, precision: u8) -> Coin<Precise<T>> {
        Coin {
            amount: self.amount,
            denom: Precise {
                currency: self.denom.inner().currency().clone(),
                decimals: precision,
            }
            .select(),
        }
    }
}

impl<T: Currency> Coin<Precise<T>> {
    pub fn precise(coin: &CwCoin, denom: &Precise<T>) -> Result<Self, String> {
        if coin.denom != denom.currency().denom() {
            return Err(format!("Invalid denomination: {}", coin.denom));
        }
        Ok(Coin {
            amount: coin.amount,
            denom: denom.clone().select(),
        })
    }
}

impl<P: Precision> AsRef<Coin<P>> for Coin<P> {
    fn as_ref(&self) -> &Coin<P> {
        self
    }
}

impl<T: Currency> Add for Coin<Imprecise<T>> {
    type Output = Coin<Imprecise<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.denom.inner().currency() != rhs.denom.inner().currency() {
            panic!("Cannot add coins of different denominations");
        }
        Coin {
            amount: self.amount + rhs.amount,
            denom: self.denom,
        }
    }
}

impl<T: Currency> Sub for Coin<Imprecise<T>> {
    type Output = Coin<Imprecise<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.denom.inner().currency() != rhs.denom.inner().currency() {
            panic!("Cannot subtract coins of different denominations");
        }
        Coin {
            amount: self.amount - rhs.amount,
            denom: self.denom,
        }
    }
}
