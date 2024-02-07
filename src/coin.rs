use cosmwasm_std::{Coin as CwCoin, Uint128};

use crate::{Currency, Imprecise, Precise, Precision, PrecisionType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coin<P: Precision> {
    pub(crate) amount: Uint128,
    pub(crate) denom: PrecisionType<P::C>,
}

impl<P: Precision> Coin<P> {
    pub fn amount(&self) -> Uint128 {
        self.amount
    }

    pub fn currency(&self) -> &P::C {
        self.denom.currency()
    }

    pub fn denom(&self) -> &P {
        match &self.denom {
            PrecisionType::Precise(p) => {
                // This is safe because we know that the precision type is precise
                unsafe { &*(p as *const Precise<P::C> as *const P) }
            }
            PrecisionType::Imprecise(p) => {
                // This is safe because we know that the precision type is imprecise
                unsafe { &*(p as *const Imprecise<P::C> as *const P) }
            }
        }
    }
}

impl<T: Currency> Coin<Imprecise<T>> {
    pub fn from(coin: CwCoin, denom: T) -> Result<Self, String> {
        if coin.denom != denom.denom() {
            return Err(format!("Invalid denomination: {}", coin.denom));
        }
        Ok(Coin {
            amount: coin.amount,
            denom: PrecisionType::Imprecise(Imprecise { currency: denom }),
        })
    }

    pub fn with_precision(&self, precision: u8) -> Coin<Precise<T>> {
        Coin {
            amount: self.amount,
            denom: PrecisionType::Precise(Precise {
                currency: self.denom.currency().clone(),
                decimals: precision,
            }),
        }
    }
}
