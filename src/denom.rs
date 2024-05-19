use std::marker::PhantomData;

#[cfg(all(feature = "serde", feature = "schemars"))]
pub trait Denomination<'de>:
    Copy
    + Default
    + Eq
    + PartialEq
    + serde::Serialize
    + serde::de::Deserialize<'de>
    + schemars::JsonSchema
{
}

#[cfg(all(feature = "serde", not(feature = "schemars")))]
pub trait Denomination<'de>:
    Copy + Default + Eq + PartialEq + serde::Serialize + serde::de::Deserialize<'de>
{
}

#[cfg(all(feature = "schemars", not(feature = "serde")))]
pub trait Denomination: Copy + Default + Eq + PartialEq + schemars::JsonSchema {}

#[cfg(not(any(feature = "serde", feature = "schemars")))]
pub trait Denomination: Copy + Default + Eq + PartialEq {}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Precise<T> {
    denom: T,
    decimals: u8,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Denom<T> {
    repr: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    denom: PhantomData<T>,
}

impl<T> Denom<T> {
    pub fn new(repr: String) -> Self {
        Denom {
            repr,
            denom: PhantomData,
        }
    }

    pub fn repr(&self) -> &str {
        &self.repr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct CurrencyA;
    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
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
