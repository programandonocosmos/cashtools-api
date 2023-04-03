use chrono::NaiveDateTime;
use diesel::prelude::*;
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

impl user::UserModel for database::DbPool {
    fn create_user(&self, user: user::NewUser) -> user::Result<user::User> {
        let username_is_available = self.check_if_username_available(&user.username)?;
        let email_is_available = self.check_if_email_available(&user.email)?;

        match (username_is_available, email_is_available) {
            (true, true) => diesel::insert_into(user_schema::table)
                .values(&user.to_model())
                .get_result::<User>(&mut self.get()?)
                .map(|u| u.to_entity())
                .map_err(user::UserModelError::FailedToCreateUser),
            _ => Err(user::UserModelError::UserAlreadyExists),
        }
    }

    fn delete_user(&self, id: &Uuid) -> user::Result<user::User> {
        diesel::delete(user_schema::table.filter(user_schema::id.eq(id)))
            .get_result::<User>(&mut self.get()?)
            .map(|u| u.to_entity())
            .map_err(user::UserModelError::FailedToDeleteUser)
    }

    fn get_user(&self, id: Uuid) -> user::Result<user::User> {
        let users = user_schema::table
            .filter(user_schema::id.eq(id))
            .load::<User>(&mut self.get()?)
            .map_err(user::UserModelError::FailedToGetUserById)?;

        match users.as_slice() {
            [] => Err(user::UserModelError::UserDoesNotExists),
            [u] => Ok(u.to_entity()),
            _ => Err(user::UserModelError::MoreThanOneIdError),
        }
    }

    fn check_if_username_available(&self, username: &str) -> user::Result<bool> {
        user_schema::table
            .filter(user_schema::username.eq(username))
            .load::<User>(&mut self.get()?)
            .map(|v| v.is_empty())
            .map_err(user::UserModelError::FailedToCheckAvailability)
    }

    fn check_if_email_available(&self, email: &str) -> user::Result<bool> {
        user_schema::table
            .filter(user_schema::email.eq(email))
            .load::<User>(&mut self.get()?)
            .map(|v| v.is_empty())
            .map_err(user::UserModelError::FailedToCheckAvailability)
    }

    fn refresh_login_code(
        &self,
        email: &str,
        login_code: i32,
        time: NaiveDateTime,
    ) -> user::Result<()> {
        let _ = diesel::update(user_schema::table.filter(user_schema::email.eq(email)))
            .set((
                user_schema::login_code.eq(login_code),
                user_schema::last_code_gen_request.eq(time),
            ))
            .get_result::<User>(&mut self.get()?)
            .map_err(user::UserModelError::FailedToUpdateLoginCode)?;
        Ok(())
    }

    fn get_login_code(&self, email: &str) -> user::Result<i32> {
        let result = user_schema::table
            .filter(user_schema::email.eq(email))
            .load::<User>(&mut self.get()?)
            .map_err(user::UserModelError::FailedToGetLoginCode)?;

        match result.as_slice() {
            [] => Err(user::UserModelError::UserDoesNotExists),
            [User {
                login_code: Some(l),
                ..
            }] => Ok(l.clone()),
            [_u] => Err(user::UserModelError::UserWithoutLoginCode),
            _ => Err(user::UserModelError::MoreThanOneEmailError),
        }
    }

    fn get_id_by_email(&self, email: &str) -> user::Result<Uuid> {
        let result = user_schema::table
            .filter(user_schema::email.eq(email))
            .load::<User>(&mut self.get()?)
            .map_err(user::UserModelError::FailedToGetIdByEmail)?;

        match result.as_slice() {
            [] => Err(user::UserModelError::UserDoesNotExists),
            [u] => Ok(u.id),
            _ => Err(user::UserModelError::MoreThanOneEmailError),
        }
    }
}
