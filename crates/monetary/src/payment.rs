use cosmwasm_std::{MessageInfo, Uint128};

use crate::{AmountU128, CheckedCoin, Denom, MonetaryError};

/// Requires exactly one denom sent, which matches the requested denom.
/// Returns the amount if only one denom and non-zero amount. Errors otherwise.
pub fn must_pay<T>(info: &MessageInfo, denom: &Denom<T>) -> Result<AmountU128<T>, MonetaryError> {
    if info.funds.len() != 1 {
        return Err(MonetaryError::DenomNotFound(denom.to_string()));
    }

    let coin = &info.funds[0];
    if coin.amount.is_zero() || coin.denom != denom.repr() {
        return Err(MonetaryError::DenomNotFound(denom.to_string()));
    }

    Ok(AmountU128::new(coin.amount))
}

/// Similar to must_pay, but it any payment is optional. Returns an error if a different
/// denom was sent. Otherwise, returns the amount of `denom` sent, or 0 if nothing sent.
pub fn may_pay<T>(info: &MessageInfo, denom: &Denom<T>) -> Result<AmountU128<T>, MonetaryError> {
    if info.funds.is_empty() {
        Ok(AmountU128::new(Uint128::zero()))
    } else if info.funds.len() == 1 {
        if info.funds[0].denom == denom.repr() {
            Ok(AmountU128::new(info.funds[0].amount))
        } else {
            Err(MonetaryError::DenomNotFound(denom.to_string()))
        }
    } else {
        Err(MonetaryError::TooManyDenoms {})
    }
}

pub fn coin<T>(amount: impl Into<Uint128>, denom: &Denom<T>) -> CheckedCoin<T> {
    CheckedCoin::new(denom.clone(), AmountU128::new(amount.into()))
}

pub fn coins<T>(amount: impl Into<Uint128>, denom: &Denom<T>) -> Vec<CheckedCoin<T>> {
    vec![coin(amount, denom)]
}

#[cfg(test)]
mod test {
    use crate::denom;

    use super::*;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::{coin, coins};

    const SENDER: &str = "sender";

    #[denom]
    pub struct Atom;

    #[test]
    fn may_pay_works() {
        let atom: Denom<Atom> = Denom::new("uatom");
        let no_payment = mock_info(SENDER, &[]);
        let atom_payment = mock_info(SENDER, &coins(100, &atom));
        let eth_payment = mock_info(SENDER, &coins(100, "wei"));
        let mixed_payment = mock_info(SENDER, &[coin(50, &atom), coin(120, "wei")]);

        let res = may_pay(&no_payment, &atom).unwrap();
        assert_eq!(res, AmountU128::zero());

        let res = may_pay(&atom_payment, &atom).unwrap();
        assert_eq!(res, AmountU128::new(100u128.into()));

        let err = may_pay(&eth_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::DenomNotFound("uatom".to_string()));

        let err = may_pay(&mixed_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::TooManyDenoms {});
    }

    #[test]
    fn must_pay_works() {
        let atom: Denom<Atom> = Denom::new("uatom");
        let no_payment = mock_info(SENDER, &[]);
        let atom_payment = mock_info(SENDER, &coins(100, &atom));
        let zero_payment = mock_info(SENDER, &coins(0, &atom));
        let eth_payment = mock_info(SENDER, &coins(100, "wei"));
        let mixed_payment = mock_info(SENDER, &[coin(50, &atom), coin(120, "wei")]);

        let res = must_pay(&atom_payment, &atom).unwrap();
        assert_eq!(res, AmountU128::new(100u128.into()));

        let err = must_pay(&no_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::DenomNotFound("uatom".to_string()));

        let err = must_pay(&zero_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::DenomNotFound("uatom".to_string()));

        let err = must_pay(&eth_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::DenomNotFound("uatom".to_string()));

        let err = must_pay(&mixed_payment, &atom).unwrap_err();
        assert_eq!(err, MonetaryError::DenomNotFound("uatom".to_string()));
    }
}
