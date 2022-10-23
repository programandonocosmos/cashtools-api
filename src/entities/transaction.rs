use chrono::NaiveDate;
use uuid::Uuid;

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
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}

pub struct NewTransactionWithRelatedUser {
    pub related_user: Uuid,
    pub entry_date: NaiveDate,
    pub entry_account_code: Option<String>,
    pub exit_account_code: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
}
