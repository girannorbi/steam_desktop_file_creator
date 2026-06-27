use std::{fs, path::{Path}};

fn _config_path() -> String {
    let home = std::env::home_dir().unwrap();
    let config_path = format!("{}/.config/steam-desktop-file-gen.cfg", home.to_str().unwrap());
    return config_path;
}

pub fn create_default_config_if_not_exists() -> (){
    let config_path = _config_path();
    if Path::new(&config_path).exists() {
        return;
    }
    let config_content = "\
    STEAM_PATH=~/.local/share/Steam\n\
    ICON_OUTPUT_PATH=~/.local/share/steamapp_desktop_gen/icons/\n\
    DESKTOP_OUTPUT_PATH=~/.local/share/steamapp_desktop_gen/desktop/\n";
    match fs::write(config_path, config_content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error writing config file to $HOME/.config/steam-desktop-file-gen.cfg: {}", e);
        }
    }    
}

pub fn read_config(key : &str) -> String{
    let file_content: String;
    match fs::read_to_string(_config_path()) {
        Ok(contents) => {
            file_content = contents;
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            print_invalid_config_message();
            return String::from("");
        }
    }
    let lines = file_content.split("\n");
    for line in lines {
        if line.starts_with('#') {
            continue;
        }
        let mut _key: String = String::from("");
        let mut key_ended : bool = false;
        let mut val: String = String::from("");
        for char in line.chars() {
            if char == '=' {
                key_ended = true;
                continue;        
            } else {
                if key_ended {
                    val.push(char);
                } else {
                    _key.push(char);
                }
            }
        }
        if !key_ended {
            eprintln!("Error reading config file!");
            return String::from("");
        }
        if key == _key.trim() {
            return val.trim().to_string();
        }
    }
    return String::from("");
}

pub fn print_invalid_config_message() {
    eprintln!("Invalid config file! Please check your $HOME/.config/steam-desktop-file-gen.cfg file, make sure the set configs are valid and the paths exist.");
}