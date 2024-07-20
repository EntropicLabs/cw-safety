mod amount;
mod coin;
mod denom;
mod error;
mod payment;
mod rate;

pub use amount::*;
pub use coin::*;
pub use denom::*;
pub use error::*;
pub use payment::*;
pub use rate::*;

pub use monetary_macros::*;

pub mod __derive_import {
    pub use schemars;
    pub use serde;
}

#[cfg(test)]
mod test;

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
