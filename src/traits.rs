use std::fmt::Debug;

use crate::Coin;

pub trait Currency: Clone + Debug + PartialEq + Eq {
    fn denom(&self) -> &str;
}

pub trait Precision {
    type C: Currency;

    fn currency(&self) -> &Self::C;
}

pub trait Exchange<P: Precision, Q: Precision> {
    type OutPrecision<C: Currency>: Precision<C = C>;
    fn apply(&self, from: impl AsRef<Coin<P>>) -> Result<Coin<Self::OutPrecision<Q::C>>, String>;
    fn apply_inv(
        &self,
        from: impl AsRef<Coin<Q>>,
    ) -> Result<Coin<Self::OutPrecision<P::C>>, String>;
}
