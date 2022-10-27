use std::fmt;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    database, entities::transaction, jwt, models::transaction as transaction_model,
    models::user as user_model,
};

#[derive(Debug)]
pub enum TransactionServiceError {
    TransactionModelFailed(transaction_model::TransactionModelError),
    UserModelFailed(user_model::UserModelError),
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

impl From<jwt::JwtError> for TransactionServiceError {
    fn from(error: jwt::JwtError) -> Self {
        TransactionServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, TransactionServiceError>;

pub fn auth_and_create_transaction(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    new_transaction: transaction::NewTransaction,
) -> Result<transaction::Transaction> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    create_transaction(conn, id, new_transaction)
}

fn create_transaction(
    conn: &database::DbPool,
    id: Uuid,
    new_transaction: transaction::NewTransaction,
) -> Result<transaction::Transaction> {
    let transaction_with_related_user = transaction::NewTransactionWithRelatedUser {
        related_user: id,
        entry_date: new_transaction.entry_date,
        entry_account_code: new_transaction.entry_account_code,
        exit_account_code: new_transaction.exit_account_code,
        amount: new_transaction.amount,
        description: new_transaction.description,
    };
    Ok(transaction_model::create_transaction(
        conn,
        transaction_with_related_user,
    )?)
}

pub fn auth_and_list_user_transactions(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<Vec<transaction::Transaction>> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(transaction_model::list_user_transactions(conn, &id)?)
}
