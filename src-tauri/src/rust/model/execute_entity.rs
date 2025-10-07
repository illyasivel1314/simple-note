#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ExecuteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 操作的日期字符串
    pub timestamp: i64,
}