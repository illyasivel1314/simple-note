use log::info;
use tauri::{AppHandle, WebviewWindow};

/**
 * @description: 通过配置文件创建窗口
 * @author: illya
 * @date: 2025/5/23 16:21
 **/
pub async fn create_window_by_config(app: AppHandle, window_label: &str) -> WebviewWindow{
    info!("Creating window: {}", window_label);
    let config = app.config();
    if let Some(window_config) = config.app.windows.iter().find(|config| config.label == window_label) {
        let builder = tauri::WebviewWindowBuilder::from_config(&app, window_config).unwrap();
        let window = builder.build().unwrap();
        info!("Window already created: {}", window_label);
        return window;
    }
    panic!("Could not find window: {}", window_label); 
}