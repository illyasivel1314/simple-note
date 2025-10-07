// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::backtrace::Backtrace;
use std::panic;
use std::panic::PanicHookInfo;
use std::process::exit;

/**
 * panic 回滚并且增加日志
 *
 * @author illya
 * @since 2025-10-04
 * @param 
 * @return 
 */
pub fn panic_handler(panic_info: &PanicHookInfo) {
    // 
    let thread = panic_info.location().unwrap();
    let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        *s
    } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
        s
    } else {
        "Box<Any>"
    };
    log::error!(
        "Panic occurred at {}:{}, {}\n{}",
        thread.file(),
        thread.line(),
        msg,
        Backtrace::force_capture()
    );
    exit(0);
}

#[tokio::main]
async fn main() {
    // 异常回滚
    panic::set_hook(Box::new(panic_handler));
    // 运行tauri程序
    simple_note_lib::run();
}
