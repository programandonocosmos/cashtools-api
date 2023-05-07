use std::fmt;

use chrono::{NaiveDateTime, Utc};
use rand::Rng;

use uuid::Uuid;

use crate::{
    entities::{integration, transaction, user, Env},
    jwt,
    sendemail::send_code,
};

#[derive(Debug)]
pub enum UserServiceError {
    UserModelFailed(user::UserModelError),
    TransactionModelFailed(transaction::TransactionModelError),
    UserIntegrationModelFailed(integration::IntegrationModelError),
    JwtError(jwt::JwtError),
    LoginCodeNotMatching,
    UserAlreadyExists,
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<user::UserModelError> for UserServiceError {
    fn from(error: user::UserModelError) -> Self {
        UserServiceError::UserModelFailed(error)
    }
}

impl From<transaction::TransactionModelError> for UserServiceError {
    fn from(error: transaction::TransactionModelError) -> Self {
        UserServiceError::TransactionModelFailed(error)
    }
}

impl From<integration::IntegrationModelError> for UserServiceError {
    fn from(error: integration::IntegrationModelError) -> Self {
        UserServiceError::UserIntegrationModelFailed(error)
    }
}

impl From<jwt::JwtError> for UserServiceError {
    fn from(error: jwt::JwtError) -> Self {
        UserServiceError::JwtError(error)
    }
}

pub type Result<T> = std::result::Result<T, UserServiceError>;

pub fn create_user<T: user::UserModel>(
    database: &T,
    username: &str,
    name: &str,
    email: &str,
) -> Result<user::UserWithIntegrations> {
    let username_is_available = database.check_if_username_available(username)?;
    let email_is_available = database.check_if_email_available(email)?;

    let new_user = user::NewUser {
        name: name.to_string(),
        username: username.to_string(),
        email: email.to_string(),
    };

    let user = match (username_is_available, email_is_available) {
        (true, true) => Ok(database
            .create_user(new_user)?
            .with_integrations(Vec::new())),
        _ => Err(UserServiceError::UserAlreadyExists),
    }?;

    refresh_login_code(database, email)?;

    Ok(user)
}

pub fn refresh_login_code<T: user::UserModel>(database: &T, email: &str) -> Result<()> {
    let last_code_gen_request = Utc::now().naive_utc();
    // TODO: Use login_code as a String to generate a code more dificult to crack
    let mut rng = rand::thread_rng();
    let login_code = rng.gen_range(100000..999999);
    send_code(&email, &login_code);
    database.refresh_login_code(email, login_code, last_code_gen_request)?;
    Ok(())
}

pub fn auth_and_delete_user<
    T: user::UserModel + transaction::TransactionModel + integration::IntegrationModel,
>(
    database: &T,
    token: &str,
    jwt_secret: &str,
) -> Result<user::UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    delete_user(database, id)
}

fn delete_user<
    T: user::UserModel + transaction::TransactionModel + integration::IntegrationModel,
>(
    database: &T,
    id: Uuid,
) -> Result<user::UserWithIntegrations> {
    database.delete_transaction_by_user_id(&id)?;
    let integrations = database.delete_integration_by_user_id(&id)?;
    Ok(database.delete_user(&id)?.with_integrations(integrations))
}

pub fn auth_and_get_user<T: user::UserModel + integration::IntegrationModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
) -> Result<user::UserWithIntegrations> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    get_user(database, id)
}

fn get_user<T: user::UserModel + integration::IntegrationModel>(
    database: &T,
    id: Uuid,
) -> Result<user::UserWithIntegrations> {
    let integrations = database.list_user_integrations(&id)?;
    Ok(database.get_user(id)?.with_integrations(integrations))
}

pub fn validate_and_generate_token<T: user::UserModel>(
    database: &T,
    email: String,
    login_code: i32,
    jwt_secret: &str,
    env: &Env,
) -> Result<String> {
    let real_login_code = database.get_login_code(&email)?;
    let id = database.get_id_by_email(&email)?;

    let token = jwt::generate_token(Utc::now().naive_utc(), &id, jwt_secret)?;

    match env {
        Env::DEV => Ok(token),
        Env::PROD => {
            if login_code == real_login_code {
                Ok(token)
            } else {
                Err(UserServiceError::LoginCodeNotMatching)
            }
        }
    }
}

