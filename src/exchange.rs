use std::cmp::Ordering;

use cosmwasm_std::{Decimal, Uint128};

use crate::{
    Coin, Currency, Exchange, Imprecise, Precise, Precision, PrecisionSelector, Unverified,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeRate<F: Precision, T: Precision> {
    pub(crate) from: F,
    pub(crate) to: T,
    /// from * rate = to
    pub(crate) rate: Decimal,
}

impl<F: Precision, T: Precision> ExchangeRate<F, T> {
    pub fn new(from: F, to: T, rate: Decimal) -> Option<Self> {
        if rate.is_zero() {
            return None;
        }
        Some(Self { from, to, rate })
    }

    pub fn from(&self) -> &F {
        &self.from
    }

    pub fn to(&self) -> &T {
        &self.to
    }

    pub fn rate(&self) -> Decimal {
        self.rate
    }
}

#[inline]
fn scale(delta: i16) -> Decimal {
    let scale = Uint128::from(10u128.pow(u32::from(delta.unsigned_abs())));
    match delta.cmp(&0) {
        Ordering::Less => {
            // From has less decimals than to, so we need to scale up
            Decimal::from_ratio(scale, 1u128)
        }
        Ordering::Equal => Decimal::one(),
        Ordering::Greater => {
            // From has more decimals than to, so we need to scale down
            Decimal::from_ratio(1u128, scale)
        }
    }
}

// ExchangeRate<Precise<T>, Precise<U>>
impl<T: Currency, U: Currency> Exchange<Precise<T>, Precise<U>>
    for ExchangeRate<Precise<T>, Precise<U>>
{
    type OutPrecision<C: Currency> = Precise<C>;

    fn apply(&self, from: impl AsRef<Coin<Precise<T>>>) -> Result<Coin<Precise<U>>, String> {
        let converted = from.as_ref().amount.mul_floor(self.rate);

        // We also need to scale the amount by the decimal delta.
        let delta = self.from.decimals as i16 - self.to.decimals as i16;
        let scale = scale(delta);

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: self.to.clone().select(),
        })
    }

    fn apply_inv(&self, from: impl AsRef<Coin<Precise<U>>>) -> Result<Coin<Precise<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);

        // We also need to scale the amount by the decimal delta.
        let delta = self.to.decimals as i16 - self.from.decimals as i16;
        let scale = scale(delta);

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: self.from.clone().select(),
        })
    }
}

// ExchangeRate<Imprecise<T>, Precise<U>>
impl<T: Currency, U: Currency> Exchange<Imprecise<T>, Precise<U>>
    for ExchangeRate<Imprecise<T>, Precise<U>>
{
    type OutPrecision<C: Currency> = Unverified<C>;
    fn apply(&self, from: impl AsRef<Coin<Imprecise<T>>>) -> Result<Coin<Unverified<U>>, String> {
        let converted = from.as_ref().amount.mul_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.to.clone().into_unverified().select(),
        })
    }
    fn apply_inv(&self, from: impl AsRef<Coin<Precise<U>>>) -> Result<Coin<Unverified<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.from.clone().into_unverified(self.to.decimals).select(),
        })
    }
}
// ExchangeRate<Precise<T>, Imprecise<U>>
impl<T: Currency, U: Currency> Exchange<Precise<T>, Imprecise<U>>
    for ExchangeRate<Precise<T>, Imprecise<U>>
{
    type OutPrecision<C: Currency> = Unverified<C>;
    fn apply(&self, from: impl AsRef<Coin<Precise<T>>>) -> Result<Coin<Unverified<U>>, String> {
        let converted = from.as_ref().amount.mul_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.to.clone().into_unverified(self.from.decimals).select(),
        })
    }
    fn apply_inv(
        &self,
        from: impl AsRef<Coin<Imprecise<U>>>,
    ) -> Result<Coin<Unverified<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.from.clone().into_unverified().select(),
        })
    }
}
// ExchangeRate<Imprecise<T>, Imprecise<U>>
impl<T: Currency, U: Currency> Exchange<Imprecise<T>, Imprecise<U>>
    for ExchangeRate<Imprecise<T>, Imprecise<U>>
{
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn apply(&self, from: impl AsRef<Coin<Imprecise<T>>>) -> Result<Coin<Imprecise<U>>, String> {
        let converted = from.as_ref().amount.mul_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.to.clone().select(),
        })
    }
    fn apply_inv(
        &self,
        from: impl AsRef<Coin<Imprecise<U>>>,
    ) -> Result<Coin<Imprecise<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.from.clone().select(),
        })
    }
}
