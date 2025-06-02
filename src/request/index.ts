import { invoke } from '@tauri-apps/api/core';
import { warn } from '@tauri-apps/plugin-log';
import { Calendar, NoteInterface, transformNoteInterface } from './interface';

/**
 * 获取日历信息
 * @param timeStamp 时间戳
 * @returns 
 */
export const acquireCalendar = async (timeStamp: number): Promise<Map<string, Calendar>> => {
    let calendar_map: Map<string, Calendar> = new Map<string, Calendar>();
    await invoke<Calendar[]>('calendar_content', { timestamp: timeStamp })
        .then((res: Calendar[]) => {
            res.forEach((item: Calendar) => {
                calendar_map.set(item.calendar, item);
            });
        }).catch((err) => {
            warn(err);
        }
    );
//     let calendar_list: Calendar[] = [{
//     "calendar": "2025-04-27",
//     "solar_calendar": "27",
//     "lunar_calendar": "三十",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-04-28",
//     "solar_calendar": "28",
//     "lunar_calendar": "四月",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-04-29",
//     "solar_calendar": "29",
//     "lunar_calendar": "初二",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-04-30",
//     "solar_calendar": "30",
//     "lunar_calendar": "初三",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-05-01",
//     "solar_calendar": "01",
//     "lunar_calendar": "劳动节",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-02",
//     "solar_calendar": "02",
//     "lunar_calendar": "初五",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-03",
//     "solar_calendar": "03",
//     "lunar_calendar": "初六",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-04",
//     "solar_calendar": "04",
//     "lunar_calendar": "初七",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-05",
//     "solar_calendar": "05",
//     "lunar_calendar": "立夏",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-06",
//     "solar_calendar": "06",
//     "lunar_calendar": "初九",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-07",
//     "solar_calendar": "07",
//     "lunar_calendar": "初十",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-08",
//     "solar_calendar": "08",
//     "lunar_calendar": "十一",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-09",
//     "solar_calendar": "09",
//     "lunar_calendar": "十二",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-10",
//     "solar_calendar": "10",
//     "lunar_calendar": "十三",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-11",
//     "solar_calendar": "11",
//     "lunar_calendar": "十四",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-12",
//     "solar_calendar": "12",
//     "lunar_calendar": "十五",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-13",
//     "solar_calendar": "13",
//     "lunar_calendar": "十六",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-14",
//     "solar_calendar": "14",
//     "lunar_calendar": "十七",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-15",
//     "solar_calendar": "15",
//     "lunar_calendar": "十八",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-16",
//     "solar_calendar": "16",
//     "lunar_calendar": "十九",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-17",
//     "solar_calendar": "17",
//     "lunar_calendar": "二十",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-18",
//     "solar_calendar": "18",
//     "lunar_calendar": "廿一",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-19",
//     "solar_calendar": "19",
//     "lunar_calendar": "廿二",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-20",
//     "solar_calendar": "20",
//     "lunar_calendar": "廿三",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-21",
//     "solar_calendar": "21",
//     "lunar_calendar": "小满",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-22",
//     "solar_calendar": "22",
//     "lunar_calendar": "廿五",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-23",
//     "solar_calendar": "23",
//     "lunar_calendar": "廿六",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-24",
//     "solar_calendar": "24",
//     "lunar_calendar": "廿七",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-25",
//     "solar_calendar": "25",
//     "lunar_calendar": "廿八",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-26",
//     "solar_calendar": "26",
//     "lunar_calendar": "廿九",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-27",
//     "solar_calendar": "27",
//     "lunar_calendar": "五月",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-28",
//     "solar_calendar": "28",
//     "lunar_calendar": "初二",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-29",
//     "solar_calendar": "29",
//     "lunar_calendar": "初三",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-30",
//     "solar_calendar": "30",
//     "lunar_calendar": "初四",
//     "rest_day_valid": 0,
//     "within_month": true,
// }, {
//     "calendar": "2025-05-31",
//     "solar_calendar": "31",
//     "lunar_calendar": "端午节",
//     "rest_day_valid": 1,
//     "within_month": true,
// }, {
//     "calendar": "2025-06-01",
//     "solar_calendar": "01",
//     "lunar_calendar": "儿童节",
//     "rest_day_valid": 1,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-02",
//     "solar_calendar": "02",
//     "lunar_calendar": "初七",
//     "rest_day_valid": 1,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-03",
//     "solar_calendar": "03",
//     "lunar_calendar": "初八",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-04",
//     "solar_calendar": "04",
//     "lunar_calendar": "初九",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-05",
//     "solar_calendar": "05",
//     "lunar_calendar": "芒种",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-06",
//     "solar_calendar": "06",
//     "lunar_calendar": "十一",
//     "rest_day_valid": 0,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-07",
//     "solar_calendar": "07",
//     "lunar_calendar": "十二",
//     "rest_day_valid": 1,
//     "within_month": false,
// }, {
//     "calendar": "2025-06-08",
//     "solar_calendar": "08",
//     "lunar_calendar": "十三",
//     "rest_day_valid": 1,
//     "within_month": false,
//     }];
    // calendar_list.forEach((item) => {
    //     calendar_map.set(item.calendar, item);
    // });
    return calendar_map;
}

