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

pub fn auth_and_create_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    new_account: account::NewAccount,
) -> Result<account::Account> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(account_model::create_account(conn, id, new_account)?)
}

pub fn auth_and_delete_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
) -> Result<()> {
    let _ = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(account_model::delete_account(conn, id)?)
}

pub fn auth_and_edit_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
    updated_account: account::UpdatedAccount,
) -> Result<account::Account> {
    let _ = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(account_model::edit_account(conn, id, updated_account)?)
}
