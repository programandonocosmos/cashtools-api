use chrono::{NaiveDate, NaiveDateTime};
use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
};
use log;
use uuid::Uuid;

use crate::database;
use crate::entities;
use crate::services;

#[derive(GraphQLEnum, Clone)]
pub enum Order {
    ASC,
    DESC,
}

#[derive(GraphQLObject, Clone, Debug)]
pub struct Integration {
    id: Uuid,
    name: String,
    time: NaiveDateTime,
}

// A Cashtools user.
#[derive(GraphQLObject, Clone, Debug)]
pub struct User {
    id: Uuid,
    username: String,
    name: String,
    email: String,
    integrations: Vec<Integration>,
    payday: Option<i32>,
}

// A simple transaction.
#[derive(GraphQLObject, Clone, Debug)]
struct Transaction {
    id: Uuid,
    related_user: Uuid,
    entry_date: NaiveDate,
    entry_account_code: Option<Uuid>,
    entry_account_name: Option<String>,
    exit_account_code: Option<Uuid>,
    exit_account_name: Option<String>,
    amount: f64,
    description: Option<String>,
}

// An input transaction.
#[derive(GraphQLInputObject, Clone, Debug)]
struct NewTransaction {
    entry_date: NaiveDate,
    entry_account_code: Option<Uuid>,
    exit_account_code: Option<Uuid>,
    amount: f64,
    description: Option<String>,
}

#[derive(GraphQLEnum, Clone, Copy, Debug)]
enum EarningIndex {
    CDI,
    FIXED,
    IPCA,
}

#[derive(GraphQLObject, Clone, Debug)]
struct Earning {
    rate: f64,
    index: EarningIndex,
}

#[derive(GraphQLInputObject, Clone, Copy, Debug)]
struct EarningInput {
    rate: f64,
    index: EarningIndex,
}

#[derive(GraphQLObject, Clone, Debug)]
struct PreAllocation {
    amount: f64,
    accumulative: bool,
}

#[derive(GraphQLInputObject, Clone, Copy, Debug)]
struct PreAllocationInput {
    amount: Option<f64>,
    accumulative: Option<bool>,
}

// Account fields that can be updated.
#[derive(GraphQLInputObject, Clone, Debug)]
struct UpdatedAccount {
    name: Option<String>,
    description: Option<String>,
    pre_allocation: Option<PreAllocationInput>,
    earning: Option<EarningInput>,
    is_available: Option<bool>,
    in_trash: Option<bool>,
}

// An input account.
#[derive(GraphQLInputObject, Clone, Debug)]
struct NewAccount {
    time: NaiveDateTime,
    initial_balance: f64,
    name: String,
    description: Option<String>,
    pre_allocation: Option<PreAllocationInput>,
    earning: Option<EarningInput>,
    is_available: bool,
}

// A simple account.
#[derive(GraphQLObject, Clone, Debug)]
struct Account {
    id: Uuid,
    time: NaiveDateTime,
    name: String,
    description: Option<String>,
    balance: f64,
    pre_allocation: Option<PreAllocation>,
    earning: Option<Earning>,
    is_available: bool,
    in_trash: bool,
}

impl entities::integration::UserIntegration {
    fn to_graphql(&self) -> Integration {
        Integration {
            id: self.id,
            name: self.name.clone(),
            time: self.time,
        }
    }
}

