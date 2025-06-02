use crate::configuration::database::data::schema::calendar_table;
use crate::configuration::database::index::acquire_database_pool;
use diesel::prelude::*;
use crate::configuration::utils::error_util::{AppError, DatabaseError};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::configuration::database::data::schema::calendar_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CalendarTable {
    // 日期字符串
    pub date: String,
    // 年
    pub year: String,
    // 月
    pub month: String,
    // 日
    pub day: String,
    // 周几
    pub weekday: i32,
    // 0-工作日, 1-假日
    pub date_type: i32,
    // 节气
    pub solar_term: String,
    // 农历日期
    pub lunar: String,
}

/**
 * @description: 查询范围内的数据
 * @author: illya
 * @date: 2025/5/14 19:40
 **/
pub fn acquire_holidays_within(start: String, end: String) -> Result<Vec<CalendarTable>, AppError> {
    let list = calendar_table::dsl::calendar_table
        .filter(calendar_table::date.ge(start))
        .filter(calendar_table::date.le(end))
        .load::<CalendarTable>(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(list)
}

/**
 * @description: 批量更新
 * @author: illya
 * @date: 2025/5/14 17:37
 **/
pub fn insert_holiday_list(list: Vec<CalendarTable>) -> Result<usize, AppError> {
    let num = diesel::insert_into(calendar_table::dsl::calendar_table)
        .values(&list)
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(num)
}

/**
 * @description: 清除表中所有数据
 * @author: illya
 * @date: 2025/5/14 17:46
 **/
pub fn delete_holiday() -> Result<usize, AppError> {
    let num = diesel::delete(calendar_table::dsl::calendar_table)
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(num)
}
