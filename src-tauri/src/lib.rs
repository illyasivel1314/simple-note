use std::sync::{Condvar, Mutex};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use log::info;
use crate::controller::{calendar_controller, note_controller};
use tauri::{App, WebviewWindowBuilder};
use tauri_plugin_positioner::WindowExt;
use crate::component::handler::InitialTaskManger;
use crate::component::listener::create_initialization_listener;
use crate::component::plugin::{tauri_plugin_database_init, tauri_plugin_log_init, tauri_plugin_single, tauri_plugin_tray};

mod configuration;
mod controller;
mod dao;
mod entity;
mod service;
mod component;

/**
 * @description:
 * @author: illya
 * @date: 2025/5/22 14:47
 **/
pub fn tauri_setup_init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("The program is being initialized");
    // 初始化内容
    // todo

    let handle = app.handle();
    #[cfg(desktop)]
    {
        // 注册监听器
        create_initialization_listener(handle.clone());
    }

    // 仅在调试构建时包含此代码
    // #[cfg(debug_assertions)]
    // {
    //     let window = app.get_webview_window("main").unwrap();
    //     window.open_devtools();
    //     window.close_devtools();
    // }
    info!("The program initialization is completed");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage((Mutex::new(InitialTaskManger::default()), Condvar::new()))
        .plugin(tauri_plugin_single_instance::init(tauri_plugin_single))    /* 单实例插件 */
        .plugin(tauri_plugin_positioner::init())                            /* 窗口定位插件 */
        .plugin(tauri_plugin_opener::init())                                /* 文件插件 */
        .plugin(tauri_plugin_log_init())                                    /* 日志插件 */
        .plugin(tauri_plugin_database_init())                               /* 自定义数据库插件 */
        .plugin(tauri_plugin_tray())                                        /* 自定义托盘插件 */
        .setup(tauri_setup_init)                                            /* 程序初始化插件 */
        .invoke_handler(tauri::generate_handler![
            calendar_controller::calendar_content,
            note_controller::save_note,
            note_controller::acquire_note,
            note_controller::delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[cfg(test)]
mod tests {
    use crate::controller;
    use crate::entity::note::NoteVO;
    use tauri::async_runtime::block_on;

    #[test]
    fn test_add() {
        let note = NoteVO {
            key: String::from("test"),
            content: String::from("test"),
            timestamp: 1747070428,
        };
        let value = block_on(controller::note_controller::save_note(note));
    }
}
