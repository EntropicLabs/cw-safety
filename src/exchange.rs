use std::cmp::Ordering;

use cosmwasm_std::{Decimal, Uint128};

use crate::{Coin, Currency, Exchange, Imprecise, Precise, Precision, PrecisionType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeRate<N: Precision, D: Precision> {
    pub(crate) from: N,
    pub(crate) to: D,
    /// from * rate = to
    pub(crate) rate: Decimal,
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
        let scale = Uint128::from(10u128.pow(u32::from(delta.unsigned_abs())));
        let scale = match delta.cmp(&0) {
            Ordering::Less => {
                // From has less decimals than to, so we need to scale up
                Decimal::from_ratio(scale, 1u128)
            }
            Ordering::Equal => Decimal::one(),
            Ordering::Greater => {
                // From has more decimals than to, so we need to scale down
                Decimal::from_ratio(1u128, scale)
            }
        };

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: PrecisionType::Precise(self.to.clone()),
        })
    }

    fn apply_inv(&self, from: impl AsRef<Coin<Precise<U>>>) -> Result<Coin<Precise<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);

        // We also need to scale the amount by the decimal delta.
        let delta = self.to.decimals as i16 - self.from.decimals as i16;
        let scale = Uint128::from(10u128.pow(u32::from(delta.unsigned_abs())));
        let scale = match delta.cmp(&0) {
            Ordering::Less => {
                // To has less decimals than from, so we need to scale up
                Decimal::from_ratio(scale, 1u128)
            }
            Ordering::Equal => Decimal::one(),
            Ordering::Greater => {
                // To has more decimals than from, so we need to scale down
                Decimal::from_ratio(1u128, scale)
            }
        };

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: PrecisionType::Precise(self.from.clone()),
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
            denom: PrecisionType::Precise(Precise {
                currency: self.to.currency().clone(),
                decimals: self.to.decimals,
            }),
        })
    }
    fn apply_inv(&self, from: impl AsRef<Coin<Precise<U>>>) -> Result<Coin<Imprecise<T>>, String> {
        let converted = from.as_ref().amount.div_floor(self.rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.from.currency().clone(),
            }),
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
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.to.currency().clone(),
            }),
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
            denom: PrecisionType::Precise(Precise {
                currency: self.from.currency().clone(),
                decimals: self.from.decimals,
            }),
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
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.to.currency().clone(),
            }),
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
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.from.currency().clone(),
            }),
        })
    }
}
