use crate::configuration::database::data::schema::note_table;
use crate::configuration::database::index::acquire_database_pool;
use crate::configuration::utils::error_util::{AppError, DatabaseError};
use diesel::prelude::*;
use diesel::QueryId;

#[derive(Queryable, Selectable, QueryId, Insertable, AsChangeset)]
#[diesel(table_name = crate::configuration::database::data::schema::note_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
#[derive(Debug, Default)]
pub struct NoteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 便笺内容
    pub content: String,
    // 创建时间
    pub create_time: i64,

    // 0-单次, 1-周期
    pub tag_type: i32,
    // 便笺开始时间
    pub start_time: i64,
    // 便笺结束时间
    pub end_time: i64,

    // 是否完成
    pub finished_valid: i32,
    // 是否提醒
    pub reminder_valid: i32,
    // 提醒时间
    pub reminder_time: Option<i64>,
}

/**
 * @description: 保存便笺
 * @author: illya
 * @date: 2025/5/12 18:19
 **/
pub fn save_note(note: NoteTable) -> Result<usize, AppError> {
    let update_nums = diesel::insert_into(note_table::dsl::note_table)
        .values(&note)
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(update_nums)
}

/**
 * @description: 更新便笺
 * @author: illya
 * @date: 2025/6/1 13:55
 **/
pub fn update_note(note: NoteTable) -> Result<usize, AppError> {
    let filter = note_table::dsl::note_table.find(note.key.clone());
    let updated_rows = diesel::update(filter)
        .set(&note)
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(updated_rows)
}

pub fn update_note_finished(key: String, finish: i32) -> Result<usize, AppError> {
    let filter = note_table::dsl::note_table.filter(note_table::key.eq(key));
    let updated_rows = diesel::update(filter)
        .set(note_table::finished_valid.eq(&finish))
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(updated_rows)
}

/**
 * @description: 统计key对应的数量
 * @author: illya
 * @date: 2025/5/13 00:02
 **/
pub fn acquire_note_by_key(id: &str) -> Result<Option<NoteTable>, AppError> {
    let note_table = note_table::dsl::note_table
        .filter(note_table::key.eq(id))
        .first::<NoteTable>(&mut acquire_database_pool())
        .optional()
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;

    Ok(note_table)
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
        .load::<NoteTable>(&mut acquire_database_pool())
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
        .execute(&mut acquire_database_pool())
        .map_err(|err| DatabaseError::DatabaseOperationError(err))?;
    Ok(count)
}
