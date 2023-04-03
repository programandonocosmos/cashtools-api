use chrono::NaiveDate;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{database, entities::transaction, schema::transactions as transaction_schema};

#[derive(Queryable, Clone)]
#[diesel(table_name = transaction_schema)]
struct Transaction {
    id: Uuid,
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<Uuid>,
    exit_account_code: Option<Uuid>,
    amount: f64,
    description: Option<String>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = transaction_schema)]
struct NewTransaction {
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<Uuid>,
    exit_account_code: Option<Uuid>,
    amount: f64,
    description: Option<String>,
}

impl transaction::NewTransaction {
    fn to_model(&self, related_user: &Uuid) -> NewTransaction {
        NewTransaction {
            related_user: related_user.clone(),
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

impl Transaction {
    fn to_entity(&self) -> transaction::Transaction {
        transaction::Transaction {
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

impl transaction::TransactionModel for database::DbPool {
    fn create_transaction(
        &self,
        user_id: &Uuid,
        new_transaction: transaction::NewTransaction,
    ) -> transaction::Result<transaction::Transaction> {
        diesel::insert_into(transaction_schema::table)
            .values(&new_transaction.to_model(user_id))
            .get_result::<Transaction>(&mut self.get()?)
            .map(|t| t.to_entity())
            .map_err(transaction::TransactionModelError::FailedToCreateTransaction)
    }

    fn list_user_transactions(
        &self,
        user_id: &Uuid,
    ) -> transaction::Result<Vec<transaction::Transaction>> {
        Ok(transaction_schema::table
            .filter(transaction_schema::related_user.eq(user_id))
            .load::<Transaction>(&mut self.get()?)
            .map_err(transaction::TransactionModelError::FailedToListTransactions)?
            .iter()
            .map(|t| t.to_entity())
            .collect())
    }

    fn delete_transaction_by_user_id(&self, user_id: &Uuid) -> transaction::Result<()> {
        diesel::delete(
            transaction_schema::table.filter(transaction_schema::related_user.eq(user_id)),
        )
        .execute(&mut self.get()?)
        .map_err(transaction::TransactionModelError::FailedToDeleteTransaction)?;
        Ok(())
    }
}
