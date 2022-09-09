use chrono::Utc;
use rand::Rng;
use uuid::Uuid;

use crate::database;
use crate::models::user::{self, NewUser, User};
use crate::sendemail::send_code;

pub fn create_user(user: NewUser) -> User {
    let mut conn = database::establish_connection();
    let now = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(0..999999);
    let _ = send_code(&user.email, &login_code);
    let modified_user = NewUser {
        last_code_gen_request: Some(now),
        login_code: Some(login_code),
        ..user
    };
    user::create_user(&mut conn, modified_user)
}

pub fn delete_user(id: Uuid) -> User {
    let mut conn = database::establish_connection();
    user::delete_user(&mut conn, id)
}
