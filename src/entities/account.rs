use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub enum EarningIndex {
    CDI,
    FIXED,
    IPCA,
}

#[derive(Copy, Clone, Debug)]
pub struct PreAllocation {
    pub amount: f64,
    pub accumulative: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Earning {
    pub rate: f64,
    pub index: EarningIndex,
}

#[derive(Clone, Debug)]
pub struct Account {
    pub id: Uuid,
    pub time: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    pub balance: f64,
    pub pre_allocation: Option<PreAllocation>,
    pub earning: Option<Earning>,
    pub is_available: bool,
    pub in_trash: bool,
}

#[derive(Debug)]
pub struct UpdatedAccount {
    pub name: Option<String>,
    pub description: Option<String>,
    pub pre_allocation: Option<PreAllocation>,
    pub earning: Option<Earning>,
    pub is_available: Option<bool>,
    pub in_trash: Option<bool>,
}

#[derive(Debug)]
pub struct NewAccount {
    pub time: NaiveDateTime,
    pub initial_balance: f64,
    pub name: String,
    pub description: Option<String>,
    pub pre_allocation: Option<PreAllocation>,
    pub earning: Option<Earning>,
    pub is_available: bool,
}

// Model-related things

#[derive(Debug)]
pub enum AccountModelError {
    FailedToGetConn(r2d2::Error),
    FailedToGetAccount(diesel::result::Error),
    AccountNotFound,
    MultipleAccountWithSameId,
    FailedToCreateAccount(diesel::result::Error),
    FailedToDeleteAccount(Box<AccountModelError>),
    FailedToUpdateAccount(diesel::result::Error),
}

impl From<r2d2::Error> for AccountModelError {
    fn from(error: r2d2::Error) -> Self {
        AccountModelError::FailedToGetConn(error)
    }
}

pub type Result<T> = std::result::Result<T, AccountModelError>;

pub trait AccountModel {
    fn create_account(&self, user_id: Uuid, new_account: NewAccount) -> Result<Account>;
    fn get_account(&self, id: &Uuid, user_id: &Uuid) -> Result<Account>;
    fn get_accounts(&self, user_id: &Uuid) -> Result<Vec<Account>>;
    fn delete_account(&self, id: &Uuid, user_id: &Uuid) -> Result<()>;
    fn edit_account(
        &self,
        id: &Uuid,
        user_id: &Uuid,
        updated_account: UpdatedAccount,
    ) -> Result<Account>;
}