/**
 * 保存便笺内容
 * @param note 便笺
 */
export const insertNote = (note: NoteInterface) => {
    let note_vo = {
        key: note.key,
        content: note.content,
        start_stamp: note.execute_stamp[0].startOf('day').valueOf(),
        end_stamp: note.execute_stamp[1].endOf('day').valueOf(),
    }
    invoke<any>('save_note', { note_vo })
        .then((_res: any) => {
            console.log(_res);
        })
        .catch((err) => {
            warn(err);
        }
    );
}

/**
 * 更新便笺配置
 * @param list 便笺列表
 */
export const updateNoteSetting = (note: NoteInterface) => {

    let note_settting = {
        key: note.key,

        tag_type: note.tag_type,
        start_stamp: note.execute_stamp[0].startOf('day').valueOf(),
        end_stamp: note.execute_stamp[1].endOf('day').valueOf(),

        reminder_valid: note.reminder_valid,
        reminder_stamp: note.reminder_stamp?.valueOf(),
    }

    invoke<any>('update_note_setting', { note_vo: note_settting })
        .then((_res) => {
            console.log(_res);
        })
        .catch((err) => {
            warn(err);
        }
    );
}

/**
 * 标记标签是否完成
 * @param key 
 * @param finished 
 */
export const updateNoteFinish = (key: string, finish: number) => {
    invoke<any>('update_note_finished', { key, finish })
        .then((_res) => {
            console.log(_res);
        })
        .catch((err) => {
            warn(err);
        }
    );
}

/**
 * 删除便笺
 * @param key 便笺主键
 */
export const deleteNote = (key: string) => {
    invoke<any>('delete_note', { key: key })
        .then((res) => {
            console.log(res);
        })
        .catch((err) => {
            warn(err);
        }
    );
}

/**
 * 获取日期对应的便笺
 * @param timeStamp 时间戳
 * @returns 
 */
export const acquireNote = async (timestamp: number): Promise<NoteInterface[]> => {
    let list: NoteInterface[] = [];
    await invoke<any>('acquire_note', { timestamp })
        .then((res: any[]) => {
            res.forEach((value: any) => {
                list.push(transformNoteInterface(value))
            });
        })
        .catch((err) => {
            warn(err);
        }
    );
    return list;
}

/**
 * 根据key获取便笺
 * @param key 
 * @returns 
 */
export const acquireNoteByKey = async (key: string): Promise<NoteInterface | null> => {
    let note: NoteInterface | null = null;
    await invoke<any>('acquire_note_by_key', {key})
        .then((value: any) => {
            if (value != null) {
                note = transformNoteInterface(value);
            }
        })
        .catch((err) => {
            warn(err);
        });
    return note;
}
