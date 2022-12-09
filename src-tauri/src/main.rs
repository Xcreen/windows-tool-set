#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::fs::File;
use std::io::Write;

const DEFAULT_HOST_FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[tauri::command]
fn read_host_file() -> String {
    let file_content = fs::read_to_string(DEFAULT_HOST_FILE_PATH);
    match file_content {
        Ok(val) => {
            return val;
        },
        Err(_err) => {
            return "".to_string();
        }
    }
}

#[tauri::command]
fn save_host_file(host_content: &str) -> bool {
    let save_result = save_content_host_file(host_content);
    match save_result {
        Ok(_val) => {
            return true;
        },
        Err(_err) => {
            return false;
        }
    }
}

fn save_content_host_file(content: &str) -> std::io::Result<()> {
    let mut f = File::options().write(true).truncate(true).open(DEFAULT_HOST_FILE_PATH)?;
    f.write_all(content.to_string().as_bytes())?;
    f.sync_data()?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_host_file, save_host_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
