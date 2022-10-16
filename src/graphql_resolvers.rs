use chrono::{NaiveDate, NaiveDateTime};
use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::database;
use crate::services;

#[derive(GraphQLObject, Clone)]
pub struct Integration {
    id: Uuid,
    name: String,
    time: NaiveDateTime,
}

// A Cashtools user.
#[derive(GraphQLObject, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    name: String,
    email: String,
    integrations: Vec<Integration>,
    payday: Option<i32>,
}

// A simple transaction.
#[derive(GraphQLObject, Clone)]
struct Transaction {
    id: Uuid,
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<String>,
    exit_account_code: Option<String>,
    amount: f64,
    description: Option<String>,
}

// An input transaction.
#[derive(GraphQLInputObject, Clone)]
struct NewTransaction {
    entry_date: NaiveDate,
    entry_account_code: Option<String>,
    exit_account_code: Option<String>,
    amount: f64,
    description: Option<String>,
}

impl services::user::UserIntegration {
    fn to_graphql(&self) -> Integration {
        Integration {
            id: self.id,
            name: self.name.clone(),
            time: self.time,
        }
    }
}

impl services::user::UserWithIntegrations {
    fn to_graphql(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            integrations: self.integrations.iter().map(|t| t.to_graphql()).collect(),
            payday: self.payday,
        }
    }
}

impl services::transaction::Transaction {
    fn to_graphql(&self) -> Transaction {
        Transaction {
            id: self.id,
            related_user: self.related_user,
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

impl NewTransaction {
    fn to_service(&self) -> services::transaction::NewTransaction {
        services::transaction::NewTransaction {
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

pub struct Context {
    pub pool: database::DbPool,
    pub jwt_secret: String,
}
impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    async fn transactions(context: &Context, token: String) -> FieldResult<Vec<Transaction>> {
        let transactions = services::transaction::list_user_transactions(
            &context.pool,
            &token,
            &context.jwt_secret,
        )?
        .iter()
        .map(|t| t.to_graphql())
        .collect();
        Ok(transactions)
    }

    async fn me(context: &Context, token: String) -> FieldResult<User> {
        let user = services::user::get_user(&context.pool, &token, &context.jwt_secret)?;
        Ok(user.to_graphql())
    }

    async fn token(context: &Context, email: String, login_code: i32) -> FieldResult<String> {
        let token = services::user::validate_and_generate_token(
            &context.pool,
            email,
            login_code,
            &context.jwt_secret,
        )?;
        Ok(token)
    }
}

pub struct Mutations;

#[graphql_object(context = Context)]
impl Mutations {
    async fn create_user(
        context: &Context,
        username: String,
        name: String,
        email: String,
    ) -> FieldResult<User> {
        let created_user = services::user::create_user(&context.pool, &username, &name, &email)?;
        Ok(created_user.to_graphql())
    }

    async fn send_login_code(context: &Context, email: String) -> FieldResult<String> {
        services::user::refresh_login_code(&context.pool, &email)?;
        Ok(email)
    }

    async fn delete_user(context: &Context, token: String) -> FieldResult<User> {
        let user = services::user::delete_user(&context.pool, &token, &context.jwt_secret)?;
        Ok(user.to_graphql())
    }
    async fn create_transaction(
        context: &Context,
        token: String,
        transaction: NewTransaction,
    ) -> FieldResult<Transaction> {
        let created_transaction = services::transaction::create_transaction(
            &context.pool,
            &token,
            &context.jwt_secret,
            transaction.to_service(),
        )?;
        Ok(created_transaction.to_graphql())
    }

    async fn create_integration(
        context: &Context,
        token: String,
        name: String,
        time: NaiveDateTime,
    ) -> FieldResult<Integration> {
        let created_integration = services::user::create_integration(
            &context.pool,
            &token,
            &context.jwt_secret,
            name,
            time,
        )?;
        Ok(created_integration.to_graphql())
    }

    async fn delete_integration(
        context: &Context,
        token: String,
        id: Uuid,
    ) -> FieldResult<Integration> {
        let integration =
            services::user::delete_integration(&context.pool, &token, &context.jwt_secret, id)?;

        Ok(integration.to_graphql())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutations, EmptySubscription<Context>>;
