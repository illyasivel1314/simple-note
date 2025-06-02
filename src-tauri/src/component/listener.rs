use crate::component::handler::{InitialTaskMangerType};
use crate::component::windows::{create_calendar_window};
use log::info;
use std::thread::sleep;
use std::time::Duration;
use chrono::Utc;
use tauri::{AppHandle, Emitter, Listener, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use crate::service::{execute_service, note_service};

/**
 * @description: 用于监听初始化是否完成
 * @author: illya
 * @date: 2025/5/23 16:09
 **/
pub fn create_initialization_listener(app: AppHandle) {
    let app_listen = app.clone();
    // 创建监听器
    app.once("initialization", move |_event| {
        // 额外等待一秒，让启动窗口优先显示
        sleep(Duration::from_secs(1));
        // 关闭等待窗口
        let app_async_splash_screen = app_listen.clone();
        tauri::async_runtime::spawn(async move {
            info!("Start closing the splash screen window");
            if let Some(window) = app_async_splash_screen.get_webview_window("splashscreen") {
                window.close().unwrap();
            }
            info!("The splash screen window closed successfully");
        });
        // 创建日历窗口
        let app_async_calendar = app_listen.clone();
        tauri::async_runtime::spawn(async move {
            // 创建日历窗口
            create_calendar_window(app_async_calendar).await;
        });
    });

    // 监听器创建成功
    let (task_manager, condvar) = app.state::<InitialTaskMangerType>().inner();
    task_manager.lock().unwrap().0 = true;
    condvar.notify_one();
}

pub async fn create_initialization_emit<R: Runtime>(app: AppHandle<R>) {
    let (task_manager, condvar) = app.state::<InitialTaskMangerType>().inner();
    let mut manager = task_manager.lock().unwrap();
    while !manager.0 {
        manager = condvar.wait(manager).unwrap();
    }
    app.emit("initialization", "").unwrap()
}

/**
 * @description: 时间轮算法，每隔5分钟触发一次事件
 * @author: illya 
 * @date: 2025/6/2 22:15
 **/
pub fn create_timing_wheel(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            // 获取当前时间的秒级时间戳
            let mut now = Utc::now().timestamp();
            info!("Current wheel: {}", now);
            if now % 300 != 0 {
                tokio::time::sleep(Duration::from_secs((300 - now % 300) as u64)).await;
                now = Utc::now().timestamp();
            }
            let app_async = app.clone();
            tauri::async_runtime::spawn(async move {
                info!("acquire reminder information");
                let start_timestamp = now * 1000;
                let end_timestamp = start_timestamp + 59999;
                let list = execute_service::acquire_execute_within(start_timestamp, end_timestamp).unwrap();
                info!("acquire reminder information successfully, list: {:?}", list);
                for value in list {
                    let note_table = note_service::acquire_note_by_key(&value.key).unwrap();
                    if let Some(note) = note_table {
                        app_async.notification().builder().title("simple-note").body(note.content).show().unwrap();
                        info!("The push message was successful. key: {}", note.key);
                    }
                }
            });
            // 暂停5分钟
            tokio::time::sleep(Duration::from_secs(300)).await;
        }
    });
}