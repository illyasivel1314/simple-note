use crate::configuration::database::{calendar, index, SqlitePool};
use crate::configuration::utils::error_util::AppError;

// 数据库对外接口
pub trait DatabaseImpl {
    /**
     * @description: 初始化数据库
     * @author: illya
     * @date: 2025/5/16 13:46
     **/
    fn system_database_init(&self) -> Result<(), AppError>;

    /**
     * @description: 获取数据库连接
     * @author: illya
     * @date: 2025/5/16 13:46
     **/
    fn acquire_connection(&self) -> SqlitePool;

    /**
     * @description: 初始化节点表
     * @author: illya
     * @date: 2025/5/16 13:46
     **/
    async fn holiday_database_init(&self) -> Result<(), AppError>;
}

#[derive(Debug, Default)]
pub struct DatabaseUtil;
impl DatabaseImpl for DatabaseUtil {
    fn system_database_init(&self) -> Result<(), AppError> {
        index::system_database_init()
    }

    fn acquire_connection(&self) -> SqlitePool {
        match index::acquire_database_connection() {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        }
    }

    async fn holiday_database_init(&self) -> Result<(), AppError> {
        calendar::update_holiday_to_database().await
    }
}
