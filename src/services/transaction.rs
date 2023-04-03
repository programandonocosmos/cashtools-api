use std::fmt;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    entities::{account, transaction, user},
    jwt,
    utils::opt_result_of_result_opt,
};

#[derive(Debug)]
pub enum TransactionServiceError {
    TransactionModelFailed(transaction::TransactionModelError),
    UserModelFailed(user::UserModelError),
    AccountModelFailed(account::AccountModelError),
    JwtError(jwt::JwtError),
}

impl fmt::Display for TransactionServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<transaction::TransactionModelError> for TransactionServiceError {
    fn from(error: transaction::TransactionModelError) -> Self {
        TransactionServiceError::TransactionModelFailed(error)
    }
}

impl From<user::UserModelError> for TransactionServiceError {
    fn from(error: user::UserModelError) -> Self {
        TransactionServiceError::UserModelFailed(error)
    }
}

impl From<account::AccountModelError> for TransactionServiceError {
    fn from(error: account::AccountModelError) -> Self {
        TransactionServiceError::AccountModelFailed(error)
    }
}

impl From<jwt::JwtError> for TransactionServiceError {
    fn from(error: jwt::JwtError) -> Self {
        TransactionServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, TransactionServiceError>;

fn fill_name<T: account::AccountModel>(
    database: &T,
    user_id: &Uuid,
    transaction: &transaction::Transaction,
) -> Result<transaction::TransactionWithNames> {
    let entry_account = transaction
        .entry_account_code
        .map(|code| database.get_account(&code, user_id));
    let exit_account = transaction
        .exit_account_code
        .map(|code| database.get_account(&code, user_id));
    let entry_account_name = opt_result_of_result_opt(entry_account)?.map(|x| x.name);
    let exit_account_name = opt_result_of_result_opt(exit_account)?.map(|x| x.name);
    Ok(transaction.with_names(entry_account_name, exit_account_name))
}

pub fn auth_and_create_transaction<T: transaction::TransactionModel + account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    new_transaction: transaction::NewTransaction,
) -> Result<transaction::TransactionWithNames> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let created_transaction = database.create_transaction(&id, new_transaction)?;
    fill_name(database, &id, &created_transaction)
}

pub fn auth_and_list_user_transactions<T: transaction::TransactionModel + account::AccountModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
) -> Result<Vec<transaction::TransactionWithNames>> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let transactions = database.list_user_transactions(&id)?;
    transactions
        .into_iter()
        .map(|t| fill_name(database, &id, &t))
        .collect()
}
