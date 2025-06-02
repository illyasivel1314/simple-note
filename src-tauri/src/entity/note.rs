use crate::configuration::utils::time_util;
use crate::dao::note::NoteTable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[derive(Debug)]
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

impl NoteVO {
    /**
     * @description: 将note转化为noteTable
     * @author: illya
     * @date: 2025/5/12 18:12
     **/
    pub fn create_new_note_table(note: NoteVO) -> NoteTable {
        // 将时间戳转化为日期
        let create_time = time_util::acquire_now_timestamp();

        NoteTable {
            key: note.key,
            content: note.content.unwrap(),
            create_time,
            tag_type: 0,
            start_time: note.start_stamp,
            end_time: note.end_stamp,
            finished_valid: 0,
            reminder_valid: 0,
            reminder_time: None,
        }
    }

    /**
     * @description: 更新数据
     * @author: illya
     * @date: 2025/6/1 13:36
     **/
    pub fn update_note_table(old_note: NoteTable, new_note: NoteVO) -> NoteTable {
        let mut note = old_note;
        note.start_time = new_note.start_stamp;
        note.end_time = new_note.end_stamp;

        if let Some(value) = new_note.content {
            note.content = value;
        }
        if let Some(value) = new_note.tag_type {
            note.tag_type = value;
        }
        if let Some(value) = new_note.finished_valid {
            note.finished_valid = value;
        }

        if let Some(value) = new_note.reminder_valid {
            note.reminder_valid = value;
            if value == 0 {
                note.reminder_time = None;
            } else if let Some(value) = new_note.reminder_stamp {
                note.reminder_time = Some(value);
            }
        }
        note
    }

    /**
     * @description: 将noteTable转化为note
     * @author: illya
     * @date: 2025/5/13 00:34
     **/
    pub fn from_note_table(note_table: NoteTable) -> Self {
        Self {
            key: note_table.key,
            start_stamp: note_table.start_time,
            end_stamp: note_table.end_time,
            content: Some(note_table.content),
            tag_type: Some(note_table.tag_type),
            finished_valid: Some(note_table.finished_valid),
            reminder_valid: Some(note_table.reminder_valid),
            reminder_stamp: note_table.reminder_time,
        }
    }
}
