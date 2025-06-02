// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::backtrace::Backtrace;
use std::panic;
use std::panic::PanicHookInfo;
use std::process::exit;
use tauri::AppHandle;

pub fn panic_handler(panic_info: &PanicHookInfo) {
    let thread = panic_info.location().unwrap();
    let msg = match panic_info.payload().downcast_ref::<&str>() {
        Some(s) => *s,
        None => "Box<Any>",
    };
    log::error!(
        "Panic occurred at {}, {}:{}\n{}",
        msg,
        thread.file(),
        thread.line(),
        Backtrace::force_capture()
    );
    exit(1);
}

fn main() {
    // 加载 .env 文件中的变量
    dotenv().ok();
    // 异常回滚
    panic::set_hook(Box::new(panic_handler));
    // 运行tauri程序
    simple_note_lib::run();
}
