use chrono::NaiveDateTime;
use diesel::prelude::*;
use r2d2;
use uuid::Uuid;

use crate::{database, entities::user, schema::users as user_schema};

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
    name: String,
    payday: Option<i32>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = user_schema)]
struct NewUser {
    name: String,
    username: String,
    email: String,
}

impl user::NewUser {
    fn to_model(&self) -> NewUser {
        NewUser {
            name: self.name.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
        }
    }
}

impl User {
    fn to_entity(&self) -> user::User {
        user::User {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            register_date: self.register_date,
            email: self.email.clone(),
            last_code_gen_request: self.last_code_gen_request,
            login_code: self.login_code,
            is_registered: self.is_registered,
            payday: self.payday,
        }
    }
}

#[derive(Debug)]
pub enum UserModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateUser(diesel::result::Error),
    FailedToDeleteUser(diesel::result::Error),
    FailedToGetUserById(diesel::result::Error),
    FailedToCheckAvailability(diesel::result::Error),
    FailedToGetLoginCode(diesel::result::Error),
    FailedToGetIdByEmail(diesel::result::Error),
    FailedToUpdateLoginCode(diesel::result::Error),
    UserAlreadyExists,
    UserDoesNotExists,
    UserWithoutLoginCode,
    MoreThanOneEmailError,
    MoreThanOneIdError,
}

impl From<r2d2::Error> for UserModelError {
    fn from(error: r2d2::Error) -> Self {
        UserModelError::FailedToGetConn(error)
    }
}

pub type Result<T> = std::result::Result<T, UserModelError>;

pub fn create_user(conn: &database::DbPool, user: user::NewUser) -> Result<user::User> {
    let username_is_available = check_if_username_available(&conn, &user.username)?;
    let email_is_available = check_if_email_available(&conn, &user.email)?;

    match (username_is_available, email_is_available) {
        (true, true) => diesel::insert_into(user_schema::table)
            .values(&user.to_model())
            .get_result::<User>(&mut conn.get()?)
            .map(|u| u.to_entity())
            .map_err(UserModelError::FailedToCreateUser),
        _ => Err(UserModelError::UserAlreadyExists),
    }
}

pub fn delete_user(conn: &database::DbPool, id: &Uuid) -> Result<user::User> {
    diesel::delete(user_schema::table.filter(user_schema::id.eq(id)))
        .get_result::<User>(&mut conn.get()?)
        .map(|u| u.to_entity())
        .map_err(UserModelError::FailedToDeleteUser)
}

pub fn get_user(conn: &database::DbPool, id: Uuid) -> Result<user::User> {
    let users = user_schema::table
        .filter(user_schema::id.eq(id))
        .load::<User>(&mut conn.get()?)
        .map_err(UserModelError::FailedToGetUserById)?;

    match users.as_slice() {
        [] => Err(UserModelError::UserDoesNotExists),
        [u] => Ok(u.to_entity()),
        _ => Err(UserModelError::MoreThanOneIdError),
    }
}

fn check_if_username_available(conn: &database::DbPool, username: &str) -> Result<bool> {
    user_schema::table
        .filter(user_schema::username.eq(username))
        .load::<User>(&mut conn.get()?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

fn check_if_email_available(conn: &database::DbPool, email: &str) -> Result<bool> {
    user_schema::table
        .filter(user_schema::email.eq(email))
        .load::<User>(&mut conn.get()?)
        .map(|v| v.is_empty())
        .map_err(UserModelError::FailedToCheckAvailability)
}

pub fn refresh_login_code(
    conn: &database::DbPool,
    email: &str,
    login_code: i32,
    time: NaiveDateTime,
) -> Result<()> {
    let _ = diesel::update(user_schema::table.filter(user_schema::email.eq(email)))
        .set((
            user_schema::login_code.eq(login_code),
            user_schema::last_code_gen_request.eq(time),
        ))
        .get_result::<User>(&mut conn.get()?)
        .map_err(UserModelError::FailedToUpdateLoginCode)?;
    Ok(())
}

pub fn get_login_code(conn: &database::DbPool, email: &str) -> Result<i32> {
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

pub fn get_id_by_email(conn: &database::DbPool, email: &str) -> Result<Uuid> {
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
