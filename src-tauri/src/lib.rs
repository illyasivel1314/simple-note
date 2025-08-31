use std::sync::{Condvar, Mutex};
use log::info;
use tauri::App;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::component::handler::InitialTaskManger;
use crate::component::listener::{create_initialization_listener, create_timing_wheel};
use crate::component::plugin::{
    tauri_plugin_database_init, tauri_plugin_log_init, tauri_plugin_single, tauri_plugin_tray,
};
use crate::controller::{calendar_controller, note_controller};

mod component;
mod configuration;
mod controller;
mod dao;
mod entity;
mod service;

/**
 * @description:
 * @author: illya
 * @date: 2025/5/22 14:47
 **/
pub fn tauri_setup_init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("The program is being initialized");

    let handle = app.handle();
    #[cfg(desktop)]
    {
        // 注册监听器
        create_initialization_listener(handle.clone());
        // 注册循环事件
        create_timing_wheel(handle.clone());
    }

    info!("The program initialization is completed");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage((Mutex::new(InitialTaskManger::default()), Condvar::new()))
        .plugin(tauri_plugin_notification::init())                          /* 通知插件 */
        .plugin(tauri_plugin_os::init())                                    /* 系统信息插件 */
        .plugin(tauri_plugin_single_instance::init(tauri_plugin_single))    /* 单实例插件 */
        .plugin(tauri_plugin_positioner::init())                            /* 窗口定位插件 */
        .plugin(tauri_plugin_opener::init())                                /* 文件插件 */
        .plugin(tauri_plugin_log_init())                                    /* 日志插件 */
        .plugin(tauri_plugin_database_init())                               /* 自定义数据库插件 */
        .plugin(tauri_plugin_tray())                                        /* 自定义托盘插件 */
        .plugin(tauri_plugin_prevent_default::init())                       /* 阻止默认浏览器快捷键 */
        .setup(tauri_setup_init)                                            /* 程序初始化插件 */
        .invoke_handler(tauri::generate_handler![
            calendar_controller::calendar_content,
            note_controller::save_note,
            note_controller::acquire_note,
            note_controller::delete_note,
            note_controller::update_note_finished,
            note_controller::update_note_setting,
            note_controller::acquire_note_by_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
