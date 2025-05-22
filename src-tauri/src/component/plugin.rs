use log::info;
use tauri::{App, AppHandle, Manager, Runtime};
use tauri::menu::{Menu, MenuItem};
use tauri::plugin::Builder;
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use crate::configuration::database::data::calendar::update_holiday_to_database;
use crate::configuration::database::index::system_database_init;
use crate::configuration::utils::{file_util, time_util};

/**
 * @description: 日志插件
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
pub fn tauri_plugin_log_init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    info!("Tauri plugin single is being initialized");
    let plugin = tauri_plugin_log::Builder::new()
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
        .build();
    info!("Tauri plugin single initialization is complete");
    plugin
}

/**
 * @description: 单实例插件
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
pub fn tauri_plugin_single(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    info!("Tauri plugin single is being initialized");
    app.get_webview_window("main")
        .expect("no main window")
        .set_focus()
        .unwrap();
    info!("Tauri plugin single initialization is complete");
}

/**
 * @description: 第一次运行，初始化数据库插件
 * @author: illya 
 * @date: 2025/5/22 14:53
 **/
pub fn tauri_plugin_database_init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    Builder::<R>::new("<database-init>")
        .setup(|app, api| {
            info!("Tauri plugin database is being initialized");
            let database_url = file_util::acquire_database_url();
            // 判断是否初始化完成
            if file_util::file_valid(database_url.as_str()) {
                return Ok(());
            }
            // 创建数据库
            system_database_init(database_url)?;
            // 初始化数据
            update_holiday_to_database()?;
            info!("Tauri plugin database initialization is complete");
            Ok(())
        })
        .build()
}

/**
 * @description: 托盘插件
 * @author: illya 
 * @date: 2025/5/22 15:15
 **/
pub fn tauri_plugin_tray<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    Builder::<R>::new("<tray-menu>")
        .setup(|app, _api| {
            info!("Tauri plugin tray is being initialized");
            create_tauri_tray_menu(app)?;
            info!("Tauri plugin tray initialization is complete");
            Ok(())
        })
        .build()
}
/**
 * @description: 托盘图标以及菜单插件
 * @author: illya 
 * @date: 2025/5/22 14:47
 **/
fn create_tauri_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
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
        .build(app)?;
    Ok(())
}

