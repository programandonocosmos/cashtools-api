use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{database, entities::user, schema::user_integrations as user_integration_schema};

#[derive(Queryable, Clone)]
#[diesel(table_name = user_integration_schema)]
struct UserIntegration {
    id: Uuid,
    related_user: Uuid,
    name: String,
    time: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = user_integration_schema)]
struct NewUserIntegration {
    related_user: Uuid,
    name: String,
    time: NaiveDateTime,
}

impl user::NewUserIntegration {
    fn to_model(&self) -> NewUserIntegration {
        NewUserIntegration {
            related_user: self.related_user.clone(),
            name: self.name.clone(),
            time: self.time,
        }
    }
}

impl UserIntegration {
    fn to_entity(&self) -> user::UserIntegration {
        user::UserIntegration {
            id: self.id.clone(),
            related_user: self.related_user.clone(),
            name: self.name.clone(),
            time: self.time,
        }
    }
}

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

pub fn create_integration(
    conn: &database::DbPool,
    t: user::NewUserIntegration,
) -> Result<user::UserIntegration> {
    diesel::insert_into(user_integration_schema::table)
        .values(&t.to_model())
        .get_result::<UserIntegration>(&mut conn.get()?)
        .map(|t| t.to_entity())
        .map_err(IntegrationModelError::FailedToCreateIntegration)
}

pub fn list_user_integrations(
    conn: &database::DbPool,
    user_id: &Uuid,
) -> Result<Vec<user::UserIntegration>> {
    Ok(user_integration_schema::table
        .filter(user_integration_schema::related_user.eq(user_id))
        .load::<UserIntegration>(&mut conn.get()?)
        .map_err(IntegrationModelError::FailedToListIntegrations)?
        .iter()
        .map(|t| t.to_entity())
        .collect())
}

pub fn delete_integration_by_user_id(
    conn: &database::DbPool,
    user_id: &Uuid,
) -> Result<Vec<user::UserIntegration>> {
    Ok(diesel::delete(
        user_integration_schema::table.filter(user_integration_schema::related_user.eq(user_id)),
    )
    .get_results::<UserIntegration>(&mut conn.get()?)
    .map_err(IntegrationModelError::FailedToDeleteIntegration)?
    .iter()
    .map(|t| t.to_entity())
    .collect())
}

pub fn delete_integration(conn: &database::DbPool, id: &Uuid) -> Result<user::UserIntegration> {
    Ok(
        diesel::delete(user_integration_schema::table.filter(user_integration_schema::id.eq(id)))
            .get_result::<UserIntegration>(&mut conn.get()?)
            .map_err(IntegrationModelError::FailedToDeleteIntegration)?
            .to_entity(),
    )
}
