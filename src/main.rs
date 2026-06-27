use std::path::Path;

use crate::{config::{create_default_config_if_not_exists, print_invalid_config_message}, steam::fetch_app_name};

mod config;
mod steam;
mod files;

fn main() {
    create_default_config_if_not_exists();
    let mut steam_path : String = config::read_config("STEAM_PATH");
    match files::resolve_home_dir(steam_path) {
        Some(dir) => {steam_path = dir;}
        None => {eprintln!("Error resolving home directory! Try to use absolute path."); return;}
    }
    if !Path::new(&steam_path).exists() {
        print_invalid_config_message();
    }
    let apps : Vec<i32> = steam::read_installed_app_ids(&steam_path);
    files::create_storage_if_not_exists();
    println!("Installed Apps:");
    for app_id in apps {
        let cached_app_icon : String = steam::get_icon_path(&steam_path, app_id).unwrap_or("Icon not found".into());
        let app_name: String = trpl::block_on(async {
            match fetch_app_name(app_id).await {
                Some(app_name) => app_name,
                None => {
                    println!("Failed to fetch app data for App ID: {}", app_id);
                    "Unknown".into()
                }
            }
        });
        if cached_app_icon == "Icon not found" || app_name == "Unknown" {
            continue;
        }
        println!("{} - {}", app_name, cached_app_icon);
        files::copy_img_to_storage(&cached_app_icon, &app_id);
        let app_icon: String;
        match files::get_icon_in_storage(&app_id) {
            Some(icon_path) => {app_icon = icon_path;}
            None => {eprintln!("Error getting icon path in storage!"); continue;}
        }
        steam::create_desktop_file(&app_id, &app_name, &app_icon);
    }    
}