use crate::configuration::utils::error_util::{AppError, TauriError};
use crate::dao::note::NoteTable;
use crate::entity::note::NoteVO;
use crate::service::note_service;

/**
 * @description: 保存便签
 * @author: illya
 * @date: 2025/5/12 18:11
 **/
#[tauri::command(rename_all = "snake_case")]
pub async fn save_note(note_vo: NoteVO) -> Result<(), AppError> {
    match note_service::acquire_note_by_key(&note_vo.key)? {
        None => {
            // 创建noteTable
            let new_note_table = NoteVO::create_new_note_table(note_vo);
            // 保存数据
            note_service::save_note(new_note_table)?;
        }
        Some(note_table) => {
            let update_note_table = NoteVO::update_note_table(note_table, note_vo);
            // 更新数据
            note_service::update_note(update_note_table)?;
        }
    }
    Ok(())
}

/**
 * @description: 获取timestamp对应的便笺
 * @author: illya
 * @date: 2025/5/12 23:49
 **/
#[tauri::command]
pub async fn acquire_note(timestamp: i64) -> Result<Vec<NoteVO>, AppError> {
    let note_list = note_service::acquire_note_by_timestamp(timestamp)?;
    let mut res_list = vec![];
    note_list.into_iter().for_each(|note| {
        res_list.push(NoteVO::from_note_table(note));
    });
    Ok(res_list)
}

/**
 * @description: 获取timestamp对应的便笺
 * @author: illya
 * @date: 2025/5/12 23:49
 **/
#[tauri::command]
pub async fn delete_note(key: String) -> Result<(), AppError> {
    tauri::async_runtime::spawn(async move { note_service::delete_note(key) });
    Ok(())
}

/**
 * @description: 更改便笺状态
 * @author: illya
 * @date: 2025/6/1 15:02
 **/
#[tauri::command]
pub async fn update_note_finished(key: String, finish: i32) -> Result<usize, AppError> {
    note_service::update_note_finished(key, finish)
}

/**
 * @description: 更新便笺配置
 * @author: illya
 * @date: 2025/6/1 17:28
 **/
#[tauri::command(rename_all = "snake_case")]
pub async fn update_note_setting(note_vo: NoteVO) -> Result<(), AppError> {
    if let Some(note_table) = note_service::acquire_note_by_key(&note_vo.key)? {
        let update_note_table = NoteVO::update_note_table(note_table, note_vo);
        // 更新数据
        note_service::update_note(update_note_table)?;
    }
    Ok(())
}

/**
 * @description: 根据key获取对应的note
 * @author: illya 
 * @date: 2025/6/1 18:24
 **/
#[tauri::command]
pub async fn acquire_note_by_key(key: String) -> Result<Option<NoteVO>, AppError> {
    match note_service::acquire_note_by_key(&key)? {
        None => Ok(None),
        Some(value) => Ok(Some(NoteVO::from_note_table(value))),
    }
}
