use std::{cmp::Ordering, marker::PhantomData};

use cosmwasm_std::{Decimal, Fraction, Uint128};

use crate::{AmountU128, Denom, Precise};

pub struct Rate<A: Denom, B: Denom>(Decimal, PhantomData<(A, B)>);

impl<A: Denom, B: Denom> Rate<A, B> {
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

impl<A: Denom, B: Denom> Rate<Precise<A>, Precise<B>> {
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
