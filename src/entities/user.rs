use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{entities::integration::UserIntegration, utils::do_vecs_match};

// User that will be returned when you try to get user information
#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
    pub payday: Option<i32>,
}

#[derive(Debug)]
pub struct UserWithIntegrations {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub register_date: Option<NaiveDateTime>,
    pub email: String,
    pub last_code_gen_request: Option<NaiveDateTime>,
    pub login_code: Option<i32>,
    pub is_registered: bool,
    pub payday: Option<i32>,
    pub integrations: Vec<UserIntegration>,
}

impl PartialEq for UserWithIntegrations {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.username == other.username
            && self.register_date == other.register_date
            && self.email == other.email
            && self.last_code_gen_request == other.last_code_gen_request
            && self.login_code == other.login_code
            && self.is_registered == other.is_registered
            && self.payday == other.payday
            && do_vecs_match(&self.integrations, &other.integrations)
    }
}

impl User {
    pub fn with_integrations(&self, integrations: Vec<UserIntegration>) -> UserWithIntegrations {
        UserWithIntegrations {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            register_date: self.register_date,
            email: self.email.clone(),
            last_code_gen_request: self.last_code_gen_request,
            login_code: self.login_code,
            is_registered: self.is_registered,
            payday: self.payday,
            integrations,
        }
    }
}

// Essential information for create a new user in the database
#[derive(Clone)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub email: String,
}

// Model-related things

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

pub trait UserModel {
    fn create_user(&self, user: NewUser) -> Result<User>;
    fn delete_user(&self, id: &Uuid) -> Result<User>;
    fn get_user(&self, id: Uuid) -> Result<User>;
    fn check_if_username_available(&self, username: &str) -> Result<bool>;
    fn check_if_email_available(&self, email: &str) -> Result<bool>;
    fn refresh_login_code(&self, email: &str, login_code: i32, time: NaiveDateTime) -> Result<()>;
    fn get_login_code(&self, email: &str) -> Result<i32>;
    fn get_id_by_email(&self, email: &str) -> Result<Uuid>;
}
