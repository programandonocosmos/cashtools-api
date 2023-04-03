use chrono::NaiveDateTime;
use uuid::Uuid;

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

// Model-related things

#[derive(Debug)]
pub enum IntegrationModelError {
    FailedToGetConn(r2d2::Error),
    FailedToCreateIntegration(diesel::result::Error),
    FailedToListIntegrations(diesel::result::Error),
    FailedToDeleteIntegration(diesel::result::Error),
}

impl From<r2d2::Error> for IntegrationModelError {
    fn from(error: r2d2::Error) -> Self {
        IntegrationModelError::FailedToGetConn(error)
    }
}

pub type Result<T> = std::result::Result<T, IntegrationModelError>;

pub trait IntegrationModel {
    fn create_integration(&self, t: NewUserIntegration) -> Result<UserIntegration>;
    fn list_user_integrations(&self, user_id: &Uuid) -> Result<Vec<UserIntegration>>;
    fn delete_integration_by_user_id(&self, user_id: &Uuid) -> Result<Vec<UserIntegration>>;
    fn delete_integration(&self, id: &Uuid) -> Result<UserIntegration>;
}
