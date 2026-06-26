use std::{fs};

pub fn read_config(key : &str) -> String{
    let file_content: String;
    match fs::read_to_string("config.cfg") {
        Ok(contents) => {
            file_content = contents;
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return String::from("");
        }
    }
    let lines = file_content.split("\n");
    for line in lines {
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