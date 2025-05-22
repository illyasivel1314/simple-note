use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::configuration::database::data::schema::execute_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExecuteTable {
    pub id: i32,
    // 存储便笺的唯一编号
    pub key: String,
    // 操作的日期字符串
    pub timestamp: i64,
    // 0-未完成,1-完成
    pub finished: i32,
}
