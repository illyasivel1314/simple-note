use crate::configuration::utils::error_util::AppError;
use crate::entity::note::NoteVO;
use crate::service::note_service;

/**
 * @description: 保存便签
 * @author: illya
 * @date: 2025/5/12 18:11
 **/
#[tauri::command]
pub async fn save_note(note_vo: NoteVO) -> Result<(), AppError> {
    let note = NoteVO::to_note_table(note_vo);
    if let Ok(value) = note_service::count_note_by_key(&note.key) {
        if value > 0 {
            note_service::update_note_with_content(note)?;
        } else {
            note_service::save_note(note)?;
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
    note_list.iter().for_each(|note| {
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
