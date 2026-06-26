use std::{fs, path::{Path}};

use crate::config;


pub fn resolve_home_dir(path: String) -> Option<String> {
    if path.starts_with('~') {
        let home_dir = std::env::home_dir();
        if home_dir.is_none() {
            return None;
        }
        return path.replace('~', home_dir.unwrap().to_str().unwrap()).into();
    }
    return path.into();
}

pub fn create_storage_if_not_exists() -> bool {
    let mut output_dir: String = config::read_config("ICON_OUTPUT_PATH");
    match resolve_home_dir(output_dir) {
        Some(dir) => {output_dir = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }

    let path : &Path = Path::new(&output_dir);
    if path.exists() {
        return true;       
    }
    println!("Creating icon storage directory: {}", output_dir);
    match fs::create_dir(path) {
        Ok(_) => {return true;}
        Err(e) => {eprintln!("Error creating icon storage directory: {}", e); return false;}
    }
}

pub fn copy_img_to_storage(icon_path: &str, app_id: &i32) -> bool {
    let mut output_dir: String = config::read_config("ICON_OUTPUT_PATH");
    match resolve_home_dir(output_dir) {
        Some(dir) => {output_dir = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }
    let from = Path::new(icon_path);
    if !from.exists() || !from.is_file() {
        return false;
    }
    let output_path_string: String = format!("{}/{}.jpg", output_dir, app_id);
    let to = Path::new(&output_path_string);
    if to.exists() {
        return true;
    }
    match fs::copy(from, to) {
        Ok(_) => {return true;}
        Err(e) => {eprintln!("Error copying file: {}", e); return false;}
    }
}