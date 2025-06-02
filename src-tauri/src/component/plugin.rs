use crate::component::listener::create_initialization_emit;
use crate::configuration::database::data::calendar::update_holiday_to_database;
use crate::configuration::database::index::system_database_init;
use crate::configuration::utils::error_util::AppError;
use crate::configuration::utils::{file_util, time_util};
use log::info;
use tauri::menu::{Menu, MenuItem};
use tauri::plugin::Builder;
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

/**
 * @description: 日志插件
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
pub fn tauri_plugin_log_init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    info!("Tauri log plugin is being initialized");
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
                now, level, file, line, message
            ))
        })
        .build();
    info!("Tauri log plugin initialization is complete");
    plugin
}

/**
 * @description: 单实例插件
 * @author: illya
 * @date: 2025/5/16 16:26
 **/
pub fn tauri_plugin_single(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    info!("Tauri single plugin is being initialized");
    app.get_webview_window("calendar")
        .expect("no calendar window")
        .set_focus()
        .unwrap();
    info!("Tauri single plugin initialization is complete");
}

/**
 * @description: 第一次运行，初始化数据库插件
 * @author: illya
 * @date: 2025/5/22 14:53
 **/
pub fn tauri_plugin_database_init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    Builder::<R>::new("<database-init>")
        .setup(|app, _api| {
            database_init(app.clone())?;
            Ok(())
        })
        .build()
}

fn database_init<R: Runtime>(app: AppHandle<R>) -> Result<(), AppError> {
    info!("Tauri database plugin is being initialized");
    let database_url = file_util::acquire_database_url();
    // 判断是否初始化完成
    if file_util::file_valid(database_url.as_str()) {
        info!("Tauri database plugin don't need to run");
        tauri::async_runtime::spawn(async move {
            // 将任务保存至manage列表中
            create_initialization_emit(app).await;
        });
        return Ok(());
    }
    // 创建数据库
    system_database_init(database_url)?;
    tauri::async_runtime::spawn(async move {
        // 初始化数据
        update_holiday_to_database().unwrap();
        // 将任务保存至manage列表中
        create_initialization_emit(app).await;
    });
    info!("Tauri database plugin initialization is complete");
    Ok(())
}

/**
 * @description: 托盘插件
 * @author: illya
 * @date: 2025/5/22 15:15
 **/
pub fn tauri_plugin_tray<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    Builder::<R>::new("<tray-menu>")
        .setup(|app, _api| {
            create_tauri_tray_menu(app)?;
            Ok(())
        })
        .build()
}
/**
 * @description: 托盘图标以及菜单插件
 * @author: illya
 * @date: 2025/5/22 14:47
 **/
fn create_tauri_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Tauri tray plugin is being initialized");
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
        .on_tray_icon_event(|tray, event| {
            // 定位插件配置
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
            // 鼠标事件
            match event {
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
            }
        })
        .tooltip("简易便笺")
        .build(app)?;
    info!("Tauri tray plugin initialization is complete");
    Ok(())
}
