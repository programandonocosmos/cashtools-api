use chrono::{NaiveDate, NaiveDateTime};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject};
use uuid::Uuid;

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

    fn transactions(context: &Context, user_uid: String) -> FieldResult<Vec<Transaction>> {
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

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
