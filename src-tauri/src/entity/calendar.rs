use serde::Serialize;

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

impl CalendarDto {
    pub fn from_naive_date(
        calendar: String,
        solar_calendar: String,
        lunar_calendar: Option<String>,
        rest_day_valid: Option<i32>,
        within_month: bool,
        todo_item: Option<i32>,
    ) -> Self {
        Self {
            calendar,
            solar_calendar,
            lunar_calendar,
            rest_day_valid,
            within_month,
            todo_item,
        }
    }
}
