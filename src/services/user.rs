use std::fmt;

use chrono::{NaiveDateTime, Utc};
use rand::Rng;

use uuid::Uuid;

use crate::jwt;
use crate::models::transaction;
use crate::models::user;
use crate::sendemail::send_code;

use crate::database;

#[derive(Debug)]
pub enum UserServiceError {
    UserModelFailed(user::UserModelError),
    TransactionModelFailed(transaction::TransactionModelError),
    JwtError(jwt::JwtError),
    LoginCodeNotMatching,
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<user::UserModelError> for UserServiceError {
    fn from(error: user::UserModelError) -> Self {
        UserServiceError::UserModelFailed(error)
    }
}

impl From<transaction::TransactionModelError> for UserServiceError {
    fn from(error: transaction::TransactionModelError) -> Self {
        UserServiceError::TransactionModelFailed(error)
    }
}

impl From<jwt::JwtError> for UserServiceError {
    fn from(error: jwt::JwtError) -> Self {
        UserServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, UserServiceError>;

// User that will be returned when you try to get user information
#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
}

// Essential information for create a new user in the database
#[derive(Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub last_code_gen_request: NaiveDateTime,
    pub login_code: i32,
}

pub fn create_user(conn: &database::DbPool, username: String, email: String) -> Result<User> {
    let last_code_gen_request = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(100000..999999);
    let _ = send_code(&email, &login_code);
    let user = NewUser {
        username,
        email,
        last_code_gen_request,
        login_code,
    };
    Ok(user::create_user(conn, user)?)
}

pub fn delete_user(conn: &database::DbPool, token: String, jwt_secret: &str) -> Result<User> {
    let email = jwt::verify_token(&token, jwt_secret)?;
    let id = user::get_id_by_email(conn, &email)?;
    transaction::delete_transaction_by_user_id(conn, &id)?;
    Ok(user::delete_user(conn, email)?)
}

pub fn validate_and_generate_token(
    conn: &database::DbPool,
    email: String,
    login_code: i32,
    jwt_secret: &str,
) -> Result<String> {
    let real_login_code = user::get_login_code(conn, &email)?;

    if login_code == real_login_code {
        Ok(jwt::generate_token(&email, jwt_secret)?)
    } else {
        Err(UserServiceError::LoginCodeNotMatching)
    }
}
