use std::fmt;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    database, entities::transaction, jwt, models::account as account_model,
    models::transaction as transaction_model, models::user as user_model, utils::invert,
};

#[derive(Debug)]
pub enum TransactionServiceError {
    TransactionModelFailed(transaction_model::TransactionModelError),
    UserModelFailed(user_model::UserModelError),
    AccountModelFailed(account_model::AccountModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for TransactionServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<transaction_model::TransactionModelError> for TransactionServiceError {
    fn from(error: transaction_model::TransactionModelError) -> Self {
        TransactionServiceError::TransactionModelFailed(error)
    }
}

impl From<user_model::UserModelError> for TransactionServiceError {
    fn from(error: user_model::UserModelError) -> Self {
        TransactionServiceError::UserModelFailed(error)
    }
}

impl From<account_model::AccountModelError> for TransactionServiceError {
    fn from(error: account_model::AccountModelError) -> Self {
        TransactionServiceError::AccountModelFailed(error)
    }
}

impl From<jwt::JwtError> for TransactionServiceError {
    fn from(error: jwt::JwtError) -> Self {
        TransactionServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, TransactionServiceError>;

fn fill_name(
    conn: &database::DbPool,
    user_id: &Uuid,
    transaction: &transaction::Transaction,
) -> Result<transaction::TransactionWithNames> {
    let entry_account = transaction
        .entry_account_code
        .map(|code| account_model::get_account(conn, &code, user_id));
    let exit_account = transaction
        .exit_account_code
        .map(|code| account_model::get_account(conn, &code, user_id));
    let entry_account_name = invert(entry_account)?.map(|x| x.name);
    let exit_account_name = invert(exit_account)?.map(|x| x.name);
    Ok(transaction.with_names(entry_account_name, exit_account_name))
}

pub fn auth_and_create_transaction(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    new_transaction: transaction::NewTransaction,
) -> Result<transaction::TransactionWithNames> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let created_transaction = transaction_model::create_transaction(conn, &id, new_transaction)?;
    fill_name(conn, &id, &created_transaction)
}

pub fn auth_and_list_user_transactions(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<Vec<transaction::TransactionWithNames>> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let transactions = transaction_model::list_user_transactions(conn, &id)?;
    transactions
        .into_iter()
        .map(|t| fill_name(conn, &id, &t))
        .collect()
}
