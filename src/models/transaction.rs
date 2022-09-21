use chrono::NaiveDate;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{database, schema::transactions};

#[derive(Queryable, Insertable, Clone)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: Uuid,
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum TransactionModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateTransaction(diesel::result::Error),
    FailedToListTransactions(diesel::result::Error),
}

pub fn create_transaction(
    conn: &database::DbPool,
    t: Transaction,
) -> Result<Transaction, TransactionModelError> {
    diesel::insert_into(transactions::table)
        .values(&t)
        .get_result::<Transaction>(&mut conn.get().map_err(TransactionModelError::FailedToGetConn)?)
        .map_err(TransactionModelError::FailedToCreateTransaction)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    user_id: Uuid,
) -> Result<Vec<Transaction>, TransactionModelError> {
    transactions::table
        .filter(transactions::related_user.eq(user_id))
        .load::<Transaction>(&mut conn.get().map_err(TransactionModelError::FailedToGetConn)?)
        .map_err(TransactionModelError::FailedToListTransactions)
}
