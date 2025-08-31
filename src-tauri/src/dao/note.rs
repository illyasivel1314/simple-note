use rbatis::{crud, impl_delete, impl_select, impl_select_page, impl_update};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NoteTable {
    // 存储便笺的唯一编号
    pub key: String,
    // 便笺内容
    pub content: String,
    // 创建时间
    pub create_time: i64,

    // 0-单次, 1-周期
    pub tag_type: i32,
    // 便笺开始时间
    pub start_time: i64,
    // 便笺结束时间
    pub end_time: i64,

    // 是否完成
    pub finished_valid: i32,
    // 是否提醒
    pub reminder_valid: i32,
    // 提醒时间
    pub reminder_time: Option<i64>,
}

impl Default for NoteTable {
    fn default() -> Self {
        Self {
            reminder_time: Some(0),
            ..Default::default()
        }
    }
}


crud!(NoteTable {});
impl_update!(NoteTable {
    update_note_by_key(key: &str) => "`where key = #{key}`"
});

impl_select!(NoteTable {
    acquire_note_by_timestamp(timestamp: i64) => "`where start_time >= #{timestamp} and end_time <= #{timestamp}`"
});

impl_delete!(NoteTable {
    delete_note_by_key(key: &str) => "`where key = #{key}`"
});
