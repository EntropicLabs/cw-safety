use std::{cmp::Ordering, marker::PhantomData};

use cosmwasm_std::{Decimal, Fraction, Uint128};

use crate::{AmountU128, Precise};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Rate<A, B>(
    Decimal,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<(A, B)>,
);

impl<A, B> Rate<A, B> {
    pub fn new(rate: Decimal) -> Self {
        Rate(rate, PhantomData)
    }

    pub fn rate(&self) -> Decimal {
        self.0
    }

    pub fn forward_floor(&self, amount: AmountU128<A>) -> AmountU128<B> {
        AmountU128::new(amount.0.mul_floor(self.0))
    }

    pub fn forward_ceil(&self, amount: AmountU128<A>) -> AmountU128<B> {
        AmountU128::new(amount.0.mul_ceil(self.0))
    }

    pub fn reverse_floor(&self, amount: AmountU128<B>) -> AmountU128<A> {
        AmountU128::new(amount.0.div_floor(self.0))
    }

    pub fn reverse_ceil(&self, amount: AmountU128<B>) -> AmountU128<A> {
        AmountU128::new(amount.0.div_ceil(self.0))
    }
}

impl<A, B> Rate<Precise<A>, Precise<B>> {
    pub fn new_precise(rate: Decimal, from: &Precise<A>, to: &Precise<B>) -> Self {
        let a_dec = from.decimals();
        let b_dec = to.decimals();
        let rate = match a_dec.cmp(&b_dec) {
            Ordering::Equal => rate,
            // If greater, we need to divide by 10^(a-b)
            Ordering::Greater => Decimal::from_ratio(
                rate.numerator(),
                rate.denominator() * Uint128::from(10u128.pow((a_dec - b_dec).into())),
            ),
            // If lesser, we need to multiply by 10^(b-a)
            Ordering::Less => Decimal::from_ratio(
                rate.numerator() * Uint128::from(10u128.pow((b_dec - a_dec).into())),
                rate.denominator(),
            ),
        };

        Rate(rate, PhantomData)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::Decimal;

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct A;

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct B;

    #[test]
    fn serialization() {
        let rate = Rate::<A, B>::new(Decimal::percent(50));
        let serialized = serde_json::to_string(&rate).unwrap();
        assert_eq!(serialized, r#""0.5""#);

        let deserialized: Rate<A, B> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rate, deserialized);
    }
}
