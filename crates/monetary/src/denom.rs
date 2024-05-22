use std::marker::PhantomData;

use cosmwasm_std::{Addr, BankMsg};

use crate::{AmountU128, CheckedCoin};

/// Marker trait for a Zero-Sized-Type representing a denomination.
/// You'll likely want to implement this trait simply by declaring an empty struct,
/// and then using the `#[denom]` attribute macro on it. [`monetary_macros::denom`]
///
/// # Safety
/// This trait is marked as unsafe because it should likely only be implemented by
/// using the `#[denom]` attribute macro. Implementing this trait manually, is
/// therefore explicitly marked.
#[cfg(all(feature = "serde", feature = "schemars"))]
pub unsafe trait Denomination:
    Copy
    + Default
    + Eq
    + PartialEq
    + schemars::JsonSchema
    + serde::Serialize
    + serde::de::DeserializeOwned
{
}

#[cfg(all(feature = "serde", not(feature = "schemars")))]
pub unsafe trait Denomination: Copy + Default + Eq + PartialEq {}

#[cfg(all(feature = "schemars", not(feature = "serde")))]
pub unsafe trait Denomination:
    Copy + Default + Eq + PartialEq + schemars::JsonSchema
{
}

#[cfg(not(any(feature = "serde", feature = "schemars")))]
pub unsafe trait Denomination: Copy + Default + Eq + PartialEq {}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Precise<T> {
    denom: T,
    decimals: u8,
}

unsafe impl<T: Denomination> Denomination for Precise<T> {}

impl<T> Precise<T> {
    pub fn new(denom: T, decimals: u8) -> Self {
        Precise { denom, decimals }
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn unwrap(self) -> T {
        self.denom
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Denom<T> {
    repr: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    denom: PhantomData<T>,
}

impl<T> Denom<T> {
    pub fn new(repr: impl ToString) -> Self {
        Denom {
            repr: repr.to_string(),
            denom: PhantomData,
        }
    }

    pub fn repr(&self) -> &str {
        &self.repr
    }

    pub fn coin(&self, amount: AmountU128<T>) -> CheckedCoin<T> {
        CheckedCoin {
            denom: self.clone(),
            amount,
        }
    }

    pub fn coins(&self, amount: AmountU128<T>) -> Vec<CheckedCoin<T>> {
        vec![self.coin(amount)]
    }

    pub fn send(&self, to: &Addr, amount: AmountU128<T>) -> BankMsg {
        BankMsg::Send {
            to_address: to.to_string(),
            amount: vec![self.coin(amount).into()],
        }
    }
}

impl<T> ToString for Denom<T> {
    fn to_string(&self) -> String {
        self.repr.clone()
    }
}

impl<T> ToString for &Denom<T> {
    fn to_string(&self) -> String {
        self.repr.clone()
    }
}

impl<T> From<Denom<T>> for String {
    fn from(val: Denom<T>) -> Self {
        val.repr
    }
}

impl<T> From<&Denom<T>> for String {
    fn from(val: &Denom<T>) -> Self {
        val.repr.clone()
    }
}

impl<T> Clone for Denom<T> {
    fn clone(&self) -> Self {
        Denom {
            repr: self.repr.clone(),
            denom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use monetary_macros::denom;

    use super::*;

    #[denom]
    pub struct CurrencyA;
    #[denom]
    pub struct CurrencyB;

    #[test]
    fn denom_serialization() {
        let a = Denom::<CurrencyA>::new("uusd".to_string());
        let b = Denom::<CurrencyB>::new("ubtc".to_string());

        let json_a = serde_json::to_string(&a).unwrap();
        let json_b = serde_json::to_string(&b).unwrap();
        assert_eq!(json_a, r#""uusd""#);
        assert_eq!(json_b, r#""ubtc""#);

        let back_a: Denom<CurrencyA> = serde_json::from_str(&json_a).unwrap();
        let back_b: Denom<CurrencyB> = serde_json::from_str(&json_b).unwrap();
        assert_eq!(a, back_a);
        assert_eq!(b, back_b);
    }
}
