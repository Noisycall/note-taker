use std::fs::exists;
use tauri::Manager;
use tauri::path::BaseDirectory;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            let docs_path = app.path().resolve("docs",BaseDirectory::AppData).unwrap();
            match exists(&docs_path) {
                Ok(t)=>{
                    match t {
                        false =>{
                           match std::fs::create_dir(&docs_path) {
                               Ok(_t)=>{}
                               Err(e)=>{
                                   panic!("Could not create docs path, {:?}",e)
                               }
                           }
                        }
                        true => {}
                    }
                    Ok(())
                }
                Err(e)=>{
                    panic!("Docs path initialization error {:?}",e)
                }

            }
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![commands::list_files,commands::create_new_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
