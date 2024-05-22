#![cfg(test)]
use super::*;
use cosmwasm_std::{Decimal, Uint128};

#[test]
fn do_something() {
    let a = CurrencyA;
    let b = CurrencyB;
    let precise_a = Precise::new(a, 6);
    let precise_b = Precise::new(b, 6);

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::<CurrencyA, CurrencyB>::new(Decimal::percent(50)).unwrap();
    let out = amount.div_floor(&rate);
    let rev = out.mul_floor(&rate);
    println!("{amount:?} {out:?} {rev:?}");
    assert_eq!(amount, rev);

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::new_precise(Decimal::percent(50), &precise_a, &precise_b).unwrap();
    let out = amount.div_floor(&rate);
    let rev = out.mul_floor(&rate);
    println!("{amount:?} {out:?} {rev:?}");
    assert_eq!(amount, rev);
}

#[denom]
pub struct CurrencyA;

#[denom]
pub struct CurrencyB;
