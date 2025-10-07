use chrono::Datelike;
use crate::rust::config::configuration::holiday::holiday_entity::CalendarJson;
use crate::rust::model::calendar_entity::CalendarTable;
use crate::rust::utils::time_util;

/**
 * @description: CalendarJson -> CalendarTable
 *
 * @author: illya
 * @date: 2025/10/6 17:26
 **/
pub fn conversion_holiday(list: Vec<CalendarJson>) -> Vec<CalendarTable> {
    let mut holiday_list = vec![];
    // 遍历年份
    list.iter().for_each(|item| {
        // 遍历月份
        item.data().iter().for_each(|holiday| {
            // 遍历每天
            holiday.days().iter().for_each(|day| {
                // 判断是几号
                let now_date = time_util::generate_naive_date_with_str(day.date());
                holiday_list.push(CalendarTable {
                    date: format!("{}", day.date()),
                    year: format!("{}", holiday.year()),
                    month: format_month_day(now_date.month()),
                    day: format_month_day(now_date.day()),
                    weekday: day.week_day().clone(),
                    date_type: if day.day_type().clone() == 0 { 0 } else { 1 },
                    solar_term: format!("{}", day.solar_terms()),
                    lunar: format!("{}", day.lunar_calendar()),
                })
            })
        })
    });
    holiday_list
}

/**
 * @description: 格式化格式
 *
 * @author: illya
 * @date: 2025/10/6 17:23
 **/
fn format_month_day(day: u32) -> String {
    if day < 10 {
        format!("0{}", day)
    } else {
        format!("{}", day)
    }
}

