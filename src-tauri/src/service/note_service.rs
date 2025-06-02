use crate::configuration::utils::error_util::AppError;
use crate::dao::note;
use crate::dao::note::NoteTable;

/**
 * @description: 保存数据
 * @author: illya
 * @date: 2025/5/12 18:27
 **/
pub fn save_note(note: NoteTable) -> Result<usize, AppError> {
    note::save_note(note)
}

/**
 * @description: 更新便笺内容
 * @author: illya
 * @date: 2025/5/13 00:25
 **/
pub fn update_note_with_content(note: NoteTable) -> Result<usize, AppError> {
    note::update_note_with_content(note.key, note.content)
}

/**
 * @description: 查询对应的note
 * @author: illya
 * @date: 2025/5/13 00:25
 **/
pub fn acquire_note_by_key(key: &str) -> Result<Option<NoteTable>, AppError> {
    note::acquire_note_by_key(key)
}

/**
 * @description: 查询时间范围内的便笺
 * @author: illya
 * @date: 2025/5/13 00:32
 **/
pub fn acquire_note_by_timestamp(timestamp: i64) -> Result<Vec<NoteTable>, AppError> {
    note::acquire_note_by_timestamp(timestamp)
}

/**
 * @description: 删除便笺
 * @author: illya 
 * @date: 2025/6/1 13:54
 **/
pub fn delete_note(key: String) -> Result<usize, AppError> {
    note::delete_note_by_key(key)
}


/**
 * @description: 更新便笺
 * @author: illya 
 * @date: 2025/6/1 13:54
 **/
pub fn update_note(note: NoteTable) -> Result<usize, AppError> {
    note::update_note(note)
}

/**
 * @description:
 * @author: illya 
 * @date: 2025/6/1 15:07
 **/
pub fn update_note_finished(key: String, finish: i32) -> Result<usize, AppError> {
    note::update_note_finished(key, finish)
}