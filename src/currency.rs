// Scaffolding what the (expanded) code for a currency might look like
// A currency represents the denomination, such as USD, EUR, etc.

mod usd {
    use std::sync::OnceLock;

    use crate::Currency;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Usd(&'static str);

    static DENOM: OnceLock<String> = OnceLock::new();
    impl Usd {
        pub fn init(denom: String) -> Result<Self, String> {
            DENOM.set(denom)?;
            Ok(Self::new().unwrap())
        }

        pub fn new() -> Option<Self> {
            DENOM.get().map(|d| Self(d.as_str()))
        }
    }

    impl Currency for Usd {
        fn denom(&self) -> &'static str {
            DENOM.get().unwrap()
        }

        fn with_precision(&self, precision: u8) -> crate::Precise<Self> {
            crate::Precise::new(*self, precision)
        }

        fn without_precision(&self) -> crate::Imprecise<Self> {
            crate::Imprecise::new(*self)
        }
    }
}