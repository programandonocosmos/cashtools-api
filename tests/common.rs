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
