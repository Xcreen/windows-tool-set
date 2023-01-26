#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mod_host_file;
mod mod_registry;

use std::io::{prelude::*};

#[tauri::command]
fn read_host_file() -> String {
    return mod_host_file::read_host_file_content();
}

#[tauri::command]
fn read_host_file_lines() -> Vec<String> {
    let mut lines = Vec::new();
    let buf_reader = mod_host_file::get_host_file_bufreader();
    for line in buf_reader.lines() {
        lines.push(line.unwrap());
    }
    return lines;
}

#[tauri::command]
fn save_host_file(host_content: &str) -> bool {
    let save_result = mod_host_file::save_content_host_file(host_content);
    match save_result {
        Ok(_val) => {
            return true;
        },
        Err(_err) => {
            return false;
        }
    }
}


#[tauri::command]
fn append_entry_to_host_file(new_line: &str) -> bool {
    let new_host_content = read_host_file() + "\n" + new_line;
    return save_host_file(new_host_content.as_str());
}

#[tauri::command]
fn delete_entry_from_host_file(delete_line: i32) -> bool {
    let mut lines = Vec::new();
    let mut line_counter = 1;
    let mut new_host_content = String::new();
    let buf_reader = mod_host_file::get_host_file_bufreader();
    for line in buf_reader.lines() {
        if line_counter != delete_line {
            lines.push(line.unwrap() + "\n");
        }
        line_counter += 1;
    }
    for line in lines {
        new_host_content.push_str(line.as_str());
    }

    return save_host_file(new_host_content.as_str());
}

#[tauri::command]
fn edit_host_line(line_no: usize, new_host_entry: &str) -> bool {
    let save_result = mod_host_file::edit_line(line_no, new_host_entry);
    match save_result {
        Ok(..) => {
            return true;
        },
        Err(_err) => {
            return false;
        }
    }
}

#[tauri::command]
fn get_user_variables() -> Vec<String> {
    return mod_registry::get_user_variables();
}

#[tauri::command]
fn save_user_variables(user_path: String) -> bool {
    let save_result = mod_registry::save_user_variables(user_path);
    match save_result {
        Ok(..) => {
            return true;
        },
        Err(_err) => {
            println!("{}", _err);
            return false;
        }
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_host_file,
            read_host_file_lines,
            save_host_file,
            append_entry_to_host_file,
            delete_entry_from_host_file,
            edit_host_line,
            get_user_variables,
            save_user_variables
        ]).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
