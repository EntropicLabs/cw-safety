use std::cmp::Ordering;

use cosmwasm_std::{Decimal, Uint128};

use crate::{Coin, Currency, Exchange, Imprecise, Precise, Precision, PrecisionSelector};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeRate<F: Precision, T: Precision> {
    pub(crate) from: F,
    pub(crate) to: T,
    /// from * rate = to
    pub(crate) rate: Decimal,
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
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn apply(&self, from: impl AsRef<Coin<Imprecise<T>>>) -> Result<Coin<Imprecise<U>>, String> {
        let converted = from.as_ref().amount.mul_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.to.clone().into_imprecise().select(),
        })
    }
    fn apply_inv(&self, from: impl AsRef<Coin<Precise<U>>>) -> Result<Coin<Imprecise<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: self.from.clone().select(),
        })
    }
}
// ExchangeRate<Precise<T>, Imprecise<U>>
impl<T: Currency, U: Currency> Exchange<Precise<T>, Imprecise<U>>
    for ExchangeRate<Precise<T>, Imprecise<U>>
{
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn apply(&self, from: impl AsRef<Coin<Precise<T>>>) -> Result<Coin<Imprecise<U>>, String> {
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
            denom: self.from.clone().into_imprecise().select(),
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
