use state::AppData;
use std::error::Error;
use std::fs::exists;
use tauri::path::BaseDirectory;
use tauri::{App, Manager};
use webdav_frontend_bindings::{get_webdav_notes_tree, init_webdav_with_creds};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod file_cache;
mod state;
pub mod tree;
pub mod webdav;
mod webdav_frontend_bindings;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            create_doc_if_not_exist(app).expect("TODO: panic message");
            app.manage(tokio::sync::Mutex::new(AppData::default()));
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![
            commands::list_files,
            commands::create_new_file,
            commands::get_file,
            commands::set_file,
            commands::delete_file,
            init_webdav_with_creds,
            get_webdav_notes_tree
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_doc_if_not_exist(app: &mut App) -> Result<(), Box<dyn Error>> {
    let docs_path = app.path().resolve("docs", BaseDirectory::AppData).unwrap();
    match exists(&docs_path) {
        Ok(t) => {
            match t {
                false => match std::fs::create_dir(&docs_path) {
                    Ok(_t) => {}
                    Err(e) => {
                        panic!("Could not create docs path, {:?}", e)
                    }
                },
                true => {}
            }
            Ok(())
        }
        Err(e) => {
            panic!("Docs path initialization error {:?}", e)
        }
    }
}
