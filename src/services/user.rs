use chrono::Utc;
use rand::Rng;

use crate::database;
use crate::jwt::{generate_token, verify_token};
use crate::models::user::{self, NewUser, User};
use crate::sendemail::send_code;

pub fn create_user(user: NewUser) -> User {
    let mut conn = database::establish_connection();
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
    user::create_user(&mut conn, modified_user)
}

pub fn delete_user(token: String) -> User {
    let email = verify_token(token);
    let mut conn = database::establish_connection();
    user::delete_user(&mut conn, email)
}

pub fn validate_and_generate_token(email: String, login_code: i32) -> String {
    let mut conn = database::establish_connection();
    let real_login_code = user::get_login_code(&mut conn, &email);

    if login_code == real_login_code {
        return generate_token(&email);
    } else {
        panic!("Login code does not match");
    }
}
