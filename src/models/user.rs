use chrono::NaiveDateTime;
use diesel::prelude::*;
use r2d2;
use uuid::Uuid;

use crate::{database, schema::users};

#[derive(Queryable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
}

#[derive(Debug)]
pub enum UserModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateUser(diesel::result::Error),
    FailedToDeleteUser(diesel::result::Error),
    FailedToCheckAvailability(diesel::result::Error),
    FailedToGetLoginCode(diesel::result::Error),
    UserAlreadyExists,
    UserDoesNotExists,
    UserWithoutLoginCode,
    MoreThanOneEmailError,
}

pub fn create_user(conn: &database::DbPool, user: NewUser) -> Result<User, UserModelError> {
    let username_is_available = check_if_username_available(&conn, &user.username)?;
    let email_is_available = check_if_email_available(&conn, &user.email)?;

    match (username_is_available, email_is_available) {
        (true, true) => diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&mut conn.get().map_err(UserModelError::FailedToGetConn)?)
            .map_err(UserModelError::FailedToCreateUser),
        _ => Err(UserModelError::UserAlreadyExists),
    }
}

pub fn delete_user(conn: &database::DbPool, email: String) -> Result<User, UserModelError> {
    diesel::delete(users::table.filter(users::email.eq(email)))
        .get_result::<User>(&mut conn.get().map_err(UserModelError::FailedToGetConn)?)
        .map_err(UserModelError::FailedToDeleteUser)
}

fn check_if_username_available(
    conn: &database::DbPool,
    username: &str,
) -> Result<bool, UserModelError> {
    users::table
        .filter(users::username.eq(username))
        .load::<User>(&mut conn.get().map_err(UserModelError::FailedToGetConn)?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

fn check_if_email_available(conn: &database::DbPool, email: &str) -> Result<bool, UserModelError> {
    users::table
        .filter(users::email.eq(email))
        .load::<User>(&mut conn.get().map_err(UserModelError::FailedToGetConn)?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

pub fn get_login_code(conn: &database::DbPool, email: &str) -> Result<i32, UserModelError> {
    let result = users::table
        .filter(users::email.eq(email))
        .load::<User>(&mut conn.get().map_err(UserModelError::FailedToGetConn)?)
        .map_err(UserModelError::FailedToGetLoginCode)?;

    match result.as_slice() {
        [] => Err(UserModelError::UserDoesNotExists),
        [User {
            login_code: Some(l),
            ..
        }] => Ok(l.clone()),
        [u] => Err(UserModelError::UserWithoutLoginCode),
        _ => Err(UserModelError::MoreThanOneEmailError),
    }
}
