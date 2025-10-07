use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters)]
pub(crate) struct CalendarJson {
    code: i32,
    msg: String,
    data: Vec<Data>,
}

#[derive(Serialize, Deserialize, Getters)]
pub(crate) struct Data {
    month: i32,
    year: i32,
    days: Vec<Days>,
}

#[derive(Serialize, Deserialize, Getters)]
pub(crate) struct Days {
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
    #[serde(rename = "avoid")]
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