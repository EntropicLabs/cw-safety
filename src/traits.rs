use crate::Coin;

pub trait Currency: Clone {
    fn denom(&self) -> &str;
}

pub trait Precision {
    type C: Currency;
    // type Precision;

    fn currency(&self) -> &Self::C;
}

pub trait Exchange<P: Precision, Q: Precision> {
    type OutPrecision<C: Currency>: Precision<C = C>;
    fn mul(&self, from: Coin<Q>) -> Result<Coin<Self::OutPrecision<P::C>>, String>;
    fn div(&self, from: Coin<P>) -> Result<Coin<Self::OutPrecision<Q::C>>, String>;
}
