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
