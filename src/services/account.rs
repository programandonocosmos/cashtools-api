use std::fmt;

use chrono::{NaiveDate, Utc};

use log;
use uuid::Uuid;

use crate::{
    entities::{account, transaction},
    jwt,
};

#[derive(Debug)]
pub enum AccountServiceError {
    AccountModelFailed(account::AccountModelError),
    TransactionModelFailed(transaction::TransactionModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for AccountServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<account::AccountModelError> for AccountServiceError {
    fn from(error: account::AccountModelError) -> Self {
        AccountServiceError::AccountModelFailed(error)
    }
}

impl From<transaction::TransactionModelError> for AccountServiceError {
    fn from(error: transaction::TransactionModelError) -> Self {
        AccountServiceError::TransactionModelFailed(error)
    }
}

impl From<jwt::JwtError> for AccountServiceError {
    fn from(error: jwt::JwtError) -> Self {
        AccountServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, AccountServiceError>;

pub fn auth_and_create_account<T: account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    new_account: account::NewAccount,
) -> Result<account::Account> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    log::debug!("Related user: {:?}", id);
    Ok(database.create_account(id, new_account)?)
}

pub fn auth_and_delete_account<T: account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
) -> Result<()> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(database.delete_account(id, &user_id)?)
}

pub fn auth_and_edit_account<T: account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
    updated_account: account::UpdatedAccount,
) -> Result<account::Account> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(database.edit_account(id, &user_id, updated_account)?)
}

pub fn preallocate<T: account::AccountModel + transaction::TransactionModel>(
    database: &T,
    user_id: &Uuid,
    time: NaiveDate,
    from: &Uuid,
    to: &Uuid,
    amount: f64,
    accumulative: bool,
) -> Result<account::PreAllocation> {
    let pre_allocation_obj = account::PreAllocation {
        amount,
        accumulative,
    };
    let _ = database.edit_account(
        user_id,
        to,
        account::UpdatedAccount {
            name: None,
            description: None,
            pre_allocation: Some(pre_allocation_obj),
            earning: None,
            is_available: None,
            in_trash: None,
        },
    )?;
    let _ = database.create_transaction(
        user_id,
        transaction::NewTransaction {
            entry_date: time,
            entry_account_code: Some(from.clone()),
            exit_account_code: Some(to.clone()),
            amount,
            description: Some("Preallocation transaction".to_string()),
        },
    )?;
    Ok(pre_allocation_obj)
}

pub fn auth_and_preallocate<T: account::AccountModel + transaction::TransactionModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    time: NaiveDate,
    from: &Uuid,
    to: &Uuid,
    amount: f64,
    accumulative: bool,
) -> Result<account::PreAllocation> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    preallocate(database, &id, time, from, to, amount, accumulative)
}

pub fn auth_and_get_account<T: account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
) -> Result<account::Account> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let account = database.get_account(&id, &user_id)?;
    Ok(account)
}

pub fn auth_and_get_accounts<T: account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    is_pre_allocation: Option<bool>,
    in_trash: Option<bool>,
    tags: Option<Vec<Uuid>>,
) -> Result<Vec<account::Account>> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let accounts = database.get_accounts(&user_id)?;
    Ok(accounts
        .iter()
        .filter(filter_accounts(is_pre_allocation, in_trash, tags))
        .cloned()
        .collect())
}

fn filter_accounts(
    is_pre_allocation: Option<bool>,
    in_trash: Option<bool>,
    tags: Option<Vec<Uuid>>,
) -> impl Fn(&&account::Account) -> bool {
    move |account: &&account::Account| {
        let pre_allocation_filter = match is_pre_allocation.clone() {
            None => true,
            Some(f) => account.pre_allocation.is_none() != f,
        };
        let in_trash_filter = match in_trash.clone() {
            None => true,
            Some(f) => account.in_trash == f,
        };
        let tags_filter = match tags.clone() {
            None => true,
            // TODO: Add tags table and implement this filter correctly
            Some(_) => true,
        };
        pre_allocation_filter & in_trash_filter & tags_filter
    }
}
