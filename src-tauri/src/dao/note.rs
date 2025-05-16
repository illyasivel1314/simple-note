use crate::configuration::database::interface::{DatabaseImpl, DatabaseUtil};
use crate::configuration::database::schema::note_table;
use crate::configuration::utils::error_util::{AppError, DatabaseError};
use diesel::prelude::*;
use diesel::QueryId;

#[derive(Queryable, Selectable, QueryId)]
#[diesel(table_name = crate::configuration::database::schema::note_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 便笺内容
    pub content: String,
    // 创建时间
    pub create_time: i64,
    // 1-单次, 2-周期, 3-循环
    pub category: i32,
    // 便笺开始时间
    pub start_time: i64,
    // 便笺结束时间
    pub end_time: Option<i64>,
    // 循环周期
    pub cycle: Option<String>,
    // 版本号
    pub version: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::configuration::database::schema::note_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoteTableInsert {
    // 存储便笺的唯一编号
    pub key: String,
    // 便笺内容
    pub content: String,
    // 创建时间
    pub create_time: i64,
    // 便笺开始时间
    pub start_time: i64,
    // 便笺结束时间
    pub end_time: Option<i64>,
    // 版本号
    pub version: i32,
}

/**
 * @description: 保存便笺
 * @author: illya
 * @date: 2025/5/12 18:19
 **/
pub fn save_note(note: NoteTableInsert) -> Result<usize, AppError> {
    let update_nums = diesel::insert_into(note_table::dsl::note_table)
        .values(&note)
        .execute(&mut DatabaseUtil::default().acquire_connection())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(update_nums)
}

/**
 * @description: 更新便笺内容
 * @author: illya
 * @date: 2025/5/13 00:19
 **/
pub fn update_note_with_content(key: String, content: String) -> Result<usize, AppError> {
    let filter = note_table::dsl::note_table.filter(note_table::key.eq(key));
    let updated_rows = diesel::update(filter)
        .set(note_table::content.eq(&content))
        .execute(&mut DatabaseUtil::default().acquire_connection())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(updated_rows)
}

/**
 * @description: 统计key对应的数量
 * @author: illya
 * @date: 2025/5/13 00:02
 **/
pub fn count_note_by_key(id: &str) -> Result<i64, AppError> {
    let count = note_table::dsl::note_table
        .filter(note_table::key.eq(id))
        .count()
        .get_result::<i64>(&mut DatabaseUtil::default().acquire_connection())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(count)
}

/**
 * @description: 查询时间范围内的便笺
 * @author: illya
 * @date: 2025/5/13 00:31
 **/
pub fn acquire_note_by_timestamp(timestamp: i64) -> Result<Vec<NoteTable>, AppError> {
    let note_list = note_table::dsl::note_table
        .filter(note_table::start_time.le(timestamp))
        .filter(note_table::end_time.ge(timestamp))
        .order_by(note_table::create_time)
        .load::<NoteTable>(&mut DatabaseUtil::default().acquire_connection())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(note_list)
}

/**
 * @description: 删除对应的便笺
 * @author: illya
 * @date: 2025/5/16 03:16
 **/
pub fn delete_note_by_key(key: String) -> Result<usize, AppError> {
    let filter = note_table::dsl::note_table.filter(note_table::key.eq(&key));
    let count = diesel::delete(filter)
        .execute(&mut DatabaseUtil::default().acquire_connection())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(count)
}
