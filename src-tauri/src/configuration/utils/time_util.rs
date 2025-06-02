use chinese_lunisolar_calendar::{LunarDay, LunarMonth, LunisolarDate};
use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc, Weekday};
use std::cell::LazyCell;
use std::collections::HashMap;
use tyme4rs::tyme::solar::{SolarTerm, SOLAR_TERM_NAMES};

// 年月日
pub const YEAR_MONTH_DAY: &str = "%Y-%m-%d";
// 年月日 时分秒
pub const YMD_HMS: &str = "%Y-%m-%d %H:%M:%S";
// 节日map
pub const HOLIDAY_MAP: LazyCell<HashMap<String, String>> = LazyCell::new(|| {
    HashMap::from([
        ("01月01日".to_string(), "元旦".to_string()),
        ("正月初一".to_string(), "春节".to_string()),
        ("03月12日".to_string(), "植树节".to_string()),
        ("正月十五".to_string(), "元宵节".to_string()),
        ("05月01日".to_string(), "劳动节".to_string()),
        ("06月01日".to_string(), "儿童节".to_string()),
        ("五月初五".to_string(), "端午节".to_string()),
        ("七月初七".to_string(), "七夕节".to_string()),
        ("七月十五".to_string(), "中元节".to_string()),
        ("八月十五".to_string(), "中秋节".to_string()),
        ("九月初九".to_string(), "重阳节".to_string()),
        ("09月10日".to_string(), "教师节".to_string()),
        ("10月01日".to_string(), "国庆节".to_string()),
        ("腊月二十三".to_string(), "小年".to_string()),
        ("腊月三十".to_string(), "除夕".to_string()),
    ])
});

// 创建 UTC+8 的时区偏移
const OFFSET: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8

/**
 * @description: 获取「年-月-日」日期组件
 * @author: illya
 * @date: 2025/5/13 16:09
 **/
pub fn acquire_datetime(timestamp: i64) -> NaiveDate {
    let datetime = Utc.timestamp_opt(timestamp / 1000, 0).unwrap();
    // 转换为东八区时间
    let datetime_utc_8 = datetime.with_timezone(&OFFSET);
    datetime_utc_8.date_naive()
}

/**
 * @description: 获取「年-月-日」日期组件
 * @author: illya
 * @date: 2025/5/14 20:11
 **/
pub fn acquire_datetime_by_str(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, YEAR_MONTH_DAY).expect("Could not parse date")
}

/**
 * @description: 获取当前时间戳
 * @author: illya 
 * @date: 2025/5/31 16:00
 **/
pub fn acquire_now_timestamp() -> i64 {
    Utc::now().with_timezone(&OFFSET).timestamp_millis()
}
/**
 * @description: 获取当前时间
 * @author: illya
 * @date: 2025/5/15 15:21
 **/
pub fn acquire_now_datetime() -> String {
    Utc::now()
        .with_timezone(&OFFSET)
        .format(YMD_HMS)
        .to_string()
}

/**
 * @description: 获取当天的开始时间 (00:00:00)
 * @author: illya
 * @date: 2025/5/13 16:12
 **/
pub fn acquire_start_timestamp(naive_date: NaiveDate) -> i64 {
    let start_of_day = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let start_of_day_utc_8 = start_of_day.checked_sub_offset(OFFSET).unwrap();
    start_of_day_utc_8.and_utc().timestamp_millis()
}

/**
 * @description: 获取当天的结束时间 (23:59:59)
 * @author: illya
 * @date: 2025/5/13 16:12
 **/
pub fn acquire_end_timestamp(naive_date: NaiveDate) -> i64 {
    let end_of_day = naive_date.and_hms_opt(23, 59, 59).unwrap();
    let end_of_day_utc_8 = end_of_day.checked_sub_offset(OFFSET).unwrap();
    end_of_day_utc_8.and_utc().timestamp_millis()
}

/**
 * @description: 将当前日期转化为月初日期
 * @author: illya
 * @date: 2025/5/13 16:23
 **/
pub fn acquire_start_month_datetime(naive_date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(naive_date.year(), naive_date.month(), 1).unwrap()
}

/**
 * @description: 对日期操作
 * @author: illya
 * @date: 2025/5/14 19:32
 **/
pub fn operation_datetime_by_day(naive_date: NaiveDate, day: i64) -> NaiveDate {
    naive_date + Duration::days(day)
}

/**
 * @description: 返回当前日期是星期几
 * @author: illya
 * @date: 2025/5/13 16:25
 **/
pub fn acquire_weekday(naive_date: NaiveDate) -> i64 {
    let week = naive_date.weekday();
    match week {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
}

/**
 * @description: 判断是否属于同一月份
 * @reference: 基准值
 * @control: 对照值
 * @author: illya
 * @date: 2025/5/13 16:44
 **/
pub fn same_month_valid(reference: NaiveDate, control: NaiveDate) -> bool {
    reference.month() == control.month()
}

/**
 * @description: 获取农历日期
 * @author: illya
 * @date: 2025/5/13 16:50
 **/
pub fn acquire_lunar(naive_date: NaiveDate) -> LunisolarDate {
    LunisolarDate::from_date(naive_date).unwrap()
}

/**
 * @description: 获取农历年
 * @author: illya
 * @date: 2025/5/13 16:56
 **/
pub fn acquire_lunar_year(lunar_date: LunisolarDate) -> String {
    lunar_date.to_lunar_year().to_string()
}

/**
 * @description: 获取农历月，并判断是否是第一天
 * @author: illya
 * @date: 2025/5/13 16:56
 **/
pub fn acquire_lunar_month(lunar_month: LunisolarDate) -> (String, bool) {
    let first_month_in_year = match lunar_month.to_lunar_month() {
        LunarMonth::First => true,
        _ => false,
    };
    (
        lunar_month.to_lunar_month().to_string(),
        first_month_in_year,
    )
}

/**
 * @description: 获取农历日期，并判断是否是第一天
 * @author: illya
 * @date: 2025/5/13 16:56
 **/
pub fn acquire_lunar_day(lunar_day: LunisolarDate) -> (String, bool) {
    let first_day_in_month = match lunar_day.to_lunar_day() {
        LunarDay::First => true,
        _ => false,
    };
    (lunar_day.to_lunar_day().to_string(), first_day_in_month)
}

/**
 * @description: 判断是否属于24节气之一
 * @author: illya
 * @date: 2025/5/14 20:23
 **/
pub fn solar_term_valid(solar_term: &str) -> bool {
    SOLAR_TERM_NAMES.contains(&solar_term)
}

/**
 * @description: 如果初一，则返回月份，否则返回日期
 * @author: illya
 * @date: 2025/5/14 20:30
 **/
pub fn acquire_lunar_string(lunar: String) -> String {
    let (month, day) = lunar.split_at("初一".len());
    if day == "初一" {
        return format!("{}", month.to_string());
    }
    format!("{}", day.to_string())
}

/**
 * @description: 判断是否是清明节
 * @author: illya
 * @date: 2025/5/14 21:45
 **/
pub fn qingming_valid(year: &str, month: &str, day: &str) -> bool {
    if month != "04" {
        return false;
    }

    let year_i32 = year.parse::<i32>().unwrap() % 100;
    let day_i32 = day.parse::<i32>().unwrap();

    (year_i32 as f64 * 0.2422 + 4.81) as i32 == day_i32
}
