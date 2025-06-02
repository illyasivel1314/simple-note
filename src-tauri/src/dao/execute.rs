use diesel::prelude::*;
use diesel::QueryId;
use crate::configuration::database::data::schema::execute_table;
use crate::configuration::database::index::acquire_database_pool;
use crate::configuration::utils::error_util::DatabaseError;

#[derive(Queryable, Selectable, QueryId, Insertable, AsChangeset)]
#[diesel(table_name = crate::configuration::database::data::schema::execute_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug, Default)]
pub struct ExecuteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 操作的日期字符串
    pub timestamp: i64,
}

/**
 * @description: 查询某一范围内的数据
 * @author: illya 
 * @date: 2025/6/3 00:26
 **/
pub fn acquire_execute_within(start: i64, end: i64) -> Result<Vec<ExecuteTable>, DatabaseError> {
    execute_table::dsl::execute_table
        .filter(execute_table::timestamp.ge(start))
        .filter(execute_table::timestamp.le(end))
        .load::<ExecuteTable>(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))
}

/**
 * @description: 插入
 * @author: illya 
 * @date: 2025/6/3 00:41
 **/
pub fn save_execute(execute: ExecuteTable) -> Result<usize, DatabaseError> {
    let update_nums = diesel::insert_into(execute_table::dsl::execute_table)
        .values(&execute)
        .execute(&mut acquire_database_pool())
    .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(update_nums)
}

/**
 * @description: 更新数据
 * @author: illya 
 * @date: 2025/6/3 00:48
 **/
pub fn update_execute(execute: ExecuteTable) -> Result<usize, DatabaseError> {
    let filter = execute_table::dsl::execute_table.find(execute.key.clone());
    let update_nums = diesel::update(filter)
        .set(&execute)
        .execute(&mut acquire_database_pool())
    .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(update_nums)
}

/**
 * @description: 删除某一个元素
 * @author: illya 
 * @date: 2025/6/3 00:52
 **/
pub fn delete_execute(key: String) -> Result<usize, DatabaseError> {
    let filter = execute_table::dsl::execute_table.filter(execute_table::key.eq(&key));
    let count = diesel::delete(filter)
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(count)
}

/**
 * @description: 通过主键查询
 * @author: illya 
 * @date: 2025/6/3 01:07
 **/
pub fn acquire_execute_by_key(key: &str) -> Result<Option<ExecuteTable>, DatabaseError> {
    let execute_table = execute_table::dsl::execute_table
        .filter(execute_table::key.eq(key))
        .first::<ExecuteTable>(&mut acquire_database_pool())
        .optional()
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(execute_table)
}
