use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::PooledConnection;

pub mod data;
pub mod index;

type SqlitePool = PooledConnection<ConnectionManager<SqliteConnection>>;
