pub trait Denom {
    fn denom(&self) -> &str;

    fn with_decimals(self, decimals: u8) -> Precise<Self>
    where
        Self: Sized,
    {
        Precise::new(self, decimals)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Precise<T: Denom> {
    denom: T,
    decimals: u8,
}

impl<T: Denom> Precise<T> {
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

impl<T: Denom> Denom for Precise<T> {
    fn denom(&self) -> &str {
        self.denom.denom()
    }
}
