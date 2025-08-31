use rbatis::{crud, impl_delete, impl_select, impl_update};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ExecuteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 操作的日期字符串
    pub timestamp: i64,
}

crud!(ExecuteTable{});

impl_select!(ExecuteTable {
    acquire_execute_within(start_time: i64, end_time: i64) => "`where timestamp >= #{start_time}` and timestamp <= #{end_time}`"
});
impl_update!(ExecuteTable {
    update_execute_by_key(key: &str) => "`where key = #{key}`"
});

impl_delete!(ExecuteTable {
    delete_execute_by_key(key: &str) => "`where key = #{key}`"
});

impl_select!(ExecuteTable {
    acquire_execute_by_key(key: &str) => "`where key = #{key}`"
});
