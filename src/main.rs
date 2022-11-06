use juniper::EmptySubscription;
use rocket::{response::content, State};

use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate rocket;

mod database;
mod entities;
mod graphql_resolvers;
mod jwt;
mod models;
mod schema;
mod sendemail;
mod services;

#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<graphql_resolvers::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql_resolvers::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<graphql_resolvers::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql_resolvers::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let env = match env::var("ENV").expect("ENV must be set").as_str() {
        "DEV" => entities::Env::DEV,
        "PROD" => entities::Env::PROD,
        _ => panic!("ENV must be DEV or PROD"),
    };

    let pool = database::establish_pooled_connection(database_url);

    let figment = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "0.0.0.0"));

    let context = graphql_resolvers::Context {
        pool,
        jwt_secret,
        env,
    };

    let schema = graphql_resolvers::Schema::new(
        graphql_resolvers::Query,
        graphql_resolvers::Mutations,
        EmptySubscription::<graphql_resolvers::Context>::new(),
    );

    let routes = rocket::routes![graphiql, get_graphql_handler, post_graphql_handler];

    rocket::custom(figment)
        .manage(context)
        .manage(schema)
        .mount("/", routes)
}
