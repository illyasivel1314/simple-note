use rbatis::{impl_delete, impl_select};

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
rbatis::crud!(CalendarTable {});

// 查询范围内的数据
impl_select!(CalendarTable {
    acquire_holidays_within(start: &str, end: &str) => "`where date >= #{start} and date <= #{end}`"
});

impl_delete!(CalendarTable {
    delete_holiday() => "`where 1=1`"
});