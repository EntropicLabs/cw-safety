#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
