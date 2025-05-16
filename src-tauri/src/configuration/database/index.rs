use crate::configuration::database::{calendar, SqlitePool};
use crate::configuration::utils::error_util::{AppError, DatabaseError};
use crate::configuration::utils::file_util;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;
use std::sync::LazyLock;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

// sqlite数据库文件地址
const DATABASE_FILE_ADDRESS: &str = "./resources/database.sqlite";
// sql文件地址
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./resources/migrations");

// 线程池局部全局变量
static DATABASE_CONNECTION: LazyLock<Pool<ConnectionManager<SqliteConnection>>> =
    LazyLock::new(|| {
        // 创建数据库连接
        let connection = ConnectionManager::<SqliteConnection>::new(DATABASE_FILE_ADDRESS);
        // 创建线程池
        match Pool::builder().build(connection) {
            Ok(pool) => pool,
            Err(err) => panic!("{}", DatabaseError::DatabasePoolCreateError(err)),
        }
    });

/**
 * @description: 创建数据库相关文件
 * @author: illya
 * @date: 2025/5/11 13:27
 **/
pub fn system_database_init() -> Result<(), AppError> {
    let file_path = file_util::acquire_file_path(DATABASE_FILE_ADDRESS);
    // 判断文件是否存在
    if file_util::file_valid(file_path) {
        return Ok(());
    }
    // 创建文件及其父文件
    file_util::create_file(file_path)?;
    // 进行数据库迁移
    acquire_database_connection()?
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| DatabaseError::DatabaseMigrationsError(err))?;
    // 初始化数据
    block_in_place(move || {
        Handle::current().block_on(async { calendar::update_holiday_to_database().await })
    })?;
    Ok(())
}

/**
 * @description: 获取数据库线程池
 * @author: illya
 * @date: 2025/5/14 18:48
 **/
pub fn acquire_database_connection() -> Result<SqlitePool, AppError> {
    let sqlite_pool = (*DATABASE_CONNECTION)
        .get()
        .map_err(|err| DatabaseError::DatabaseConnectionError(err))?;
    Ok(sqlite_pool)
}
