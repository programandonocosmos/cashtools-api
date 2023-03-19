use std::fmt;

use chrono::{NaiveDate, Utc};

use log;
use uuid::Uuid;

use crate::{
    database,
    entities::{account, transaction},
    jwt,
    models::account as account_model,
    models::transaction as transaction_model,
};

#[derive(Debug)]
pub enum AccountServiceError {
    AccountModelFailed(account_model::AccountModelError),
    TransactionModelFailed(transaction_model::TransactionModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for AccountServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<account_model::AccountModelError> for AccountServiceError {
    fn from(error: account_model::AccountModelError) -> Self {
        AccountServiceError::AccountModelFailed(error)
    }
}

impl From<transaction_model::TransactionModelError> for AccountServiceError {
    fn from(error: transaction_model::TransactionModelError) -> Self {
        AccountServiceError::TransactionModelFailed(error)
    }
}

impl From<jwt::JwtError> for AccountServiceError {
    fn from(error: jwt::JwtError) -> Self {
        AccountServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, AccountServiceError>;

pub fn auth_and_create_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    new_account: account::NewAccount,
) -> Result<account::Account> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    log::debug!("Related user: {:?}", id);
    Ok(account_model::create_account(conn, id, new_account)?)
}

pub fn auth_and_delete_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
) -> Result<()> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(account_model::delete_account(conn, id, &user_id)?)
}

pub fn auth_and_edit_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
    updated_account: account::UpdatedAccount,
) -> Result<account::Account> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(account_model::edit_account(
        conn,
        id,
        &user_id,
        updated_account,
    )?)
}

pub fn preallocate(
    conn: &database::DbPool,
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
    let _ = account_model::edit_account(
        conn,
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
    let _ = transaction_model::create_transaction(
        conn,
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

pub fn auth_and_preallocate(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    time: NaiveDate,
    from: &Uuid,
    to: &Uuid,
    amount: f64,
    accumulative: bool,
) -> Result<account::PreAllocation> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    preallocate(conn, &id, time, from, to, amount, accumulative)
}

pub fn auth_and_get_account(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: &Uuid,
) -> Result<account::Account> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let account = account_model::get_account(conn, &id, &user_id)?;
    Ok(account)
}

pub fn auth_and_get_accounts(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    is_pre_allocation: Option<bool>,
    in_trash: Option<bool>,
    tags: Option<Vec<Uuid>>,
) -> Result<Vec<account::Account>> {
    let user_id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let accounts = account_model::get_accounts(conn, &user_id)?;
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
