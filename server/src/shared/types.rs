use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

/// A type alias for a connection pool to the Postgres database
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// A type alias for a connection to the Postgres database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;
