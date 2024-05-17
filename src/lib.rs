mod coin;
mod currency;
mod exchange;
mod precision;
mod traits;

pub use coin::*;
pub use exchange::*;
pub use precision::*;
pub use traits::*;

#[macro_export]
// macro_rules! make_denom {
//     ($name:ident) => {
//         impl $crate::Currency for $name {
//             fn denom(&self) -> &str {
//                 &self.0
//             }
//             fn with_precision(&self, precision: u8) -> $crate::Precise<$name> {
//                 $crate::Precise::new(self.clone(), precision)
//             }

//             fn without_precision(&self) -> $crate::Imprecise<$name> {
//                 $crate::Imprecise::new(self.clone())
//             }
//         }

//         impl $name {
//             pub fn as_unknown(&self) -> $crate::Unknown {
//                 $crate::Unknown(self.0.clone())
//             }
//         }

//         impl From<$name> for $crate::Imprecise<$name> {
//             fn from(currency: $name) -> $crate::Imprecise<$name> {
//                 $crate::Imprecise::new(currency)
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! make_denom {
    ($name:ident, $denom:expr) => {
        mod $name {
            use std::sync::OnceLock;

            use $crate::Currency;

            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $name(&'static str);

            static DENOM: OnceLock<String> = OnceLock::new();
            impl $name {
                pub fn init(denom: String) -> Result<Self, String> {
                    DENOM.set(denom)?;
                    Ok(Self::new().unwrap())
                }

                pub fn new() -> Option<Self> {
                    DENOM.get().map(|d| Self(d.as_str()))
                }
            }

            impl Currency for $name {
                fn denom(&self) -> &'static str {
                    DENOM.get().unwrap()
                }

                fn with_precision(&self, precision: u8) -> $crate::Precise<Self> {
                    $crate::Precise::new(*self, precision)
                }

                fn without_precision(&self) -> $crate::Imprecise<Self> {
                    $crate::Imprecise::new(*self)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! make_static_denom {
    ($name:ident, $denom: expr) => {
        impl $crate::Currency for $name {
            fn denom(&self) -> &str {
                $denom
            }
            fn with_precision(&self, precision: u8) -> $crate::Precise<$name> {
                $crate::Precise::new($name, precision)
            }
            fn without_precision(&self) -> $crate::Imprecise<$name> {
                $crate::Imprecise::new($name)
            }
        }

        impl $name {
            pub fn as_unknown(&self) -> $crate::Unknown {
                $crate::Unknown($denom.to_string())
            }
        }

        impl From<$name> for $crate::Imprecise<$name> {
            fn from(_: $name) -> $crate::Imprecise<$name> {
                $crate::Imprecise::new($name)
            }
        }
    };
}

#[derive(
    Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema,
)]
pub struct Unknown(pub String);
make_denom!(Unknown);

#[derive(
    Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema,
)]
pub struct Empty;
make_static_denom!(Empty, "");

/*
Here's the problem:
We have many different representations of currencies, and even different vocabulary for when we talk about them.

For example, a denomination, or coin, might be:
* A string representing the SDK denomination (e.g. "uusd")
* A coin representing a specific amount of a specific denomination (e.g. 1234uusd)
* A string and number of decimal places (e.g. "uusd" and 6)

We want to differentiate between all these types, whilst adding logical operations between them.
The end goal is to leverage the type system to prevent bugs at compile time, whilst also making the code
more readable.

In an effort to do so, let's define a couple of terms:
1. Currency: A type that represents which asset type, or which "currency" we are talking about.
   for example, "uusd", "ukuji", or other custom token currencies like "ibc/..." or "factory/..."
2. Precise<Currency>: A type that represents a precision and currency type. For example, "uusd" with 6 decimal places.
   Coins on chain have no intrinsic knowledge of their precision, so this would be a type that is defined in
   contract configuration (or code).
3. Imprecise<Currency>: A type that represents a raw amount of a currency, with no precision information.
   This is the raw data we get from the chain, and we need to interpret it using a Denomination to get a Coin.
3. Coin: an amount (u128) along with either a Precise<Currency> or Imprecise<Currency> to represent the currency type.

The Precise<T> type is a sort of "safety" net to ensure that we never run into precision issues, which are common if
not handled properly, and can lead to serious loss of funds. Thus, operations involving two Precise types will always
result in another Precise type, and should be safe, whilst operations involving any Imprecise types anywhere will always
be unchecked and potentially unsafe, and thus result in another Imprecise type.

We will, obviously, need a way to add precision information to an Imprecise type. We can manually add precision to an
Imprecise type, which will give us a Precise type. This process is called "hydrating" the imprecise type.

Now let's move on to some implementation details. Since the Coin type abstracts over both Precise and Imprecise types, we
will need to implement a trait on Imprecise and Precise that allow them to be used interchangeably within the Coin type,
but only when it makes absolute sense to do so.

Since we want to have compile-time safety, we also need to define the possible currencies that we can use beforehand.
We can accomplish this by making Currency a trait, and then creating a macro that generates some NewType wrappers around
strings that implement the Currency trait.

The oracle module in the chain has no knowledge of precision, since it works with Currencies only. Thus, the oracle query
methods will all return Imprecise<Currency> types. An example signature is here:

fn query_exchange_rate<T: Currency>(denom: impl AsRef<T>) -> StdResult<ExchangeRate<Imprecise<T>, Imprecise<USD>>>;
*/

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Decimal};

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct USD;
    make_static_denom!(USD, "uusd");

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct EUR;
    make_static_denom!(EUR, "ueur");

    fn get_exchange_rate<T: Currency>(denom: Precise<T>) -> ExchangeRate<Precise<T>, Precise<USD>> {
        // In an actual implementation, query oracle:
        // let actual_rate = deps.querier.query_exchange_rate(denom)?;
        let precision = denom.decimals;
        ExchangeRate {
            from: denom,
            to: USD.with_precision(precision),
            rate: Decimal::percent(108), // EUR is at $1.08, for example
        }
    }

    #[test]
    fn type_checked_currencies() {
        let test_coin: cosmwasm_std::Coin = coin(1_000_000u128, "ueur");

        // Either:
        let _amount = Coin::imprecise(&test_coin, &EUR).expect("Coin should be EUR");
        // Or:
        let eur = EUR.with_precision(6);
        let amount = Coin::precise(&test_coin, &eur).expect("Coin should be EUR");

        let rate: ExchangeRate<Precise<EUR>, Precise<USD>> = get_exchange_rate(eur); // EUR/USD = 1.08
        let converted = rate.apply(amount).expect("Conversion should work");
        let converted_back = rate.apply_inv(&converted).expect("Conversion should work");

        assert_eq!(
            converted.amount().u128(),
            1_080_000u128,
            "EUR to USD output incorrect"
        );
        assert_eq!(
            converted_back.amount().u128(),
            1_000_000u128,
            "USD to EUR output incorrect"
        );
    }
}
