use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject};

#[derive(Clone, Copy)]
struct MockedDatabase {}
impl MockedDatabase {
    fn get(&self, name: &str) -> Transaction {
        Transaction {
            id: name.to_string(),
            date: 121212,
            entry_account_code: "1".to_string(),
            exit_account_code: "2".to_string(),
            amount: 150.0,
            description: "A very expensive banana".to_string(),
        }
    }
    fn new() -> Self {
        MockedDatabase {}
    }
}

#[derive(GraphQLObject, Clone)]
#[graphql(description = "A simple transaction.")]
struct Transaction {
    id: String,
    date: i32,
    entry_account_code: String,
    exit_account_code: String,
    amount: f64,
    description: String,
}

pub struct Context {
    pool: MockedDatabase,
}

impl juniper::Context for Context {}
impl Context {
    pub fn new() -> Self {
        Context {
            pool: MockedDatabase::new(),
        }
    }
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn transaction(context: &Context, id: String) -> FieldResult<Transaction> {
        let connection = context.pool;
        Ok(connection.get(&id))
    }
}

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