impl entities::user::UserWithIntegrations {
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

impl entities::transaction::TransactionWithNames {
    fn to_graphql(&self) -> Transaction {
        Transaction {
            id: self.id,
            related_user: self.related_user,
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code,
            entry_account_name: self.entry_account_name.clone(),
            exit_account_code: self.exit_account_code,
            exit_account_name: self.exit_account_name.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

impl entities::account::EarningIndex {
    fn to_graphql(&self) -> EarningIndex {
        match self {
            entities::account::EarningIndex::CDI => EarningIndex::CDI,
            entities::account::EarningIndex::FIXED => EarningIndex::FIXED,
            entities::account::EarningIndex::IPCA => EarningIndex::IPCA,
        }
    }
}

impl entities::account::PreAllocation {
    fn to_graphql(&self) -> PreAllocation {
        PreAllocation {
            amount: self.amount,
            accumulative: self.accumulative,
        }
    }
}

impl entities::account::Earning {
    fn to_graphql(&self) -> Earning {
        Earning {
            rate: self.rate,
            index: self.index.to_graphql(),
        }
    }
}

impl entities::account::Account {
    fn to_graphql(&self) -> Account {
        Account {
            id: self.id,
            time: self.time,
            name: self.name.clone(),
            description: self.description.clone(),
            balance: self.balance,
            pre_allocation: self.pre_allocation.map(|x| x.to_graphql()),
            earning: self.earning.map(|x| x.to_graphql()),
            is_available: self.is_available,
            in_trash: self.in_trash,
        }
    }
}

impl UpdatedAccount {
    fn to_entity(&self) -> entities::account::UpdatedAccount {
        entities::account::UpdatedAccount {
            name: self.name.clone(),
            description: self.description.clone(),
            pre_allocation: self.pre_allocation.and_then(|x| x.to_entity()),
            earning: self.earning.map(|x| x.to_entity()),
            is_available: self.is_available,
            in_trash: self.in_trash,
        }
    }
}

impl NewTransaction {
    fn to_entity(&self) -> entities::transaction::NewTransaction {
        entities::transaction::NewTransaction {
            entry_date: self.entry_date,
            entry_account_code: self.entry_account_code.clone(),
            exit_account_code: self.exit_account_code.clone(),
            amount: self.amount,
            description: self.description.clone(),
        }
    }
}

impl PreAllocationInput {
    fn to_entity(&self) -> Option<entities::account::PreAllocation> {
        match (self.amount, self.accumulative) {
            (Some(amount), Some(accumulative)) => Some(entities::account::PreAllocation {
                amount,
                accumulative,
            }),
            _ => None,
        }
    }
}

impl EarningIndex {
    fn to_entity(&self) -> entities::account::EarningIndex {
        match self {
            EarningIndex::CDI => entities::account::EarningIndex::CDI,
            EarningIndex::FIXED => entities::account::EarningIndex::FIXED,
            EarningIndex::IPCA => entities::account::EarningIndex::IPCA,
        }
    }
}

impl EarningInput {
    fn to_entity(&self) -> entities::account::Earning {
        entities::account::Earning {
            rate: self.rate,
            index: self.index.to_entity(),
        }
    }
}

impl NewAccount {
    fn to_entity(&self) -> entities::account::NewAccount {
        entities::account::NewAccount {
            time: self.time,
            initial_balance: self.initial_balance,
            name: self.name.clone(),
            description: self.description.clone(),
            pre_allocation: self.pre_allocation.and_then(|x| x.to_entity()),
            earning: self.earning.map(|x| x.to_entity()),
            is_available: self.is_available,
        }
    }
}

pub struct Context {
    pub pool: database::DbPool,
    pub jwt_secret: String,
    pub env: entities::Env,
}
impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    async fn account(context: &Context, token: String, id: Uuid) -> FieldResult<Account> {
        let account = services::account::auth_and_get_account(
            &context.pool,
            &token,
            &context.jwt_secret,
            &id,
        )?;
        Ok(account.to_graphql())
    }
    async fn accounts(
        context: &Context,
        token: String,
        is_pre_allocation: Option<bool>,
        in_trash: Option<bool>,
        tags: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Account>> {
        let accounts = services::account::auth_and_get_accounts(
            &context.pool,
            &token,
            &context.jwt_secret,
            is_pre_allocation,
            in_trash,
            tags,
        )?
        .iter()
        .map(|t| t.to_graphql())
        .collect();
        Ok(accounts)
    }

    async fn transactions(context: &Context, token: String) -> FieldResult<Vec<Transaction>> {
        let transactions = services::transaction::auth_and_list_user_transactions(
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
        let user = services::user::auth_and_get_user(&context.pool, &token, &context.jwt_secret)?;
        Ok(user.to_graphql())
    }

    async fn token(context: &Context, email: String, login_code: i32) -> FieldResult<String> {
        let token = services::user::validate_and_generate_token(
            &context.pool,
            email,
            login_code,
            &context.jwt_secret,
            &context.env,
        )?;
        Ok(token)
    }
}

pub struct Mutations;

#[graphql_object(context = Context)]
impl Mutations {
    async fn create_account(
        context: &Context,
        token: String,
        account: NewAccount,
    ) -> FieldResult<Account> {
        println!("Received new account");
        log::debug!("New account: {:?}", account);
        let account = services::account::auth_and_create_account(
            &context.pool,
            &token,
            &context.jwt_secret,
            account.to_entity(),
        )?;
        Ok(account.to_graphql())
    }

    async fn edit_account(
        context: &Context,
        token: String,
        id: Uuid,
        updated_account: UpdatedAccount,
    ) -> FieldResult<Account> {
        let account = services::account::auth_and_edit_account(
            &context.pool,
            &token,
            &context.jwt_secret,
            &id,
            updated_account.to_entity(),
        )?;
        Ok(account.to_graphql())
    }

    async fn delete_account(context: &Context, token: String, id: Uuid) -> FieldResult<Uuid> {
        let _ = services::account::auth_and_delete_account(
            &context.pool,
            &token,
            &context.jwt_secret,
            &id,
        )?;
        Ok(id)
    }

    async fn pre_allocate(
        context: &Context,
        token: String,
        time: NaiveDate,
        from: Uuid,
        to: Uuid,
        amount: f64,
        accumulative: bool,
    ) -> FieldResult<PreAllocation> {
        let pre_allocation_obj = services::account::auth_and_preallocate(
            &context.pool,
            &token,
            &context.jwt_secret,
            time,
            &from,
            &to,
            amount,
            accumulative,
        )?;
        Ok(pre_allocation_obj.to_graphql())
    }

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
        let user =
            services::user::auth_and_delete_user(&context.pool, &token, &context.jwt_secret)?;
        Ok(user.to_graphql())
    }
    async fn create_transaction(
        context: &Context,
        token: String,
        transaction: NewTransaction,
    ) -> FieldResult<Transaction> {
        let created_transaction = services::transaction::auth_and_create_transaction(
            &context.pool,
            &token,
            &context.jwt_secret,
            transaction.to_entity(),
        )?;
        Ok(created_transaction.to_graphql())
    }

    async fn create_integration(
        context: &Context,
        token: String,
        name: String,
        time: NaiveDateTime,
    ) -> FieldResult<Integration> {
        let created_integration = services::user::auth_and_create_integration(
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
        let integration = services::user::auth_and_delete_integration(
            &context.pool,
            &token,
            &context.jwt_secret,
            id,
        )?;

        Ok(integration.to_graphql())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutations, EmptySubscription<Context>>;
