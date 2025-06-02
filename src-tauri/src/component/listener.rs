use crate::component::handler::{InitialTaskManger, InitialTaskMangerType};
use crate::component::windows::{create_calendar_window, create_window_by_config};
use log::info;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;
use diesel::IntoSql;
use tauri::async_runtime::Runtime::Tokio;
use tauri::{AppHandle, Emitter, Listener, Manager, Runtime};
use tauri_plugin_positioner::{Position, WindowExt};

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