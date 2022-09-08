use chrono::{NaiveDate, NaiveDateTime};
use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLObject};
use uuid::Uuid;

use crate::models;
use crate::services;

// A My Money user.
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

pub struct Context {}
impl juniper::Context for Context {}
impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn transactions(user_uid: String) -> FieldResult<Vec<Transaction>> {
        let parsed_user_uid = Uuid::parse_str(&user_uid)?;
        let transactions = services::transaction::list_user_transactions(parsed_user_uid)
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
}

pub struct Mutations;

#[graphql_object(context = Context)]
impl Mutations {
    fn create_user(username: String, email: String) -> FieldResult<User> {
        let user = models::user::NewUser {
            username,
            register_date: None,
            email,
            last_code_gen_request: None,
            login_code: None,
        };
        let created_user = services::user::create_user(user);
        Ok(User {
            id: created_user.id,
            username: created_user.username,
            register_date: created_user.register_date,
            email: created_user.email,
            last_code_gen_request: created_user.last_code_gen_request,
            login_code: created_user.login_code,
            is_registered: created_user.is_registered,
        })
    }
    fn delete_user(id: Uuid) -> FieldResult<User> {
        let user = services::user::delete_user(id);
        Ok(User {
            id: user.id,
            username: user.username,
            register_date: user.register_date,
            email: user.email,
            last_code_gen_request: user.last_code_gen_request,
            login_code: user.login_code,
            is_registered: user.is_registered,
        })
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutations, EmptySubscription<Context>>;
