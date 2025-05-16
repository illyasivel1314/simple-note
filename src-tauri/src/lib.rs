// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::configuration::database::interface::{DatabaseImpl, DatabaseUtil};
use crate::controller::{calendar_controller, note_controller};
use configuration::utils::time_util;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager, Runtime, Wry};

mod configuration;
mod controller;
mod dao;
mod entity;
mod service;

fn create_tauri_tray_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_menu = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_menu])?;
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false) /* 取消左键菜单功能 */
        .on_menu_event(move |app, event| match event.id.as_ref() {
            /* 菜单功能 */
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click { .. } => {}
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            TrayIconEvent::Enter { .. } => {}
            TrayIconEvent::Move { .. } => {}
            TrayIconEvent::Leave { .. } => {}
            _ => {}
        })
        .tooltip("简易便笺")
        .build(app.handle())?;
    Ok(())
}

fn tauri_setup_init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 应用初始化
    DatabaseUtil::default().system_database_init()?;

    #[cfg(desktop)]
    {
        // 窗口定位
        app.handle().plugin(tauri_plugin_positioner::init())?;
        // 创建托盘图标
        create_tauri_tray_menu(app)?;
    }

    // 仅在调试构建时包含此代码
    // #[cfg(debug_assertions)]
    // {
    //     let window = app.get_webview_window("main").unwrap();
    //     window.open_devtools();
    //     window.close_devtools();
    // }
    Ok(())
}

/**
 * @description: 日志插件
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
fn tauri_plugin_log_init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: std::path::PathBuf::from("./resources/logs"),
                file_name: None,
            },
        ))
        .max_file_size(50_000)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        .format(|out, message, record| {
            // 当前时间
            let now = time_util::acquire_now_datetime();
            // 等级
            let level = record.level();
            // 文件地址
            let file = record.file().unwrap_or("unknown");
            // 所在行数
            let line = record.line().unwrap_or(0);
            out.finish(format_args!(
                "{} [{}] [{}:{}] {}",
                now,
                record.level(),
                file,
                line,
                message
            ))
        })
        .build()
}

/**
 * @description: 单实例
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
fn tauri_plugin_single(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    app.get_webview_window("main")
        .expect("no main window")
        .set_focus()
        .unwrap();
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(tauri_plugin_single))
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(tauri_setup_init)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log_init())
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
