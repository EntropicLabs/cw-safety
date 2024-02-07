use std::marker::PhantomData;

use crate::{Currency, Precision};

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with verified decimal precision information
pub struct Precise<T> {
    pub(crate) currency: T,
    pub(crate) decimals: u8,
}
impl<T: Currency> Precision for Precise<T> {
    type C = T;

    fn currency(&self) -> &T {
        &self.currency
    }
}

impl<T: Currency> Precise<T> {
    pub fn into_imprecise(self) -> Imprecise<T> {
        Imprecise {
            currency: self.currency,
        }
    }

    pub fn into_unverified(self) -> Unverified<T> {
        Unverified {
            currency: self.currency,
            decimals: self.decimals,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with no decimal precision information
pub struct Imprecise<T: Currency> {
    pub(crate) currency: T,
}
impl<T: Currency> Precision for Imprecise<T> {
    type C = T;
    fn currency(&self) -> &T {
        &self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with unverified decimal precision information
pub struct Unverified<T: Currency> {
    pub(crate) currency: T,
    pub(crate) decimals: u8,
}
impl<T: Currency> Precision for Unverified<T> {
    type C = T;
    fn currency(&self) -> &T {
        &self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PrecisionType<T: Currency> {
    Precise(Precise<T>),
    Imprecise(Imprecise<T>),
    Unverified(Unverified<T>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PrecisionTypeWrapper<P: Precision>(PrecisionType<P::C>, PhantomData<P>);

impl<P: Precision> PrecisionTypeWrapper<P> {
    pub(crate) fn inner(&self) -> &PrecisionType<P::C> {
        &self.0
    }
}

pub(crate) trait PrecisionSelector<P: Precision> {
    fn select(self) -> PrecisionTypeWrapper<P>;
}

impl<T: Currency> PrecisionSelector<Precise<T>> for Precise<T> {
    fn select(self) -> PrecisionTypeWrapper<Precise<T>> {
        PrecisionTypeWrapper(PrecisionType::Precise(self), PhantomData)
    }
}

impl<T: Currency> PrecisionSelector<Imprecise<T>> for Imprecise<T> {
    fn select(self) -> PrecisionTypeWrapper<Imprecise<T>> {
        PrecisionTypeWrapper(PrecisionType::Imprecise(self), PhantomData)
    }
}

impl<T: Currency> PrecisionSelector<Unverified<T>> for Unverified<T> {
    fn select(self) -> PrecisionTypeWrapper<Unverified<T>> {
        PrecisionTypeWrapper(PrecisionType::Unverified(self), PhantomData)
    }
}

impl<T: Currency> PrecisionType<T> {
    pub fn currency(&self) -> &T {
        match self {
            PrecisionType::Precise(p) => p.currency(),
            PrecisionType::Imprecise(p) => p.currency(),
            PrecisionType::Unverified(p) => p.currency(),
        }
    }
}
