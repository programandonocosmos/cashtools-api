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

fn create_transaction(
    conn: &database::DbPool,
    t: Transaction,
) -> Result<Transaction, TransactionServiceError> {
    transaction::create_transaction(conn, t)
        .map_err(TransactionServiceError::TransactionModelFailed)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    user_id: Uuid,
) -> Result<Vec<Transaction>, TransactionServiceError> {
    transaction::list_user_transactions(conn, user_id)
        .map_err(TransactionServiceError::TransactionModelFailed)
}
