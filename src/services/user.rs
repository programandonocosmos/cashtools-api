use std::fmt;

use chrono::{NaiveDateTime, Utc};
use rand::Rng;

use uuid::Uuid;

use crate::{
    database, entities::user, jwt, models::transaction as transaction_model,
    models::user as user_model, models::user_integration as user_integration_model,
    sendemail::send_code,
};

#[derive(Debug)]
pub enum UserServiceError {
    UserModelFailed(user_model::UserModelError),
    TransactionModelFailed(transaction_model::TransactionModelError),
    UserIntegrationModelFailed(user_integration_model::IntegrationModelError),
    JwtError(jwt::JwtError),
    LoginCodeNotMatching,
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<user_model::UserModelError> for UserServiceError {
    fn from(error: user_model::UserModelError) -> Self {
        UserServiceError::UserModelFailed(error)
    }
}

impl From<transaction_model::TransactionModelError> for UserServiceError {
    fn from(error: transaction_model::TransactionModelError) -> Self {
        UserServiceError::TransactionModelFailed(error)
    }
}

impl From<user_integration_model::IntegrationModelError> for UserServiceError {
    fn from(error: user_integration_model::IntegrationModelError) -> Self {
        UserServiceError::UserIntegrationModelFailed(error)
    }
}

impl From<jwt::JwtError> for UserServiceError {
    fn from(error: jwt::JwtError) -> Self {
        UserServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, UserServiceError>;

pub fn create_user(
    conn: &database::DbPool,
    username: &str,
    name: &str,
    email: &str,
) -> Result<user::UserWithIntegrations> {
    let new_user = user::NewUser {
        name: name.to_string(),
        username: username.to_string(),
        email: email.to_string(),
    };
    let user = user_model::create_user(conn, new_user)?.with_integrations(Vec::new());
    refresh_login_code(conn, email)?;

    Ok(user)
}

pub fn refresh_login_code(conn: &database::DbPool, email: &str) -> Result<()> {
    let last_code_gen_request = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(100000..999999);
    send_code(&email, &login_code);
    user_model::refresh_login_code(conn, email, login_code, last_code_gen_request)?;
    Ok(())
}

pub fn delete_user(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<user::UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    transaction_model::delete_transaction_by_user_id(conn, &id)?;
    let integrations = user_integration_model::delete_integration_by_user_id(conn, &id)?;
    Ok(user_model::delete_user(conn, &id)?.with_integrations(integrations))
}

pub fn get_user(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
) -> Result<user::UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let integrations = user_integration_model::list_user_integrations(conn, &id)?;
    Ok(user_model::get_user(conn, id)?.with_integrations(integrations))
}

pub fn validate_and_generate_token(
    conn: &database::DbPool,
    email: String,
    login_code: i32,
    jwt_secret: &str,
) -> Result<String> {
    let real_login_code = user_model::get_login_code(conn, &email)?;
    let id = user_model::get_id_by_email(conn, &email)?;

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
) -> Result<user::UserIntegration> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    let new_integration = user::NewUserIntegration {
        related_user: id,
        name,
        time,
    };
    Ok(user_integration_model::create_integration(
        conn,
        new_integration,
    )?)
}

pub fn delete_integration(
    conn: &database::DbPool,
    token: &str,
    jwt_secret: &str,
    id: Uuid,
) -> Result<user::UserIntegration> {
    let _ = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(user_integration_model::delete_integration(conn, &id)?)
}
