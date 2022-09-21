use chrono::{NaiveDate, NaiveDateTime};
use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLObject};
use uuid::Uuid;

use crate::database;
use crate::services;

// A Cashtools user.
#[derive(GraphQLObject, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    register_date: Option<NaiveDateTime>,
    email: String,
    last_code_gen_request: Option<NaiveDateTime>,
    login_code: Option<i32>,
    is_registered: bool,
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

impl services::user::User {
    fn convert(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            register_date: self.register_date,
            email: self.email.clone(),
            last_code_gen_request: self.last_code_gen_request,
            login_code: self.login_code,
            is_registered: self.is_registered,
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
    async fn apiVersion() -> &'static str {
        "1.0"
    }

    async fn transactions(context: &Context, user_uid: String) -> FieldResult<Vec<Transaction>> {
        let parsed_user_uid = Uuid::parse_str(&user_uid)?;
        let transactions =
            services::transaction::list_user_transactions(&context.pool, parsed_user_uid)?
                .iter()
                .map(|t| Transaction {
                    id: t.id,
                    related_user: t.related_user,
                    entry_date: t.entry_date,
                    entry_account_code: t.clone().entry_account_code,
                    exit_account_code: t.clone().exit_account_code,
                    amount: t.amount,
                    description: t.clone().description,
                })
                .collect();
        Ok(transactions)
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
    async fn create_user(context: &Context, username: String, email: String) -> FieldResult<User> {
        let created_user = services::user::create_user(&context.pool, username, email)?;
        Ok(created_user.convert())
    }
    fn delete_user(context: &Context, token: String) -> FieldResult<User> {
        let user = services::user::delete_user(&context.pool, token, &context.jwt_secret)?;
        Ok(user.convert())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutations, EmptySubscription<Context>>;
