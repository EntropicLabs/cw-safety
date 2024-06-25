use std::marker::PhantomData;

use cosmwasm_std::{Decimal, DivideByZeroError, OverflowError, Uint128};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

    #[inline]
    pub const fn u128(&self) -> u128 {
        self.0.u128()
    }

    #[inline]
    pub const fn uint128(&self) -> Uint128 {
        self.0
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new(Uint128::zero())
    }

    #[must_use]
    #[inline]
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

    pub fn dec_mul_floor(self, dec: Decimal) -> Self {
        Self::new(self.0.mul_floor(dec))
    }

    pub fn dec_mul_ceil(self, dec: Decimal) -> Self {
        Self::new(self.0.mul_ceil(dec))
    }

    pub fn dec_div_floor(self, dec: Decimal) -> Result<Self, DivideByZeroError> {
        Ok(Self::new(self.0.div_floor(dec)))
    }

    pub fn dec_div_ceil(self, dec: Decimal) -> Result<Self, DivideByZeroError> {
        Ok(Self::new(self.0.div_ceil(dec)))
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

impl<T> From<AmountU128<T>> for String {
    fn from(val: AmountU128<T>) -> Self {
        val.0.to_string()
    }
}

impl<T: PartialEq> PartialOrd for AmountU128<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T: Eq> Ord for AmountU128<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Default> Default for AmountU128<T> {
    fn default() -> Self {
        Self::new(Uint128::default())
    }
}

#[cfg(test)]
mod test {
    use monetary_macros::denom;

    use crate::AmountU128;

    #[denom]
    pub struct Denom;

    #[test]
    fn serialization() {
        let a = AmountU128::<Denom>::new(12345u128.into());
        let serialized = serde_json_wasm::to_string(&a).unwrap();
        assert_eq!(serialized, r#""12345""#);

        let b: AmountU128<Denom> = serde_json_wasm::from_str(&serialized).unwrap();
        assert_eq!(a, b);
    }
}
