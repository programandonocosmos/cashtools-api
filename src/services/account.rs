use std::fmt;

use chrono::{NaiveDateTime, Utc};
use rand::Rng;

use uuid::Uuid;

use crate::{
    database,
    entities::{account, Env},
    jwt,
    models::account as account_model,
};

#[derive(Debug)]
pub enum AccountServiceError {
    AccountModelFailed(account_model::AccountModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for AccountServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<account_model::AccountModelError> for AccountServiceError {
    fn from(error: account_model::AccountModelError) -> Self {
        AccountServiceError::AccountModelFailed(error)
    }
}

impl From<jwt::JwtError> for AccountServiceError {
    fn from(error: jwt::JwtError) -> Self {
        AccountServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, AccountServiceError>;

pub fn create_account(
    conn: &database::DbPool,
    new_account: account::NewAccount,
) -> Result<account::Account> {
    Ok(account_model::create_account(conn, new_account)?)
}
