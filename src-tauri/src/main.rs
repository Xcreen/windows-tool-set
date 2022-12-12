#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mod_host_file;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, prelude::*, BufReader};
use mod_host_file::mod_host_file::*;

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
fn read_host_file_lines() -> Vec<String> {
    let mut lines = Vec::new();
    let buf_reader = get_host_file_bufreader();
    for line in buf_reader.lines() {
        lines.push(line.unwrap());
    }
    return lines;
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_host_file, read_host_file_lines, save_host_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
