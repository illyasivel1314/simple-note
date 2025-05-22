// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::time::Duration;
use log::info;
use crate::controller::{calendar_controller, note_controller};
use tauri::{App, Manager, Runtime, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};
use tokio::task::spawn_blocking;
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
    std::thread::sleep(Duration::from_secs(3));

    let handle = app.handle();
    #[cfg(desktop)]
    {
        // 窗口定位
        handle.plugin(tauri_plugin_positioner::init())?;
        let window_handle = handle.clone();

        let windows = async move {
            // 加载窗口
            let splashscreen = window_handle.get_webview_window("splashscreen").unwrap();

            // 日历窗口
            let calendar_window = WebviewWindowBuilder::from_config(
                &window_handle, &window_handle.config().app.windows.get(0).unwrap().clone()).unwrap();
            if let Ok(window) = calendar_window.build() {
                // 关闭加载窗口
                splashscreen.close().unwrap();
                // 移至右上角
                window.as_ref().window().move_window(Position::TopRight).unwrap();
                // 显示
                window.show().unwrap();
            }
        };
        tokio::task::spawn(windows);
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
