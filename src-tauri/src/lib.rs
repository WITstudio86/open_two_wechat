use std::{
    io,
    process::{Child, Command},
};

#[tauri::command]
fn doit(num: u8) -> String {
    let mut com = "";
    let mut arg = "";
    let mut os = tauri_plugin_os::platform();
    let mut _output = String::new();
    if os.contains("window") {
        os = "windows";
        com = "start";
        arg = "C:\\Program Files (x86)\\Tencent\\WeChat\\WeChat.exe";
    } else if os.contains("macos") {
        os = "macos";
        com = "nohup";
        arg = "/Applications/WeChat.app/Contents/MacOS/WeChat";
    } else {
        _output = format!("{}$$运行失败,不支持的系统", os);
    }

    let result = run_(com, arg, os);
    // 运行命令
    match result {
        Ok(_) => {
            for _ in 1..num {
                let _ = run_(com, arg, os);
            }
            _output = format!("运行成功, 请稍等片刻");
        }
        Err(e) => _output = format!("{}$$运行失败{}", os, e),
    }
    _output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![doit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn run_in_win(arg: &str) -> Result<Child, io::Error> {
    // Command::new("cmd").args(&["/C", com, ""]).arg(arg).output()
    Command::new(arg).spawn()
}

fn run_in_mac(com: &str, arg: &str) -> Result<Child, io::Error> {
    Command::new(com).arg(arg).spawn()
}

fn run_(com: &str, arg: &str, os: &str) -> Result<Child, io::Error> {
    if os == "Win" {
        run_in_win(arg)
    } else {
        run_in_mac(com, arg)
    }
}
