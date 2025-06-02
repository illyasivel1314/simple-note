import dayjs, { Dayjs } from 'dayjs';


// 便笺内容接口
export interface NoteInterface {
    key: string,                      /* 唯一标识 */
    content: string,                  /* 笔记内容 */

    setting: boolean,                 /* 是否打开配置窗口 */
    finished_valid: 0 | 1,            /* 是否完成 */
    
    tag_type: 0 | 1,                  /* 便笺类型 */
    tag_content: boolean,             /* 是否是新增 */
    execute_stamp: [Dayjs, Dayjs],    /* 执行时间戳 */

    reminder_valid: 0 | 1,            /* 是否提醒 */
    reminder_stamp: null | Dayjs,     /* 提醒时间 */
}

// 日历内容接口
export interface Calendar {
    calendar: string,                 /* 日期 */
    solar_calendar: string,           /* 阳历day */
    lunar_calendar: string,           /* 农历day */
    rest_day_valid: 0 | 1 | null,     /* 是否休息日（1-是, 0-否） */
    within_month: null | boolean,     /* 是否是当月日期 */
}

/**
 * 将后端传入的数据进行转化
 * @param note 后端传入的note
 * @returns 
 */
export const transformNoteInterface = (note: any): NoteInterface => {
    let new_note = {
        key: note.key,
        content: note.content,

        setting: false,
        finished_valid: note.finished_valid,

        tag_type: note.tag_type,
        tag_content: false,
        execute_stamp: [dayjs(note.start_stamp), dayjs(note.end_stamp)],

        reminder_valid: note.reminder_valid,
        reminder_stamp: note.reminder_stamp == null ? null : dayjs(note.reminder_stamp)
    } as NoteInterface;
    return new_note;
}