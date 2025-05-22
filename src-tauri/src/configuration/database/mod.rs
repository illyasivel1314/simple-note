use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::PooledConnection;

pub mod index;
pub mod data;

type SqlitePool = PooledConnection<ConnectionManager<SqliteConnection>>;
