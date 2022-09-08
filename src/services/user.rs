use crate::database;
use crate::models::user::{self, NewUser};

pub fn create_user(user: NewUser) -> NewUser {
    let mut conn = database::establish_connection();
    user::create_user(&mut conn, user)
}
