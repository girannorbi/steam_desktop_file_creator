use std::{fs::{self, read_to_string}, path::{Path}};
use steam_vdf_parser::{self, Obj, Vdf, parse_text};
use regex::Regex;

use crate::files;

pub struct SteamApp {
    pub app_id: i32,
    pub name: String
}

pub fn get_steam_libraries(steam_path: &str) -> Vec<String> {
    let mut libraries: Vec<String> = Vec::new();
    let path_string : String = format!("{}/config/libraryfolders.vdf", steam_path);
    let file_content: String;
    match fs::read_to_string(path_string) {
        Ok(contents) => {
            file_content = contents.trim().replace("\n", "");
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            file_content = String::from("");
        }
    }
    if parse_text(&file_content).is_err(){
        eprintln!("Error parsing VDF: {}", parse_text(&file_content).unwrap_err());
        return libraries;
    }
    let vdf: Vdf = parse_text(&file_content).unwrap();
    let obj: &Obj = vdf.as_obj().unwrap();
    for library in obj.iter() {
        match library.1.get_str(&["path"]) {
            Some(_str) => {
                libraries.push(_str.to_string());
            }
            None => {
                eprintln!("Error: Library {} has no apps!", library.0);
                continue;
            }
        }
    }
    return libraries;
}

pub fn get_manifests_in_library(library_path: &str) -> Option<Vec<String>> {
    let mut manifests: Vec<String> = Vec::new();
    let path_string : String = format!("{}/steamapps", library_path);
    let path = Path::new(&path_string);
    if !path.exists() || !path.is_dir() {
        eprintln!("Library path does not exist: {}", library_path);
        return None;
    }
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                let path = entry.unwrap().path();
                if !path.is_file() { continue; }
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let regex = Regex::new(r"appmanifest_(\d+)\.acf").unwrap();
                let capture = regex.captures(file_name);
                if capture.is_none() {
                    continue;
                }
                manifests.push(path.to_str().unwrap().into());
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return None;
        }
    };
    return Some(manifests);
}

pub fn get_steam_app_data(manifest_path: &str) -> Option<SteamApp> {
    let path : &Path = Path::new(manifest_path);
    if !path.is_file() {
        eprintln!("Given manifest file path does not exists!");
        eprintln!("Path: {}", manifest_path);
        return None;
    }
    let file_content: String;
    match fs::read_to_string(path) {
        Ok(content ) => {
            file_content = content;
        }
        Err(err) => {
            eprintln!("Error reading steam manifest: {}", err);
            return None;
        }
    }
    let acf: Vdf;
    match parse_text(&file_content) {
        Ok(_acf) => {
            acf = _acf;
        }
        Err(err) => {
            eprintln!("Error parsing steam manifest file: {}", err);
            return None;
        }
    }
    if acf.as_obj().is_none() {
        eprintln!("Error: Steam manifest is invalid!");
        return None;
    }
    let obj: &Obj = acf.as_obj().unwrap();
    let mut app_id: i32 = -1;
    let mut app_name: String = String::from("");
    for pair in obj.iter() {
        if pair.0 == "appid" {
            app_id = pair.1.as_str().unwrap().parse::<i32>().unwrap_or(-1);
        }
        if pair.0 == "name" {
            app_name = pair.1.as_str().unwrap().into();
        }
    }
    if app_id == -1 || app_name == "" {
        eprintln!("Error: Couldnt find app_id or name in steam manifest file...");
        eprintln!("Path: {}", manifest_path);
        return None;
    }
    return Some(SteamApp { app_id: app_id, name: app_name });
}

pub fn get_icon_path(steam_path : &str, app_id: i32) -> Option<String> {
    let folder_path : String = format!("{}/appcache/librarycache/{}/", steam_path, app_id);
    if !Path::new(&folder_path).is_dir() {
        eprintln!("Folder not found: {}", folder_path);
        return None;
    }
    match fs::read_dir(folder_path) {
       Ok(readdir) => {
            for file in readdir {
                if file.is_err() {
                    eprintln!("Error reading file: {}", file.unwrap_err());
                    continue;
                }
                let file = file.unwrap();
                if file.file_name().len() != 44 {continue;}
                if !(file.file_name().to_str().unwrap().contains(".jpg")) {continue;}
                return Some(file.path().to_str().unwrap().to_string());
            }
        }
       Err(error) => {
            eprintln!("Error reading directory: {}", error);
            return None;
       }
    }
    return None;
}

