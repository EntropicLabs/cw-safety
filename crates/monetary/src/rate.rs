use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{Div, Mul},
};

use cosmwasm_std::{Decimal, Fraction, StdResult, Uint128};

use crate::{AmountU128, Precise};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent, bound = ""))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
/// A rate representing A / B.
pub struct Rate<A, B>(
    Decimal,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<(A, B)>,
);

impl<A, B> Rate<A, B> {
    /// Create a new rate from a decimal.
    /// Returns None if the rate is zero.
    pub fn new(rate: Decimal) -> Option<Self> {
        if rate.is_zero() {
            None
        } else {
            Some(Rate(rate, PhantomData))
        }
    }

    /// Create a new rate from a decimal without checking if it is zero.
    ///
    /// # Safety
    /// The user ensures that the rate is non-zero. Violating this contract
    /// will lead to
    pub unsafe fn new_unchecked(rate: Decimal) -> Self {
        Rate(rate, PhantomData)
    }

    pub fn rate(&self) -> Decimal {
        self.0
    }

    pub fn inv(&self) -> Rate<B, A> {
        Rate(self.0.inv().unwrap(), PhantomData)
    }

    /// Adds a Decimal value to this. This is useful for adding a directly calculated
    /// decimal to a typed rate.
    pub fn add_decimal(&self, decimal: Decimal) -> StdResult<Rate<A, B>> {
        Ok(Rate(self.0.checked_add(decimal)?, PhantomData))
    }

    /// Subtracts a Decimal value from this. This is useful for subtracting a directly calculated
    /// decimal from a typed rate.
    pub fn sub_decimal(&self, decimal: Decimal) -> StdResult<Rate<A, B>> {
        Ok(Rate(self.0.checked_sub(decimal)?, PhantomData))
    }
}

impl<A, B> Rate<Precise<A>, Precise<B>> {
    pub fn new_precise(rate: Decimal, from: &Precise<A>, to: &Precise<B>) -> Option<Self> {
        let a_dec = from.decimals();
        let b_dec = to.decimals();
        let delta = a_dec.abs_diff(b_dec) as u32;
        let rate = match a_dec.cmp(&b_dec) {
            Ordering::Equal => rate,
            // If greater, we need to multiply by 10^delta
            Ordering::Greater => Decimal::from_ratio(
                rate.numerator() * Uint128::from(10u128.pow(delta)),
                rate.denominator(),
            ),
            // If lesser, we need to divide by 10^delta
            Ordering::Less => Decimal::from_ratio(
                rate.numerator(),
                rate.denominator() * Uint128::from(10u128.pow(delta)),
            ),
        };

        Self::new(rate)
    }
}

impl<N, S, D> Mul<Rate<S, D>> for Rate<N, S> {
    type Output = Rate<N, D>;

    fn mul(self, rhs: Rate<S, D>) -> Self::Output {
        // N / S * S / D = N / D
        // SAFETY: The product of two non-zero decimals is non-zero
        unsafe { Self::Output::new_unchecked(self.0 * rhs.0) }
    }
}

impl<N, S, D> Div<Rate<D, S>> for Rate<N, S> {
    type Output = Rate<N, D>;

    fn div(self, rhs: Rate<D, S>) -> Self::Output {
        // (N / S) / (D / S) = N / D
        // SAFETY: The division of two non-zero decimals is non-zero
        unsafe { Self::Output::new_unchecked(self.0 / rhs.0) }
    }
}

pub trait Exchange<A, B> {
    fn mul_floor(&self, rate: &Rate<B, A>) -> AmountU128<B>;
    fn mul_ceil(&self, rate: &Rate<B, A>) -> AmountU128<B>;
    fn div_floor(&self, rate: &Rate<A, B>) -> AmountU128<B>;
    fn div_ceil(&self, rate: &Rate<A, B>) -> AmountU128<B>;
}

impl<A, B> Exchange<A, B> for AmountU128<A> {
    fn mul_floor(&self, rate: &Rate<B, A>) -> AmountU128<B> {
        AmountU128::new(self.0.mul_floor(rate.0))
    }

    fn mul_ceil(&self, rate: &Rate<B, A>) -> AmountU128<B> {
        AmountU128::new(self.0.mul_ceil(rate.0))
    }

    fn div_floor(&self, rate: &Rate<A, B>) -> AmountU128<B> {
        AmountU128::new(self.0.div_floor(rate.0))
    }

    fn div_ceil(&self, rate: &Rate<A, B>) -> AmountU128<B> {
        AmountU128::new(self.0.div_ceil(rate.0))
    }
}

impl<A, B> std::fmt::Display for Rate<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::Decimal;
    use monetary_macros::denom;

    #[denom]
    pub struct A;

    #[denom]
    pub struct B;

    #[test]
    fn serialization() {
        let rate = Rate::<A, B>::new(Decimal::percent(50)).unwrap();
        let serialized = serde_json::to_string(&rate).unwrap();
        assert_eq!(serialized, r#""0.5""#);

        let deserialized: Rate<A, B> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rate, deserialized);
    }
}
