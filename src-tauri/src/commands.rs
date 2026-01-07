use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::fs;
use std::fs::{read_dir, DirEntry};
use std::io::Write;
use std::path::{Path, PathBuf};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

#[derive(Default, Serialize, Deserialize)]
pub struct FileTree {
    is_dir: bool,
    path_from_docs: String,
    files: Vec<FileTree>,
    name: String,
}

fn build_file_tree(docs_path: &String) -> FileTree {
    recursive_file_tree_gen(docs_path, 0)
}

fn recursive_file_tree_gen(path: &String, depth: u8) -> FileTree {
    let mut file_tree = FileTree::default();
    let file_path = Path::new(path);
    if !file_path.is_dir() {
        file_tree.files = vec![];
        file_tree.name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        file_tree.path_from_docs = path.to_string();
        file_tree.is_dir = false;
        return file_tree;
    }
    let val = read_dir(file_path).unwrap();
    let mut val_sorted = val.map(|x1| x1.unwrap()).collect::<Vec<DirEntry>>();
    val_sorted.sort_by_key(|x2| x2.path());
    let val_collected = val_sorted
        .iter()
        .map(|x| {
            let tmp = depth + 1;
            return recursive_file_tree_gen(&x.path().to_str().unwrap().to_string(), tmp);
        })
        .collect::<Vec<_>>();
    file_tree.path_from_docs = path.to_string();
    file_tree.files = val_collected;
    file_tree.is_dir = true;
    file_tree.name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    file_tree
}

#[tauri::command]
pub fn list_files(app: AppHandle) -> FileTree {
    let dir_path_result = get_docs_path(&app);
    let dir_path: String;
    match dir_path_result {
        Ok(path) => {
            println!("{:?}", path);
            dir_path = path.to_str().unwrap().to_string()
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
    build_file_tree(&dir_path)
}

fn get_docs_path(app: &AppHandle) -> tauri::Result<PathBuf> {
    app.path().resolve("docs", BaseDirectory::AppData)
}

#[tauri::command]
pub fn create_new_file(app: AppHandle) -> () {
    let docs_path = get_docs_path(&app).unwrap();
    let dir = read_dir(&docs_path).unwrap();
    let mut collected_dir = dir
        .filter_map(|x| {
            let val = x.unwrap().file_name().to_str().unwrap().to_string();
            if val.contains("Untitled") {
                return Some(val);
            }
            return None;
        })
        .collect::<Vec<_>>();
    collected_dir.sort();
    println!("collected_dir {:?}", collected_dir);
    if collected_dir.len() <= 0 {
        let mut new_file_path = (&docs_path.clone()).to_str().unwrap().to_owned();
        new_file_path.push_str("/Untitled.md");
        fs::File::create(new_file_path).expect("TODO: panic message");
    } else {
        let val: i32 = find_highest_number_file(&collected_dir);
        let mut new_file_path = (&docs_path.clone()).to_str().unwrap().to_owned();
        new_file_path.push_str(&format!("/Untitled{}.md", val + 1));
        fs::File::create(new_file_path).expect("New file could not be created");
    }
}

fn path_in_docs(app: &AppHandle, path: &String) -> bool {
    let docs_path = get_docs_path(&app).expect("Could not resolve docs path while deleting");
    let file_path = Path::new(&path);
    file_path.starts_with(docs_path)
}

#[tauri::command]
pub fn get_file(app: AppHandle, path: String) -> String {
    let val = path_in_docs(&app, &path);
    println!("Is path in docs?{}",val);
    if val {
        println!("reached get doc");
        return fs::read_to_string(&path).unwrap();
    }
    "".to_string()
}

#[tauri::command]
pub fn set_file(app:AppHandle,path:String,value:String)->bool{
    let val = path_in_docs(&app,&path);
    if val {
        println!("path and val {},{}",path,value);
        let mut writer = fs::OpenOptions::new().write(true).truncate(true).open(path).expect("Failed to open file in write mode");
        writer.write_all(value.as_ref()).expect("Failed to write file");
        return true
    }
    false
}


#[tauri::command]
fn delete_file(app: AppHandle, path: String) -> bool {
    let file_path = Path::new(&path);
    if path_in_docs(&app, &path) {
        fs::remove_file(file_path).expect("File deletion error");
        return true;
    }
    false
}

mod tests {
    use crate::commands::find_highest_number_file;

    #[test]
    fn test1() {
        let collected_dir: Vec<String> = vec![
            "Untitled.md".to_string(),
            "Untitled1.md".to_string(),
            "Untitled1234.md".to_string(),
        ];
        let val: i32 = find_highest_number_file(&collected_dir);
        assert_eq!(val, 1234)
    }
}

fn find_highest_number_file(collected_dir: &Vec<String>) -> i32 {
    collected_dir.iter().fold(1, |b, x1| {
        let matcher = Regex::new(r"Untitled(\d+)\.md").unwrap();
        let matched = matcher.captures(x1);
        match matched {
            Some(caps) => {
                let temp = caps.get(1).unwrap();
                return max(temp.as_str().parse::<i32>().expect("Should be a number"), b);
            }
            _ => {}
        }
        return b;
    })
}
