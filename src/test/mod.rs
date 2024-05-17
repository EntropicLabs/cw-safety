#![cfg(test)]
use super::*;
use cosmwasm_std::{Decimal, Uint128};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyA(pub String);
impl Denom for CurrencyA {
    fn denom(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyB(pub String);
impl Denom for CurrencyB {
    fn denom(&self) -> &str {
        &self.0
    }
}

#[test]
fn do_something() {
    let a = CurrencyA("A".to_string());
    let b = CurrencyB("B".to_string());
    let precise_a = a.with_decimals(6);
    let precise_b = b.with_decimals(6);

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::<CurrencyA, CurrencyB>::new(Decimal::percent(50));
    let out = rate.forward_floor(amount);
    let rev = rate.reverse_floor(out);
    println!("{amount:?} {out:?} {rev:?}");

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::new_precise(Decimal::percent(50), &precise_a, &precise_b);
    let out = rate.forward_floor(amount);
    let rev = rate.reverse_floor(out);
    println!("{amount:?} {out:?} {rev:?}");
}
