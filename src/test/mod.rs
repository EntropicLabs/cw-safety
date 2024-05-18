#![cfg(test)]
use super::*;
use cosmwasm_std::{Decimal, Uint128};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct CurrencyA;
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct CurrencyB;

#[test]
fn do_something() {
    let a = CurrencyA;
    let b = CurrencyB;
    let precise_a = Precise::new(a, 6);
    let precise_b = Precise::new(b, 6);

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::<CurrencyA, CurrencyB>::new(Decimal::percent(50));
    let out = rate.forward_floor(&amount);
    let rev = rate.reverse_floor(&out);
    println!("{amount:?} {out:?} {rev:?}");

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::new_precise(Decimal::percent(50), &precise_a, &precise_b);
    let out = rate.forward_floor(&amount);
    let rev = rate.reverse_floor(&out);
    println!("{amount:?} {out:?} {rev:?}");
}
