use crate::database;
use crate::models::user::{self, NewUser, User};

pub fn create_user(user: NewUser) -> User {
    let mut conn = database::establish_connection();
    user::create_user(&mut conn, user)
}
