// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn show(os: &str) -> String {
    let mut com = "";
    let mut arg = "";
    let mut _output = String::new();
    if os.contains("Win") {
        com = "start";
        arg = "C:\\Program Files (x86)\\Tencent\\WeChat\\WeChat.exe";
    } else if os.contains("Mac") {
        com = "nohup";
        arg = "/Applications/WeChat.app/Contents/MacOS/WeChat";
    } else {
        _output = format!("{}$$运行失败,不支持的系统", os);
    }

    let result = Command::new(com).arg(arg).spawn(); // 运行命令
    match result {
        Ok(_) => {
            let _ = Command::new(com).arg(arg).spawn();
            _output = format!("{}$$运行成功", os);
        }
        Err(_) => _output = format!("{}$$运行失败", os),
    }
    _output
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
