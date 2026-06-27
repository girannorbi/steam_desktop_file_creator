use std::{fs::{self, read_to_string}, path::Path};
use serde_json::Value;
use steam_vdf_parser::{self, Obj, Vdf, parse_text};

pub fn read_installed_app_ids(steam_path : &str) -> Vec<i32> {
    let path_string : String = format!("{}/config/libraryfolders.vdf", steam_path);
    let path : &Path = Path::new(&path_string);
    if !path.exists() || !path.is_file() {
        eprintln!("Steam Config file not found! \n Path: {} \n Is it a file?", path_string);
        return Vec::new();
    }
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
    //println!("File content: {}", file_content);
    
    let vdf : Vdf;
    match parse_text(&file_content) {
        Ok(_vdf) => {
            vdf = _vdf;
        }
        Err(error) => {
            eprintln!("Error parsing VDF: {}", error);
            return Vec::new();
        }
    }
    let obj: &Obj;
    match vdf.as_obj() {
        Some(_obj) => {
            obj = _obj;
        }
        None => {
            eprintln!("Error: VDF is invalid!");
            return Vec::new();
        }
    }
    let mut installed_apps : Vec<i32> = Vec::new();
    for library in obj.iter() {
        match library.1.get_obj(&["apps"]) {
            Some(_obj) => {
                for app in _obj.iter() {
                    let app_id : i32 = app.0.to_string().parse::<i32>().unwrap_or(-1);
                    if app_id == -1 {
                        eprintln!("Error: Invalid app ID found!");
                        continue;
                    }
                    installed_apps.push(app_id);
                }
            }
            None => {
                eprintln!("Error: Library {} has no apps!", library.0);
                continue;
            }
        }
    }
    return installed_apps;
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

pub async fn fetch_app_name(app_id: i32) -> Option<String> {
    let url : String = format!("https://store.steampowered.com/api/appdetails?appids={}", app_id);
    let response = reqwest::get(url).await;
    if response.is_err() {
        eprintln!("Error fetching app data for ID {}: {}", app_id, response.unwrap_err());
        return None;
    }
    let response : String = response.unwrap().text().await.unwrap();
    let object : Value = serde_json::from_str(&response).unwrap();
    let found = object.get(app_id.to_string()).unwrap().get("success").unwrap().as_bool().unwrap();
    if !found {
        eprintln!("App ID {} not found in Steam API response.", app_id);
        return None;
    }
    let app_data = object.get(app_id.to_string()).unwrap().get("data").unwrap();
    let name = app_data.get("name").unwrap().as_str().unwrap();

    return Some(name.into());
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