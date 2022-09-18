use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::users;

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

pub fn create_user(conn: &mut PgConnection, user: NewUser) -> User {
    let username_is_available = check_if_username_available(conn, &user.username);
    let email_is_available = check_if_email_available(conn, &user.email);

    match (username_is_available, email_is_available) {
        (true, true) => diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(conn)
            .expect("Error saving user"),
        _ => panic!("User already exists"),
    }
}

pub fn delete_user(conn: &mut PgConnection, email: String) -> User {
    diesel::delete(users::table.filter(users::email.eq(email)))
        .get_result::<User>(conn)
        .expect("Error deleting user")
}

fn check_if_username_available(conn: &mut PgConnection, username: &str) -> bool {
    match users::table
        .filter(users::username.eq(username))
        .load::<User>(conn)
    {
        Ok(v) => v.is_empty(),
        Err(_) => panic!("Error checking if username is available"),
    }
}

fn check_if_email_available(conn: &mut PgConnection, email: &str) -> bool {
    match users::table
        .filter(users::email.eq(email))
        .load::<User>(conn)
    {
        Ok(v) => v.is_empty(),
        Err(_) => panic!("Error checking if email is available"),
    }
}

pub fn get_login_code(conn: &mut PgConnection, email: &str) -> i32 {
    let result = users::table
        .filter(users::email.eq(email))
        .load::<User>(conn)
        .expect("Error getting login code");

    match result.as_slice() {
        [User {
            login_code: Some(l),
            ..
        }] => *l,
        _ => panic!("Error extracting a valid login code"),
    }
}
