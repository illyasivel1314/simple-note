use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::PooledConnection;

mod calendar;
mod index;
pub mod interface;
pub mod schema;

type SqlitePool = PooledConnection<ConnectionManager<SqliteConnection>>;
