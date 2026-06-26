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
    let mut dir_to_create: String = config::read_config("ICON_OUTPUT_PATH");
    match resolve_home_dir(dir_to_create) {
        Some(dir) => {dir_to_create = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }
    let path : &Path = Path::new(&dir_to_create);
    if !path.exists() {
        println!("Creating icon storage directory: {}", dir_to_create);
        match fs::create_dir_all(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error creating icon storage directory: {}", e);
                return false;
            }
        }
    }
    let mut dir_to_create: String = config::read_config("DESKTOP_OUTPUT_PATH");
    match resolve_home_dir(dir_to_create) {
        Some(dir) => {dir_to_create = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }
    let path: &Path = Path::new(&dir_to_create);
    if path.exists() {
        return true;
    }
    println!("Creating desktop storage directory: {}", dir_to_create);
    match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error creating desktop storage directory: {}", e);
            return false;
        }
    }
    return true;
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
    let output_path_string: String = format!("{}/{}.png", output_dir, app_id);
    let to = Path::new(&output_path_string);
    if to.exists() {
        return true;
    }
    match fs::copy(from, to) {
        Ok(_) => {return true;}
        Err(e) => {eprintln!("Error copying file: {}", e); return false;}
    }
}

pub fn get_icon_in_storage(app_id: &i32) -> Option<String> {
    let mut storage_path_str: String = config::read_config("ICON_OUTPUT_PATH");
    match resolve_home_dir(storage_path_str) {
        Some(dir) => {storage_path_str = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return None;}
    }
    let storage_path : &Path = Path::new(&storage_path_str);
    if !storage_path.exists() || !storage_path.is_dir() {
        return None;
    }
    let icon_path_string: String = format!("{}{}.png", storage_path_str, app_id);
    let icon_path : &Path = Path::new(&icon_path_string);
    if !icon_path.exists() || !icon_path.is_file() {
        return None;
    }
    return Some(icon_path_string);
}