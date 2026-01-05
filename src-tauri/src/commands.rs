use std::path::{Path, PathBuf};
use std::fs::{read_dir, DirEntry};
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
use serde::{Deserialize, Serialize};

#[derive(Default,Serialize,Deserialize)]
pub struct FileTree {
    is_dir:bool,
    path_from_docs:String,
    files:Vec<FileTree>,
    name:String
}

fn build_file_tree(docs_path:&String)->FileTree{
    recursive_file_tree_gen(docs_path,0)
}

fn recursive_file_tree_gen(path:&String,depth:u8)->FileTree{
    let mut file_tree = FileTree::default();
    let file_path = Path::new(path);
    if !file_path.is_dir() {
        file_tree.files = vec![];
        file_tree.name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        file_tree.path_from_docs = path.to_string();
        file_tree.is_dir=false;
        return file_tree;
    }
    let val = read_dir(file_path).unwrap();
    let mut val_sorted = val.map(|x1| {x1.unwrap()}).collect::<Vec<DirEntry>>();
    val_sorted.sort_by_key(|x2| {x2.path()});
    let val_collected = val_sorted.iter().map(|x| {
        let tmp = depth+1;
        return recursive_file_tree_gen(&x.path().to_str().unwrap().to_string(),tmp);
    }).collect::<Vec<_>>();
    file_tree.path_from_docs = path.to_string();
    file_tree.files = val_collected;
    file_tree.is_dir = true;
    file_tree.name =  file_path.file_name().unwrap().to_str().unwrap().to_string();
    file_tree
}

#[tauri::command]
pub fn list_files(app:AppHandle)->FileTree{
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
    build_file_tree(&dir_path)
}

fn get_docs_path(app: AppHandle) -> tauri::Result<PathBuf> {
    app.path().resolve("docs", BaseDirectory::AppData)
}

#[tauri::command]
pub fn create_new_file(app:AppHandle)->(){
    let docs_path = get_docs_path(app).unwrap();
    let dir = read_dir(&docs_path).unwrap();
    let mut collected_dir = dir.filter_map(|x| {
        let val = x.unwrap().file_name().to_str().unwrap().to_string();
        if val.contains("Untitled"){
            return Some(val);
        }
        return None
    }).collect::<Vec<_>>();
    collected_dir.sort();
    println!("collected_dir {:?}",collected_dir);
    if collected_dir.len() <= 0 {
        let mut new_file_path = (&docs_path.clone()).to_str().unwrap().to_owned();
        new_file_path.push_str("/Untitled.md");
        std::fs::File::create(new_file_path).expect("TODO: panic message");
    }
    // let new_file_name = Regex::new(r"\d").unwrap().find(collected_dir.iter().rev().next().unwrap()).unwrap().as_str();
    // println!("{}",new_file_name)
}