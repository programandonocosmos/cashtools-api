use cashtools::database;
use chrono::NaiveDateTime;
use std::env;

pub fn make_conn() -> database::DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    database::establish_pooled_connection(database_url)
}
pub fn now() -> NaiveDateTime {
    const NOW: i64 = 1665366563127;
    NaiveDateTime::from_timestamp(NOW, 0)
}

pub const DEFAULT_MESSAGE: &str = "It's an integration test. If all the unit tests are passing but this one is not, there's probably a problem with the database. Make sure the instance is running. If you are trying to use the remote database, make sure the proxy is running too.";
