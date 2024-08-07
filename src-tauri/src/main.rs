// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    io,
    process::{Child, Command},
};

fn run_in_win(arg: &str) -> Result<Child, io::Error> {
    // Command::new("cmd").args(&["/C", com, ""]).arg(arg).output()
    Command::new(arg).spawn()
}

fn run_in_mac(com: &str, arg: &str) -> Result<Child, io::Error> {
    Command::new(com).arg(arg).spawn()
}

fn run(com: &str, arg: &str, os: &str) -> Result<Child, io::Error> {
    if os == "Win" {
        run_in_win(arg)
    } else {
        run_in_mac(com, arg)
    }
}

#[tauri::command]
fn doit(os: &str, num: u8) -> String {
    let mut com = "";
    let mut arg = "";
    let mut os = os;
    let mut _output = String::new();
    if os.contains("Win") {
        os = "Win";
        com = "start";
        arg = "C:\\Program Files (x86)\\Tencent\\WeChat\\WeChat.exe";
    } else if os.contains("Mac") {
        os = "Mac";
        com = "nohup";
        arg = "/Applications/WeChat.app/Contents/MacOS/WeChat";
    } else {
        _output = format!("{}$$运行失败,不支持的系统", os);
    }

    let result = run(com, arg, os);
    // 运行命令
    match result {
        Ok(_) => {
            for _ in 1..num {
                let _ = run(com, arg, os);
            }
            _output = format!("运行成功, 请稍等片刻");
        }
        Err(e) => _output = format!("{}$$运行失败{}", os, e),
    }
    _output
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![doit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
