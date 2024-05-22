use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MonetaryError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Denomination mismatch: {0} != {1}")]
    DenomMismatch(String, String),
    #[error("Too many denoms")]
    TooManyDenoms {},
    #[error("Denom not found: {0}")]
    DenomNotFound(String),
}
