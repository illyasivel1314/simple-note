use crate::configuration::database::interface::{DatabaseImpl, DatabaseUtil};
use crate::configuration::database::schema::calendar_table;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::configuration::database::schema::calendar_table)]
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
 * @description: 查询在list中数据
 * @author: illya
 * @date: 2025/5/12 17:18
 **/
pub fn acquire_holidays(list: Vec<String>) -> Vec<CalendarTable> {
    calendar_table::dsl::calendar_table
        .filter(calendar_table::date.eq_any(list))
        .load::<CalendarTable>(&mut DatabaseUtil::default().acquire_connection())
        .expect("Error loading holidays")
}

/**
 * @description: 查询范围内的数据
 * @author: illya
 * @date: 2025/5/14 19:40
 **/
pub fn acquire_holidays_within(start: String, end: String) -> Vec<CalendarTable> {
    calendar_table::dsl::calendar_table
        .filter(calendar_table::date.ge(start))
        .filter(calendar_table::date.le(end))
        .load::<CalendarTable>(&mut DatabaseUtil::default().acquire_connection())
        .expect("Error loading holidays")
}

/**
 * @description: 批量更新
 * @author: illya
 * @date: 2025/5/14 17:37
 **/
pub fn insert_holiday_list(list: Vec<CalendarTable>) -> usize {
    diesel::insert_into(calendar_table::dsl::calendar_table)
        .values(&list)
        .execute(&mut DatabaseUtil::default().acquire_connection())
        .expect("Error saving holiday table")
}

/**
 * @description: 清除表中所有数据
 * @author: illya
 * @date: 2025/5/14 17:46
 **/
pub fn delete_holiday() -> usize {
    diesel::delete(calendar_table::dsl::calendar_table)
        .execute(&mut DatabaseUtil::default().acquire_connection())
        .expect("Error deleting holidays")
}
