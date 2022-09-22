use chrono::NaiveDateTime;
use diesel::prelude::*;
use r2d2;
use uuid::Uuid;

use crate::{database, schema::users as user_schema, services::user as user_service};

#[derive(Queryable, Clone)]
#[diesel(table_name = user_schema)]
struct User {
    id: Uuid,
    username: String,
    register_date: Option<NaiveDateTime>,
    email: String,
    last_code_gen_request: Option<NaiveDateTime>,
    login_code: Option<i32>,
    is_registered: bool,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = user_schema)]
struct NewUser {
    username: String,
    register_date: Option<NaiveDateTime>,
    email: String,
    last_code_gen_request: Option<NaiveDateTime>,
    login_code: Option<i32>,
}

impl user_service::NewUser {
    fn to_model(&self) -> NewUser {
        NewUser {
            username: self.username.clone(),
            register_date: None,
            email: self.email.clone(),
            last_code_gen_request: Some(self.last_code_gen_request),
            login_code: Some(self.login_code),
        }
    }
}

impl User {
    fn to_service(&self) -> user_service::User {
        user_service::User {
            id: self.id,
            username: self.username.clone(),
            register_date: self.register_date,
            email: self.email.clone(),
            last_code_gen_request: self.last_code_gen_request,
            login_code: self.login_code,
            is_registered: self.is_registered,
        }
    }
}

#[derive(Debug)]
pub enum UserModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateUser(diesel::result::Error),
    FailedToDeleteUser(diesel::result::Error),
    FailedToCheckAvailability(diesel::result::Error),
    FailedToGetLoginCode(diesel::result::Error),
    FailedToGetIdByEmail(diesel::result::Error),
    UserAlreadyExists,
    UserDoesNotExists,
    UserWithoutLoginCode,
    MoreThanOneEmailError,
}

impl From<r2d2::Error> for UserModelError {
    fn from(error: r2d2::Error) -> Self {
        UserModelError::FailedToGetConn(error)
    }
}

pub fn create_user(
    conn: &database::DbPool,
    user: user_service::NewUser,
) -> Result<user_service::User, UserModelError> {
    let username_is_available = check_if_username_available(&conn, &user.username)?;
    let email_is_available = check_if_email_available(&conn, &user.email)?;

    match (username_is_available, email_is_available) {
        (true, true) => diesel::insert_into(user_schema::table)
            .values(&user.to_model())
            .get_result::<User>(&mut conn.get()?)
            .map(|u| u.to_service())
            .map_err(UserModelError::FailedToCreateUser),
        _ => Err(UserModelError::UserAlreadyExists),
    }
}

pub fn delete_user(
    conn: &database::DbPool,
    email: String,
) -> Result<user_service::User, UserModelError> {
    diesel::delete(user_schema::table.filter(user_schema::email.eq(email)))
        .get_result::<User>(&mut conn.get()?)
        .map(|u| u.to_service())
        .map_err(UserModelError::FailedToDeleteUser)
}

fn check_if_username_available(
    conn: &database::DbPool,
    username: &str,
) -> Result<bool, UserModelError> {
    user_schema::table
        .filter(user_schema::username.eq(username))
        .load::<User>(&mut conn.get()?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

fn check_if_email_available(conn: &database::DbPool, email: &str) -> Result<bool, UserModelError> {
    user_schema::table
        .filter(user_schema::email.eq(email))
        .load::<User>(&mut conn.get()?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

pub fn get_login_code(conn: &database::DbPool, email: &str) -> Result<i32, UserModelError> {
    let result = user_schema::table
        .filter(user_schema::email.eq(email))
        .load::<User>(&mut conn.get()?)
        .map_err(UserModelError::FailedToGetLoginCode)?;

    match result.as_slice() {
        [] => Err(UserModelError::UserDoesNotExists),
        [User {
            login_code: Some(l),
            ..
        }] => Ok(l.clone()),
        [_u] => Err(UserModelError::UserWithoutLoginCode),
        _ => Err(UserModelError::MoreThanOneEmailError),
    }
}

pub fn get_id_by_email(conn: &database::DbPool, email: &str) -> Result<Uuid, UserModelError> {
    let result = user_schema::table
        .filter(user_schema::email.eq(email))
        .load::<User>(&mut conn.get()?)
        .map_err(UserModelError::FailedToGetIdByEmail)?;

    match result.as_slice() {
        [] => Err(UserModelError::UserDoesNotExists),
        [u] => Ok(u.id),
        _ => Err(UserModelError::MoreThanOneEmailError),
    }
}
