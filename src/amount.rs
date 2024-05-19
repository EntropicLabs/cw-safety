use std::marker::PhantomData;

use cosmwasm_std::{DivideByZeroError, OverflowError, Uint128};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct AmountU128<T>(
    pub(crate) Uint128,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<T>,
);

impl<T> AmountU128<T> {
    #[inline]
    pub const fn new(amount: Uint128) -> Self {
        AmountU128(amount, PhantomData)
    }

    pub fn u128(&self) -> u128 {
        self.0.u128()
    }

    pub fn uint128(&self) -> Uint128 {
        self.0
    }

    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn checked_add(self, other: Self) -> Result<Self, OverflowError> {
        Ok(Self::new(self.0.checked_add(other.0)?))
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, OverflowError> {
        Ok(Self::new(self.0.checked_sub(other.0)?))
    }

    pub fn checked_mul(self, other: Self) -> Result<Self, OverflowError> {
        Ok(Self::new(self.0.checked_mul(other.0)?))
    }

    pub fn checked_div(self, other: Self) -> Result<Self, DivideByZeroError> {
        Ok(Self::new(self.0.checked_div(other.0)?))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_add(self, other: Self) -> Self {
        Self::new(self.0.saturating_add(other.0))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_sub(self, other: Self) -> Self {
        Self::new(self.0.saturating_sub(other.0))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_mul(self, other: Self) -> Self {
        Self::new(self.0.saturating_mul(other.0))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn abs_diff(self, other: Self) -> Self {
        Self::new(self.0.abs_diff(other.0))
    }
}

impl<T> std::ops::Add for AmountU128<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl<T> std::ops::Sub for AmountU128<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl<T> std::ops::Mul for AmountU128<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl<T> std::ops::Div for AmountU128<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}

impl<T> std::ops::AddAssign for AmountU128<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T> std::ops::SubAssign for AmountU128<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T> std::ops::MulAssign for AmountU128<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<T> std::ops::DivAssign for AmountU128<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<T> std::fmt::Display for AmountU128<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: PartialEq> PartialOrd for AmountU128<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Eq> Ord for AmountU128<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct Denom;

    #[test]
    fn serialization() {
        let a = AmountU128::<Denom>::new(12345u128.into());
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!(serialized, r#""12345""#);

        let b: AmountU128<Denom> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, b);
    }
}
