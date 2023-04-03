use chrono::NaiveDate;
use uuid::Uuid;

pub struct Transaction {
    pub id: Uuid,
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<Uuid>,
    pub exit_account_code: Option<Uuid>,
    pub amount: f64,
    pub description: Option<String>,
}

pub struct TransactionWithNames {
    pub id: Uuid,
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<Uuid>,
    pub entry_account_name: Option<String>,
    pub exit_account_code: Option<Uuid>,
    pub exit_account_name: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

pub struct NewTransaction {
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<Uuid>,
    pub exit_account_code: Option<Uuid>,
    pub amount: f64,
    pub description: Option<String>,
}

impl Transaction {
    pub fn with_names(&self, entry: Option<String>, exit: Option<String>) -> TransactionWithNames {
        TransactionWithNames {
            id: self.id,
            related_user: self.related_user,
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code,
            entry_account_name: entry,
            exit_account_code: self.exit_account_code,
            exit_account_name: exit,
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

// Model-related things

#[derive(Debug)]
pub enum TransactionModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateTransaction(diesel::result::Error),
    FailedToListTransactions(diesel::result::Error),
    FailedToDeleteTransaction(diesel::result::Error),
}

impl From<r2d2::Error> for TransactionModelError {
    fn from(error: r2d2::Error) -> Self {
        TransactionModelError::FailedToGetConn(error)
    }
}

pub type Result<T> = std::result::Result<T, TransactionModelError>;

pub trait TransactionModel {
    fn create_transaction(
        &self,
        user_id: &Uuid,
        new_transaction: NewTransaction,
    ) -> Result<Transaction>;
    fn list_user_transactions(&self, user_id: &Uuid) -> Result<Vec<Transaction>>;
    fn delete_transaction_by_user_id(&self, user_id: &Uuid) -> Result<()>;
}
