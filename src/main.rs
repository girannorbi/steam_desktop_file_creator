use std::path::Path;
mod config;
mod steam;
mod files;

fn main() {
    config::create_default_config_if_not_exists();
    files::create_storage_if_not_exists();
    let mut steam_path : String = config::read_config("STEAM_PATH");
    match files::resolve_home_dir(steam_path) {
        Some(dir) => {steam_path = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return;}
    }
    if !Path::new(&steam_path).exists() {
        config::print_invalid_config_message();
    }
    println!("STEAM_PATH: {}", steam_path);

    let mut steam_apps: Vec<steam::SteamApp> = Vec::new();
    let libraries: Vec<String> = steam::get_steam_libraries(&steam_path);
    let mut manifests : Vec<String> = Vec::new();
    for lib in libraries {
        for manifest_path in steam::get_manifests_in_library(&lib).unwrap_or(Vec::new()) {
            manifests.push(manifest_path);
        }
    }
    for manifest_path in manifests {
        let app_data = steam::get_steam_app_data(&manifest_path);
        if app_data.is_none() { continue; }
        steam_apps.push(app_data.unwrap());
    }
    for app in steam_apps.iter() {
        println!("--- App: {} || {} ---", app.app_id, app.name);
        if steam::desktop_file_is_in_storage(&app.app_id) { 
            println!("Desktop file already exists, skipping...");
            continue; 
        }
        let app_icon_path: String;
        if files::get_icon_in_storage(&app.app_id).is_none() {
            println!("Copying app icon to storage...");
            let cached_app_icon_path: String;
            match steam::get_icon_path(&steam_path, app.app_id) {
                Some(path) => { cached_app_icon_path = path; }
                None => { 
                    eprintln!("Error getting icon path for App ID: {}", app.app_id);
                    continue;
                }
            }
            if !files::copy_img_to_storage(&cached_app_icon_path, &app.app_id) {
                eprintln!("Error copying icon to storage for App ID: {}", app.app_id);
                continue;
            }
        }
        app_icon_path = files::get_icon_in_storage(&app.app_id).unwrap();
        if !steam::create_desktop_file(&app.app_id, &app.name, &app_icon_path){
            eprintln!("Error creating desktop file for App {}", app.app_id);
        }
    }
    println!("Checking for broken desktop files...");
    steam::check_for_broken_dekstop_files(steam_apps);
    return;
}