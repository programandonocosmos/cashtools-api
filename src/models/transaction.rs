use chrono::NaiveDate;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    database, schema::transactions as transaction_schema,
    services::transaction as transaction_service,
};

#[derive(Queryable, Clone)]
#[diesel(table_name = transaction_schema)]
struct Transaction {
    id: Uuid,
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<String>,
    exit_account_code: Option<String>,
    amount: f64,
    description: Option<String>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = transaction_schema)]
struct NewTransaction {
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<String>,
    exit_account_code: Option<String>,
    amount: f64,
    description: Option<String>,
}

impl transaction_service::NewTransaction {
    fn to_model(&self) -> NewTransaction {
        NewTransaction {
            related_user: self.related_user,
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

impl Transaction {
    fn to_service(&self) -> transaction_service::Transaction {
        transaction_service::Transaction {
            id: self.id,
            related_user: self.related_user,
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

#[derive(Debug)]
pub enum TransactionModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateTransaction(diesel::result::Error),
    FailedToListTransactions(diesel::result::Error),
}

impl From<r2d2::Error> for TransactionModelError {
    fn from(error: r2d2::Error) -> Self {
        TransactionModelError::FailedToGetConn(error)
    }
}

pub fn create_transaction(
    conn: &database::DbPool,
    t: transaction_service::NewTransaction,
) -> Result<transaction_service::Transaction, TransactionModelError> {
    diesel::insert_into(transaction_schema::table)
        .values(&t.to_model())
        .get_result::<Transaction>(&mut conn.get()?)
        .map(|t| t.to_service())
        .map_err(TransactionModelError::FailedToCreateTransaction)
}

pub fn list_user_transactions(
    conn: &database::DbPool,
    user_id: Uuid,
) -> Result<Vec<transaction_service::Transaction>, TransactionModelError> {
    Ok(transaction_schema::table
        .filter(transaction_schema::related_user.eq(user_id))
        .load::<Transaction>(&mut conn.get()?)
        .map_err(TransactionModelError::FailedToListTransactions)?
        .iter()
        .map(|t| t.to_service())
        .collect())
}
