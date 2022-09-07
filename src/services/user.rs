use crate::database;
use crate::models::user::{self, NewUser};

fn create_user(user: NewUser) -> NewUser {
    let mut conn = database::establish_connection();
    user::create_user(&mut conn, user)
}
