use std::fs::DirEntry;
use log::info;
use crate::rust::config::configuration::holiday::holiday_entity::CalendarJson;
use crate::rust::utils::error_util::{AppError, FileError, JsonError};
use crate::rust::utils::file_util;

pub mod holiday_entity;

// 节假日文件
const HOLIDAY_ADDRESS: &str = "./resources/holiday";

/**
 * @description: 读取节日文件信息
 *
 * @author: illya
 * @date: 2025/10/6 14:53
 **/
pub fn read_holiday_file() -> Result<Vec<CalendarJson>, AppError>{
    // 读取文件夹中的所有文件
    let file_list = file_util::acquire_file_list(HOLIDAY_ADDRESS)?;
    info!("Tauri read holiday file list successfully!");

    // 读取所有文件内容
    let mut holiday_table_list = vec![];
    for file_result in file_list {
        let file = file_result.map_err(|err| FileError::FileTraverseError(err))?;
        holiday_table_list.push(acquire_holiday(file)?);
    }
    info!("Tauri acquire holiday file content successfully!");
    Ok(holiday_table_list)
}

/**
 * @description: 将文件中的内容转化为holiday表
 * @author: illya
 * @date: 2025/5/14 16:36
 **/
fn acquire_holiday(file: DirEntry) -> Result<CalendarJson, AppError> {
    let content = file_util::read_file_content(file.path())?
        .replace("廿\"", "二十\"")
        .replace("卅", "三十");
    let holiday = serde_json::from_str::<CalendarJson>(content.as_str())
        .map_err(|err| JsonError::JsonTransformError(err))?;
    Ok(holiday)
}