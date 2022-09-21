use std::fmt;

use chrono::Utc;
use rand::Rng;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::models::user::{self, NewUser, User};
use crate::sendemail::send_code;

use crate::database;

#[derive(Debug)]
pub enum UserServiceError {
    UserModelFailed(user::UserModelError),
    FailedToGenerateJwtKey(sha2::digest::InvalidLength),
    FailedToGenerateJwtToken(jwt::Error),
    FailedToVerifyJwtToken(jwt::Error),
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

#[derive(Serialize, Deserialize)]
struct TokenContent {
    email: String,
}

pub fn create_user(conn: &database::DbPool, user: NewUser) -> Result<User, UserServiceError> {
    let now = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(100000..999999);
    let _ = send_code(&user.email, &login_code);
    let modified_user = NewUser {
        last_code_gen_request: Some(now),
        login_code: Some(login_code),
        ..user
    };
    Ok(user::create_user(conn, modified_user)?)
}

pub fn delete_user(
    conn: &database::DbPool,
    token: String,
    jwt_secret: &str,
) -> Result<User, UserServiceError> {
    let email = verify_token(token, jwt_secret)?;
    Ok(user::delete_user(conn, email)?)
}

pub fn validate_and_generate_token(
    conn: &database::DbPool,
    email: String,
    login_code: i32,
    jwt_secret: &str,
) -> Result<String, UserServiceError> {
    let real_login_code = user::get_login_code(conn, &email)?;

    if login_code == real_login_code {
        return generate_token(&email, jwt_secret);
    } else {
        return Err(UserServiceError::LoginCodeNotMatching);
    }
}

fn generate_token(email: &str, jwt_secret: &str) -> Result<String, UserServiceError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
        .map_err(UserServiceError::FailedToGenerateJwtKey)?;

    TokenContent {
        email: email.to_string(),
    }
    .sign_with_key(&key)
    .map_err(UserServiceError::FailedToGenerateJwtToken)
}

fn verify_token(token: String, jwt_secret: &str) -> Result<String, UserServiceError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())
        .map_err(UserServiceError::FailedToGenerateJwtKey)?;
    let content: TokenContent = token
        .verify_with_key(&key)
        .map_err(UserServiceError::FailedToVerifyJwtToken)?;
    Ok(content.email)
}
