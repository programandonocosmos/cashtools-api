use std::fmt;

use uuid::Uuid;

use crate::{
    database,
    models::transaction::{self, Transaction},
};

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

fn create_transaction(
    conn: &database::DbPool,
    t: Transaction,
) -> Result<Transaction, TransactionServiceError> {
    Ok(transaction::create_transaction(conn, t)?)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    user_id: Uuid,
) -> Result<Vec<Transaction>, TransactionServiceError> {
    Ok(transaction::list_user_transactions(conn, user_id)?)
}
