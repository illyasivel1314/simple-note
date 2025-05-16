use crate::configuration::utils::error_util::AppError;
use crate::entity::calendar::CalendarDto;
use crate::service::calendar_service;

/**
 * @description: 根据当前月份，获取显示内容
 * date: 当前月份的1号时间戳
 * @author: illya
 * @date: 2025/4/29 下午5:02
 **/
#[tauri::command]
pub async fn calendar_content(timestamp: i64) -> Result<Vec<CalendarDto>, AppError> {
    // 日历开始与结束日期
    let (start_timestamp, end_timestamp) = calendar_service::calendar_timestamp(timestamp);
    // 获取当月的日期
    let calendar_list =
        calendar_service::acquire_calendar(timestamp, start_timestamp, end_timestamp);
    Ok(calendar_list)
}
