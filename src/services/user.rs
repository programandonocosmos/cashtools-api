use std::fmt;

use chrono::{NaiveDateTime, Utc};
use rand::Rng;

use uuid::Uuid;

use crate::jwt;
use crate::models::transaction;
use crate::models::user;
use crate::models::user_integration;
use crate::sendemail::send_code;

use crate::database;

#[derive(Debug)]
pub enum UserServiceError {
    UserModelFailed(user::UserModelError),
    TransactionModelFailed(transaction::TransactionModelError),
    UserIntegrationModelFailed(user_integration::IntegrationModelError),
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

impl From<user_integration::IntegrationModelError> for UserServiceError {
    fn from(error: user_integration::IntegrationModelError) -> Self {
        UserServiceError::UserIntegrationModelFailed(error)
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
    pub name: String,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
    pub payday: Option<i32>,
}

pub struct UserWithIntegrations {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
    pub payday: Option<i32>,
    pub integrations: Vec<UserIntegration>,
}

impl User {
    fn with_integrations(&self, integrations: Vec<UserIntegration>) -> UserWithIntegrations {
        UserWithIntegrations {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            register_date: self.register_date,
            email: self.email.clone(),
            last_code_gen_request: self.last_code_gen_request,
            login_code: self.login_code,
            is_registered: self.is_registered,
            payday: self.payday,
            integrations,
        }
    }
}

// Essential information for create a new user in the database
#[derive(Clone)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub last_code_gen_request: NaiveDateTime,
    pub login_code: i32,
}

#[derive(Clone)]
pub struct UserIntegration {
    pub id: Uuid,
    pub related_user: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
}

#[derive(Clone)]
pub struct NewUserIntegration {
    pub related_user: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
}

pub fn create_user(
    conn: &database::DbPool,
    username: &str,
    name: &str,
    email: &str,
) -> Result<UserWithIntegrations> {
    let last_code_gen_request = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(100000..999999);
    let _ = send_code(&email, &login_code);
    let user = NewUser {
        name: name.to_string(),
        username: username.to_string(),
        email: email.to_string(),
        last_code_gen_request,
        login_code,
    };
    Ok(user::create_user(conn, user)?.with_integrations(Vec::new()))
}

pub fn delete_user(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    transaction::delete_transaction_by_user_id(conn, &id)?;
    let integrations = user_integration::delete_integration_by_user_id(conn, &id)?;
    Ok(user::delete_user(conn, &id)?.with_integrations(integrations))
}

pub fn get_user(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let integrations = user_integration::list_user_integrations(conn, &id)?;
    Ok(user::get_user(conn, id)?.with_integrations(integrations))
}

pub fn validate_and_generate_token(
    conn: &database::DbPool,
    email: String,
    login_code: i32,
    jwt_secret: &str,
) -> Result<String> {
    let real_login_code = user::get_login_code(conn, &email)?;
    let id = user::get_id_by_email(conn, &email)?;

    if login_code == real_login_code {
        Ok(jwt::generate_token(
            Utc::now().naive_utc(),
            &id,
            jwt_secret,
        )?)
    } else {
        Err(UserServiceError::LoginCodeNotMatching)
    }
}

pub fn create_integration(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    name: String,
    time: NaiveDateTime,
) -> Result<UserIntegration> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let new_integration = NewUserIntegration {
        related_user: id,
        name,
        time,
    };
    Ok(user_integration::create_integration(conn, new_integration)?)
}

pub fn delete_integration(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: Uuid,
) -> Result<UserIntegration> {
    let _ = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(user_integration::delete_integration(conn, &id)?)
}
