use std::path::PathBuf;
use std::fs::read_dir;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

#[tauri::command]
pub fn list_files(app:AppHandle)->(){
    let dir_path_result = get_docs_path(app);
    let dir_path:String;
    match dir_path_result {
        Ok(path)=>{
            println!("{:?}",path);
            dir_path=path.to_str().unwrap().to_string()
        }
        Err(e)=>{
            panic!("{}",e)
        }
    }
    let val = read_dir(dir_path).unwrap();
    println!("val is {:?}",val.collect::<Vec<_>>());
    return;
}

fn get_docs_path(app: AppHandle) -> tauri::Result<PathBuf> {
    app.path().resolve("docs", BaseDirectory::AppData)
}

#[tauri::command]
pub fn create_new_file(app:AppHandle)->(){
    let docs_path = get_docs_path(app);
    let dir = read_dir(docs_path.unwrap()).unwrap();
    let mut collected_dir = dir.filter_map(|x| {
        let val = x.unwrap().file_name().to_str().unwrap().to_string();
        if val.contains("Untitled"){
            return Some(val);
        }
        return None
    }).collect::<Vec<_>>();
    collected_dir.sort();
    println!("collected_dir {:?}",collected_dir);
    return;
}