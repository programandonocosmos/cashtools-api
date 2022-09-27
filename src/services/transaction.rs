use std::fmt;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{database, jwt, models::transaction, models::user};

pub struct Transaction {
    pub id: Uuid,
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

pub struct NewTransaction {
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

pub struct NewTransactionWithRelatedUser {
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum TransactionServiceError {
    TransactionModelFailed(transaction::TransactionModelError),
    UserModelFailed(user::UserModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for TransactionServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<transaction::TransactionModelError> for TransactionServiceError {
    fn from(error: transaction::TransactionModelError) -> Self {
        TransactionServiceError::TransactionModelFailed(error)
    }
}

impl From<user::UserModelError> for TransactionServiceError {
    fn from(error: user::UserModelError) -> Self {
        TransactionServiceError::UserModelFailed(error)
    }
}

impl From<jwt::JwtError> for TransactionServiceError {
    fn from(error: jwt::JwtError) -> Self {
        TransactionServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, TransactionServiceError>;

pub fn create_transaction(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    new_transaction: NewTransaction,
) -> Result<Transaction> {
    let email = jwt::verify_token(token, jwt_secret)?;
    let id = user::get_id_by_email(conn, &email)?;
    let transaction_with_related_user = NewTransactionWithRelatedUser {
        related_user: id,
        entry_date: new_transaction.entry_date,
        entry_account_code: new_transaction.entry_account_code,
        exit_account_code: new_transaction.exit_account_code,
        amount: new_transaction.amount,
        description: new_transaction.description,
    };
    Ok(transaction::create_transaction(
        conn,
        transaction_with_related_user,
    )?)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<Vec<Transaction>> {
    let email = jwt::verify_token(token, jwt_secret)?;
    let id = user::get_id_by_email(conn, &email)?;
    Ok(transaction::list_user_transactions(conn, &id)?)
}