fn gen_desktop_file_content(app_id: i32, app_name: &str, app_icon: &str) -> String {
    return format!("\
    [Desktop Entry]\n\
    Name={}\n\
    Comment=Play this game on Steam\n\
    Exec=steam steam://rungameid/{}\n\
    Icon={}\n\
    Terminal=false\n\
    Type=Application\n\
    Categories=Game;",
    app_name, app_id, app_icon);
}

pub fn create_desktop_file(app_id: &i32, app_name: &str, app_icon: &str) -> bool {
    let file_content = gen_desktop_file_content(*app_id, app_name, app_icon);
    let mut output_dir: String = crate::config::read_config("DESKTOP_OUTPUT_PATH");
    match crate::files::resolve_home_dir(output_dir) {
        Some(dir) => {output_dir = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }
    let file_path_string: String = format!("{}/steam_gen_{}.desktop", output_dir, *app_id);
    if desktop_file_is_in_storage(app_id) {
        println!("Skipping app {} - desktop file exists...", app_id);
        return true;
    }
    match fs::write(file_path_string, file_content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error writing desktop file: {}", e);
            return false;
        }
    }
    return true;
}

/// For some applications, steam can generate a working desktop file with the right icon and runner command.
/// The function checks if any desktop file in directory runs the app_id.
pub fn desktop_file_is_in_storage(app_id: &i32) -> bool {
    let mut output_dir: String = crate::config::read_config("DESKTOP_OUTPUT_PATH");
    match crate::files::resolve_home_dir(output_dir) {
        Some(dir) => {output_dir = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return false;}
    }
    match Path::new(&output_dir).read_dir() {
        Ok(file_entries) => {
            for file in file_entries{
                if file.is_err() { continue; }
                let file = file.unwrap();
                if file.path().is_dir() { continue; }
                match read_to_string(file.path()) {
                    Ok(str) => {
                        if str.contains(format!("steam://rungameid/{}", app_id).as_str()) {
                            return true;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading desktop file, skipping: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading desktop storage directory: {}", e);
            return false;
        }
    }


    return false;
}

/// If a game is uninstalled, the desktop files need to be removed.
/// Checks if there is a desktop file generated by this program that runs an app that is no longer installed on steam.
pub fn check_for_broken_dekstop_files(installed_apps: Vec<SteamApp>) -> () {
    let mut output_dir: String = crate::config::read_config("DESKTOP_OUTPUT_PATH");
    match crate::files::resolve_home_dir(output_dir) {
        Some(dir) => {output_dir = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path in your config."); return;}
    }
    match Path::new(&output_dir).read_dir() {
        Ok(dir_iterator) => {
            for dir_entry in dir_iterator {
                if dir_entry.is_err() {
                    continue;
                }
                let file: fs::DirEntry = dir_entry.unwrap();
                let file_name_os = file.file_name();
                let file_name: &str = file_name_os.to_str().unwrap();
                if file.path().is_dir() { continue; }
                if !file_name.starts_with("steam_gen_") { continue; }
                let capture = Regex::new("steam_gen_(.*)\\.desktop").unwrap().captures(file_name);
                if capture.is_none() { continue; }
                let capture = capture.unwrap();
                let app_id_str = capture.get(1).unwrap().as_str();
                let app_id: i32;
                match app_id_str.parse::<i32>() {
                    Ok(i) => {app_id = i;}
                    Err(_) => {continue;}
                }
                let mut contains = false;
                for app in installed_apps.iter() {
                    if app.app_id == app_id {
                        contains = true;
                        break;
                    }
                }
                if !contains {
                    println!("Removing broken desktop file for uninstalled app {}", app_id);
                    match fs::remove_file(file.path()) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(" - Error removing broken desktop file: {}", file_name);
                            eprintln!("Error: {}", e);
                        }
                    }
                    files::remove_icon_from_storage(&app_id);
                }
            }
        } 
        Err(e) => {
            eprintln!("Error reading desktop storage directory: {}", e);
            return;
        }
    }
}