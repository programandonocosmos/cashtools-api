use uuid::Uuid;

use crate::database;
use crate::models::transaction::{self, Transaction};

fn create_transaction(t: Transaction) -> Transaction {
    let mut conn = database::establish_connection();
    transaction::create_transaction(&mut conn, t)
}

pub fn list_user_transactions(user_id: Uuid) -> Vec<Transaction> {
    let mut conn = database::establish_connection();
    transaction::list_user_transactions(&mut conn, user_id)
}
