use uuid::Uuid;

use crate::{
    database,
    models::transaction::{self, Transaction},
};

// TODO: Use the same connection pool through multiple requests

fn create_transaction(conn: &database::DbPool, t: Transaction) -> Transaction {
    transaction::create_transaction(conn, t)
}

pub fn list_user_transactions(conn: &database::DbPool, user_id: Uuid) -> Vec<Transaction> {
    transaction::list_user_transactions(conn, user_id)
}
