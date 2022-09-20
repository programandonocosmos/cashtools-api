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

pub fn create_transaction(conn: &database::DbPool, t: Transaction) -> Transaction {
    diesel::insert_into(transactions::table)
        .values(&t)
        .execute(&mut conn.get().unwrap())
        .expect("Error saving transaction");
    t
}

pub fn list_user_transactions(conn: &database::DbPool, user_id: Uuid) -> Vec<Transaction> {
    match transactions::table
        .filter(transactions::related_user.eq(user_id))
        .load::<Transaction>(&mut conn.get().unwrap())
    {
        Ok(v) => v,
        Err(_) => panic!("Error loading transactions"),
    }
}
