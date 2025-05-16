use crate::configuration::utils::error_util::AppError;
use crate::dao::note;
use crate::dao::note::{NoteTable, NoteTableInsert};

/**
 * @description: 保存数据
 * @author: illya
 * @date: 2025/5/12 18:27
 **/
pub fn save_note(note: NoteTableInsert) -> Result<usize, AppError> {
    note::save_note(note)
}

/**
 * @description: 更新便笺内容
 * @author: illya
 * @date: 2025/5/13 00:25
 **/
pub fn update_note_with_content(note: NoteTableInsert) -> Result<usize, AppError> {
    note::update_note_with_content(note.key, note.content)
}

/**
 * @description: 根据key统计便笺数量
 * @author: illya
 * @date: 2025/5/13 00:25
 **/
pub fn count_note_by_key(key: &str) -> Result<i64, AppError> {
    note::count_note_by_key(key)
}

/**
 * @description: 查询时间范围内的便笺
 * @author: illya
 * @date: 2025/5/13 00:32
 **/
pub fn acquire_note_by_timestamp(timestamp: i64) -> Result<Vec<NoteTable>, AppError> {
    note::acquire_note_by_timestamp(timestamp)
}

pub fn delete_note(key: String) -> Result<usize, AppError> {
    note::delete_note_by_key(key)
}
