use maturity_macro::maturity;

use crate::utilities::logger::error::LogError;

#[maturity]
pub type LogResult<T> = Result<T, LogError>;
