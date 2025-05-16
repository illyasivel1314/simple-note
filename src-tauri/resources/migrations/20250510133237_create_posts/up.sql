CREATE TABLE calendar_table (
    date CHAR NOT NULL PRIMARY KEY, /* 日期字符串 */
    year CHAR(4) NOT NULL, /* 年 */
    month CHAR(2) NOT NULL, /* 月 */
    day CHAR(2) NOT NULL, /* 日 */
    weekday INTEGER NOT NULL, /* 星期 */
    date_type INTEGER NOT NULL, /* 0-工作日, 1-假日 */
    solar_term CHAR(4) NOT NULL, /* 节气 */
    lunar CHAR(10) NOT NULL /* 农历 */
);

CREATE TABLE note_table (
    "key" CHAR NOT NULL PRIMARY KEY, /* 存储便笺的唯一编号 */
    content VARCHAR NOT NULL, /* 便笺内容 */
    create_time BIGINT NOT NULL, /* 创建时间（排序标准）*/
    "type" INTEGER DEFAULT (1) NOT NULL, /* 1-单次, 2-周期, 3-循环 */
    start_time BIGINT NOT NULL, /* 便笺开始时间 */
    end_time BIGINT NOT NULL, /* 便笺结束时间 */
    cycle CHAR, /* 循环周期 */
    version INTEGER DEFAULT (1) NOT NULL /* 版本号 */
);

CREATE TABLE execute_table (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "key" TEXT NOT NULL, /* 存储便笺的唯一编号 */
    timestamp BIGINT NOT NULL, /* 操作的日期字符串 */
    finished INTEGER DEFAULT (0) NOT NULL /* 0-未完成,1-完成 */
);


CREATE INDEX note_table_key_IDX ON note_table ("key");

CREATE INDEX calendar_table_day_IDX ON calendar_table ("date");

CREATE INDEX execute_table_day_IDX ON execute_table ("key");
