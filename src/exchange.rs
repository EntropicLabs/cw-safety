use std::cmp::Ordering;

use cosmwasm_std::{Decimal, Fraction, Uint128};

use crate::{Coin, Currency, Exchange, Imprecise, Precise, Precision, PrecisionType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeRate<N: Precision, D: Precision> {
    pub(crate) numerator: N,
    pub(crate) denominator: D,
    pub(crate) rate: Decimal,
}

// ExchangeRate<Precise<T>, Precise<U>>
impl<T: Currency, U: Currency> Exchange<Precise<T>, Precise<U>>
    for ExchangeRate<Precise<T>, Precise<U>>
{
    type OutPrecision<C: Currency> = Precise<C>;
    fn mul(&self, from: Coin<Precise<U>>) -> Result<Coin<Precise<T>>, String> {
        let rate = self.rate;
        let converted = from.amount.mul_floor(rate);

        // We also need to scale the amount by the decimal delta.
        let delta = self.numerator.decimals as i16 - self.denominator.decimals as i16;
        let scale = Uint128::from(10u128.pow(u32::from(delta.unsigned_abs())));
        let scale = match delta.cmp(&0) {
            Ordering::Less => {
                // Denominator has more decimals than numerator, so we need to scale down
                Decimal::from_ratio(1u128, scale)
            }
            Ordering::Equal => {
                // No precision change
                Decimal::one()
            }
            Ordering::Greater => {
                // Numerator has more decimals than denominator, so we need to scale up
                Decimal::from_ratio(scale, 1u128)
            }
        };

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: PrecisionType::Precise(self.numerator.clone()),
        })
    }

    fn div(&self, from: Coin<Precise<T>>) -> Result<Coin<Precise<U>>, String> {
        let rate = self.rate.inv().ok_or("Cannot invert zero rate")?;
        let converted = from.amount.mul_floor(rate);

        // We also need to scale the amount by the decimal delta.
        let delta = self.numerator.decimals as i16 - self.denominator.decimals as i16;
        let scale = Uint128::from(10u128.pow(u32::from(delta.unsigned_abs())));
        let scale = match delta.cmp(&0) {
            Ordering::Less => {
                // Denominator has more decimals than numerator, so we need to scale up
                Decimal::from_ratio(scale, 1u128)
            }
            Ordering::Equal => {
                // No precision change
                Decimal::one()
            }
            Ordering::Greater => {
                // Numerator has more decimals than denominator, so we need to scale down
                Decimal::from_ratio(1u128, scale)
            }
        };

        let new_amount = converted * scale;
        Ok(Coin {
            amount: new_amount,
            denom: PrecisionType::Precise(self.denominator.clone()),
        })
    }
}
// ExchangeRate<Imprecise<T>, Precise<U>>
impl<T: Currency, U: Currency> Exchange<Imprecise<T>, Precise<U>>
    for ExchangeRate<Imprecise<T>, Precise<U>>
{
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn mul(&self, from: Coin<Precise<U>>) -> Result<Coin<Imprecise<T>>, String> {
        let rate = self.rate;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.numerator.currency().clone(),
            }),
        })
    }

    fn div(&self, from: Coin<Imprecise<T>>) -> Result<Coin<Imprecise<U>>, String> {
        let rate = self.rate.inv().ok_or("Cannot invert zero rate")?;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.denominator.currency().clone(),
            }),
        })
    }
}
// ExchangeRate<Precise<T>, Imprecise<U>>
impl<T: Currency, U: Currency> Exchange<Precise<T>, Imprecise<U>>
    for ExchangeRate<Precise<T>, Imprecise<U>>
{
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn mul(&self, from: Coin<Imprecise<U>>) -> Result<Coin<Imprecise<T>>, String> {
        let rate = self.rate;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.numerator.currency().clone(),
            }),
        })
    }

    fn div(&self, from: Coin<Precise<T>>) -> Result<Coin<Imprecise<U>>, String> {
        let rate = self.rate.inv().ok_or("Cannot invert zero rate")?;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.denominator.currency().clone(),
            }),
        })
    }
}
// ExchangeRate<Imprecise<T>, Imprecise<U>>
impl<T: Currency, U: Currency> Exchange<Imprecise<T>, Imprecise<U>>
    for ExchangeRate<Imprecise<T>, Imprecise<U>>
{
    type OutPrecision<C: Currency> = Imprecise<C>;
    fn mul(&self, from: Coin<Imprecise<U>>) -> Result<Coin<Imprecise<T>>, String> {
        let rate = self.rate;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.numerator.currency().clone(),
            }),
        })
    }

    fn div(&self, from: Coin<Imprecise<T>>) -> Result<Coin<Imprecise<U>>, String> {
        let rate = self.rate.inv().ok_or("Cannot invert zero rate")?;
        let converted = from.amount.mul_floor(rate);
        // No precision change since we don't know the output precision
        Ok(Coin {
            amount: converted,
            denom: PrecisionType::Imprecise(Imprecise {
                currency: self.denominator.currency().clone(),
            }),
        })
    }
}
