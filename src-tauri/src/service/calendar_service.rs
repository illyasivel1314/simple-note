use crate::configuration::utils::time_util;
use crate::configuration::utils::time_util::{HOLIDAY_MAP, YEAR_MONTH_DAY};
use crate::entity::calendar::CalendarDto;
use chrono::{Datelike, Duration, NaiveDate};
use rbatis_crate::transaction;
use crate::configuration::utils::error_util::AppError;
use crate::dao::calendar::CalendarTable;

pub async fn calendar_content(timestamp: i64) -> Result<Vec<CalendarDto>, AppError> {
    let (start_timestamp, end_timestamp) = calendar_timestamp(timestamp);
    acquire_calendar(timestamp, start_timestamp, end_timestamp)
}


/**
 * @description: 获取日历开始、结束日期、月初时间
 * @author: illya
 * @date: 2025/5/3 下午4:13
 **/
fn calendar_timestamp(timestamp: i64) -> (NaiveDate, NaiveDate) {
    // 获取月初日期
    let month_begin =
        time_util::acquire_start_month_datetime(time_util::acquire_datetime(timestamp));
    // 获取日历开始时间
    let start_timestamp = time_util::operation_datetime_by_day(
        month_begin,
        (time_util::acquire_weekday(month_begin) % 7) * -1,
    );
    // 获取日历结束日期（共显示42天）
    let end_timestamp = time_util::operation_datetime_by_day(start_timestamp, 42);
    (start_timestamp, end_timestamp)
}

/**
 * @description: 获取基础内容
 * @author: illya
 * @date: 2025/5/3 下午4:10
 **/
#[transaction(conn = "pool")]
async fn acquire_calendar(
    timestamp: i64,
    start_timestamp: NaiveDate,
    end_timestamp: NaiveDate,
) -> Result<Vec<CalendarDto>, AppError> {
    // 当前日期
    let date_time_month = {
        let month = time_util::acquire_datetime(timestamp).month();
        if month < 10 {
            format!("0{}", month)
        } else {
            format!("{}", month)
        }
    };

    // 获取范围内的日历
    let start_date = start_timestamp.format(YEAR_MONTH_DAY).to_string();
    let end_date = end_timestamp.format(YEAR_MONTH_DAY).to_string();
    let calendar_list: Vec<CalendarTable> = CalendarTable::acquire_holidays_within(&pool, start_date.as_str(), end_date.as_str())?;

    // 如果数据库无相关数据
    if calendar_list.len() == 0 {
        return Ok(acquire_calendar_list(timestamp, start_timestamp, end_timestamp));
    }

    let mut calendar_dto_list = vec![];
    for calendar in &calendar_list {
        let calendar_str = format!("{}", calendar.date);
        let solar_calendar = format!("{}", calendar.day);

        let lunar_calendar = {
            let solar = format!("{}月{}日", calendar.month, calendar.day);
            let lunar = format!("{}", calendar.lunar);
            if let Some(value) = HOLIDAY_MAP.get(&solar) {
                /* 获取阳历节日 */
                format!("{}", value)
            } else if let Some(value) = HOLIDAY_MAP.get(&lunar) {
                /* 获取农历节日 */
                format!("{}", value)
            } else if time_util::qingming_valid(&calendar.year, &calendar.month, &calendar.day) {
                "清明节".to_string()
            } else if time_util::solar_term_valid(&calendar.solar_term) {
                /* 获取农历 */
                format!("{}", calendar.solar_term)
            } else {
                time_util::acquire_lunar_string(format!("{}", calendar.lunar))
            }
        };
        let rest_day = calendar.date_type;
        let within_month = calendar.month == date_time_month;
        let calendar_dto = CalendarDto::from_naive_date(
            calendar_str,
            solar_calendar,
            Some(lunar_calendar),
            Some(rest_day),
            within_month,
            None,
        );

        calendar_dto_list.push(calendar_dto);
    }

    // 返回数据
    Ok(calendar_dto_list)
}

/**
 * @description: 数据库查询不到对应数据，手动生成
 * @author: illya
 * @date: 2025/5/14 19:59
 **/
fn acquire_calendar_list(
    timestamp: i64,
    start_timestamp: NaiveDate,
    end_timestamp: NaiveDate,
) -> Vec<CalendarDto> {
    let mut calendar_list = vec![];
    // 当前日期
    let date_time = time_util::acquire_datetime(timestamp);

    // 遍历所有日期
    let mut timestamp = start_timestamp;
    while timestamp < end_timestamp {
        // 判断日期是否属于同一月份
        let within_month = time_util::same_month_valid(date_time, timestamp);

        // 封装数据
        let calendar = timestamp.format(YEAR_MONTH_DAY).to_string();
        let solar_calendar = timestamp.day().to_string();
        let calendar_dto =
            CalendarDto::from_naive_date(calendar, solar_calendar, None, None, within_month, None);
        calendar_list.push(calendar_dto);
        // 下一天
        timestamp += Duration::days(1);
    }

    calendar_list
}
