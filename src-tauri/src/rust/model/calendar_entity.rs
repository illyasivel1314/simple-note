use serde::Serialize;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
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

#[derive(Serialize)]
pub struct CalendarDto {
    // 当前日期
    pub calendar: String,
    // 阳历日期
    pub solar_calendar: String,
    // 农历日期
    pub lunar_calendar: Option<String>,
    // 是否工作日
    pub rest_day_valid: Option<i32>,
    // 是否在本月内
    pub within_month: bool,
    // 待办事项
    pub todo_item: Option<i32>,
}