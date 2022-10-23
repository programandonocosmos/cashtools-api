use chrono::NaiveDateTime;

use uuid::Uuid;

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

#[derive(Clone)]
pub struct UserIntegration {
    pub id: Uuid,
    pub related_user: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
}

#[derive(Clone)]
pub struct NewUserIntegration {
    pub related_user: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
}
