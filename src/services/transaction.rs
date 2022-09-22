use std::fmt;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{database, models::transaction};

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

pub fn create_transaction(
    conn: &database::DbPool,
    t: NewTransaction,
) -> Result<Transaction, TransactionServiceError> {
    Ok(transaction::create_transaction(conn, t)?)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    user_id: Uuid,
) -> Result<Vec<Transaction>, TransactionServiceError> {
    Ok(transaction::list_user_transactions(conn, user_id)?)
}
