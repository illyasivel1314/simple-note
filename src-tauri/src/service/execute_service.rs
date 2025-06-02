use crate::configuration::utils::error_util::DatabaseError;
use crate::dao::execute;
use crate::dao::execute::ExecuteTable;

/**
 * @description: 查询某一范围内的数据
 * @author: illya 
 * @date: 2025/6/3 00:26
 **/
pub fn acquire_execute_within(start: i64, end: i64) -> Result<Vec<ExecuteTable>, DatabaseError> {
    execute::acquire_execute_within(start, end)
}

/**
 * @description: 插入
 * @author: illya 
 * @date: 2025/6/3 00:41
 **/
pub fn save_execute(execute: ExecuteTable) -> Result<usize, DatabaseError> {
    execute::save_execute(execute)
}

/**
 * @description: 更新数据
 * @author: illya 
 * @date: 2025/6/3 00:48
 **/
pub fn update_execute(execute: ExecuteTable) -> Result<usize, DatabaseError> {
    execute::update_execute(execute)
}

/**
 * @description: 删除某一个元素
 * @author: illya 
 * @date: 2025/6/3 00:52
 **/
pub fn delete_execute(key: String) -> Result<usize, DatabaseError> {
    execute::delete_execute(key)
}

/**
 * @description: 通过主键查询
 * @author: illya 
 * @date: 2025/6/3 01:07
 **/
pub fn acquire_execute_by_key(key: &str) -> Result<Option<ExecuteTable>, DatabaseError> {
    execute::acquire_execute_by_key(key)
}
