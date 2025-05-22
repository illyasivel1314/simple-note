use crate::configuration::utils::error_util::{AppError, FileError, JsonError, TokioError};
use crate::configuration::utils::file_util;
use crate::configuration::utils::time_util;
use crate::dao;
use crate::dao::calendar::CalendarTable;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;

#[derive(Serialize, Deserialize)]
struct CalendarJson {
    code: i32,
    msg: String,
    data: Vec<Data>,
}

impl CalendarJson {
    pub fn into_holiday(&self) -> Vec<CalendarTable> {
        let mut holiday_list = vec![];
        for data in &self.data {
            for days in &data.days {
                // 判断是几号
                let now_date = time_util::acquire_datetime_by_str(days.date.as_str());
                let holiday = CalendarTable {
                    date: format!("{}", days.date),
                    year: format!("{}", data.year),
                    month: if now_date.month() < 10 {
                        format!("0{}", now_date.month())
                    } else {
                        format!("{}", now_date.month())
                    },
                    day: if now_date.day() < 10 {
                        format!("0{}", now_date.day())
                    } else {
                        format!("{}", now_date.day())
                    },
                    weekday: days.week_day,
                    date_type: if days.day_type == 0 { 0 } else { 1 },
                    solar_term: format!("{}", days.solar_terms),
                    lunar: format!("{}", days.lunar_calendar),
                };
                holiday_list.push(holiday);
            }
        }
        holiday_list
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    month: i32,
    year: i32,
    days: Vec<Days>,
}

#[derive(Serialize, Deserialize)]
struct Days {
    date: String,
    #[serde(rename = "weekDay")]
    week_day: i32,
    #[serde(rename = "yearTips")]
    year_tips: String,
    #[serde(rename = "type")]
    day_type: i32,
    #[serde(rename = "chineseZodiac")]
    chinese_zodiac: String,
    #[serde(rename = "solarTerms")]
    solar_terms: String,
    #[serde(rename = "typeDes")]
    type_des: String,
    avoid: String,
    #[serde(rename = "lunarCalendar")]
    lunar_calendar: String,
    suit: String,
    #[serde(rename = "dayOfYear")]
    day_of_year: i32,
    #[serde(rename = "weekOfYear")]
    week_of_year: i32,
    constellation: String,
    #[serde(rename = "indexWorkDayOfMonth")]
    index_work_day_of_month: i32,
}

// 节假日文件
const HOLIDAY_ADDRESS: &str = "./resources/holiday";
/**
 * @description:
 * @author: illya
 * @date: 2025/5/14 15:42
 **/
pub fn update_holiday_to_database() -> Result<(), AppError> {
    // 读取文件夹中的所有文件
    let file_list = file_util::acquire_file_list(HOLIDAY_ADDRESS)?;

    // 通过多线程读取所有文件内容
    let mut holiday_table_list = vec![];
    for file_result in file_list {
        let file = file_result.map_err(|err| FileError::FileTraverseError(err))?;
        holiday_table_list.extend(acquire_holiday(file)?.into_holiday());
    }
    
    // 先删除表中所有数据
    dao::calendar::delete_holiday();
    // 向表中新增所有数据
    dao::calendar::insert_holiday_list(holiday_table_list);

    Ok(())
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
    Ok((holiday))
}
