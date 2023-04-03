use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    database, entities::integration, schema::user_integrations as user_integration_schema,
};

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

impl integration::NewUserIntegration {
    fn to_model(&self) -> NewUserIntegration {
        NewUserIntegration {
            related_user: self.related_user.clone(),
            name: self.name.clone(),
            time: self.time,
        }
    }
}

impl UserIntegration {
    fn to_entity(&self) -> integration::UserIntegration {
        integration::UserIntegration {
            id: self.id.clone(),
            related_user: self.related_user.clone(),
            name: self.name.clone(),
            time: self.time,
        }
    }
}

impl integration::IntegrationModel for database::DbPool {
    fn create_integration(
        &self,
        t: integration::NewUserIntegration,
    ) -> integration::Result<integration::UserIntegration> {
        diesel::insert_into(user_integration_schema::table)
            .values(&t.to_model())
            .get_result::<UserIntegration>(&mut self.get()?)
            .map(|t| t.to_entity())
            .map_err(integration::IntegrationModelError::FailedToCreateIntegration)
    }

    fn list_user_integrations(
        &self,
        user_id: &Uuid,
    ) -> integration::Result<Vec<integration::UserIntegration>> {
        Ok(user_integration_schema::table
            .filter(user_integration_schema::related_user.eq(user_id))
            .load::<UserIntegration>(&mut self.get()?)
            .map_err(integration::IntegrationModelError::FailedToListIntegrations)?
            .iter()
            .map(|t| t.to_entity())
            .collect())
    }

    fn delete_integration_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> integration::Result<Vec<integration::UserIntegration>> {
        Ok(diesel::delete(
            user_integration_schema::table
                .filter(user_integration_schema::related_user.eq(user_id)),
        )
        .get_results::<UserIntegration>(&mut self.get()?)
        .map_err(integration::IntegrationModelError::FailedToDeleteIntegration)?
        .iter()
        .map(|t| t.to_entity())
        .collect())
    }

    fn delete_integration(&self, id: &Uuid) -> integration::Result<integration::UserIntegration> {
        Ok(diesel::delete(
            user_integration_schema::table.filter(user_integration_schema::id.eq(id)),
        )
        .get_result::<UserIntegration>(&mut self.get()?)
        .map_err(integration::IntegrationModelError::FailedToDeleteIntegration)?
        .to_entity())
    }
}
