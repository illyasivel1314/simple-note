use log::info;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_positioner::{Position, WindowExt};

/**
 * @description: 通过配置文件创建窗口
 * @author: illya
 * @date: 2025/5/23 16:21
 **/
pub async fn create_window_by_config(app: AppHandle, window_label: &str) -> WebviewWindow {
    info!("Creating window: {}", window_label);
    let config = app.config();
    if let Some(window_config) = config
        .app
        .windows
        .iter()
        .find(|config| config.label == window_label)
    {
        let builder = tauri::WebviewWindowBuilder::from_config(&app, window_config).unwrap();
        let window = builder.build().unwrap();
        info!("Window already created: {}", window_label);
        return window;
    }
    panic!("Could not find window: {}", window_label);
}

/**
 * @description: 创建日历窗口
 * @author: illya
 * @date: 2025/5/25 19:13
 **/
pub async fn create_calendar_window(app: AppHandle) -> WebviewWindow {
    info!("Start creating the splash screen window");
    let window = create_window_by_config(app, "calendar").await;
    window.move_window(Position::TopRight).unwrap(); // 定位到右上角
    window.set_always_on_bottom(true).unwrap(); // 永远在最低下
    window.show().unwrap();
    info!("The splash screen window creating successfully.");
    // window.open_devtools();
    window
}
