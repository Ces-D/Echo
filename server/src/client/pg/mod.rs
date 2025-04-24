use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

use crate::shared::config::{get_env_var, Environment};

/// Track models and database functions
pub mod track;
/// User models and database functions
pub mod user;
/// User Track models and database functions
pub mod user_track;

/// A type alias for a connection pool to the Postgres database
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// A type alias for a connection to the Postgres database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates a connection pool to the Postgres database
pub fn database_pool() -> DbPool {
    let database_uri = get_env_var(Environment::DatabaseUrl);

    // Create a connection pool to the Postgres database
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB")
}
