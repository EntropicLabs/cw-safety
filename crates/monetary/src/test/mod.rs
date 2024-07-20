use super::*;
use crate::Exchange;
use cosmwasm_std::{Decimal, Uint128};

#[test]
fn do_something() {
    let a = A;
    let b = B;
    let precise_a = Precise::new(a, 6);
    let precise_b = Precise::new(b, 6);

    let amount = AmountU128::new(Uint128::from(1_000_000u128));
    let rate = Rate::<A, B>::new(Decimal::percent(50)).unwrap();
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

fn oracle_rate_a() -> Rate<Usd, A> {
    Rate::new(Decimal::percent(200)).unwrap()
}

fn oracle_rate_b() -> Rate<Usd, B> {
    Rate::new(Decimal::percent(70)).unwrap()
}

#[denom]
pub struct Usd;
#[denom]
pub struct A;

#[denom]
pub struct B;

#[allow(dead_code, unused)]
#[test]
fn type_checked_currency() {
    let amount_a = AmountU128::<A>::new(Uint128::from(1_000_000u128));
    let a_rate = oracle_rate_a(); // A = $2
    let b_rate = oracle_rate_b(); // B = $0.7

    // Calculate rate from A to B
    let rate_a_to_b = b_rate.inv() * a_rate;

    // Convert amount from A to B
    let amount_b = amount_a.mul_floor(&rate_a_to_b);
    assert_eq!(
        amount_b.uint128(),
        Uint128::from(1_000_000u128).mul_floor(Decimal::from_ratio(200u128, 70u128))
    )
}
