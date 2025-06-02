use crate::configuration::database::SqlitePool;
use crate::configuration::utils::error_util::{AppError, DatabaseError};
use crate::configuration::utils::file_util;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::{Error, Pool, PooledConnection};
use std::env;
use std::sync::LazyLock;

// sql文件地址
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./resources/migrations");

// 线程池局部全局变量
static DATABASE_CONNECTION: LazyLock<Pool<ConnectionManager<SqliteConnection>>> =
    LazyLock::new(create_database_connection);

/**
 * @description: 建立数据库连接，并创建数据库线程池
 * @author: illya
 * @date: 2025/5/22 14:32
 **/
pub fn create_database_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    // 创建数据库连接
    let connection = ConnectionManager::<SqliteConnection>::new(file_util::acquire_database_url());
    // 创建线程池
    match Pool::builder().build(connection) {
        Ok(pool) => pool,
        Err(err) => panic!("{}", DatabaseError::DatabasePoolCreateError(err)),
    }
}

/**
 * @description: 创建数据库
 * @author: illya
 * @date: 2025/5/11 13:27
 **/
pub fn system_database_init(database_url: String) -> Result<(), AppError> {
    // 判断文件是否存在
    if file_util::file_valid(database_url.as_str()) {
        return Ok(());
    }
    // 创建文件及其父文件
    file_util::create_file(file_util::acquire_file_path(database_url.as_str()))?;
    // 进行数据库迁移
    acquire_database_pool()
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| {
            // 删除文件
            file_util::delete_file(file_util::acquire_file_path(database_url.as_str())).unwrap();
            DatabaseError::DatabaseMigrationsError(err)
        })?;
    Ok(())
}

/**
 * @description: 获取数据库线程池
 * @author: illya
 * @date: 2025/5/14 18:48
 **/
pub fn acquire_database_pool() -> SqlitePool {
    match (*DATABASE_CONNECTION).get() {
        Ok(pool) => pool,
        Err(err) => panic!("{}", DatabaseError::DatabaseConnectionError(err)),
    }
}
