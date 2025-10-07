use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Default)]
pub struct NoteVO {
    // 唯一标识
    pub key: String,
    // 执行开始时间戳
    pub start_stamp: i64,
    // 执行结束时间戳
    pub end_stamp: i64,

    // 内容
    pub content: Option<String>,
    // 标签类型
    pub tag_type: Option<i32>,
    // 是否完成
    pub finished_valid: Option<i32>,

    // 是否提醒
    pub reminder_valid: Option<i32>,
    // 提醒时间
    pub reminder_stamp: Option<i64>,
}