pub fn auth_and_create_integration<T: user::UserModel + integration::IntegrationModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    name: String,
    time: NaiveDateTime,
) -> Result<integration::UserIntegration> {
    let id = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    create_integration(database, id, name, time)
}

fn create_integration<T: user::UserModel + integration::IntegrationModel>(
    database: &T,
    id: Uuid,
    name: String,
    time: NaiveDateTime,
) -> Result<integration::UserIntegration> {
    let new_integration = integration::NewUserIntegration {
        related_user: id,
        name,
        time,
    };
    Ok(database.create_integration(new_integration)?)
}

pub fn auth_and_delete_integration<T: user::UserModel + integration::IntegrationModel>(
    database: &T,
    token: &str,
    jwt_secret: &str,
    id: Uuid,
) -> Result<integration::UserIntegration> {
    let _ = jwt::verify_token(Utc::now().naive_utc(), token, jwt_secret)?;
    Ok(database.delete_integration(&id)?)
}

#[cfg(test)]
mod user_tests {

    use super::*;

    struct TestModel {}

    impl integration::IntegrationModel for TestModel {
        fn create_integration(
            &self,
            t: integration::NewUserIntegration,
        ) -> integration::Result<integration::UserIntegration> {
            Err(
                integration::IntegrationModelError::FailedToCreateIntegration(
                    diesel::result::Error::NotFound,
                ),
            )
        }
        fn list_user_integrations(
            &self,
            user_id: &Uuid,
        ) -> integration::Result<Vec<integration::UserIntegration>> {
            Ok(Vec::new())
        }
        fn delete_integration_by_user_id(
            &self,
            user_id: &Uuid,
        ) -> integration::Result<Vec<integration::UserIntegration>> {
            Ok(Vec::new())
        }
        fn delete_integration(
            &self,
            id: &Uuid,
        ) -> integration::Result<integration::UserIntegration> {
            Err(
                integration::IntegrationModelError::FailedToDeleteIntegration(
                    diesel::result::Error::NotFound,
                ),
            )
        }
    }

    impl transaction::TransactionModel for TestModel {
        fn create_transaction(
            &self,
            user_id: &Uuid,
            new_transaction: transaction::NewTransaction,
        ) -> transaction::Result<transaction::Transaction> {
            Err(
                transaction::TransactionModelError::FailedToCreateTransaction(
                    diesel::result::Error::NotFound,
                ),
            )
        }
        fn list_user_transactions(
            &self,
            user_id: &Uuid,
        ) -> transaction::Result<Vec<transaction::Transaction>> {
            Ok(Vec::new())
        }
        fn delete_transaction_by_user_id(&self, user_id: &Uuid) -> transaction::Result<()> {
            Ok(())
        }
    }

    impl user::UserModel for TestModel {
        fn create_user(&self, user: user::NewUser) -> user::Result<user::User> {
            Ok(user::User {
                id: Uuid::from_u128(160141200314647599499076565412518613020),
                name: user.name,
                username: user.username,
                register_date: None,
                email: user.email,
                last_code_gen_request: None,
                login_code: None,
                is_registered: true,
                payday: None,
            })
        }
        fn delete_user(&self, id: &Uuid) -> user::Result<user::User> {
            Ok(user::User {
                id: id.clone(),
                name: "Usuário 1".to_string(),
                username: "usuario1".to_string(),
                register_date: None,
                email: "usuario1@gmail.com".to_string(),
                last_code_gen_request: None,
                login_code: None,
                is_registered: true,
                payday: None,
            })
        }
        fn get_user(&self, id: Uuid) -> user::Result<user::User> {
            Ok(user::User {
                id: Uuid::from_u128(160141200314647599499076565412518613020),
                name: "Usuário 1".to_string(),
                username: "usuario1".to_string(),
                register_date: None,
                email: "usuario1@gmail.com".to_string(),
                last_code_gen_request: None,
                login_code: None,
                is_registered: true,
                payday: None,
            })
        }
        fn check_if_username_available(&self, username: &str) -> user::Result<bool> {
            match username {
                "usuario3" | "usuario4" => Ok(false),
                "usuario1" | "usuario2" | _ => Ok(true),
            }
        }
        fn check_if_email_available(&self, email: &str) -> user::Result<bool> {
            match email {
                "usuario2@gmail.com" | "usuario4@gmail.com" => Ok(false),
                "usuario1@gmail.com" | "usuario3@gmail.com" | _ => Ok(true),
            }
        }
        fn refresh_login_code(
            &self,
            email: &str,
            login_code: i32,
            time: NaiveDateTime,
        ) -> user::Result<()> {
            Ok(())
        }
        fn get_login_code(&self, email: &str) -> user::Result<i32> {
            Ok(123123)
        }
        fn get_id_by_email(&self, email: &str) -> user::Result<Uuid> {
            Ok(Uuid::from_u128(160141200314647599499076565412518613020))
        }
    }

