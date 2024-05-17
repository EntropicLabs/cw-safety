use std::marker::PhantomData;

use cosmwasm_std::Uint128;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Denom;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmountU128<T: Denom>(
    pub(crate) Uint128,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<T>,
);

impl<T: Denom> AmountU128<T> {
    pub fn new(amount: Uint128) -> Self {
        AmountU128(amount, PhantomData)
    }

    pub fn unwrap(self) -> Uint128 {
        self.0
    }

    pub fn u128(&self) -> u128 {
        self.0.u128()
    }
}

impl<T: Denom + Clone> Copy for AmountU128<T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Denom(pub String);
    impl crate::Denom for Denom {
        fn denom(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn serialization() {
        let a = AmountU128::<Denom>::new(12345u128.into());
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!(serialized, r#""12345""#);

        let b: AmountU128<Denom> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, b);
    }
}
