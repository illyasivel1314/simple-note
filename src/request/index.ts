import { invoke } from '@tauri-apps/api/core';
import { warn } from '@tauri-apps/plugin-log';

/**
 * 获取日历信息
 * @param timeStamp 时间戳
 * @returns 
 */
export function acquireCalendar(timeStamp: number): any {
    return invoke<any>('calendar_content', { timestamp: timeStamp })
    .then((res: any) => {
        return res.reduce((map: Map<string, any>, value: { calendar: any; }) => {
            map.set(value.calendar, value);
            return map;
        }, new Map());
    }).catch((err) => {
        warn(err);
    });
}

/**
 * 保存便笺内容
 * @param note 便笺
 */
export function insertNote(note: any) {
    invoke<any>('save_note', { noteVo: note })
    .catch((err) => {
        warn(err);
    });
}

/**
 * 删除便笺
 * @param key 便笺主键
 */
export function deleteNote(key: string) {
    invoke<any>('delete_note', { key: key })
    .catch((err) => {
        warn(err);
    });
}

/**
 * 获取日期对应的便笺
 * @param timeStamp 时间戳
 * @returns 
 */
export function acquireNote(timeStamp: number): any {
    return invoke<string>('acquire_note', { timestamp: timeStamp })
    .then((res: any) => {
        return res;
    })
    .catch((err) => {
        warn(err);
    });
}