    #[test]
    fn try_create_new_user() -> Result<()> {
        let created_user =
            create_user(&TestModel {}, "usuario1", "Usuário 1", "usuario1@gmail.com")?;
        let expected_user = user::UserWithIntegrations {
            id: Uuid::from_u128(160141200314647599499076565412518613020),
            name: "Usuário 1".to_string(),
            username: "usuario1".to_string(),
            register_date: None,
            email: "usuario1@gmail.com".to_string(),
            last_code_gen_request: None,
            login_code: None,
            is_registered: true,
            payday: None,
            integrations: Vec::new(),
        };
        assert_eq!(created_user, expected_user);
        Ok(())
    }

    #[test]
    fn try_create_user_with_taken_email() {
        assert!(create_user(&TestModel {}, "usuario2", "Usuário 2", "usuario2@gmail.com").is_err());
    }

    #[test]
    fn try_create_user_with_taken_username() {
        assert!(create_user(&TestModel {}, "usuario3", "Usuário 3", "usuario3@gmail.com").is_err());
    }

    #[test]
    fn try_create_user_with_taken_email_username() {
        assert!(create_user(&TestModel {}, "usuario4", "Usuário 4", "usuario4@gmail.com").is_err());
    }

    #[test]
    fn try_delete_user() -> Result<()> {
        let user_id = Uuid::from_u128(160141200314647599499076565412518613020);
        let created_user = delete_user(&TestModel {}, user_id)?;
        let expected_user = user::UserWithIntegrations {
            id: user_id,
            name: "Usuário 1".to_string(),
            username: "usuario1".to_string(),
            register_date: None,
            email: "usuario1@gmail.com".to_string(),
            last_code_gen_request: None,
            login_code: None,
            is_registered: true,
            payday: None,
            integrations: Vec::new(),
        };
        assert_eq!(created_user, expected_user);
        Ok(())
    }

    #[test]
    fn try_get_user() -> Result<()> {
        let user_id = Uuid::from_u128(160141200314647599499076565412518613020);
        let created_user = get_user(&TestModel {}, user_id)?;
        let expected_user = user::UserWithIntegrations {
            id: user_id,
            name: "Usuário 1".to_string(),
            username: "usuario1".to_string(),
            register_date: None,
            email: "usuario1@gmail.com".to_string(),
            last_code_gen_request: None,
            login_code: None,
            is_registered: true,
            payday: None,
            integrations: Vec::new(),
        };
        assert_eq!(created_user, expected_user);
        Ok(())
    }

    #[test]
    fn try_validate_and_generate_token() -> Result<()> {
        assert!(validate_and_generate_token(
            &TestModel {},
            "usuario1".to_string(),
            123123,
            "XXXXXXXXXXXXXXX",
            &Env::PROD,
        )
        .is_ok());
        Ok(())
    }

    #[test]
    fn try_validate_and_generate_token_with_wrong_code() -> Result<()> {
        assert!(validate_and_generate_token(
            &TestModel {},
            "usuario1".to_string(),
            321321,
            "XXXXXXXXXXXXXXX",
            &Env::PROD,
        )
        .is_err());
        Ok(())
    }
}
