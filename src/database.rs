use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pooled_connection(database_url: String) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
