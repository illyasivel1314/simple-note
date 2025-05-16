use crate::configuration::utils::time_util;
use crate::dao::note::{NoteTable, NoteTableInsert};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoteVO {
    // 唯一标识
    pub key: String,
    // 内容
    pub content: String,
    // 创建时间
    pub timestamp: i64,
}

impl NoteVO {
    /**
     * @description: 将note转化为noteTable
     * @author: illya
     * @date: 2025/5/12 18:12
     **/
    pub fn to_note_table(note: NoteVO) -> NoteTableInsert {
        // 将时间戳转化为日期
        let datetime = time_util::acquire_datetime(note.timestamp);
        // 获取当天的开始时间时间戳
        let start_of_day_timestamp = time_util::acquire_start_timestamp(datetime);
        // 获取当天的结束时间时间戳
        let end_of_day_timestamp = time_util::acquire_end_timestamp(datetime);

        NoteTableInsert {
            key: note.key,
            content: note.content,
            create_time: note.timestamp,
            start_time: start_of_day_timestamp,
            end_time: Some(end_of_day_timestamp),
            version: 1,
        }
    }

    /**
     * @description: 将noteTable转化为note
     * @author: illya
     * @date: 2025/5/13 00:34
     **/
    pub fn from_note_table(note_table: &NoteTable) -> Self {
        Self {
            key: note_table.key.clone(),
            content: note_table.content.clone(),
            timestamp: note_table.create_time,
        }
    }
}
